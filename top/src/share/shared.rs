use std::sync::Arc;

use async_trait::async_trait;
use futures::lock::{Mutex, MutexGuard};
use uuid::Uuid;

use crate::html::event::Feedback;
use crate::share::{Share, ShareId, ShareRead, ShareWrite};
use crate::task::TaskValue;

#[derive(Clone, Debug)]
pub struct Shared<T> {
    id: Uuid,
    value: Arc<Mutex<TaskValue<T>>>,
}

impl<T> Shared<T> {
    pub fn new(value: TaskValue<T>) -> Self {
        Shared {
            id: Uuid::new_v4(),
            value: Arc::new(Mutex::new(value)),
        }
    }
}

impl<T> ShareId for Shared<T> {
    fn id(&self) -> Uuid {
        self.id
    }
}

#[async_trait]
impl<T> ShareRead for Shared<T>
where
    T: Clone + Send,
{
    async fn read(&self) -> MutexGuard<'_, TaskValue<Self::Value>> {
        self.value.lock().await
    }
}

#[async_trait]
impl<T> ShareWrite for Shared<T>
where
    T: Clone + Send,
{
    async fn write(&self, value: TaskValue<Self::Value>) -> Feedback {
        *self.value.lock().await = value;
        Feedback::update_share(self.id)
    }
}

#[async_trait]
impl<T> Share for Shared<T>
where
    T: Clone + Send,
{
    type Value = T;

    async fn clone_value(&self) -> TaskValue<Self::Value> {
        self.value.lock().await.clone()
    }
}
