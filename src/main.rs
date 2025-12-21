#![deny(warnings)]
use log::info;
use serde::Serialize;
use warp::Filter;

#[derive(Serialize)]
struct HelloWorld {
    message: String,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // Configure CORS to allow all origins and GET method
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET"])
        .allow_headers(vec!["content-type"]);

    // Match any request and return hello world as JSON!
    let routes = warp::any()
        .map(|| {
            let hello = HelloWorld {
                message: "Hello, World!".to_string(),
            };
            warp::reply::json(&hello)
        })
        .with(cors);

    info!("start server");

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
