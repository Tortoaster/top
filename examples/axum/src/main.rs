use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;

pub use toprs::prelude::*;

use crate::temp::toprs_router;

mod temp;

async fn enter_name() -> impl Task<Output = String> {
    enter_with(TextField::default().with_label("Name".to_owned())).then(view)
}

#[tokio::main]
async fn main() {
    const IP: &str = "0.0.0.0:3000";
    println!("Listening on http://{IP}");
    axum::Server::bind(&IP.parse().unwrap())
        .serve(toprs_router().into_make_service())
        .await
        .unwrap();
}
