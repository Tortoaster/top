use axum::extract::ws::WebSocket;
use axum::extract::WebSocketUpgrade;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, get_service};
use axum::Router;
use tower_http::services::ServeDir;

pub use toprs::prelude::*;

async fn handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            // client disconnected
            return;
        };

        if socket.send(msg).await.is_err() {
            // client disconnected
            return;
        }
    }
}

async fn enter_name() -> Task<String, TextField> {
    enter(TextField::new())
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest(
            "/static",
            get_service(ServeDir::new("../../web/dist/static")).handle_error(
                |error: std::io::Error| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {}", error),
                    )
                },
            ),
        )
        .route("/", get(enter_name))
        .route("/ws", get(handler));

    const IP: &str = "0.0.0.0:3000";
    println!("Listening on http://{IP}");
    axum::Server::bind(&IP.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
