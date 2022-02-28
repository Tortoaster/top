use crate::component::{Component, INDEX};
use axum::response::{IntoResponse, Response};
use serde_json::json;

use crate::editor::Editor;
use crate::prelude::Task;

impl<T, E> IntoResponse for Task<T, E>
where
    E: Editor<Read = T>,
{
    fn into_response(self) -> Response {
        let reg = Component::registry();
        let content = self.editor.ui().html(&reg);
        let html = reg
            .render(
                INDEX,
                &json!({
                    "title": "TopRs Axum",
                    "content": content,
                }),
            )
            .unwrap();
        let mut response = html.into_response();

        response
            .headers_mut()
            .insert("Content-Type", "text/html; charset=utf-8".parse().unwrap());
        response
    }
}
