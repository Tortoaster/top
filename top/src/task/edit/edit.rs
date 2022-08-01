pub use top_derive::Edit;

use crate::share::{ShareValue, ShareVec};
use crate::task::edit::edit_shared::EditShared;
use crate::task::edit::value::EditValue;
use crate::task::edit::EditVec;
use crate::task::Value;

pub trait Edit: Sized {
    type Task: Value<Output = Self>;

    fn edit(value: Option<Self>) -> Self::Task;
}

macro_rules! impl_edit_for_default {
    ($($ty:ty),*) => {
        $(
            impl Edit for $ty {
                type Task = EditValue<ShareValue<$ty>>;

                fn edit(value: Option<Self>) -> Self::Task {
                    EditValue::new(ShareValue::new(Some(value.unwrap_or_default())))
                }
            }
        )*
    };
}

impl_edit_for_default!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, bool, String
);

impl Edit for char {
    type Task = EditValue<ShareValue<char>>;

    fn edit(value: Option<Self>) -> Self::Task {
        EditValue::new(ShareValue::new(value))
    }
}

impl<T> Edit for Vec<T>
where
    T: EditShared<ShareValue<T>> + Clone + Send,
    T::Task: Send + Sync,
{
    type Task = EditVec<ShareVec<ShareValue<T>>, T::Task>;

    fn edit(value: Option<Self>) -> Self::Task {
        EditVec::new(ShareVec::new(value))
    }
}
