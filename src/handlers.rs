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
