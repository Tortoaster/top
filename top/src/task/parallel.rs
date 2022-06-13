use std::marker::PhantomData;

use async_trait::async_trait;
use uuid::Uuid;

use top_derive::html;

use crate::html::event::{Event, Feedback};
use crate::html::{Handler, Html, Refresh, ToHtml};
use crate::share::Share;
use crate::task::{TaskValue, Value};

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
impl<T1, T2, F> Refresh for Parallel<T1, T2, F>
where
    T1: Refresh + Send + Sync,
    T2: Refresh + Send + Sync,
    F: Send + Sync,
{
    async fn refresh(&self, id: Uuid) -> Feedback {
        let a = self.tasks.0.refresh(id).await;
        let b = self.tasks.1.refresh(id).await;

        a.merged_with(b).unwrap()
    }
}

#[async_trait]
impl<T1, T2> Value for Parallel<T1, T2, Both>
where
    T1: Value + Send + Sync,
    T2: Value + Send + Sync,
    T1::Output: Send,
    T1::Share: Send + Sync,
    T2::Share: Send + Sync,
    <T1::Share as Share>::Value: Send,
{
    type Output = (T1::Output, T2::Output);
    type Share = (T1::Share, T2::Share);

    async fn share(&self) -> Self::Share {
        let a = self.tasks.0.share().await;
        let b = self.tasks.1.share().await;

        (a, b)
    }

    async fn value(self) -> TaskValue<Self::Output> {
        let a = self.tasks.0.value().await;
        let b = self.tasks.1.value().await;

        a.and(b)
    }
}

#[async_trait]
impl<T1, T2> Value for Parallel<T1, T2, Left>
where
    T1: Value + Send + Sync,
    T2: Value + Send + Sync,
{
    type Output = T1::Output;
    type Share = T1::Share;

    async fn share(&self) -> Self::Share {
        self.tasks.0.share().await
    }

    async fn value(self) -> TaskValue<Self::Output> {
        self.tasks.0.value().await
    }
}

#[async_trait]
impl<T1, T2> Value for Parallel<T1, T2, Right>
where
    T1: Value + Send + Sync,
    T2: Value + Send + Sync,
{
    type Output = T2::Output;
    type Share = T2::Share;

    async fn share(&self) -> Self::Share {
        self.tasks.1.share().await
    }

    async fn value(self) -> TaskValue<Self::Output> {
        self.tasks.1.value().await
    }
}

#[async_trait]
impl<T1, T2> Value for Parallel<T1, T2, Either>
where
    T1: Value + Send + Sync,
    T2: Value<Output = T1::Output, Share = T1::Share> + Send + Sync,
    T1::Output: Send,
{
    type Output = T1::Output;
    type Share = ();

    async fn share(&self) -> Self::Share {
        ()
    }

    async fn value(self) -> TaskValue<Self::Output> {
        let a = self.tasks.0.value().await;
        let b = self.tasks.1.value().await;

        a.or(b)
    }
}

pub trait TaskParallelExt: Value {
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

impl<T> TaskParallelExt for T where T: Value {}
