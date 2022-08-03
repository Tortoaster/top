use crate::share::{ShareValue, ShareVec};
use crate::task::view::display::ViewDisplay;
use crate::task::view::view_shared::ViewShared;
use crate::task::view::ViewVec;
use crate::task::Value;

pub trait View: Sized {
    type Task: Value<Output = Self>;

    fn view(self) -> Self::Task;
}

macro_rules! impl_view {
    ($($ty:ty),*) => {
        $(
            impl View for $ty {
                type Task = ViewDisplay<ShareValue<$ty>>;

                fn view(self) -> Self::Task {
                    ViewDisplay::new(ShareValue::new(Some(self)))
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

impl<T> View for Vec<T>
where
    T: ViewShared<ShareValue<T>> + Clone + Send,
    T::Task: Send + Sync,
{
    type Task = ViewVec<ShareVec<ShareValue<T>>, T::Task>;

    fn view(self) -> Self::Task {
        ViewVec::new(ShareVec::new(Some(self)))
    }
}
