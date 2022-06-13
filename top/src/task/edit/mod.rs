use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::Deref;
use std::str::FromStr;

use async_trait::async_trait;
use serde::Serialize;
use uuid::Uuid;

use top_derive::html;

use crate::html::event::{Change, Event, Feedback};
use crate::html::{Handler, Html, Refresh, ToHtml};
use crate::share::{ShareId, ShareRead, ShareWrite, Shared};
use crate::task::{OptionExt, TaskValue, Value};

pub mod choice;
pub mod container;
pub mod convert;
pub mod generic;
pub mod tuple;

#[derive(Clone, Debug)]
pub struct Edit<S, T> {
    id: Uuid,
    share: S,
    label: Option<String>,
    // Necessary for the `ToHtml` impls.
    _type: PhantomData<T>,
}

impl<T> Edit<Shared<T>, T> {
    pub fn new(value: Option<T>) -> Self {
        Edit::new_shared(Shared::new(value.into_unstable()))
    }
}

impl<S, T> Edit<S, T> {
    pub fn new_shared(share: S) -> Self {
        Edit {
            id: Uuid::new_v4(),
            share,
            label: None,
            _type: PhantomData,
        }
    }

    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }
}

#[async_trait]
impl<S, T> Value for Edit<S, T>
where
    S: ShareId + ShareWrite<Value = T> + Clone + Send + Sync,
    T: Serialize + FromStr + Clone + Send + Sync,
    T::Err: Send,
{
    type Output = T;
    type Share = S;

    async fn share(&self) -> Self::Share {
        self.share.clone()
    }

    async fn value(self) -> TaskValue<Self::Output> {
        self.share.clone_value().await
    }
}

#[async_trait]
impl<S, T> Handler for Edit<S, T>
where
    S: ShareId + ShareWrite<Value = T> + Clone + Send + Sync,
    T: FromStr + Clone + Send,
    T::Err: Send,
{
    async fn on_event(&mut self, event: Event) -> Feedback {
        match event {
            Event::Update { id, value } if id == self.id => match value.parse::<T>() {
                Ok(value) => {
                    let feedback = self.share.write(TaskValue::Unstable(value)).await;
                    Feedback::from(Change::Valid { id })
                        .merged_with(feedback)
                        .unwrap()
                }
                Err(_) => {
                    let feedback = self.share.write(TaskValue::Empty).await;
                    Feedback::from(Change::Invalid { id })
                        .merged_with(feedback)
                        .unwrap()
                }
            },
            _ => Feedback::new(),
        }
    }
}

#[async_trait]
impl<S, T> Refresh for Edit<S, T>
where
    S: ShareId + ShareRead<Value = T> + Send + Sync,
    T: Display + Send + Sync,
{
    async fn refresh(&self, id: Uuid) -> Feedback {
        if self.share.id() == id {
            match self.share.read().await.deref() {
                TaskValue::Stable(value) | TaskValue::Unstable(value) => {
                    Feedback::from(Change::UpdateValue {
                        id: self.id,
                        value: value.to_string(),
                    })
                }
                TaskValue::Empty => Feedback::from(Change::Invalid { id: self.id }),
            }
        } else {
            Feedback::new()
        }
    }
}

#[async_trait]
impl<S> ToHtml for Edit<S, String>
where
    S: ShareRead<Value = String> + Send + Sync,
{
    async fn to_html(&self) -> Html {
        let value = self.share.read().await;
        html! {r#"
            <label for="{self.id}" class="label">{self.label}</label>
            <input id="{self.id}" class="input" value="{value}" oninput="update(this)"/>
        "#}
    }
}

macro_rules! impl_to_html_for_number {
    ($($ty:ty),*) => {
        $(
            #[async_trait]
            impl<S> ToHtml for Edit<S, $ty>
            where
                S: ShareRead<Value = $ty> + Send + Sync,
            {
                async fn to_html(&self) -> Html {
                    let value = self.share.read().await;
                    let number = value.as_ref().map(ToString::to_string);
                    html! {r#"
                        <label for="{self.id}" class="label">{self.label}</label>
                        <input id="{self.id}" type="number" class="input" value="{number}" oninput="update(this)"/>
                    "#}
                }
            }
        )*
    };
}

impl_to_html_for_number!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

#[async_trait]
impl<S> ToHtml for Edit<S, bool>
where
    S: ShareRead<Value = bool> + Send + Sync,
{
    async fn to_html(&self) -> Html {
        let value = self.share.read().await;
        let checked = value.as_ref().unwrap_or(&false).then(|| "checked");
        html! {r#"
            <label class="checkbox">
                <input id="{self.id}" type="checkbox" onclick="update(this, this.checked.toString())" {checked}>
                {self.label}
            </label>
        "#}
    }
}

#[async_trait]
impl<S> ToHtml for Edit<S, char>
where
    S: ShareRead<Value = char> + Send + Sync,
{
    async fn to_html(&self) -> Html {
        let value = self
            .share
            .read()
            .await
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_default();
        html! {r#"
            <label for="{self.id}" class="label">{self.label}</label>
            <input id="{self.id}" class="input" value="{value}" oninput="update(this)" maxlength="1"/>
        "#}
    }
}
