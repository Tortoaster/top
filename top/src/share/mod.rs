use std::sync::Arc;

use async_trait::async_trait;
use futures::lock::{Mutex, MutexGuard};
use uuid::Uuid;

use crate::html::event::Feedback;
use crate::prelude::TaskValue;

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

    pub async fn read(&self) -> MutexGuard<'_, TaskValue<T>> {
        self.value.lock().await
    }

    pub async fn write(&self, value: TaskValue<T>) -> Feedback {
        *self.value.lock().await = value;
        Feedback::update_share(self.id)
    }

    pub fn id(&self) -> Uuid {
        self.id
    }
}

#[async_trait]
pub trait ShareValue {
    type Value;

    async fn clone_value(&self) -> TaskValue<Self::Value>;
}

#[async_trait]
impl<T> ShareValue for Share<T>
where
    T: Clone + Send,
{
    type Value = T;

    async fn clone_value(&self) -> TaskValue<Self::Value> {
        self.value.lock().await.clone()
    }
}

#[async_trait]
impl<T, U> ShareValue for (T, U)
where
    T: ShareValue + Send + Sync,
    U: ShareValue + Send + Sync,
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
impl ShareValue for () {
    type Value = ();

    async fn clone_value(&self) -> TaskValue<Self::Value> {
        TaskValue::Stable(())
    }
}
