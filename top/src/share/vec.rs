use std::collections::BTreeSet;
use std::sync::{Arc, Mutex, MutexGuard};

use uuid::Uuid;

use crate::share::{ShareChildren, ShareRead, ShareUpdate, ShareWrite};
use crate::task::{OptionExt, TaskValue};

#[derive(Clone, Debug)]
pub struct ShareVec<S> {
    shares: Arc<Mutex<Vec<S>>>,
}

impl<S> ShareVec<S>
where
    S: ShareWrite,
{
    pub fn new(value: Option<Vec<S::Value>>) -> Self {
        ShareVec::create(value.into_unstable())
    }
}

pub struct VecWrapper<T>(TaskValue<Vec<T>>);

impl<T> AsRef<TaskValue<Vec<T>>> for VecWrapper<T> {
    fn as_ref(&self) -> &TaskValue<Vec<T>> {
        &self.0
    }
}

impl<S> ShareRead for ShareVec<S>
where
    S: ShareRead,
    S::Value: Clone,
{
    type Value = Vec<S::Value>;
    type Read<'a> = VecWrapper<S::Value> where S: 'a;

    fn read<'a>(&'a self) -> Self::Read<'a> {
        let vec = self
            .shares
            .lock()
            .unwrap()
            .iter()
            .map(|share| share.read().as_ref().clone())
            .collect();
        VecWrapper(vec)
    }
}

impl<S> ShareWrite for ShareVec<S>
where
    S: ShareWrite,
{
    type Value = Vec<S::Value>;

    fn create(value: TaskValue<Self::Value>) -> Self {
        let shares = match value {
            TaskValue::Stable(value) => value
                .into_iter()
                .map(|value| S::create(TaskValue::Stable(value)))
                .collect(),
            TaskValue::Unstable(value) => value
                .into_iter()
                .map(|value| S::create(TaskValue::Unstable(value)))
                .collect(),
            TaskValue::Error(error) => vec![S::create(TaskValue::Error(error))],
            TaskValue::Empty => vec![S::create(TaskValue::Empty)],
        };
        ShareVec {
            shares: Arc::new(Mutex::new(shares)),
        }
    }

    fn write(&self, value: TaskValue<Self::Value>) {
        *self.shares.lock().unwrap() = match value {
            TaskValue::Stable(value) => value
                .into_iter()
                .map(|value| S::create(TaskValue::Stable(value)))
                .collect(),
            TaskValue::Unstable(value) => value
                .into_iter()
                .map(|value| S::create(TaskValue::Unstable(value)))
                .collect(),
            TaskValue::Error(error) => vec![S::create(TaskValue::Error(error))],
            TaskValue::Empty => vec![S::create(TaskValue::Empty)],
        };
    }
}

impl<S> ShareUpdate for ShareVec<S>
where
    S: ShareUpdate,
{
    fn id(&self) -> Uuid {
        todo!()
    }

    fn updated(&self, _ids: &BTreeSet<Uuid>) -> bool {
        // self.shares.iter().any(|share| share.updated(ids))
        true
    }
}

impl<S> ShareChildren for ShareVec<S> {
    type Child = S;

    fn children(&self) -> MutexGuard<Vec<Self::Child>> {
        self.shares.lock().unwrap()
    }
}
