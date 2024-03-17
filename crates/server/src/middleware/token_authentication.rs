use std::fmt::Debug;

use axum::{
    response::Response,
    middleware::Next,
    extract::Request
};
use http::HeaderValue;
use serde::Serialize;
use serde_json::json;
use base64::prelude::*;
use crate::strategies::authentication::Claims;

// middleware function for authenticating token
pub async fn authenticate_token<T>(
    claims: T,
    mut request: Request,
    next: Next,
) -> Response
where T: Claims,T: Serialize, T: Debug {
    let header_map = request.headers_mut();
    while header_map.contains_key("X-Claims") {
        header_map.remove("X-Claims");
    }
    let json = json!(claims);
    let encoded_text = BASE64_STANDARD.encode(json.to_string());
    request.headers_mut().insert("X-Claims", HeaderValue::from_str(&encoded_text).unwrap());
    next.run(request).await
}