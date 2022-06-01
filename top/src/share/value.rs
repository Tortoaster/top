use async_trait::async_trait;

use crate::task::TaskValue;

#[async_trait]
pub trait SharedValue {
    type Value;

    async fn clone_value(&self) -> TaskValue<Self::Value>;
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
