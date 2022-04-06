use axum::Router;
use log::info;

use top::integration::axum::{task, TopService};
use top::prelude::*;
use top::tune::InputTuner;

async fn name() -> impl Task {
    enter::<bool>().tune(InputTuner::default().with_label("Name".to_string()))
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
