use async_trait::async_trait;
use axum::extract::ws::{Message, WebSocket};
use axum::extract::WebSocketUpgrade;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::routing::{get, get_service, IntoMakeService};
use axum::Router;
use log::info;
use thiserror::Error;
use tower_http::services::ServeDir;

use crate::component::event::{Event, EventHandler, Feedback};
use crate::component::Component;
use crate::task::{Executor, Task};

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

async fn connect<F, T>(ws: WebSocketUpgrade, task: F) -> impl IntoResponse
where
    F: FnOnce() -> T + Send + 'static,
    T: Task + Send + 'static,
{
    ws.on_upgrade(|socket| async move {
        task()
            .execute(&mut Executor::new(AxumEventHandler::new(socket)))
            .await;
    })
}

pub struct AxumEventHandler {
    socket: WebSocket,
}

impl AxumEventHandler {
    pub fn new(socket: WebSocket) -> Self {
        AxumEventHandler { socket }
    }
}

#[async_trait]
impl EventHandler for AxumEventHandler {
    type Error = AxumEventError;

    async fn receive(&mut self) -> Option<Event> {
        // TODO: Error handling
        let message = self.socket.recv().await?.ok()?;
        let event = match message {
            Message::Text(text) => match serde_json::from_str::<Event>(&text) {
                Ok(event) => event,
                Err(_) => self.receive().await?,
            },
            _ => self.receive().await?,
        };
        Some(event)
    }

    async fn send(&mut self, feedback: Feedback) -> Result<(), Self::Error> {
        let serialized = serde_json::to_string(&feedback)?;
        self.socket.send(Message::Text(serialized)).await?;
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum AxumEventError {
    #[error("failed to transmit event")]
    Inner(#[from] axum_core::Error),
    #[error("failed to serialize feedback")]
    Serialize(#[from] serde_json::Error),
}
