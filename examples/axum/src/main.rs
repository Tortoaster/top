use axum::Router;
use log::info;

use top::editor::primitive::InputEditor;
use top::integration::axum::{task, TopService};
use top::prelude::*;
use top::viewer::primitive::OutputViewer;

async fn name() -> impl Task {
    let share: Share<String> = Share::new(TaskValue::Empty);
    let uwuified = share
        .clone()
        .map(|s| s.map(|s| uwuifier::uwuify_str_sse(s.as_str())))
        .await;

    edit_shared(share).and(view_shared(uwuified))
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
