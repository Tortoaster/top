use axum::Router;
use log::info;

use top::editor::primitive::InputEditor;
use top::integration::axum::{task, TopService};
use top::prelude::*;
use top::share::Share;

async fn name() -> impl Task {
    let share = Share::new(TaskValue::Unstable(5));

    edit_with(InputEditor::new_shared(share.clone())).and(edit_with(InputEditor::new_shared(share)))
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
