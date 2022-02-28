use axum::response::{IntoResponse, Response};

use crate::editor::Editor;
use crate::prelude::Task;

impl<T, E> IntoResponse for Task<T, E>
where
    E: Editor<Read = T>,
{
    fn into_response(self) -> Response {
        let mut response = self.editor.ui().render_page("TopRs Axum").into_response();

        response
            .headers_mut()
            .insert("Content-Type", "text/html; charset=utf-8".parse().unwrap());

        response
    }
}
