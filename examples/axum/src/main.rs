use axum::Router;
use log::info;

use top::integration::axum::{task, TopService};
use top::prelude::*;

async fn name() -> impl Task {
    enter::<Vec<i32>>()
        .steps()
        .on_action(
            Action::OK,
            has_value(|nums: Vec<i32>| update(nums.into_iter().sum::<i32>().to_string())),
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
