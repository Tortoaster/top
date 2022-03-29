use crate::viewer::primitive::TextViewer;
use crate::viewer::Viewer;

/// Specifies the default viewer for a certain type. Can be derived for arbitrary types, as long as
/// all its fields also implement [`View`].
pub trait View: Sized {
    type Viewer: Viewer<Input = Self, Output = Self>;
}

impl View for String {
    type Viewer = TextViewer;
}
