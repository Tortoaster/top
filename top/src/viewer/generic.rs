use crate::viewer::primitive::OutputViewer;
use crate::viewer::Viewer;

/// Specifies the default viewer for a certain type. Can be derived for arbitrary types, as long as
/// all its fields also implement [`View`].
pub trait View: Sized {
    type Viewer: Viewer<Value = Self>;

    fn view(self) -> Self::Viewer;
}

macro_rules! impl_view {
    ($($ty:ty),*) => {
        $(
            impl View for $ty {
                type Viewer = OutputViewer<$ty>;

                fn view(self) -> Self::Viewer {
                    OutputViewer::new(self)
                }
            }
        )*
    };
}

impl_view!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, bool, char, String
);

impl<'a> View for &'a str {
    type Viewer = OutputViewer<&'a str>;

    fn view(self) -> Self::Viewer {
        OutputViewer::new(self)
    }
}
