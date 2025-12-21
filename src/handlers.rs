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

pub async fn search_verbs_handler(
    query: String,
    verbs: Arc<Vec<verbs::Verb>>,
) -> Result<warp::reply::Response, Rejection> {
    let query_lower = query.to_lowercase();
    let mut results = Vec::new();

    // Use binary search to find insertion point for the query
    // This gives us a starting point near where matching verbs would be
    let start_index = match verbs.binary_search_by(|v| v.verb.to_lowercase().cmp(&query_lower)) {
        Ok(idx) => idx,
        Err(idx) => idx,
    };

    // Find the first verb that starts with the query by scanning backwards
    let mut first_match = start_index;
    while first_match > 0
        && verbs[first_match - 1]
            .verb
            .to_lowercase()
            .starts_with(&query_lower)
    {
        first_match -= 1;
    }

    // Collect up to 20 verbs that start with the query (case-insensitive)
    for verb in verbs.iter().skip(first_match) {
        if verb.verb.to_lowercase().starts_with(&query_lower) {
            results.push(verb);
            if results.len() >= 20 {
                break;
            }
        } else {
            // Since verbs are sorted, if we find one that doesn't match, we can stop
            break;
        }
    }

    Ok(warp::reply::json(&results).into_response())
}

pub async fn not_found_handler() -> Result<impl Reply, Rejection> {
    Ok(warp::reply::with_status("", warp::http::StatusCode::NOT_FOUND).into_response())
}
