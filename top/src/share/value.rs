use crate::share::{ShareRead, ShareWrite};
use std::collections::BTreeSet;
use std::ops::Deref;
use std::sync::{Arc, Mutex, MutexGuard};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct ShareValue<T> {
    id: Uuid,
    value: Arc<Mutex<T>>,
}

impl<T> ShareValue<T> {
    pub fn new(value: T) -> Self {
        ShareValue {
            id: Uuid::new_v4(),
            value: Arc::new(Mutex::new(value)),
        }
    }
}

pub struct ShareGuard<'a, T>(MutexGuard<'a, T>);

impl<'a, T> AsRef<T> for ShareGuard<'a, T> {
    fn as_ref(&self) -> &T {
        self.0.deref()
    }
}

impl<'a, T> From<MutexGuard<'a, T>> for ShareGuard<'a, T> {
    fn from(guard: MutexGuard<'a, T>) -> Self {
        ShareGuard(guard)
    }
}

impl<T> ShareRead for ShareValue<T> {
    type Value = T;
    type Borrow<'a> = ShareGuard<'a, T> where T: 'a;

    fn read<'a>(&'a self) -> Self::Borrow<'a> {
        self.value.lock().unwrap().into()
    }

    fn updated(&self, ids: &BTreeSet<Uuid>) -> bool {
        ids.contains(&self.id)
    }
}

impl<T> ShareWrite for ShareValue<T> {
    type Value = T;

    fn write(&mut self, value: Self::Value) {
        *self.value.lock().unwrap() = value;
    }
}
