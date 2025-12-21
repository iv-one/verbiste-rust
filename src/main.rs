#![deny(warnings)]
mod verbs;

use log::{error, info};
use serde::Serialize;
use std::sync::Arc;
use warp::{Filter, Rejection, Reply};

async fn get_verb_handler(
    verb_name: String,
    verbs: Arc<Vec<verbs::Verb>>,
) -> Result<warp::reply::Response, Rejection> {
    match verbs.binary_search_by(|v| v.verb.cmp(&verb_name)) {
        Ok(index) => Ok(warp::reply::json(&verbs[index]).into_response()),
        Err(_) => {
            Ok(warp::reply::with_status("", warp::http::StatusCode::NOT_FOUND).into_response())
        }
    }
}

#[derive(Serialize)]
struct HelloWorld {
    message: String,
}

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

    // Configure CORS to allow all origins and GET method
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET"])
        .allow_headers(vec!["content-type"]);

    // Clone Arc for use in closures
    let verbs_for_verb_handler = verbs.clone();

    // Handler for /verb/:verb endpoint
    let verb_route = warp::path("verb")
        .and(warp::path::param::<String>())
        .and(warp::get())
        .and_then(move |verb_name: String| {
            let verbs = verbs_for_verb_handler.clone();
            async move { get_verb_handler(verb_name, verbs).await }
        });

    // Match any request and return hello world as JSON!
    let hello_route = warp::any().map(|| {
        let hello = HelloWorld {
            message: "Hello, World!".to_string(),
        };
        warp::reply::json(&hello)
    });

    // Combine routes
    let routes = verb_route.or(hello_route).with(cors);

    info!("start server");

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
