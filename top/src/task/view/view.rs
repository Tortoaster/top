use crate::share::ShareValue;
use crate::task::view::display::ViewDisplay;
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
                    ViewDisplay::new(ShareValue::new(self))
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
