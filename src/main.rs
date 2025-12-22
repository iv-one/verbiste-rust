#![deny(warnings)]
mod handlers;
mod template;
mod verbs;

use clap::Parser;
use log::{error, info};
use rust_embed::RustEmbed;
use std::net::IpAddr;
use std::sync::Arc;
use warp::Filter;

// Embed XML data files into the binary at compile time
const VERBS_XML: &str = include_str!("../data/verbs-fr.xml");
const CONJUGATION_XML: &str = include_str!("../data/conjugation-fr.xml");

// Embed public directory into binary at compile time
#[derive(RustEmbed)]
#[folder = "public"]
struct Asset;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Host address to bind to
    #[arg(long, default_value = "0.0.0.0")]
    host: String,

    /// Port to bind to
    #[arg(long, default_value_t = 3030)]
    port: u16,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let args = Args::parse();

    // Load all verbs from embedded data
    info!("Loading verbs from embedded data...");
    let verbs = match verbs::load_all_verbs(VERBS_XML) {
        Ok(v) => {
            info!("Loaded {} verbs", v.len());
            Arc::new(v)
        }
        Err(e) => {
            error!("Failed to load verbs: {}", e);
            std::process::exit(1);
        }
    };

    // Load all templates from embedded data
    info!("Loading templates from embedded data...");
    let templates = match template::load_all_templates(CONJUGATION_XML) {
        Ok(t) => {
            info!("Loaded {} templates", t.len());
            Arc::new(t)
        }
        Err(e) => {
            error!("Failed to load templates: {}", e);
            std::process::exit(1);
        }
    };

    // Configure CORS to allow all origins and GET method
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET"])
        .allow_headers(vec!["content-type"]);

    // Clone Arc for use in closures
    let verbs_for_verb_handler = verbs.clone();
    let verbs_for_search_handler = verbs.clone();
    let templates_for_template_handler = templates.clone();

    // API routes with /api prefix
    let api_verb_route = warp::path("api")
        .and(warp::path("verb"))
        .and(warp::path::param::<String>())
        .and(warp::get())
        .and_then(move |verb_name: String| {
            let verbs = verbs_for_verb_handler.clone();
            async move { handlers::get_verb_handler(verb_name, verbs).await }
        });

    let api_template_route = warp::path("api")
        .and(warp::path("t"))
        .and(warp::path::param::<String>())
        .and(warp::get())
        .and_then(move |template_name: String| {
            let templates = templates_for_template_handler.clone();
            async move { handlers::get_template_handler(template_name, templates).await }
        });

    let api_search_route = warp::path("api")
        .and(warp::path("search"))
        .and(warp::query::<std::collections::HashMap<String, String>>())
        .and(warp::get())
        .and_then(move |params: std::collections::HashMap<String, String>| {
            let verbs = verbs_for_search_handler.clone();
            async move {
                let query = params.get("q").cloned().unwrap_or_default();
                handlers::search_verbs_handler(query, verbs).await
            }
        });

    // Combine API routes
    let api_routes = api_verb_route.or(api_template_route).or(api_search_route);

    // Serve static files from embedded public directory
    let static_files = warp::path::tail().and_then(|path: warp::path::Tail| {
        let path_str = path.as_str().to_string();
        async move {
            if let Some(file) = Asset::get(&path_str) {
                let mime = mime_guess::from_path(&path_str).first_or_octet_stream();
                let mut response = warp::reply::Response::new(file.data.to_vec().into());
                response.headers_mut().insert(
                    "content-type",
                    warp::http::HeaderValue::from_str(mime.as_ref()).unwrap(),
                );
                Ok::<_, warp::Rejection>(response)
            } else {
                Err(warp::reject::not_found())
            }
        }
    });

    // Serve index.html for SPA routing (fallback for any non-API route)
    let index = warp::get().and_then(|| async move {
        if let Some(file) = Asset::get("index.html") {
            let mut response = warp::reply::Response::new(file.data.to_vec().into());
            response.headers_mut().insert(
                "content-type",
                warp::http::HeaderValue::from_str("text/html").unwrap(),
            );
            Ok::<_, warp::Rejection>(response)
        } else {
            Err(warp::reject::not_found())
        }
    });

    // Combine all routes: API first, then static files, then index.html for SPA
    // The order matters: API routes have highest priority, then static files, then index.html
    let routes = api_routes.or(static_files).or(index).with(cors);

    // Parse host address
    let host: IpAddr = args.host.parse().unwrap_or_else(|_| {
        error!("Invalid host address: {}", args.host);
        std::process::exit(1);
    });

    let addr = (host, args.port);
    info!("Starting server on {}:{}", host, args.port);

    warp::serve(routes).run(addr).await;
}
