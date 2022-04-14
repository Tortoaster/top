use axum::Router;
use log::info;

use top::integration::axum::{task, TopService};
use top::prelude::*;
use top::task::parallel::TaskParallelExt;

async fn name() -> impl Task {
    choose_with(vec![1, 2, 3].into_iter().map(DisplayViewer::new).collect())
        .left(enter::<Vec<u32>>())
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
        .nest("/top", TopService::new())
        .route("/", task(name));

    axum::Server::bind(&HOST.parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
