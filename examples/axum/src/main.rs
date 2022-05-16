use axum::Router;
use log::info;

use top::integration::axum::{task, TopService};
use top::prelude::*;

async fn name() -> impl Task {
    edit("Hello, world!".to_owned())
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
