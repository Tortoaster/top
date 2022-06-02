use std::marker::PhantomData;
use std::ops::Deref;

use async_trait::async_trait;
use uuid::Uuid;

use top_derive::html;

use crate::html::event::{Change, Feedback};
use crate::html::icon::Icon;
use crate::html::{Html, ToHtml};
use crate::prelude::TaskValue;
use crate::share::{Share, SharedId, SharedRead, SharedValue};
use crate::task::tune::{OutputTuner, Tune};
use crate::viewer::Viewer;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OutputViewer<S, T> {
    id: Uuid,
    share: S,
    _type: PhantomData<T>,
    tuner: OutputTuner,
}

impl<T> OutputViewer<Share<T>, T> {
    pub fn new(value: T) -> Self {
        OutputViewer::new_shared(Share::new(TaskValue::Stable(value)))
    }
}

impl<S, T> OutputViewer<S, T> {
    pub fn new_shared(share: S) -> Self {
        OutputViewer {
            id: Uuid::new_v4(),
            share,
            _type: PhantomData,
            tuner: OutputTuner::default(),
        }
    }
}

#[async_trait]
impl<S, T> Viewer for OutputViewer<S, T>
where
    S: SharedRead<Value = T> + SharedId + SharedValue<Value = T> + Clone + Send + Sync,
    T: Send + Sync,
    Self: ToHtml,
{
    type Value = T;
    type Share = S;

    async fn redraw(&self, id: Uuid) -> Feedback {
        if self.share.id() == id {
            Feedback::from(Change::Replace {
                id: self.id,
                html: self.to_html().await,
            })
        } else {
            Feedback::new()
        }
    }

    fn share(&self) -> Self::Share {
        self.share.clone()
    }

    async fn value(&self) -> TaskValue<Self::Value> {
        self.share.clone_value().await
    }
}

impl<S, T> Tune for OutputViewer<S, T> {
    type Tuner = OutputTuner;

    fn tune(&mut self, tuner: Self::Tuner) {
        self.tuner = tuner;
    }
}

macro_rules! impl_to_html {
    ($($ty:ty),*) => {
        $(
            #[async_trait]
            impl<S> ToHtml for OutputViewer<S, $ty>
            where
                S: SharedRead<Value = $ty> + Send + Sync,
            {
                async fn to_html(&self) -> Html {
                    let value = self.share.read().await;
                    html! {r#"
                        <div id="{self.id}">
                            <span style="color: {self.tuner.color};">{value.deref()}</span>
                        </div>
                    "#}
                }
            }
        )*
    };
}

impl_to_html!(
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    f32,
    f64,
    char,
    &'static str,
    String
);

#[async_trait]
impl<S> ToHtml for OutputViewer<S, bool>
where
    S: SharedRead<Value = bool> + Send + Sync,
{
    async fn to_html(&self) -> Html {
        match self.share.read().await.deref() {
            TaskValue::Stable(b) | TaskValue::Unstable(b) => {
                if *b {
                    Icon::Check.to_html().await
                } else {
                    Icon::XMark.to_html().await
                }
            }
            TaskValue::Empty => Html::default(),
        }
    }
}
