use axum::{
    response::{Response, IntoResponse},
    middleware::Next,
    extract::Request
};

use crate::strategies::authentication::Claims;

// middleware function for authenticating a token outside of supplied jsonwebtoken crate functionality
pub async fn authenticate_token(
    claims: Claims,
    request: Request,
    next: Next,
) -> Response {
    // validate claims
    if let Err(e) = claims.validate() {
        return e.into_response()
    }
    // proceed to next layer
    next.run(request).await
}