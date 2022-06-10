use std::marker::PhantomData;

use async_trait::async_trait;

use top_derive::html;

use crate::html::event::{Event, Feedback};
use crate::html::{Handler, Html, ToHtml};
use crate::share::SharedValue;
use crate::task::{Task, TaskValue};

#[derive(Debug)]
pub struct Left;

#[derive(Debug)]
pub struct Right;

#[derive(Debug)]
pub struct Both;

#[derive(Debug)]
pub struct Either;

#[derive(Debug)]
pub struct Parallel<T1, T2, F> {
    tasks: (T1, T2),
    combine: PhantomData<F>,
}

#[async_trait]
impl<T1, T2, F> ToHtml for Parallel<T1, T2, F>
where
    T1: ToHtml + Send + Sync,
    T2: ToHtml + Send + Sync,
    F: Send + Sync,
{
    async fn to_html(&self) -> Html {
        let left = self.tasks.0.to_html().await;
        let right = self.tasks.1.to_html().await;

        html! {r#"
            {left}
            {right}
        "#}
    }
}

#[async_trait]
impl<T1, T2, F> Handler for Parallel<T1, T2, F>
where
    T1: Handler + Send + Sync,
    T2: Handler + Send + Sync,
    F: Send + Sync,
{
    async fn on_event(&mut self, event: Event) -> Feedback {
        let a = self.tasks.0.on_event(event.clone()).await;
        let b = self.tasks.1.on_event(event).await;

        a.merged_with(b).unwrap()
    }
}

#[async_trait]
impl<T1, T2> Task for Parallel<T1, T2, Both>
where
    T1: Task + Send + Sync,
    T2: Task + Send + Sync,
    T1::Value: Send,
    T1::Share: Send + Sync,
    T2::Share: Send + Sync,
    <T1::Share as SharedValue>::Value: Send,
{
    type Value = (T1::Value, T2::Value);
    type Share = (T1::Share, T2::Share);

    async fn share(&self) -> Self::Share {
        let a = self.tasks.0.share().await;
        let b = self.tasks.1.share().await;

        (a, b)
    }

    async fn value(self) -> TaskValue<Self::Value> {
        let a = self.tasks.0.value().await;
        let b = self.tasks.1.value().await;

        a.and(b)
    }
}

#[async_trait]
impl<T1, T2> Task for Parallel<T1, T2, Left>
where
    T1: Task + Send + Sync,
    T2: Task + Send + Sync,
{
    type Value = T1::Value;
    type Share = T1::Share;

    async fn share(&self) -> Self::Share {
        self.tasks.0.share().await
    }

    async fn value(self) -> TaskValue<Self::Value> {
        self.tasks.0.value().await
    }
}

#[async_trait]
impl<T1, T2> Task for Parallel<T1, T2, Right>
where
    T1: Task + Send + Sync,
    T2: Task + Send + Sync,
{
    type Value = T2::Value;
    type Share = T2::Share;

    async fn share(&self) -> Self::Share {
        self.tasks.1.share().await
    }

    async fn value(self) -> TaskValue<Self::Value> {
        self.tasks.1.value().await
    }
}

#[async_trait]
impl<T1, T2> Task for Parallel<T1, T2, Either>
where
    T1: Task + Send + Sync,
    T2: Task<Value = T1::Value, Share = T1::Share> + Send + Sync,
    T1::Value: Send,
{
    type Value = T1::Value;
    type Share = ();

    async fn share(&self) -> Self::Share {
        ()
    }

    async fn value(self) -> TaskValue<Self::Value> {
        let a = self.tasks.0.value().await;
        let b = self.tasks.1.value().await;

        a.or(b)
    }
}

pub trait TaskParallelExt: Task {
    fn and<T>(self, other: T) -> Parallel<Self, T, Both>
    where
        Self: Sized,
    {
        Parallel {
            tasks: (self, other),
            combine: PhantomData,
        }
    }

    fn or<T>(self, other: T) -> Parallel<Self, T, Either>
    where
        Self: Sized,
    {
        Parallel {
            tasks: (self, other),
            combine: PhantomData,
        }
    }

    fn left<T>(self, other: T) -> Parallel<Self, T, Left>
    where
        Self: Sized,
    {
        Parallel {
            tasks: (self, other),
            combine: PhantomData,
        }
    }

    fn right<T>(self, other: T) -> Parallel<Self, T, Right>
    where
        Self: Sized,
    {
        Parallel {
            tasks: (self, other),
            combine: PhantomData,
        }
    }
}

impl<T> TaskParallelExt for T where T: Task {}
