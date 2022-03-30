use axum::Router;
use log::info;

use top::component::event::{Event, Feedback};
use top::component::{Component, ComponentCreator, Widget};
use top::editor::convert::FromStrEditor;
use top::editor::{Editor, Report};
use top::integration::axum::{task, TopService};
use top::prelude::*;
use top::task::inspect::{view, view_with};
use top::task::interact::{choose, choose_with};
use top::viewer::convert::DisplayViewer;

#[derive(Clone, Debug, Default, Edit)]
pub struct Person {
    name: String,
    title: Option<String>,
    age: u8,
    cool: bool,
    quotes: Vec<String>,
}

async fn name() -> impl Task {
    choose_with::<DisplayViewer<_>>(vec!["1", "2", "3"])
        .steps()
        .on_action(
            Action::OK,
            has_value(|choice: &str| view(choice.to_owned())),
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
