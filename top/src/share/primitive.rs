use std::sync::Arc;

use async_trait::async_trait;
use futures::lock::{Mutex, MutexGuard};
use uuid::Uuid;

use crate::html::event::Feedback;
use crate::share::value::SharedValue;
use crate::share::{SharedId, SharedRead, SharedWrite};
use crate::task::TaskValue;

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
impl<T> SharedValue for Share<T>
where
    T: Clone + Send,
{
    type Value = T;

    async fn clone_value(&self) -> TaskValue<Self::Value> {
        self.value.lock().await.clone()
    }
}
