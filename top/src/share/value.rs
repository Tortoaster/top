use std::collections::BTreeSet;
use std::ops::Deref;
use std::sync::{Arc, Mutex, MutexGuard};

use uuid::Uuid;

use crate::share::{ShareRead, ShareUpdate, ShareWrite};
use crate::task::{OptionExt, TaskValue};

#[derive(Clone, Debug)]
pub struct ShareValue<T> {
    id: Uuid,
    value: Arc<Mutex<TaskValue<T>>>,
}

impl<T> ShareValue<T> {
    pub fn new(value: Option<T>) -> Self {
        ShareValue {
            id: Uuid::new_v4(),
            value: Arc::new(Mutex::new(value.into_unstable())),
        }
    }
}

pub struct ShareGuard<'a, T>(MutexGuard<'a, TaskValue<T>>);

impl<'a, T> AsRef<TaskValue<T>> for ShareGuard<'a, T> {
    fn as_ref(&self) -> &TaskValue<T> {
        self.0.deref()
    }
}

impl<'a, T> From<MutexGuard<'a, TaskValue<T>>> for ShareGuard<'a, T> {
    fn from(guard: MutexGuard<'a, TaskValue<T>>) -> Self {
        ShareGuard(guard)
    }
}

impl<T> ShareRead for ShareValue<T> {
    type Value = T;
    type Borrow<'a> = ShareGuard<'a, T> where T: 'a;

    fn read<'a>(&'a self) -> Self::Borrow<'a> {
        self.value.lock().unwrap().into()
    }
}

impl<T> ShareWrite for ShareValue<T> {
    type Value = T;

    fn write(&mut self, value: TaskValue<Self::Value>) {
        *self.value.lock().unwrap() = value;
    }
}

impl<T> ShareUpdate for ShareValue<T> {
    fn id(&self) -> Uuid {
        self.id
    }

    fn updated(&self, ids: &BTreeSet<Uuid>) -> bool {
        ids.contains(&self.id)
    }
}
