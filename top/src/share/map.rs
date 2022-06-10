use std::ops::Deref;

use async_trait::async_trait;
use futures::lock::MutexGuard;
use uuid::Uuid;

use crate::prelude::TaskValue;
use crate::share::{Share, ShareId, ShareRead, ShareWrite, Shared};

#[derive(Clone, Debug)]
pub struct Map<S, T, F> {
    share: S,
    value: Shared<T>,
    f: F,
}

pub trait SharedReadMapExt: ShareRead + Clone + Sized + Sync {
    fn map<T, F>(&self, f: F) -> Map<Self, T, F>
    where
        T: Clone,
        F: Fn(TaskValue<Self::Value>) -> TaskValue<T> + Send + Sync,
        Self::Value: Clone,
    {
        let value = Shared::new(f(futures::executor::block_on(self.read()).deref().clone()));
        Map {
            share: self.clone(),
            value,
            f,
        }
    }
}

impl<T> SharedReadMapExt for T where T: ShareRead + Clone + Sync {}

impl<S, T, F> ShareId for Map<S, T, F>
where
    S: ShareId,
{
    fn id(&self) -> Uuid {
        self.share.id()
    }
}

#[async_trait]
impl<S, T, F> ShareRead for Map<S, T, F>
where
    S: ShareRead + Send + Sync,
    S::Value: Clone + Send + Sync,
    T: Clone + Send,
    F: Fn(TaskValue<S::Value>) -> TaskValue<T> + Send + Sync,
{
    async fn read(&self) -> MutexGuard<'_, TaskValue<Self::Value>> {
        let _ = self
            .value
            .write((self.f)(self.share.read().await.deref().clone()))
            .await;
        self.value.read().await
    }
}

#[async_trait]
impl<S, T, F> Share for Map<S, T, F>
where
    S: Share + Send + Sync,
    T: Send,
    F: Fn(TaskValue<S::Value>) -> TaskValue<T> + Send + Sync,
{
    type Value = T;

    async fn clone_value(&self) -> TaskValue<Self::Value> {
        (self.f)(self.share.clone_value().await)
    }
}
