use crate::task::Value;

pub trait EditShared<S>: Sized {
    type Task: Value<Output = Self>;

    fn edit_shared(share: S) -> Self::Task;
}
