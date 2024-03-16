use axum::{
    response::Response,
    middleware::Next,
    extract::Request
};

use crate::strategies::authentication::Claims;

// middleware function for authenticating a requester token outside of supplied jsonwebtoken crate functionality
pub async fn authenticate_requester_token<T>(
    _claims: T,
    request: Request,
    next: Next,
) -> Response
where T: Claims {
    next.run(request).await
}