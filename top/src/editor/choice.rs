use top_derive::html;

use crate::editor::{Editor, EditorError};
use crate::html::event::{Event, Feedback};
use crate::html::id::{Generator, Id};
use crate::html::{Html, ToHtml};
use crate::viewer::Viewer;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChoiceEditor<V> {
    id: Id,
    choices: Vec<V>,
    choice: Option<usize>,
}

impl<V> ChoiceEditor<V> {
    pub fn new(options: Vec<V>) -> Self {
        ChoiceEditor {
            id: Id::INVALID,
            choices: options,
            choice: None,
        }
    }
}

impl<V> ToHtml for ChoiceEditor<V>
where
    V: ToHtml,
{
    fn to_html(&self) -> Html {
        // TODO: Join with <br/> instead
        let options: Html = self.choices.iter().enumerate().map(|(index, choice)| html! {r#"
            <label class="radio">
                <input type="radio" id="{self.id}-{index}" name="{self.id}" value="{index}" onclick="update(this.parentElement.parentElement, this.value)">
                {choice}
            </label><br/>
        "#}).collect();

        html! {r#"
            <div id="{self.id}" class="control">
                {options}
            </div>
        "#}
    }
}

impl<V> Editor for ChoiceEditor<V>
where
    V: Viewer,
{
    type Value = Option<V::Value>;

    fn start(&mut self, gen: &mut Generator) {
        self.id = gen.next();
    }

    fn on_event(&mut self, event: Event, _gen: &mut Generator) -> Option<Feedback> {
        match event {
            Event::Update { id, value } if self.id == id => match value.parse() {
                Ok(usize) => {
                    self.choice = Some(usize);
                    Some(Feedback::Valid { id })
                }
                Err(_) => Some(Feedback::Invalid { id }),
            },
            _ => None,
        }
    }

    fn finish(&self) -> Result<Self::Value, EditorError> {
        let choice = self
            .choice
            .and_then(|index| self.choices.get(index).map(|choice| choice.finish()));

        Ok(choice)
    }
}
