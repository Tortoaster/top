use axum::Router;
use log::info;

use top::integration::axum::{task, TopService};
use top::prelude::derive::*;
use top::prelude::*;

#[derive(Default, Edit)]
pub struct Person {
    name: String,
    title: Option<String>,
    age: u8,
    cool: bool,
    quotes: Vec<String>,
}

#[derive(Default, Edit)]
pub struct PersonTuple(String, Option<String>, u8, bool, Vec<String>);

#[derive(Default, Edit)]
pub struct PersonUnit;

async fn name() -> impl Task {
    choose_with::<DisplayViewer<_>>(vec![3, 5, 2])
        .steps()
        .on_action(
            Action::OK,
            has_value(|choice: u32| view_with::<DisplayViewer<_>>(choice)),
        )
        .confirm()
}

const HOST: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Listening on http://{HOST}");

    let router = Router::new()
        .nest("/static", TopService::new())
        .route("/", task(name));

    axum::Server::bind(&HOST.parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
