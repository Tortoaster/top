use crate::viewer::primitive::StringViewer;
use crate::viewer::Viewer;

/// Specifies the default viewer for a certain type. Can be derived for arbitrary types, as long as
/// all its fields also implement [`View`].
pub trait View: Sized {
    type Viewer: Viewer<Output = Self>;

    fn view(self) -> Self::Viewer;
}

impl View for String {
    type Viewer = StringViewer;

    fn view(self) -> Self::Viewer {
        StringViewer::new(self)
    }
}
