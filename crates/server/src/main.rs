use std::{net::SocketAddr, path::PathBuf};

use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    http::Method,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use tower_http::cors::{Any, CorsLayer};
use types::user::ResponseUser;

mod pool;

#[tokio::main]
async fn main() {

    // load environment variables with dotenv if debug
    if cfg!(debug_assertions) {
        match dotenv::dotenv() {
            Ok(path) => path,
            Err(error) => {
                println!("Cannot pull .env file: {}", error);
                PathBuf::from("")
            }
        };
    }

    //create pg pool
    pool::create_pool().await;

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .route("/user", get(user_handler))
        .route("/", get(handler))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(vec![Method::GET, Method::POST]),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> impl IntoResponse {
    "Hello, from server!"
}

async fn user_handler() -> impl IntoResponse {
    let user = ResponseUser {
        uuid: "1".to_owned(),
        email: "test@test.com".to_owned(),
        username: "Backend user".to_owned()
    };

    Json(user)
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
