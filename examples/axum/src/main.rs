use axum::Router;
use log::info;

use top::integration::axum::{task, TopService};
use top::prelude::*;

async fn name() -> impl Task {
    choose(vec!["Option A", "Option B", "Option C"])
        .then(Trigger::Button(Button::OK), TaskValue::has_value, |value| {
            view(value.unwrap().unwrap_or("No option"))
        })
        .then(Trigger::Update, TaskValue::is_stable, |value| {
            view(format!("Stable {}!", value.unwrap()))
        })
}

const HOST: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Listening on http://{HOST}");

    let router = Router::new()
        .nest("/top", TopService::new())
        .route("/", task(name));

    axum::Server::bind(&HOST.parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
