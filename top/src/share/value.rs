use std::sync::Arc;

use async_trait::async_trait;
use futures::lock::Mutex;
use uuid::Uuid;

use crate::html::event::Feedback;
use crate::share::guard::ShareGuard;
use crate::share::{ShareConsume, ShareId, ShareRead, ShareWrite};
use crate::task::TaskValue;

#[derive(Clone, Debug)]
pub struct ShareValue<T> {
    id: Uuid,
    value: Arc<Mutex<TaskValue<T>>>,
}

impl<T> ShareValue<T> {
    pub fn new(value: TaskValue<T>) -> Self {
        ShareValue {
            id: Uuid::new_v4(),
            value: Arc::new(Mutex::new(value)),
        }
    }
}

impl<T> ShareId for ShareValue<T> {
    fn id(&self) -> Uuid {
        self.id
    }
}

#[async_trait]
impl<T> ShareRead for ShareValue<T>
where
    T: Clone + Send,
{
    async fn read(&self) -> ShareGuard<'_, TaskValue<Self::Value>> {
        self.value.lock().await.into()
    }
}

#[async_trait]
impl<T> ShareWrite for ShareValue<T>
where
    T: Clone + Send,
{
    async fn write(&self, value: TaskValue<Self::Value>) -> Feedback {
        *self.value.lock().await = value;
        Feedback::update_share(self.id)
    }
}

#[async_trait]
impl<T> ShareConsume for ShareValue<T>
where
    T: Clone + Send,
{
    type Value = T;

    async fn consume(self) -> TaskValue<Self::Value> {
        self.value.lock().await.clone()
    }
}
