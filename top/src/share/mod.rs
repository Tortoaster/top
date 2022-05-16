use std::sync::Arc;

use futures::lock::{Mutex, MutexGuard};

#[derive(Clone, Debug)]
pub struct Share<T> {
    value: Arc<Mutex<T>>,
}

impl<T> Share<T> {
    pub fn new(value: T) -> Self {
        Share {
            value: Arc::new(Mutex::new(value)),
        }
    }

    pub async fn read(&self) -> MutexGuard<'_, T> {
        self.value.lock().await
    }

    pub async fn write(&self, value: T) {
        *self.value.lock().await = value;
    }

    pub fn into_inner(self) -> T {
        self.value.into_inner()
    }
}
