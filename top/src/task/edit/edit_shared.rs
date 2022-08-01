use crate::share::{ShareChildren, ShareRead};
use crate::task::edit::value::EditValue;
use crate::task::edit::EditVec;
use crate::task::Value;

pub trait EditShared<S>: Sized {
    type Task: Value<Output = Self>;

    fn edit_shared(share: S) -> Self::Task;
}

macro_rules! impl_edit_shared {
    ($($ty:ty),*) => {
        $(
            impl<S> EditShared<S> for $ty
            where
                S: ShareRead<Value = Self> + Send + Sync,
            {
                type Task = EditValue<S>;

                fn edit_shared(share: S) -> Self::Task {
                    EditValue::new(share)
                }
            }
        )*
    };
}

impl_edit_shared!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, bool, String, char
);

impl<S, T> EditShared<S> for Vec<T>
where
    T: EditShared<S::Child> + Clone,
    T::Task: Send + Sync,
    S: ShareChildren + ShareRead<Value = Vec<T>> + Send,
    S::Child: ShareRead<Value = T> + Clone,
{
    type Task = EditVec<S, T::Task>;

    fn edit_shared(share: S) -> Self::Task {
        EditVec::new(share)
    }
}
