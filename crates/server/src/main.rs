use std::{net::SocketAddr, path::PathBuf};

use axum::Router;
use http::header::{AUTHORIZATION, CONTENT_TYPE};
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

mod pool;
mod strategies;
mod controllers;
mod middleware;

#[tokio::main]
async fn main() {

    // load environment variables with dotenv if debug
    if cfg!(debug_assertions) {
        match dotenv::dotenv() {
            Ok(path) => path,
            Err(error) => {
                println!("Cannot access .env file: {}", error);
                PathBuf::from("")
            }
        };
    }

    //create pg pool
    pool::create_pool().await;

    let cors = CorsLayer::permissive()
        .allow_origin(Any)
        .allow_headers([AUTHORIZATION, CONTENT_TYPE])
        .expose_headers(Any);

    let app = Router::new()
        .nest("/ws", controllers::ws_controller::routes())
        .nest("/auth", controllers::auth_controller::routes())
        .nest("/user", controllers::users_controller::routes())
        .layer(
            ServiceBuilder::new()
            .layer(cors));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("Server listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    match axum::serve(listener, app).await {
        Ok(_) => {},
        Err(error) => panic!("Could not bind to {}: {}", addr,error)
    }
}
