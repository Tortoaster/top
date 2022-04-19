use std::marker::PhantomData;

use async_trait::async_trait;

use crate::html::event::Event;
use crate::task::{Context, OptionExt, Task, TaskError, TaskResult};

#[derive(Debug)]
pub struct Left;

#[derive(Debug)]
pub struct Right;

#[derive(Debug)]
pub struct Both;

// TODO: Useful?
#[derive(Debug)]
pub struct Either;

#[derive(Debug)]
pub struct Parallel<T1, T2, F> {
    tasks: (T1, T2),
    combine: PhantomData<F>,
}

#[async_trait]
impl<T1, T2> Task for Parallel<T1, T2, Both>
where
    T1: Task,
    T2: Task,
    T1::Value: Send,
{
    type Value = (T1::Value, T2::Value);

    async fn start(&mut self, ctx: &mut Context) -> Result<(), TaskError> {
        self.tasks.0.start(ctx).await?;
        self.tasks.1.start(ctx).await?;
        Ok(())
    }

    async fn on_event(&mut self, event: Event, ctx: &mut Context) -> TaskResult<Self::Value> {
        let a = self.tasks.0.on_event(event.clone(), ctx).await?;
        let b = self.tasks.1.on_event(event, ctx).await?;
        let combined = a
            .into_option()
            .and_then(|a| b.into_option().map(|b| (a, b)))
            .into_unstable();

        Ok(combined)
    }

    async fn finish(&mut self, ctx: &mut Context) -> Result<(), TaskError> {
        self.tasks.0.finish(ctx).await?;
        self.tasks.1.finish(ctx).await?;

        Ok(())
    }
}

#[async_trait]
impl<T1, T2> Task for Parallel<T1, T2, Left>
where
    T1: Task,
    T2: Task,
    T1::Value: Send,
{
    type Value = T1::Value;

    async fn start(&mut self, ctx: &mut Context) -> Result<(), TaskError> {
        self.tasks.0.start(ctx).await?;
        self.tasks.1.start(ctx).await?;
        Ok(())
    }

    async fn on_event(&mut self, event: Event, ctx: &mut Context) -> TaskResult<Self::Value> {
        let a = self.tasks.0.on_event(event.clone(), ctx).await?;
        let _ = self.tasks.1.on_event(event, ctx).await?;

        Ok(a)
    }

    async fn finish(&mut self, ctx: &mut Context) -> Result<(), TaskError> {
        self.tasks.0.finish(ctx).await?;
        self.tasks.1.finish(ctx).await?;

        Ok(())
    }
}

#[async_trait]
impl<T1, T2> Task for Parallel<T1, T2, Right>
where
    T1: Task,
    T2: Task,
    T1::Value: Send,
{
    type Value = T2::Value;

    async fn start(&mut self, ctx: &mut Context) -> Result<(), TaskError> {
        self.tasks.0.start(ctx).await?;
        self.tasks.1.start(ctx).await?;
        Ok(())
    }

    async fn on_event(&mut self, event: Event, ctx: &mut Context) -> TaskResult<Self::Value> {
        let _ = self.tasks.0.on_event(event.clone(), ctx).await?;
        let b = self.tasks.1.on_event(event, ctx).await?;

        Ok(b)
    }

    async fn finish(&mut self, ctx: &mut Context) -> Result<(), TaskError> {
        self.tasks.0.finish(ctx).await?;
        self.tasks.1.finish(ctx).await?;

        Ok(())
    }
}

#[async_trait]
impl<T1, T2> Task for Parallel<T1, T2, Either>
where
    T1: Task,
    T2: Task<Value = T1::Value>,
    T1::Value: Send,
{
    type Value = T1::Value;

    async fn start(&mut self, ctx: &mut Context) -> Result<(), TaskError> {
        self.tasks.0.start(ctx).await?;
        self.tasks.1.start(ctx).await?;
        Ok(())
    }

    async fn on_event(&mut self, event: Event, ctx: &mut Context) -> TaskResult<Self::Value> {
        let a = self.tasks.0.on_event(event.clone(), ctx).await?;
        let b = self.tasks.1.on_event(event, ctx).await?;
        let combined = a.into_option().or(b.into_option()).into_unstable();

        Ok(combined)
    }

    async fn finish(&mut self, ctx: &mut Context) -> Result<(), TaskError> {
        self.tasks.0.finish(ctx).await?;
        self.tasks.1.finish(ctx).await?;

        Ok(())
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
