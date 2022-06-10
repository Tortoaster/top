use crate::html::{Refresh, ToHtml};
use crate::share::{Share, ShareId, ShareRead, Shared};
use crate::task::Value;
use crate::viewer::viewer::Viewer;

/// Specifies the default viewer for a certain type. Can be derived for arbitrary types, as long as
/// all its fields also implement [`View`].
pub trait View: Sized {
    type Viewer: Value<Output = Self> + Refresh + ToHtml;

    fn view(self) -> Self::Viewer;
}

/// Show a value to the user. To use a custom editor, see [`view_with`].
#[inline]
pub fn view<T>(value: T) -> T::Viewer
where
    T: View,
{
    value.view()
}

macro_rules! impl_view {
    ($($ty:ty),*) => {
        $(
            impl View for $ty {
                type Viewer = Viewer<Shared<$ty>, $ty>;

                fn view(self) -> Self::Viewer {
                    Viewer::new(self)
                }
            }
        )*
    };
}

impl_view!(
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    f32,
    f64,
    bool,
    char,
    &'static str,
    String
);

pub trait SharedView<S>: Sized {
    type Viewer: Value<Output = Self> + Refresh + ToHtml;

    fn view_shared(share: S) -> Self::Viewer;
}

#[inline]
pub fn view_shared<S>(share: S) -> <S::Value as SharedView<S>>::Viewer
where
    S: Share,
    S::Value: SharedView<S>,
{
    <S::Value>::view_shared(share)
}

macro_rules! impl_shared_view {
    ($($ty:ty),*) => {
        $(
            impl<S> SharedView<S> for $ty
            where
                S: ShareId + ShareRead<Value = $ty> + Clone + Send + Sync,
            {
                type Viewer = Viewer<S, $ty>;

                fn view_shared(share: S) -> Self::Viewer {
                    Viewer::new_shared(share)
                }
            }
        )*
    };
}

impl_shared_view!(
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    f32,
    f64,
    bool,
    char,
    &'static str,
    String
);
