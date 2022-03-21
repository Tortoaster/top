use std::convert::Infallible;
use std::future::Future;
use std::task::Poll;

use async_trait::async_trait;
use axum::body::Body;
use axum::extract::ws::{Message, WebSocket};
use axum::extract::WebSocketUpgrade;
use axum::http::{Request, StatusCode};
use axum::response::{Html, IntoResponse};
use axum::routing::{get, get_service, MethodRouter};
use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use log::info;
use tower_http::services::ServeDir;
use tower_service::Service;

use crate::component::event::{Feedback, FeedbackHandler};
use crate::component::Component;
use crate::task::{Context, Error, HandlerError, Task};

#[derive(Clone, Debug)]
pub struct TopService(MethodRouter);

impl TopService {
    pub fn new() -> Self {
        // TODO: Fix path
        TopService(
            get_service(ServeDir::new("../../web/dist/static"))
                .handle_error(|_: std::io::Error| async move { StatusCode::NOT_FOUND }),
        )
    }
}

impl Service<Request<Body>> for TopService {
    type Response = <MethodRouter as Service<Request<Body>>>::Response;
    type Error = <MethodRouter as Service<Request<Body>>>::Error;
    type Future = <MethodRouter as Service<Request<Body>>>::Future;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.0.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        self.0.call(req)
    }
}

#[derive(Clone, Debug)]
pub struct TaskRouter {
    wrapper: MethodRouter<Body, Infallible>,
    connect: MethodRouter<Body, Infallible>,
}

pub fn task<H, Fut, T>(handler: H) -> TaskRouter
where
    H: FnOnce() -> Fut + Clone + Send + 'static,
    Fut: Future<Output = T> + Send + 'static,
    T: Task + Send + 'static,
{
    let wrapper = get(wrapper);
    let connect = get(|ws| connect(ws, handler));

    TaskRouter { wrapper, connect }
}

impl Service<Request<Body>> for TaskRouter {
    type Response = <MethodRouter as Service<Request<Body>>>::Response;
    type Error = <MethodRouter as Service<Request<Body>>>::Error;
    type Future = <MethodRouter as Service<Request<Body>>>::Future;

    fn poll_ready(&mut self, _cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        match req.headers().get("upgrade") {
            Some(header) if header == "websocket" => self.connect.call(req),
            _ => self.wrapper.call(req),
        }
    }
}

async fn wrapper() -> impl IntoResponse {
    Html(Component::html_wrapper("Top Axum"))
}

async fn connect<H, Fut, T>(ws: WebSocketUpgrade, handler: H) -> impl IntoResponse
where
    H: FnOnce() -> Fut + Clone + Send + 'static,
    Fut: Future<Output = T> + Send + 'static,
    T: Task + Send + 'static,
{
    ws.on_upgrade(|socket| async move {
        let (sender, mut receiver) = socket.split();
        let mut task = handler().await;
        let mut ctx = Context::new(AxumFeedbackHandler::new(sender));

        task.start(&mut ctx).await;

        while let Some(result) = receiver.next().await {
            if let Ok(message) = result {
                if let Message::Text(text) = message {
                    if let Ok(event) = serde_json::from_str(&text) {
                        info!("received event: {:?}", event);
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
        info!("sent feedback: {:?}", feedback);
        Ok(())
    }
}

impl HandlerError for axum::Error {}
