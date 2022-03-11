use axum::http::StatusCode;
use axum::response::IntoResponse;
use log::info;

pub use toprs::prelude::*;
use toprs::task::Task;

use crate::temp::toprs_router;

mod temp;

async fn enter_name() -> impl Task {
    enter::<i32>().then(|n| {
        enter::<String>().then(move |s| {
            view(
                std::iter::repeat(s)
                    .take(n as usize)
                    .collect::<Vec<_>>()
                    .join("\n"),
            )
        })
    })
}

#[tokio::main]
async fn main() {
    const IP: &str = "0.0.0.0:3000";
    env_logger::init();

    info!("Listening on http://{IP}");
    axum::Server::bind(&IP.parse().unwrap())
        .serve(toprs_router().into_make_service())
        .await
        .unwrap();
}
