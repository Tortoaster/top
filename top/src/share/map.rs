use async_trait::async_trait;
use uuid::Uuid;

use crate::prelude::TaskValue;
use crate::share::guard::ShareGuard;
use crate::share::{ShareConsume, ShareId, ShareRead};

#[derive(Clone, Debug)]
pub struct Map<S, F> {
    share: S,
    f: F,
}

pub trait SharedReadMapExt: ShareRead + Clone + Sized + Sync {
    fn map<F, T>(&self, f: F) -> Map<Self, F>
    where
        F: Fn(&TaskValue<Self::Value>) -> TaskValue<T> + Send + Sync,
    {
        Map {
            share: self.clone(),
            f,
        }
    }
}

impl<T> SharedReadMapExt for T where T: ShareRead + Clone + Sync {}

impl<S, F> ShareId for Map<S, F>
where
    S: ShareId,
{
    fn id(&self) -> Uuid {
        self.share.id()
    }
}

#[async_trait]
impl<S, F, T> ShareRead for Map<S, F>
where
    S: ShareRead + Send + Sync,
    F: Fn(&TaskValue<S::Value>) -> TaskValue<T> + Send + Sync,
    T: Clone,
{
    async fn read(&self) -> ShareGuard<'_, TaskValue<Self::Value>> {
        self.share.read().await.map(&self.f)
    }
}

#[async_trait]
impl<S, F, T> ShareConsume for Map<S, F>
where
    S: ShareRead + Send + Sync,
    F: FnOnce(&TaskValue<S::Value>) -> TaskValue<T> + Send + Sync,
    T: Clone,
{
    type Value = T;

    async fn consume(self) -> TaskValue<Self::Value> {
        (self.f)(&self.share.consume().await)
    }
}
