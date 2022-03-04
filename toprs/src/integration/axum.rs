use crate::component::Component;
use axum::response::{Html, IntoResponse, Response};

use crate::editor::Editor;
use crate::task::Task;

pub async fn index() -> impl IntoResponse {
    Html(Component::render_wrapper("TopRs Axum"))
}
