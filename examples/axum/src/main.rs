use axum::Router;
use log::info;

use top::integration::axum::{task, TopService};
use top::prelude::*;
use top::task::parallel::TaskParallelExt;

async fn name() -> impl Task {
    view("hello!".to_owned())
        .right(enter::<bool>())
        .steps()
        .on_action(Action::OK, has_value(|b| edit(b)))
        .finish()
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
