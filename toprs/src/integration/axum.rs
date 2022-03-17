use async_trait::async_trait;
use axum::extract::ws::{Message, WebSocket};
use axum::extract::WebSocketUpgrade;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse};
use axum::routing::{get, get_service, IntoMakeService};
use axum::Router;
use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use tower_http::services::ServeDir;

use crate::component::event::{Feedback, FeedbackHandler};
use crate::component::Component;
use crate::task::{Context, Error, HandlerError, Task};

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
        let (sender, mut receiver) = socket.split();
        let mut task = task();
        let mut ctx = Context::new(AxumFeedbackHandler::new(sender));

        task.start(&mut ctx).await;

        while let Some(result) = receiver.next().await {
            if let Ok(message) = result {
                if let Message::Text(text) = message {
                    if let Ok(event) = serde_json::from_str(&text) {
                        task.on_event(event, &mut ctx).await;
                    } else {
                        // TODO: Send feedback
                    }
                };
            } else {
                // TODO: Send feedback
            }
        }
    })
}

pub struct AxumFeedbackHandler {
    sender: SplitSink<WebSocket, Message>,
}

impl AxumFeedbackHandler {
    pub fn new(sender: SplitSink<WebSocket, Message>) -> Self {
        AxumFeedbackHandler { sender }
    }
}

#[async_trait]
impl FeedbackHandler for AxumFeedbackHandler {
    type Error = axum::Error;

    async fn send(&mut self, feedback: Feedback) -> Result<(), Error<Self::Error>> {
        let serialized = serde_json::to_string(&feedback)?;
        self.sender.send(Message::Text(serialized)).await?;
        Ok(())
    }
}

impl HandlerError for axum::Error {}
