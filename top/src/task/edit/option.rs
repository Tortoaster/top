use async_trait::async_trait;
use uuid::Uuid;

use top_derive::html;

use crate::html::event::{Change, Event, Feedback};
use crate::html::icon::Icon;
use crate::html::{Handler, Html, ToHtml};
use crate::prelude::TaskValue;
use crate::task::tune::{ContentTune, Tune};
use crate::task::Value;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OptionEditor<E> {
    id: Uuid,
    /// Represents the plus button if there is no value present.
    add_id: Uuid,
    /// Represents the row containing the edit and the minus button if a value is present.
    row: Row,
    editor: E,
    /// True if this edit contains a value, false otherwise.
    enabled: bool,
}

impl<E> OptionEditor<E>
where
    E: Value + Handler + ToHtml,
{
    pub fn new(editor: E, enabled: bool) -> Self {
        OptionEditor {
            id: Uuid::new_v4(),
            add_id: Uuid::new_v4(),
            row: Row::new(),
            editor,
            enabled,
        }
    }
}

#[async_trait]
impl<E> ToHtml for OptionEditor<E>
where
    E: ToHtml + Send + Sync,
{
    async fn to_html(&self) -> Html {
        let content = if self.enabled {
            self.row.to_html(&self.editor).await
        } else {
            Row::add_button(self.add_id).await
        };

        html! {r#"
            <div id="{self.id}">
                {content}
            </div>
        "#}
    }
}

#[async_trait]
impl<E> Value for OptionEditor<E>
where
    E: Value + Send + Sync,
{
    type Output = Option<E::Output>;
    type Share = E::Share;

    async fn share(&self) -> Self::Share {
        self.editor.share().await
    }

    async fn value(self) -> TaskValue<Self::Output> {
        if self.enabled {
            self.editor.value().await.map(Option::Some)
        } else {
            TaskValue::Empty
        }
    }
}

#[async_trait]
impl<E> Handler for OptionEditor<E>
where
    E: ToHtml + Handler + Send + Sync,
{
    async fn on_event(&mut self, event: Event) -> Feedback {
        match event {
            Event::Press { id } if id == self.add_id && !self.enabled => {
                // Add value
                let html = self.row.to_html(&mut self.editor).await;
                self.enabled = true;

                Feedback::from(Change::ReplaceContent { id: self.id, html })
            }
            Event::Press { id } if id == self.row.sub_id && self.enabled => {
                // Remove value

                let html = Row::add_button(self.add_id).await;
                self.enabled = false;

                Feedback::from(Change::ReplaceContent { id: self.id, html })
            }
            _ => {
                if self.enabled {
                    self.editor.on_event(event).await
                } else {
                    Feedback::new()
                }
            }
        }
    }
}

impl<E> ContentTune for OptionEditor<E>
where
    E: Tune,
{
    type ContentTuner = E::Tuner;

    fn tune_content(&mut self, tuner: Self::ContentTuner) {
        self.editor.tune(tuner)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Row {
    id: Uuid,
    sub_id: Uuid,
}

impl Row {
    fn new() -> Self {
        Row {
            id: Uuid::new_v4(),
            sub_id: Uuid::new_v4(),
        }
    }

    /// Creates a row consisting of the edit and a button to remove it.
    async fn to_html<E>(&self, editor: &E) -> Html
    where
        E: ToHtml,
    {
        html! {r#"
            <div id="{self.id}" class="level">
                {editor}
                <button id="{self.sub_id}" class="button is-outlined" type="button" onclick="press(this)">
                    {Icon::Minus}
                </button>
            </div>
        "#}
    }

    async fn add_button(id: Uuid) -> Html {
        html! {r#"
            <button id="{id}" class="button is-outlined" type="button" onclick="press(this)">
                {Icon::Plus}
            </button>
        "#}
    }
}
