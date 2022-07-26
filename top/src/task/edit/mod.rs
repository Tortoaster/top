use crate::share::Share;
use crate::task::edit::edit::Edit;
use crate::task::edit::edit_shared::EditShared;

mod edit;
mod edit_shared;
mod form;
mod value;

#[inline]
pub fn enter<T>() -> T::Task
where
    T: Edit,
{
    T::edit(None)
}

#[inline]
pub fn edit<T>(value: T) -> T::Task
where
    T: Edit,
{
    T::edit(Some(value))
}

#[inline]
pub fn edit_shared<S>(share: S) -> <S::Value as EditShared<S>>::Task
where
    S: Share,
    S::Value: EditShared<S>,
{
    <S::Value as EditShared<S>>::edit_shared(share)
}
