use crate::template;
use crate::verbs;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;
use warp::{Rejection, Reply};

#[derive(Serialize)]
pub struct HelloWorld {
    pub message: String,
}

pub async fn get_verb_handler(
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

pub async fn hello_handler() -> Result<impl Reply, Rejection> {
    let hello = HelloWorld {
        message: "Hello, World!".to_string(),
    };
    Ok(warp::reply::json(&hello))
}

pub async fn get_template_handler(
    template_name: String,
    templates: Arc<HashMap<String, template::Template>>,
) -> Result<warp::reply::Response, Rejection> {
    match templates.get(&template_name) {
        Some(template) => Ok(warp::reply::json(template).into_response()),
        None => Ok(warp::reply::with_status("", warp::http::StatusCode::NOT_FOUND).into_response()),
    }
}

pub async fn not_found_handler() -> Result<impl Reply, Rejection> {
    Ok(warp::reply::with_status("", warp::http::StatusCode::NOT_FOUND).into_response())
}
