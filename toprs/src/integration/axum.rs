use axum::response::{IntoResponse, Response};

use crate::editor::Editor;
use crate::task::Task;

pub struct AxumResponse<T>(T);

impl<T> IntoResponse for AxumResponse<T>
where
    T: Task,
{
    fn into_response(self) -> Response {
        let mut response = self
            .0
            .get_editor()
            .ui()
            .render_page("TopRs Axum")
            .into_response();

        response
            .headers_mut()
            .insert("Content-Type", "text/html; charset=utf-8".parse().unwrap());

        response
    }
}

pub trait TaskAxumExt<T>: private::Sealed {
    fn into_axum(self) -> AxumResponse<T>;
}

impl<T> TaskAxumExt<T> for T
where
    T: Task,
{
    fn into_axum(self) -> AxumResponse<T> {
        AxumResponse(self)
    }
}

mod private {
    use crate::task::Task;

    pub trait Sealed {}

    impl<T> Sealed for T where T: Task {}
}
