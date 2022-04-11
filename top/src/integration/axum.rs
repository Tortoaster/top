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
use futures::stream::SplitStream;
use futures::StreamExt;
use log::{error, trace, warn};
use tower_http::services::ServeDir;
use tower_service::Service;

use crate::event::handler::FeedbackHandler;
use crate::event::{Event, EventError, EventHandler};
use crate::task::{Context, Task};

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

impl Default for TopService {
    fn default() -> Self {
        TopService::new()
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
    Html(crate::html::Html::wrapper("Top Axum").to_string())
}

async fn connect<H, Fut, T>(ws: WebSocketUpgrade, handler: H) -> impl IntoResponse
where
    H: FnOnce() -> Fut + Clone + Send + 'static,
    Fut: Future<Output = T> + Send + 'static,
    T: Task + Send + 'static,
{
    ws.on_upgrade(|socket| async move {
        let mut task = handler().await;
        let (sender, mut receiver) = socket.split();

        let mut ctx = Context::new(FeedbackHandler::new(sender));

        if let Err(error) = task.start(&mut ctx).await {
            error!("failed to start task: {error}")
        }

        while let Some(result) = receiver.receive().await {
            match result {
                Ok(event) => {
                    if let Err(error) = task.on_event(event, &mut ctx).await {
                        error!("failed to update task: {error}");
                    }
                }
                Err(error) => error!("failed to handle event: {error}"),
            }
        }
    })
}

#[async_trait]
impl EventHandler for SplitStream<WebSocket> {
    async fn receive(&mut self) -> Option<Result<Event, EventError>> {
        match self.next().await {
            None => None,
            Some(result) => match result {
                Ok(Message::Text(text)) => match serde_json::from_str(&text) {
                    Ok(event) => {
                        trace!("received event: {:?}", event);
                        Some(Ok(event))
                    }
                    Err(error) => Some(Err(error.into())),
                },
                Ok(_) => {
                    warn!("non-text message");
                    self.receive().await
                }
                Err(_) => Some(Err(EventError::Receive)),
            },
        }
    }
}
