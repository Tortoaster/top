use async_trait::async_trait;
use uuid::Uuid;

pub use map::*;
pub use option::ShareOption;
pub use value::*;

use crate::html::event::Feedback;
use crate::prelude::TaskValue;
use crate::share::guard::ShareGuard;

pub mod guard;
mod map;
mod option;
mod value;

pub trait Share: Sized {
    type Share: ShareConsume<Value = Self>;

    fn share(self) -> Self::Share;
}

#[inline]
pub fn share<T>(value: T) -> T::Share
where
    T: Share,
{
    value.share()
}

macro_rules! impl_share {
    ($($ty:ty),*) => {
        $(
            impl Share for $ty {
                type Share = ShareValue<$ty>;

                fn share(self) -> Self::Share {
                    ShareValue::new(TaskValue::Unstable(self))
                }
            }
        )*
    };
}

impl_share!(
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

impl<T> Share for Option<T>
where
    T: Share + Default,
    T::Share: Send + Sync,
{
    type Share = ShareOption<T::Share>;

    fn share(self) -> Self::Share {
        let enabled = self.is_some();
        ShareOption::new(self.unwrap_or_default().share(), enabled)
    }
}

pub trait ShareId {
    // TODO: `ShareId` newtype
    fn id(&self) -> Uuid;
}

#[async_trait]
pub trait ShareConsume {
    type Value;

    async fn consume(self) -> TaskValue<Self::Value>;
}

#[async_trait]
pub trait ShareRead: ShareConsume {
    async fn read(&self) -> ShareGuard<'_, TaskValue<Self::Value>>;
}

#[async_trait]
pub trait ShareWrite: ShareRead {
    async fn write(&self, value: TaskValue<Self::Value>) -> Feedback;
}

#[async_trait]
impl<T, U> ShareConsume for (T, U)
where
    T: ShareConsume + Send + Sync,
    U: ShareConsume + Send + Sync,
    T::Value: Send,
{
    type Value = (T::Value, U::Value);

    async fn consume(self) -> TaskValue<Self::Value> {
        let a = self.0.consume().await;
        let b = self.1.consume().await;

        a.and(b)
    }
}

#[async_trait]
impl ShareConsume for () {
    type Value = ();

    async fn consume(self) -> TaskValue<Self::Value> {
        TaskValue::Stable(())
    }
}
