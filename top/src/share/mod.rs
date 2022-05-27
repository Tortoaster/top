use std::sync::Arc;

use async_trait::async_trait;
use futures::lock::{Mutex, MutexGuard};
use uuid::Uuid;

use crate::html::event::Feedback;
use crate::prelude::TaskValue;

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

#[derive(Clone, Debug)]
pub struct Share<T> {
    id: Uuid,
    value: Arc<Mutex<TaskValue<T>>>,
}

impl<T> Share<T>
where
    T: Clone,
{
    pub fn new(value: TaskValue<T>) -> Self {
        Share {
            id: Uuid::new_v4(),
            value: Arc::new(Mutex::new(value)),
        }
    }
}

#[async_trait]
impl<T> SharedRead for Share<T>
where
    T: Send,
{
    type Value = T;

    async fn read(&self) -> MutexGuard<'_, TaskValue<Self::Value>> {
        self.value.lock().await
    }
}

#[async_trait]
impl<T> SharedWrite for Share<T>
where
    T: Send,
{
    type Value = T;

    async fn write(&self, value: TaskValue<Self::Value>) -> Feedback {
        *self.value.lock().await = value;
        Feedback::update_share(self.id)
    }
}

impl<T> SharedId for Share<T> {
    fn id(&self) -> Uuid {
        self.id
    }
}

#[async_trait]
pub trait SharedValue {
    type Value;

    async fn clone_value(&self) -> TaskValue<Self::Value>;
}

#[async_trait]
impl<T> SharedValue for Share<T>
where
    T: Clone + Send,
{
    type Value = T;

    async fn clone_value(&self) -> TaskValue<Self::Value> {
        self.value.lock().await.clone()
    }
}

#[async_trait]
impl<T, U> SharedValue for (T, U)
where
    T: SharedValue + Send + Sync,
    U: SharedValue + Send + Sync,
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
impl SharedValue for () {
    type Value = ();

    async fn clone_value(&self) -> TaskValue<Self::Value> {
        TaskValue::Stable(())
    }
}
