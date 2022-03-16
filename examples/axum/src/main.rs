use log::info;

use toprs::integration::axum::TopRsRouter;
pub use toprs::prelude::*;

fn repeat() -> impl Task {
    update("hello!".to_owned())
}

const HOST: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Listening on http://{HOST}");

    let router = TopRsRouter::new().task("/", repeat);

    axum::Server::bind(&HOST.parse().unwrap())
        .serve(router.into_make_service())
        .await
        .unwrap();
}
