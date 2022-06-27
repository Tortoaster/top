use std::fmt::Display;
use std::marker::PhantomData;
use std::ops::Deref;
use std::str::FromStr;

use async_trait::async_trait;
use uuid::Uuid;

use top_derive::html;

use crate::html::event::{Change, Event, Feedback};
use crate::html::{Handler, Html, Refresh, ToHtml};
use crate::share::{Share, ShareId, ShareRead, ShareWrite, Shared};
use crate::task::{OptionExt, TaskValue, Value};

#[derive(Clone, Debug)]
pub struct EditValue<S: Share>(InnerEditValue<S, S::Value>);

impl<T> EditValue<Shared<T>>
where
    T: Clone + Send,
{
    pub fn new(value: Option<T>) -> Self {
        EditValue::new_shared(Shared::new(value.into_unstable()))
    }
}

impl<S> EditValue<S>
where
    S: Share,
{
    pub fn new_shared(share: S) -> Self {
        EditValue(InnerEditValue {
            id: Uuid::new_v4(),
            share,
            label: None,
            _type: PhantomData,
        })
    }

    pub fn with_label(mut self, label: String) -> Self {
        self.0.label = Some(label);
        self
    }
}

#[async_trait]
impl<S> Value for EditValue<S>
where
    S: Share + Clone + Send + Sync,
    S::Value: Clone + Send + Sync,
{
    type Output = S::Value;
    type Share = S;

    async fn share(&self) -> Self::Share {
        self.0.share().await
    }

    async fn value(self) -> TaskValue<Self::Output> {
        self.0.value().await
    }
}

#[async_trait]
impl<S> Handler for EditValue<S>
where
    S: ShareWrite + Clone + Send + Sync,
    S::Value: FromStr + Clone + Send,
    <S::Value as FromStr>::Err: Send,
{
    async fn on_event(&mut self, event: Event) -> Feedback {
        self.0.on_event(event).await
    }
}

#[async_trait]
impl<S> Refresh for EditValue<S>
where
    S: ShareId + ShareRead + Send + Sync,
    S::Value: Display + Send + Sync,
{
    async fn refresh(&self, id: Uuid) -> Feedback {
        self.0.refresh(id).await
    }
}

#[async_trait]
impl<S> ToHtml for EditValue<S>
where
    S: ShareId + ShareRead + Send + Sync,
    S::Value: Display + Send + Sync,
    InnerEditValue<S, S::Value>: ToHtml,
{
    async fn to_html(&self) -> Html {
        self.0.to_html().await
    }
}

#[derive(Clone, Debug)]
struct InnerEditValue<S, T> {
    id: Uuid,
    share: S,
    label: Option<String>,
    // Necessary for the `ToHtml` impls.
    _type: PhantomData<T>,
}

#[async_trait]
impl<S, T> Value for InnerEditValue<S, T>
where
    S: Share<Value = T> + Clone + Send + Sync,
    T: Clone + Send + Sync,
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
impl<S, T> Handler for InnerEditValue<S, T>
where
    S: ShareWrite<Value = T> + Clone + Send + Sync,
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
impl<S, T> Refresh for InnerEditValue<S, T>
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
impl<S> ToHtml for InnerEditValue<S, String>
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
            impl<S> ToHtml for InnerEditValue<S, $ty>
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
impl<S> ToHtml for InnerEditValue<S, bool>
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
impl<S> ToHtml for InnerEditValue<S, char>
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
