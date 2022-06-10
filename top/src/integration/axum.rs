use std::convert::Infallible;
use std::future::Future;
use std::task::Poll;

use axum::body::Body;
use axum::extract::ws::{Message, WebSocket};
use axum::extract::WebSocketUpgrade;
use axum::http::{Request, StatusCode};
use axum::response::{Html, IntoResponse};
use axum::routing::{get, get_service, MethodRouter};
use futures::stream::SplitSink;
use futures::{SinkExt, StreamExt};
use log::{error, warn};
use tower_http::services::ServeFile;
use tower_service::Service;
use uuid::Uuid;

use crate::html::event::{Change, Event, Feedback};
use crate::html::{Handler, ToHtml};

pub trait Task: crate::task::Task + Handler + ToHtml {}

impl<T> Task for T where T: crate::task::Task + Handler + ToHtml {}

#[derive(Clone, Debug)]
pub struct TopService(MethodRouter);

impl TopService {
    pub fn new() -> Self {
        // TODO: Fix path
        TopService(
            get_service(ServeFile::new("../../web/dist/top.js"))
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
    T: crate::task::Task + Handler + ToHtml + Send + Sync + 'static,
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
    Html(crate::html::Html::wrapper("Top Axum").await.to_string())
}

async fn connect<H, Fut, T>(ws: WebSocketUpgrade, handler: H) -> impl IntoResponse
where
    H: FnOnce() -> Fut + Clone + Send + 'static,
    Fut: Future<Output = T> + Send + 'static,
    T: crate::task::Task + Handler + ToHtml + Send + Sync + 'static,
{
    ws.on_upgrade(|socket| async move {
        let (mut sender, mut receiver) = socket.split();
        let mut task = handler().await;

        let html = task.to_html().await;
        let feedback = Feedback::from(Change::AppendContent {
            id: Uuid::nil(),
            html,
        });
        send_feedback(&mut sender, feedback).await;

        while let Some(Ok(message)) = receiver.next().await {
            match message.into_text() {
                Ok(text) => match serde_json::from_str(&text) {
                    Ok(event) => match task.on_event(event).await {
                        Ok(mut feedback) => {
                            let mut shares = feedback.shares().clone();
                            while !shares.is_empty() {
                                let first = *shares.iter().next().unwrap();
                                let id = shares.take(&first).unwrap();
                                match task.on_event(Event::Redraw { id }).await {
                                    Ok(new) => feedback = feedback.merged_with(new).unwrap(),
                                    Err(error) => error!("failed to handle event: {error}"),
                                }
                            }
                            if !feedback.is_empty() {
                                send_feedback(&mut sender, feedback).await;
                            }
                        }
                        Err(error) => error!("failed to handle event: {error}"),
                    },
                    Err(_) => warn!("not an event"),
                },
                Err(_) => warn!("non-text message"),
            }
        }
    })
}

async fn send_feedback(sender: &mut SplitSink<WebSocket, Message>, feedback: Feedback) {
    match serde_json::to_string(&feedback.changes()) {
        Ok(text) => sender
            .send(Message::Text(text))
            .await
            .unwrap_or_else(|error| error!("failed to send feedback: {error}")),
        Err(error) => error!("failed to serialize: {error}"),
    }
}
