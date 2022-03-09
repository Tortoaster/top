use axum::http::StatusCode;
use axum::response::IntoResponse;
use log::info;

pub use toprs::prelude::*;

use crate::temp::toprs_router;

mod temp;

async fn enter_name() -> impl Task<Output = String> {
    enter()
    // enter_with(TextEditor::default().with_label("Name".to_owned())).then(|name| {
    //     let mut buf = Vec::new();
    //     ferris_says::say(name.as_bytes(), 24, &mut buf).unwrap();
    //     let text = String::from_utf8(buf).unwrap();
    //     view(text)
    // })
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
