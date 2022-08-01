use async_trait::async_trait;

pub mod edit;
pub mod parallel;
pub mod sequential;
pub mod view;

#[async_trait]
pub trait Value {
    type Output;

    async fn value(self) -> TaskValue<Self::Output>;
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum TaskValue<T> {
    /// The task's value is stable, meaning it cannot be changed by the user anymore.
    Stable(T),
    /// The task's value is unstable, meaning the user can still change it.
    Unstable(T),
    /// The task has an invalid value.
    Error(String),
    /// The task has no value yet.
    #[default]
    Empty,
}

impl<T> TaskValue<T> {
    pub fn as_ref(&self) -> TaskValue<&T> {
        match *self {
            TaskValue::Stable(ref x) | TaskValue::Unstable(ref x) => TaskValue::Unstable(x),
            TaskValue::Error(_) | TaskValue::Empty => TaskValue::Empty,
        }
    }

    pub fn map<U, F>(self, f: F) -> TaskValue<U>
    where
        F: FnOnce(T) -> U,
    {
        match self {
            TaskValue::Stable(x) | TaskValue::Unstable(x) => TaskValue::Unstable(f(x)),
            TaskValue::Error(_) | TaskValue::Empty => TaskValue::Empty,
        }
    }

    pub fn unwrap(self) -> T {
        match self {
            TaskValue::Stable(value) | TaskValue::Unstable(value) => value,
            TaskValue::Error(_) | TaskValue::Empty => {
                panic!("called `TaskValue::unwrap` on an `Error` or `Empty` value")
            }
        }
    }

    pub fn unwrap_or(self, default: T) -> T {
        match self {
            TaskValue::Stable(x) | TaskValue::Unstable(x) => x,
            TaskValue::Error(_) | TaskValue::Empty => default,
        }
    }

    pub fn unwrap_or_default(self) -> T
    where
        T: Default,
    {
        match self {
            TaskValue::Stable(x) | TaskValue::Unstable(x) => x,
            TaskValue::Error(_) | TaskValue::Empty => T::default(),
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

    pub fn is_error(&self) -> bool {
        match self {
            TaskValue::Error(_) => true,
            _ => false,
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            TaskValue::Empty => true,
            _ => false,
        }
    }

    pub fn and<U>(self, other: TaskValue<U>) -> TaskValue<(T, U)> {
        match self {
            TaskValue::Stable(a) => match other {
                TaskValue::Stable(b) => TaskValue::Stable((a, b)),
                TaskValue::Unstable(b) => TaskValue::Unstable((a, b)),
                TaskValue::Error(error) => TaskValue::Error(error),
                TaskValue::Empty => TaskValue::Empty,
            },
            TaskValue::Unstable(a) => match other {
                TaskValue::Stable(b) | TaskValue::Unstable(b) => TaskValue::Unstable((a, b)),
                TaskValue::Error(error) => TaskValue::Error(error),
                TaskValue::Empty => TaskValue::Empty,
            },
            TaskValue::Error(error) => match other {
                TaskValue::Stable(_) | TaskValue::Unstable(_) | TaskValue::Empty => {
                    TaskValue::Error(error)
                }
                TaskValue::Error(other) => TaskValue::Error(format!("{error}\n{other}")),
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

impl<T> IntoIterator for TaskValue<T> {
    type Item = T;
    type IntoIter = <Option<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        Option::from(self).into_iter()
    }
}

impl<A, V: FromIterator<A>> FromIterator<TaskValue<A>> for TaskValue<V> {
    fn from_iter<T: IntoIterator<Item = TaskValue<A>>>(iter: T) -> Self {
        iter.into_iter()
            .map(Into::<Option<A>>::into)
            .collect::<Option<V>>()
            .into_unstable()
    }
}

impl<T> From<TaskValue<T>> for Option<T> {
    fn from(value: TaskValue<T>) -> Self {
        match value {
            TaskValue::Stable(x) | TaskValue::Unstable(x) => Some(x),
            TaskValue::Error(_) | TaskValue::Empty => None,
        }
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
