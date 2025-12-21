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
    // Match any request and return hello world as JSON!
    let routes = warp::any().map(|| {
        let hello = HelloWorld {
            message: "Hello, World!".to_string(),
        };
        warp::reply::json(&hello)
    });

    info!("start server");

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
