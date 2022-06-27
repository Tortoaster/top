use futures::lock::MutexGuard;
use std::ops::{Deref, DerefMut};

pub enum ShareGuard<'a, T> {
    Guard(MutexGuard<'a, T>),
    Value(T),
}

impl<'a, T> ShareGuard<'a, T> {
    pub fn map<U>(self, f: impl FnOnce(&T) -> U) -> ShareGuard<'a, U> {
        match self {
            ShareGuard::Guard(guard) => ShareGuard::Value(f(guard.deref())),
            ShareGuard::Value(value) => ShareGuard::Value(f(&value)),
        }
    }
}

impl<T> Deref for ShareGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            ShareGuard::Guard(guard) => guard.deref(),
            ShareGuard::Value(value) => value,
        }
    }
}

impl<T> DerefMut for ShareGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            ShareGuard::Guard(guard) => guard.deref_mut(),
            ShareGuard::Value(value) => value,
        }
    }
}

impl<'a, T> From<MutexGuard<'a, T>> for ShareGuard<'a, T> {
    fn from(guard: MutexGuard<'a, T>) -> Self {
        ShareGuard::Guard(guard)
    }
}
