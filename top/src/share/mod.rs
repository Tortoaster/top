use std::collections::BTreeSet;

use uuid::Uuid;

use crate::task::TaskValue;
pub use value::ShareValue;

mod value;

pub trait ShareRead {
    type Value;
    type Borrow<'a>: AsRef<TaskValue<Self::Value>>
    where
        Self: 'a;

    fn read<'a>(&'a self) -> Self::Borrow<'a>;
}

pub trait ShareWrite {
    type Value;

    fn write(&mut self, value: TaskValue<Self::Value>);
}

pub trait ShareUpdate {
    fn id(&self) -> Uuid;

    fn updated(&self, ids: &BTreeSet<Uuid>) -> bool;
}
