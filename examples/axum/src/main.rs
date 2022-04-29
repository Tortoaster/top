use axum::Router;
use log::info;

use top::integration::axum::{task, TopService};
use top::prelude::*;

async fn name() -> impl Task {
    choose(vec!["Option A", "Option B", "Option C"])
        .then()
        .on_value(Trigger::Button(Button::OK), |x| {
            view(x.unwrap_or("No option"))
        })
        .then()
        .on_stable(Trigger::Update, |x| view(format!("Stable {x}!")))
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
