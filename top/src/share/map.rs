use std::ops::Deref;

use async_trait::async_trait;
use futures::lock::MutexGuard;
use uuid::Uuid;

use crate::prelude::TaskValue;
use crate::share::value::SharedValue;
use crate::share::{Share, SharedId, SharedRead, SharedWrite};

#[derive(Clone, Debug)]
pub struct MappedShare<S, T, F> {
    share: S,
    value: Share<T>,
    f: F,
}

pub trait SharedReadMapExt: SharedRead + Clone + Sized + Sync {
    fn map<T, F>(&self, f: F) -> MappedShare<Self, T, F>
    where
        T: Clone,
        F: Fn(TaskValue<Self::Value>) -> TaskValue<T> + Send + Sync,
        Self::Value: Clone,
    {
        let value = Share::new(f(futures::executor::block_on(self.read()).deref().clone()));
        MappedShare {
            share: self.clone(),
            value,
            f,
        }
    }
}

impl<T> SharedReadMapExt for T where T: SharedRead + Clone + Sync {}

#[async_trait]
impl<S, T, F> SharedRead for MappedShare<S, T, F>
where
    S: SharedRead + Send + Sync,
    S::Value: Clone + Send + Sync,
    T: Send,
    F: Fn(TaskValue<S::Value>) -> TaskValue<T> + Send + Sync,
{
    type Value = T;

    async fn read(&self) -> MutexGuard<'_, TaskValue<Self::Value>> {
        let _ = self
            .value
            .write((self.f)(self.share.read().await.deref().clone()))
            .await;
        self.value.read().await
    }
}

impl<S, T, F> SharedId for MappedShare<S, T, F>
where
    S: SharedId,
{
    fn id(&self) -> Uuid {
        self.share.id()
    }
}

#[async_trait]
impl<S, T, F> SharedValue for MappedShare<S, T, F>
where
    S: SharedValue + Send + Sync,
    T: Send,
    F: Fn(TaskValue<S::Value>) -> TaskValue<T> + Send + Sync,
{
    type Value = T;

    async fn clone_value(&self) -> TaskValue<Self::Value> {
        (self.f)(self.share.clone_value().await)
    }
}
