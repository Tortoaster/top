use std::sync::Arc;

use async_trait::async_trait;
use futures::lock::Mutex;
use uuid::Uuid;

use crate::html::event::Feedback;
use crate::share::guard::ShareGuard;
use crate::share::{ShareConsume, ShareId, ShareRead, ShareWrite};
use crate::task::TaskValue;

#[derive(Clone, Debug)]
pub struct ShareOption<S> {
    share: S,
    enabled: Arc<Mutex<bool>>,
}

impl<S> ShareOption<S> {
    pub fn new(share: S, enabled: bool) -> Self {
        ShareOption {
            share,
            enabled: Arc::new(Mutex::new(enabled)),
        }
    }

    pub async fn is_some(&self) -> bool {
        *self.enabled.lock().await
    }

    pub fn inner(&self) -> &S {
        &self.share
    }
}

impl<S> ShareOption<S>
where
    S: ShareId,
{
    pub async fn enable(&self) -> Feedback {
        *self.enabled.lock().await = true;
        Feedback::update_share(self.id())
    }

    pub async fn disable(&self) -> Feedback {
        *self.enabled.lock().await = false;
        Feedback::update_share(self.id())
    }
}

#[async_trait]
impl<S> ShareConsume for ShareOption<S>
where
    S: ShareConsume + Send + Sync,
{
    type Value = Option<S::Value>;

    async fn consume(self) -> TaskValue<Self::Value> {
        if *self.enabled.lock().await {
            self.share.consume().await.map(Some)
        } else {
            TaskValue::Empty
        }
    }
}

impl<S> ShareId for ShareOption<S>
where
    S: ShareId,
{
    fn id(&self) -> Uuid {
        self.share.id()
    }
}

#[async_trait]
impl<S> ShareRead for ShareOption<S>
where
    S: ShareRead + Send + Sync,
    S::Value: Clone,
{
    async fn read(&self) -> ShareGuard<'_, TaskValue<Self::Value>> {
        if *self.enabled.lock().await {
            self.share.read().await.map(|value| value.clone().map(Some))
        } else {
            ShareGuard::Value(TaskValue::Unstable(None))
        }
    }
}

#[async_trait]
impl<S> ShareWrite for ShareOption<S>
where
    S: ShareWrite + Send + Sync,
    S::Value: Clone + Send,
{
    async fn write(&self, value: TaskValue<Self::Value>) -> Feedback {
        let enabled = value.as_ref().map(Option::is_some).unwrap_or_default();
        *self.enabled.lock().await = enabled;
        if enabled {
            self.share.write(value.map(Option::unwrap)).await
        } else {
            Feedback::new()
        }
    }
}
