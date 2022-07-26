use async_trait::async_trait;

// pub mod edit;
pub mod parallel;
pub mod sequential;
pub mod view;

#[async_trait]
pub trait Value {
    type Output;

    async fn value(self) -> TaskValue<Self::Output>;
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum TaskValue<T> {
    /// The task's value is stable, meaning it cannot be changed by the user anymore.
    Stable(T),
    /// The task's value is unstable, meaning the user can still change it.
    Unstable(T),
    /// The task has no value yet.
    Empty,
}

impl<T> TaskValue<T> {
    pub fn as_ref(&self) -> TaskValue<&T> {
        match *self {
            TaskValue::Stable(ref x) | TaskValue::Unstable(ref x) => TaskValue::Unstable(x),
            TaskValue::Empty => TaskValue::Empty,
        }
    }

    pub fn map<U, F>(self, f: F) -> TaskValue<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            TaskValue::Stable(x) | TaskValue::Unstable(x) => TaskValue::Unstable(f(x)),
            TaskValue::Empty => TaskValue::Empty,
        }
    }

    pub fn unwrap(self) -> T {
        match self {
            TaskValue::Stable(value) | TaskValue::Unstable(value) => value,
            TaskValue::Empty => panic!("called `TaskValue::unwrap` on an `Empty` value"),
        }
    }

    pub fn unwrap_or(self, default: T) -> T {
        match self {
            TaskValue::Stable(x) | TaskValue::Unstable(x) => x,
            TaskValue::Empty => default,
        }
    }

    pub fn unwrap_or_default(self) -> T
    where
        T: Default,
    {
        match self {
            TaskValue::Stable(x) | TaskValue::Unstable(x) => x,
            TaskValue::Empty => Default::default(),
        }
    }

    pub fn is_stable(&self) -> bool {
        match self {
            TaskValue::Stable(_) => true,
            _ => false,
        }
    }

    pub fn is_unstable(&self) -> bool {
        match self {
            TaskValue::Unstable(_) => true,
            _ => false,
        }
    }

    pub fn has_value(&self) -> bool {
        match self {
            TaskValue::Empty => false,
            _ => true,
        }
    }

    pub fn and<U>(self, other: TaskValue<U>) -> TaskValue<(T, U)> {
        match self {
            TaskValue::Stable(a) => match other {
                TaskValue::Stable(b) => TaskValue::Stable((a, b)),
                TaskValue::Unstable(b) => TaskValue::Unstable((a, b)),
                TaskValue::Empty => TaskValue::Empty,
            },
            TaskValue::Unstable(a) => match other {
                TaskValue::Stable(b) | TaskValue::Unstable(b) => TaskValue::Unstable((a, b)),
                TaskValue::Empty => TaskValue::Empty,
            },
            TaskValue::Empty => TaskValue::Empty,
        }
    }

    pub fn or(self, other: TaskValue<T>) -> TaskValue<T> {
        match self {
            TaskValue::Empty => other,
            _ => self,
        }
    }
}

impl<T> From<TaskValue<T>> for Option<T> {
    fn from(value: TaskValue<T>) -> Self {
        match value {
            TaskValue::Stable(x) | TaskValue::Unstable(x) => Some(x),
            TaskValue::Empty => None,
        }
    }
}

impl<T> Default for TaskValue<T> {
    fn default() -> Self {
        TaskValue::Empty
    }
}

pub trait OptionExt {
    type Value;

    fn into_stable(self) -> TaskValue<Self::Value>;

    fn into_unstable(self) -> TaskValue<Self::Value>;
}

impl<T> OptionExt for Option<T> {
    type Value = T;

    fn into_stable(self) -> TaskValue<Self::Value> {
        match self {
            None => TaskValue::Empty,
            Some(value) => TaskValue::Stable(value),
        }
    }

    fn into_unstable(self) -> TaskValue<Self::Value> {
        match self {
            None => TaskValue::Empty,
            Some(value) => TaskValue::Unstable(value),
        }
    }
}
