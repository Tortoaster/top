use axum::extract::{ws, WebSocketUpgrade};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::routing::{get, get_service, IntoMakeService};
use axum::Router;
use log::{error, info};
use tower_http::services::ServeDir;

use crate::component::{Component, ComponentId, Context};
use crate::editor::event::{EditorError, Event, Response};
use crate::editor::Editor;
use crate::task::Task;

#[derive(Debug)]
pub struct TopRsRouter(Router);

// TODO: Use a TaskRouter service so tasks can be intertwined with other web services without nesting an entire router.
impl TopRsRouter {
    pub fn new() -> Self {
        // TODO: Improve path
        TopRsRouter(
            Router::new().nest(
                "/static",
                get_service(ServeDir::new("../../web/dist/static"))
                    .handle_error(|_: std::io::Error| async move { StatusCode::NOT_FOUND }),
            ),
        )
    }

    pub fn task<F, T>(mut self, path: &str, handler: F) -> Self
    where
        F: FnOnce() -> T + Clone + Send + 'static,
        T: Task + Send + 'static,
    {
        self.0 = self.0.route(path, get(wrapper));
        self.0 = self.0.route(
            format!("{path}/ws").as_str(),
            get(|ws| connect(ws, handler)),
        );
        self
    }

    pub fn into_make_service(self) -> IntoMakeService<Router> {
        self.0.into_make_service()
    }
}

async fn wrapper() -> impl IntoResponse {
    Html(Component::html_wrapper("TopRs Axum"))
}

async fn connect<F, T>(ws: WebSocketUpgrade, handler: F) -> impl IntoResponse
where
    F: FnOnce() -> T + Send + 'static,
    T: Task + Send + 'static,
{
    ws.on_upgrade(|mut socket| async move {
        let task = handler();
        let mut editor = task.editor();
        let mut ctx = Context::new();
        let component = editor.start(&mut ctx);

        info!("Client connected");

        let initial: Result<Response, EditorError> = Ok(Response::Replace {
            id: ComponentId::default(),
            content: component.html(),
        });
        socket
            .send(ws::Message::Text(serde_json::to_string(&initial).unwrap()))
            .await
            .unwrap();

        while let Some(message) = socket.recv().await {
            if let Ok(ws::Message::Text(text)) = message {
                if let Ok(event) = serde_json::from_str::<Event>(&text) {
                    info!("Received event: {:?}", event);
                    if let Some(response) = editor.respond_to(event, &mut ctx) {
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
    })
}
