use async_trait::async_trait;
use futures::lock::MutexGuard;
use uuid::Uuid;

pub use map::*;
pub use primitive::*;
pub use value::*;

use crate::html::event::Feedback;
use crate::prelude::TaskValue;

mod map;
mod primitive;
mod value;

#[async_trait]
pub trait SharedRead {
    type Value;

    async fn read(&self) -> MutexGuard<'_, TaskValue<Self::Value>>;
}

#[async_trait]
pub trait SharedWrite {
    type Value;

    async fn write(&self, value: TaskValue<Self::Value>) -> Feedback;
}

pub trait SharedId {
    fn id(&self) -> Uuid;
}
