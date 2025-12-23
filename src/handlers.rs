use crate::template;
use crate::verbs;
use percent_encoding::percent_decode_str;
use std::collections::HashMap;
use std::sync::Arc;
use warp::Rejection;
use warp::Reply;

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

pub async fn get_template_handler(
    template_name: String,
    templates: Arc<HashMap<String, template::Template>>,
) -> Result<warp::reply::Response, Rejection> {
    // Decode URL-encoded template name (e.g., %C3%AAtre -> être)
    let decoded_name = percent_decode_str(&template_name)
        .decode_utf8()
        .map_err(|_| warp::reject::not_found())?
        .to_string();

    match templates.get(&decoded_name) {
        Some(template) => Ok(warp::reply::json(template).into_response()),
        None => Ok(warp::reply::with_status("", warp::http::StatusCode::NOT_FOUND).into_response()),
    }
}

pub async fn search_verbs_handler(
    query: String,
    search_index: Arc<verbs::VerbSearchIndex>,
) -> Result<warp::reply::Response, Rejection> {
    let results = search_index.search(&query);
    Ok(warp::reply::json(&results).into_response())
}
