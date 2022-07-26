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

    fn updated(&self, ids: &BTreeSet<Uuid>) -> bool;
}

pub trait ShareWrite {
    type Value;

    fn write(&mut self, value: Self::Value);
}
