#![deny(warnings)]
mod handlers;
mod template;
mod verbs;

use log::{error, info};
use std::sync::Arc;
use warp::Filter;

// Embed XML data files into the binary at compile time
const VERBS_XML: &str = include_str!("../data/verbs-fr.xml");
const CONJUGATION_XML: &str = include_str!("../data/conjugation-fr.xml");

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

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

    let api_hello_route = warp::path("api")
        .and(warp::path("hello"))
        .and(warp::get())
        .and_then(|| async { handlers::hello_handler().await });

    // Combine API routes
    let api_routes = api_verb_route
        .or(api_template_route)
        .or(api_search_route)
        .or(api_hello_route);

    // Serve static files from public directory
    let static_files = warp::fs::dir("public");

    // Serve index.html for SPA routing (fallback for any non-API route)
    let index = warp::get().and(warp::fs::file("public/index.html"));

    // Combine all routes: API first, then static files, then index.html for SPA
    // The order matters: API routes have highest priority, then static files, then index.html
    let routes = api_routes.or(static_files).or(index).with(cors);

    info!("start server");

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
