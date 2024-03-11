use std::{net::SocketAddr, path::PathBuf};

use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade}, response::IntoResponse, routing::get, Router
};
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


    let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_headers(Any)
            .allow_methods(Any);

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .nest("/auth", controllers::auth_controller::routes())
        .nest("/user", controllers::users_controller::routes())
        .route("/", get(handler))
        .layer(
            ServiceBuilder::new()
            .layer(cors));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    match axum::serve(listener, app).await {
        Ok(_) => println!("Server listening on {}", addr),
        Err(error) => panic!("Could not listen on port {}: {}", addr, error)
    }
}

async fn handler() -> impl IntoResponse {
    "Hello, from server!"
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    loop {
        if let Some(msg) = socket.recv().await {
            if let Ok(msg) = msg {
                match msg {
                    Message::Text(t) => {
                        // Echo
                        if socket
                            .send(Message::Text(format!("Echo from backend: {}", t)))
                            .await
                            .is_err()
                        {
                            return;
                        }
                    }
                    Message::Close(_) => {
                        return;
                    }
                    _ => {}
                }
            } else {
                return;
            }
        }
    }
}
