use axum::Router;
use log::info;

use top::integration::axum::{task, TopService};
use top::prelude::*;
use top::task::sequential::if_stable;

async fn name() -> impl Task {
    enter::<u32>()
        .then()
        .on_action(Action::OK, has_value(|n: u32| view(n)))
        .left(view("Hello, world!"))
        .then()
        // TODO: Make it work without action
        .on_action(Action::OK, if_stable(|n: u32| view(n)))
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
