use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use axum::{
    routing::get,
    Router
};
use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, Request, State}, response::IntoResponse
};
use tokio::sync::broadcast;
use futures::{sink::SinkExt, stream::StreamExt};

use crate::middleware::token_authentication;
use crate::strategies::authentication::{AuthRequesterClaims, Claims};
use crate::strategies::users::get_db_user_by_uuid;

struct AppState {
    user_set: Mutex<HashSet<String>>,
    tx: broadcast::Sender<String>
}

// route function to nest endpoints in router
pub fn routes() -> Router {
    let user_set = Mutex::new(HashSet::new());
    let (tx, _rx) = broadcast::channel(100);
    let app_state = Arc::new(AppState{user_set, tx});
    // create routes
    Router::new()
        .route("/", get(ws_handler))
        .with_state(app_state)
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>, request: Request) -> impl IntoResponse {
    let claims = AuthRequesterClaims::from_header(request.headers());
    ws.on_upgrade(|socket| {handle_socket(socket, state)})
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    let mut authenticated = false;
    let (mut sender, mut receiver) = socket.split();
    let mut username = String::new();
    while let Some(Ok(message)) = receiver.next().await {
        if let Message::Text(name) = message {
            
        }
    }
}
