use axum::Router;
use log::info;

use toprs::integration::axum::{task, TopService};
use toprs::prelude::*;

async fn name() -> impl Task {
    enter::<String>()
        .steps()
        .on_value(if_value(
            |name| name == "Bob",
            |name| update(format!("Hi, {name}!")),
        ))
        .on_action(
            Action::OK,
            if_value(
                |name: &String| !name.is_empty(),
                |name| update(format!("Hello, {name}!")),
            ),
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
