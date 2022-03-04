use crate::{enter_name, IntoResponse, StatusCode};
use axum::extract::ws::WebSocket;
use axum::extract::{ws, WebSocketUpgrade};
use axum::routing::{get, get_service};
use axum::Router;
use serde::{Deserialize, Serialize};
use toprs::editor::Editor;
use toprs::integration::axum::index;
use toprs::task::Task;
use tower_http::services::ServeDir;

pub type InputId = String;
pub type ButtonId = String;
pub type FormId = String;
pub type Html = String;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ClientMessage {
    Update { id: InputId, value: String },
    Press { id: ButtonId },
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ServerMessage {
    NewContent { content: Html },
}

async fn handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    let route = enter_name().await;
    let mut editor = route.get_editor();
    let message = ServerMessage::NewContent {
        content: editor.ui().render(),
    };
    socket
        .send(ws::Message::Text(serde_json::to_string(&message).unwrap()))
        .await
        .unwrap();
    while let Some(message) = socket.recv().await {
        if let Ok(ws::Message::Text(text)) = message {
            if let Ok(message) = serde_json::from_str::<'_, ClientMessage>(&text) {
                match message {
                    ClientMessage::Update { id, value } => println!("update {id} to {value}"),
                    ClientMessage::Press { .. } => {}
                }
            };
        }
    }
}

pub fn toprs_router() -> Router {
    // TODO: Improve path
    Router::new()
        .route("/", get(index))
        .route("/ws", get(handler))
        .nest(
            "/static",
            get_service(ServeDir::new("../../web/dist/static")).handle_error(
                |error: std::io::Error| async move { (StatusCode::INTERNAL_SERVER_ERROR, ":(") },
            ),
        )
}
