use axum::{
    response::{Response, IntoResponse},
    middleware::Next,
    extract::Request
};
use http::HeaderValue;

use crate::strategies::authentication::{AuthClaims, AuthRequesterClaims, Claims};

// middleware function for authenticating a token outside of supplied jsonwebtoken crate functionality
pub async fn authenticate_token(
    claims: AuthClaims,
    request: Request,
    next: Next,
) -> Response {
    // validate claims
    if let Err(e) = claims.validate() {
        return e.into_response()
    }
    println!("{:?}", claims);
    // proceed to next layer
    next.run(request).await
}

// middleware function for authenticating a requester token outside of supplied jsonwebtoken crate functionality
pub async fn authenticate_requester_token(
    claims: AuthRequesterClaims,
    mut request: Request,
    next: Next,
) -> Response {
    // validate claims
    if let Err(e) = claims.validate() {
        return e.into_response()
    }
    request.headers_mut().append("User-UUID", HeaderValue::from_str(claims.aud.as_str()).unwrap());
    println!("{:?}", claims);
    // proceed to next layer
    next.run(request).await
}