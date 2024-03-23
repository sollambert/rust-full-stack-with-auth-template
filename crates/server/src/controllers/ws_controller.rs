use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use axum::{
    routing::get,
    Router
};
use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State}, response::IntoResponse
};
use tokio::sync::broadcast;
use futures::{sink::SinkExt, stream::StreamExt};

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

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    ws.on_upgrade(|socket| {handle_socket(socket, state)})
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    let (mut sender, mut receiver) = socket.split();
    let mut username = String::new();
    while let Some(Ok(auth)) = receiver.next().await {
        if let Message::Text(text) = auth {
            if let Ok(claims) = AuthRequesterClaims::from_string(&text) {
                username = get_db_user_by_uuid(claims.sub.clone()).await.unwrap().username;
                break;
            } else {
                sender.close().await.unwrap();
                return;
            }
        }
    }

    let mut rx = state.tx.subscribe();

    let msg = format!("{username} joined.");
    let _ = state.tx.send(msg);

    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            if msg == String::new() {
                break;
            }
            if sender.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
    });

    let tx = state.tx.clone();
    let name = username.clone();

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            if text == String::new() {
                break;
            }
            let _ = tx.send(format!("{name}: {text}"));
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    let msg = format!("{username} left.");
    let _ = state.tx.send(msg);

    state.user_set.lock().unwrap().remove(&username);
}
