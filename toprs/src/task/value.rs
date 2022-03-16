use std::mem;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum TaskValue<T> {
    Stable(T),
    Unstable(T),
    Empty,
}

impl<T> TaskValue<T> {
    pub fn into_option(self) -> Option<T> {
        match self {
            TaskValue::Stable(t) => Some(t),
            TaskValue::Unstable(t) => Some(t),
            TaskValue::Empty => None,
        }
    }

    pub const fn as_ref(&self) -> TaskValue<&T> {
        match *self {
            TaskValue::Stable(ref x) => TaskValue::Stable(x),
            TaskValue::Unstable(ref x) => TaskValue::Unstable(x),
            TaskValue::Empty => TaskValue::Empty,
        }
    }

    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> TaskValue<U> {
        match self {
            TaskValue::Stable(x) => TaskValue::Stable(f(x)),
            TaskValue::Unstable(x) => TaskValue::Unstable(f(x)),
            TaskValue::Empty => TaskValue::Empty,
        }
    }

    pub fn take(&mut self) -> Option<T> {
        mem::take(self).into_option()
    }
}

impl<T: Clone> TaskValue<&T> {
    pub fn cloned(self) -> TaskValue<T> {
        self.map(|t| t.clone())
    }
}

impl<T> Default for TaskValue<T> {
    fn default() -> Self {
        TaskValue::Empty
    }
}

pub trait OptionExt<T>: private::Sealed {
    fn into_stable(self) -> TaskValue<T>;

    fn into_unstable(self) -> TaskValue<T>;
}

impl<T> OptionExt<T> for Option<T> {
    fn into_stable(self) -> TaskValue<T> {
        match self {
            Some(t) => TaskValue::Stable(t),
            None => TaskValue::Empty,
        }
    }

    fn into_unstable(self) -> TaskValue<T> {
        match self {
            Some(t) => TaskValue::Unstable(t),
            None => TaskValue::Empty,
        }
    }
}

mod private {
    pub trait Sealed {}

    impl<T> Sealed for Option<T> {}
}
