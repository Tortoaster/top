pub use display::ViewDisplay;
pub use vec::ViewVec;

use crate::share::ShareRead;
use crate::task::view::view::View;
use crate::task::view::view_shared::ViewShared;

mod display;
mod vec;
mod view;
mod view_shared;

#[inline]
pub fn view<T>(value: T) -> T::Task
where
    T: View,
{
    value.view()
}

#[inline]
pub fn view_shared<S>(share: S) -> <S::Value as ViewShared<S>>::Task
where
    S: ShareRead,
    S::Value: ViewShared<S>,
{
    <S::Value as ViewShared<S>>::view_shared(share)
}
