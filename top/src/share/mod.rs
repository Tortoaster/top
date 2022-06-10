use async_trait::async_trait;
use futures::lock::MutexGuard;
use uuid::Uuid;

pub use map::*;
pub use shared::*;

use crate::html::event::Feedback;
use crate::prelude::TaskValue;

mod map;
mod shared;

pub trait ShareId {
    fn id(&self) -> Uuid;
}

#[async_trait]
pub trait ShareRead: Share {
    async fn read(&self) -> MutexGuard<'_, TaskValue<Self::Value>>;
}

#[async_trait]
pub trait ShareWrite: ShareRead {
    async fn write(&self, value: TaskValue<Self::Value>) -> Feedback;
}

#[async_trait]
pub trait Share {
    type Value;

    async fn clone_value(&self) -> TaskValue<Self::Value>;
}

#[async_trait]
impl<T, U> Share for (T, U)
where
    T: Share + Send + Sync,
    U: Share + Send + Sync,
    T::Value: Send,
{
    type Value = (T::Value, U::Value);

    async fn clone_value(&self) -> TaskValue<Self::Value> {
        let a = self.0.clone_value().await;
        let b = self.1.clone_value().await;

        a.and(b)
    }
}

#[async_trait]
impl Share for () {
    type Value = ();

    async fn clone_value(&self) -> TaskValue<Self::Value> {
        TaskValue::Stable(())
    }
}
