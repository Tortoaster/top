use async_trait::async_trait;
use std::collections::BTreeSet;
use uuid::Uuid;

use crate::html::event::{Event, Feedback};
use crate::html::{Handler, Html, Refresh, ToHtml};
use crate::task::{TaskValue, Value};

#[derive(Debug)]
pub struct Parallel<L, R, F> {
    left: L,
    right: R,
    combine: F,
}

#[async_trait]
impl<L, R, F> ToHtml for Parallel<L, R, F>
where
    L: ToHtml + Send + Sync,
    R: ToHtml + Send + Sync,
    F: Send + Sync,
{
    async fn to_html(&self) -> Html {
        let left = self.left.to_html().await;
        let right = self.right.to_html().await;
        Html(format!("{left}{right}"))
    }
}

#[async_trait]
impl<L, R, F> Handler for Parallel<L, R, F>
where
    L: Handler + Send + Sync,
    R: Handler + Send + Sync,
    F: Send + Sync,
{
    async fn on_event(&mut self, event: Event) -> Feedback {
        let a = self.left.on_event(event.clone()).await;
        let b = self.right.on_event(event).await;

        a.merged_with(b).unwrap()
    }
}

#[async_trait]
impl<L, R, F> Refresh for Parallel<L, R, F>
where
    L: Refresh + Send + Sync,
    R: Refresh + Send + Sync,
    F: Send + Sync,
{
    async fn refresh(&mut self, ids: &BTreeSet<Uuid>) -> Feedback {
        let a = self.left.refresh(ids).await;
        let b = self.right.refresh(ids).await;
        a.merged_with(b).unwrap()
    }
}

#[async_trait]
impl<L, R, F, T> Value for Parallel<L, R, F>
where
    L: Value + Send + Sync,
    R: Value + Send + Sync,
    L::Output: Send,
    F: FnOnce(TaskValue<L::Output>, TaskValue<R::Output>) -> TaskValue<T> + Send + Sync,
    T: Send + Sync,
{
    type Output = T;

    async fn value(self) -> TaskValue<Self::Output> {
        let a = self.left.value().await;
        let b = self.right.value().await;
        (self.combine)(a, b)
    }
}

type And<L, R> = fn(TaskValue<L>, TaskValue<R>) -> TaskValue<(L, R)>;
type Or<T> = fn(TaskValue<T>, TaskValue<T>) -> TaskValue<T>;
type Left<L, R> = fn(TaskValue<L>, TaskValue<R>) -> TaskValue<L>;
type Right<L, R> = fn(TaskValue<L>, TaskValue<R>) -> TaskValue<R>;

pub trait TaskParallelExt: Value + Sized {
    fn and<R>(self, right: R) -> Parallel<Self, R, And<Self::Output, R::Output>>
    where
        R: Value,
    {
        Parallel {
            left: self,
            right,
            combine: |a: TaskValue<Self::Output>, b: TaskValue<R::Output>| a.and(b),
        }
    }

    fn or<R>(self, right: R) -> Parallel<Self, R, Or<Self::Output>>
    where
        R: Value<Output = Self::Output>,
    {
        Parallel {
            left: self,
            right,
            combine: |a: TaskValue<Self::Output>, b: TaskValue<R::Output>| a.or(b),
        }
    }

    fn left<R>(self, right: R) -> Parallel<Self, R, Left<Self::Output, R::Output>>
    where
        R: Value,
    {
        Parallel {
            left: self,
            right,
            combine: |a: TaskValue<Self::Output>, _: TaskValue<R::Output>| a,
        }
    }

    fn right<R>(self, right: R) -> Parallel<Self, R, Right<Self::Output, R::Output>>
    where
        R: Value,
    {
        Parallel {
            left: self,
            right,
            combine: |_: TaskValue<Self::Output>, b: TaskValue<R::Output>| b,
        }
    }
}

impl<T> TaskParallelExt for T where T: Value {}
