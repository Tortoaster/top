use axum::response::{Html, IntoResponse};

use crate::component::Component;

pub async fn index() -> impl IntoResponse {
    Html(Component::html_wrapper("TopRs Axum"))
}
