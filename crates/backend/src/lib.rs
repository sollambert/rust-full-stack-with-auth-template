use std::net::SocketAddr;

use axum::{
    http::Method, Router,
};
use tower_http::cors::{Any, CorsLayer};

pub async fn app(port: u16) {
    let app = Router::new()
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(vec![
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ]));

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("Backend is listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
