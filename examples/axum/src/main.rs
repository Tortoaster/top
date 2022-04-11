use axum::Router;
use log::info;

use top::integration::axum::{task, TopService};
use top::prelude::*;

async fn name() -> impl Task {
    enter::<bool>()
        .tune(InputTuner::default().with_label("hello".to_owned()))
        .steps()
        .on_action(
            Action::OK,
            has_value(|x: bool| view_with(DebugViewer::new(x))),
        )
        .on_action(
            Action::CONTINUE,
            has_value(|x: bool| {
                edit(x)
                    .steps()
                    .on_action(
                        Action::OK,
                        has_value(move |y| view_with(DisplayViewer::new(x && y))),
                    )
                    .finish()
            }),
        )
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
