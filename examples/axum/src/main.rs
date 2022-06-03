use std::net::SocketAddr;

use axum::response::Html;
use axum::routing::get;
use axum::Router;
use log::info;

use top::integration::axum::{task, TopService};
use top::prelude::*;

async fn index() -> Html<&'static str> {
    Html(
        r#"
<a href="/one">1</a><br/>
<a href="/two">2</a><br/>
<a href="/three">3</a><br/>
<a href="/four">4</a><br/>
"#,
    )
}

async fn one() -> impl Task {
    enter::<Vec<u8>>()
}

async fn two() -> impl Task {
    choose_with::<DisplayViewer<_>>(vec!["Option one", "Option two", "Option three"])
        .steps()
        .on_action(Action::OK, has_value(|value: &str| view(value.to_string())))
        .confirm()
}

async fn three() -> impl Task<Value = SocketAddr> {
    enter_with(FromStrEditor::new())
}

async fn four() -> impl Task {
    enter::<Person>()
        .steps()
        .on_action(Action::OK, has_value(|person: Person| update(person)))
        .confirm()
}

#[derive(Clone, Debug, Default, Edit)]
pub struct Person {
    name: String,
    title: Option<String>,
    age: u8,
    cool: bool,
    quotes: Vec<String>,
}

const HOST: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Listening on http://{HOST}");

    let router = Router::new()
        .nest("/static", TopService::new())
        .route("/", get(index))
        .route("/one", task(one))
        .route("/two", task(two))
        .route("/three", task(three))
        .route("/four", task(four));

    axum::Server::bind(&HOST.parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
