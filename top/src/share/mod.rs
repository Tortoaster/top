use std::collections::BTreeSet;
use std::sync::MutexGuard;

use uuid::Uuid;

use crate::task::TaskValue;
pub use value::ShareValue;
pub use vec::ShareVec;

mod value;
mod vec;

pub trait ShareRead {
    type Value;
    type Read<'a>: AsRef<TaskValue<Self::Value>>
    where
        Self: 'a;

    fn read<'a>(&'a self) -> Self::Read<'a>;
}

pub trait ShareWrite {
    type Value;

    fn create(value: TaskValue<Self::Value>) -> Self;

    fn write(&self, value: TaskValue<Self::Value>);
}

pub trait ShareUpdate {
    fn id(&self) -> Uuid;

    fn updated(&self, ids: &BTreeSet<Uuid>) -> bool;
}

pub trait ShareChildren {
    type Child;

    fn children(&self) -> MutexGuard<Vec<Self::Child>>;
}
