#![deny(warnings)]
mod handlers;
mod template;
mod verbs;

use log::{error, info};
use std::sync::Arc;
use warp::Filter;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // Load all verbs on startup
    info!("Loading verbs from data/verbs-fr.xml...");
    let verbs = match verbs::load_all_verbs() {
        Ok(v) => {
            info!("Loaded {} verbs", v.len());
            Arc::new(v)
        }
        Err(e) => {
            error!("Failed to load verbs: {}", e);
            std::process::exit(1);
        }
    };

    // Load all templates on startup
    info!("Loading templates from data/conjugation-fr.xml...");
    let templates = match template::load_all_templates() {
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

    // 404 handler for unmatched routes
    let not_found = warp::any().and_then(|| async { handlers::not_found_handler().await });

    // Combine all routes with 404 fallback
    let routes = api_routes.or(not_found).with(cors);

    info!("start server");

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
