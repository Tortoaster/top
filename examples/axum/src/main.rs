use axum::http::StatusCode;
use axum::routing::{get, get_service};
use axum::Router;
use toprs::integration::axum::{AxumResponse, TaskAxumExt};
use tower_http::services::ServeDir;

pub use toprs::prelude::*;

async fn enter_name() -> AxumResponse<impl Task<Output = String>> {
    enter_with(TextField::default().with_label("Name".to_owned()))
        .then(view)
        .into_axum()
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest(
            "/static",
            get_service(ServeDir::new("../../web/dist/static")).handle_error(
                |error: std::io::Error| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {}", error),
                    )
                },
            ),
        )
        .route("/", get(enter_name));

    const IP: &str = "0.0.0.0:3000";
    println!("Listening on http://{IP}");
    axum::Server::bind(&IP.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
