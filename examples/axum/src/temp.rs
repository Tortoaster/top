use axum::extract::ws::WebSocket;
use axum::extract::{ws, WebSocketUpgrade};
use axum::routing::{get, get_service};
use axum::Router;
use log::{error, info};
use tower_http::services::ServeDir;

use toprs::component::Context;
use toprs::editor::event::{Event, Response};
use toprs::editor::Editor;
use toprs::integration::axum::index;
use toprs::task::Task;

use crate::{enter_name, IntoResponse, StatusCode};

async fn handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    let route = enter_name().await;
    let mut editor = route.get_editor();
    let mut ctx = Context::new();
    let component = editor.start(&mut ctx);

    info!("Client connected");

    let initial = Response::NewContent {
        content: component.html(),
    };
    socket
        .send(ws::Message::Text(serde_json::to_string(&initial).unwrap()))
        .await
        .unwrap();

    while let Some(message) = socket.recv().await {
        if let Ok(ws::Message::Text(text)) = message {
            if let Ok(event) = serde_json::from_str::<Event>(&text) {
                info!("Received event: {:?}", event);
                if let Some(response) = editor.respond_to(event) {
                    socket
                        .send(ws::Message::Text(serde_json::to_string(&response).unwrap()))
                        .await
                        .unwrap();
                    info!("Sent response: {:?}", response);
                }
            } else {
                error!("Received ill-formatted event: {:?}", text);
            }
        } else {
            error!("Received non-text message: {:?}", message);
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
                |_: std::io::Error| async move { (StatusCode::INTERNAL_SERVER_ERROR, ":(") },
            ),
        )
}
