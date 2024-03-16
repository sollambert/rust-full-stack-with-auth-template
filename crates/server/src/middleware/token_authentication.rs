use axum::{
    response::{Response, IntoResponse},
    middleware::Next,
    extract::Request
};
use http::HeaderValue;

use crate::strategies::authentication::Claims;

// middleware function for authenticating a token outside of supplied jsonwebtoken crate functionality
// pub async fn authenticate_token(
//     claims: AuthClaims,
//     request: Request,
//     next: Next,
// ) -> Response {
//     println!("{:?}", claims);
//     // proceed to next layer
//     next.run(request).await
// }

// middleware function for authenticating a requester token outside of supplied jsonwebtoken crate functionality
pub async fn authenticate_requester_token<T>(
    _claims: T,
    request: Request,
    next: Next,
) -> Response
where T: Claims {
    // println!("{:?}", claims);
    // // proceed to next layer
    next.run(request).await
}