use crate::share::ShareRead;
use crate::task::view::display::ViewDisplay;
use crate::task::Value;

pub trait ViewShared<S>: Sized {
    type Task: Value<Output = Self>;

    fn view_shared(share: S) -> Self::Task;
}

macro_rules! impl_view_shared {
    ($($ty:ty),*) => {
        $(
            impl<S> ViewShared<S> for $ty
            where
                S: ShareRead<Value = $ty> + Send + Sync,
            {
                type Task = ViewDisplay<S>;

                fn view_shared(share: S) -> Self::Task {
                    ViewDisplay::new(share)
                }
            }
        )*
    };
}

impl_view_shared!(
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
