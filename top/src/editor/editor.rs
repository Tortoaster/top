//! This module contains basic editors for primitive types.

use std::marker::PhantomData;
use std::ops::Deref;
use std::str::FromStr;

use async_trait::async_trait;
use serde::Serialize;
use uuid::Uuid;

use top_derive::html;

use crate::html::event::{Change, Event, Feedback};
use crate::html::{Handler, Html, ToHtml};
use crate::share::{Share, SharedId, SharedRead, SharedValue, SharedWrite};
use crate::task::{OptionExt, TaskValue, Value};

#[derive(Clone, Debug)]
pub struct Editor<S, T> {
    id: Uuid,
    share: S,
    label: Option<String>,
    // Necessary for the `ToHtml` impls.
    _type: PhantomData<T>,
}

impl<T> Editor<Share<T>, T> {
    pub fn new(value: Option<T>) -> Self {
        Editor::new_shared(Share::new(value.into_unstable()))
    }
}

impl<S, T> Editor<S, T> {
    pub fn new_shared(share: S) -> Self {
        Editor {
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
impl<S, T> Value for Editor<S, T>
where
    S: SharedId
        + SharedRead<Value = T>
        + SharedWrite<Value = T>
        + SharedValue<Value = T>
        + Clone
        + Send
        + Sync,
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
impl<S, T> Handler for Editor<S, T>
where
    S: SharedId
        + SharedRead<Value = T>
        + SharedWrite<Value = T>
        + SharedValue<Value = T>
        + Clone
        + Send
        + Sync,
    T: Serialize + FromStr + Clone + Send,
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
            Event::Redraw { id } if self.share.id() == id => {
                match self.share.read().await.deref() {
                    TaskValue::Stable(value) | TaskValue::Unstable(value) => {
                        Feedback::from(Change::UpdateValue {
                            id: self.id,
                            value: serde_json::to_string(value).unwrap(),
                        })
                    }
                    TaskValue::Empty => Feedback::from(Change::Invalid { id: self.id }),
                }
            }
            _ => Feedback::new(),
        }
    }
}

#[async_trait]
impl<S> ToHtml for Editor<S, String>
where
    S: SharedRead<Value = String> + Send + Sync,
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
            impl<S> ToHtml for Editor<S, $ty>
            where
                S: SharedRead<Value = $ty> + Send + Sync,
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
impl<S> ToHtml for Editor<S, bool>
where
    S: SharedRead<Value = bool> + Send + Sync,
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
impl<S> ToHtml for Editor<S, char>
where
    S: SharedRead<Value = char> + Send + Sync,
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
