use crate::html::{Refresh, ToHtml};
use crate::share::{Share, ShareId, ShareRead, Shared};
use crate::task::view::View as ViewTask;
use crate::task::Value;

/// Specifies the default view for a certain type. Can be derived for arbitrary types, as long as
/// all its fields also implement [`View`].
pub trait View: Sized {
    type Task: Value<Output = Self> + Refresh + ToHtml;

    fn view(self) -> Self::Task;
}

/// Show a value to the user. To use a custom edit, see [`view_with`].
#[inline]
pub fn view<T>(value: T) -> T::Task
where
    T: View,
{
    value.view()
}

macro_rules! impl_view {
    ($($ty:ty),*) => {
        $(
            impl View for $ty {
                type Task = ViewTask<Shared<$ty>, $ty>;

                fn view(self) -> Self::Task {
                    ViewTask::new(self)
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
    type Task: Value<Output = Self> + Refresh + ToHtml;

    fn view_shared(share: S) -> Self::Task;
}

#[inline]
pub fn view_shared<S>(share: S) -> <S::Value as SharedView<S>>::Task
where
    S: Share,
    S::Value: SharedView<S>,
{
    <S::Value>::view_shared(share)
}

macro_rules! impl_view_for_share {
    ($($ty:ty),*) => {
        $(
            impl<S> SharedView<S> for $ty
            where
                S: ShareId + ShareRead<Value = $ty> + Clone + Send + Sync,
            {
                type Task = ViewTask<S, $ty>;

                fn view_shared(share: S) -> Self::Task {
                    ViewTask::new_shared(share)
                }
            }
        )*
    };
}

impl_view_for_share!(
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
