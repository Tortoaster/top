#[derive(Debug)]
pub enum TaskValue<T> {
    Stable(T),
    Unstable(T),
    Empty,
}

impl<T> TaskValue<T> {
    pub fn unwrap(self) -> T {
        match self {
            TaskValue::Stable(value) => value,
            TaskValue::Unstable(value) => value,
            TaskValue::Empty => panic!("unwrap on `Empty` value"),
        }
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
