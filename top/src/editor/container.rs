use top_derive::html;

use crate::editor::{Editor, EditorError};
use crate::html::event::{Change, Event, Feedback};
use crate::html::icon::Icon;
use crate::html::id::{Generator, Id};
use crate::html::{Html, ToHtml};
use crate::task::tune::{ContentTune, Tune};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VecEditor<E> {
    /// Represents the list containing all elements.
    group_id: Id,
    /// Represents the plus button.
    add_id: Id,
    /// Represents of each of the elements with their respective identifiers.
    rows: Vec<Row>,
    editors: Vec<E>,
    template: E,
}

impl<E> VecEditor<E> {
    pub fn new(editors: Vec<E>, template: E) -> Self {
        VecEditor {
            group_id: Id::INVALID,
            add_id: Id::INVALID,
            rows: Vec::new(),
            editors,
            template,
        }
    }
}

impl<E> ToHtml for VecEditor<E>
where
    E: ToHtml,
{
    fn to_html(&self) -> Html {
        let children: Html = self
            .editors
            .iter()
            .zip(&self.rows)
            .map(|(editor, row)| row.to_html(editor))
            .collect();

        let button = Row::add_button(self.add_id);

        html! {r#"
            <div class="column">
                <div id="{self.group_id}" class="column">
                    {children}
                </div>
                {button}
            </div>
        "#}
    }
}

impl<E> Editor for VecEditor<E>
where
    E: Editor + ToHtml + Clone,
{
    type Value = Vec<E::Value>;

    fn start(&mut self, gen: &mut Generator) {
        self.group_id = gen.next();
        self.add_id = gen.next();
        self.rows = self.editors.iter().map(|_| Row::new(gen)).collect();

        for editor in &mut self.editors {
            editor.start(gen);
        }
    }

    fn on_event(&mut self, event: Event, gen: &mut Generator) -> Feedback {
        match event {
            Event::Press { id } if id == self.add_id => {
                // Add a new row
                let mut editor = self.template.clone();
                editor.start(gen);
                let row = Row::new(gen);
                let html = row.to_html(&editor);

                self.editors.push(editor);
                self.rows.push(row);

                Feedback::from(Change::AppendContent {
                    id: self.group_id,
                    html,
                })
            }
            Event::Press { id } if self.rows.iter().any(|row| row.sub_id == id) => {
                // Remove an existing row
                let index = self.rows.iter().position(|row| row.sub_id == id).unwrap();
                let Row { id, .. } = self.rows.remove(index);
                self.editors.remove(index);

                Feedback::from(Change::Remove { id })
            }
            _ => self
                .editors
                .iter_mut()
                .find_map(|editor| {
                    let feedback = editor.on_event(event.clone(), gen);
                    (!feedback.is_empty()).then(|| feedback)
                })
                .unwrap_or_default(),
        }
    }

    fn value(&self) -> Result<Self::Value, EditorError> {
        // TODO: Return all errors
        self.editors
            .iter()
            .map(|editor| editor.value())
            .collect::<Result<Vec<_>, _>>()
    }
}

impl<E> ContentTune for VecEditor<E>
where
    E: Tune,
    E::Tuner: Clone,
{
    type ContentTuner = E::Tuner;

    fn tune_content(&mut self, tuner: Self::ContentTuner) {
        self.editors
            .iter_mut()
            .for_each(|choice| choice.tune(tuner.clone()));
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OptionEditor<E> {
    id: Id,
    /// Represents the plus button if there is no value present.
    add_id: Id,
    /// Represents the row containing the editor and the minus button if a value is present.
    row: Row,
    editor: E,
    /// True if this editor contains a value, false otherwise.
    enabled: bool,
}

impl<E> OptionEditor<E>
where
    E: Editor,
{
    pub fn new(editor: E, enabled: bool) -> Self {
        OptionEditor {
            id: Id::INVALID,
            add_id: Id::INVALID,
            row: Row {
                id: Id::INVALID,
                sub_id: Id::INVALID,
            },
            editor,
            enabled,
        }
    }
}

impl<E> ToHtml for OptionEditor<E>
where
    E: ToHtml,
{
    fn to_html(&self) -> Html {
        let content = if self.enabled {
            self.row.to_html(&self.editor)
        } else {
            Row::add_button(self.add_id).to_html()
        };

        html! {r#"
            <div id="{self.id}">
                {content}
            </div>
        "#}
    }
}

impl<E> Editor for OptionEditor<E>
where
    E: Editor + ToHtml,
{
    type Value = Option<E::Value>;

    fn start(&mut self, gen: &mut Generator) {
        self.id = gen.next();
        self.add_id = gen.next();
        self.row = Row::new(gen);

        self.editor.start(gen);
    }

    fn on_event(&mut self, event: Event, gen: &mut Generator) -> Feedback {
        match event {
            Event::Press { id } if id == self.add_id && !self.enabled => {
                // Add value
                let html = self.row.to_html(&mut self.editor);
                self.enabled = true;

                Feedback::from(Change::ReplaceContent { id: self.id, html })
            }
            Event::Press { id } if id == self.row.sub_id && self.enabled => {
                // Remove value

                let html = Row::add_button(self.add_id).to_html();
                self.enabled = false;

                Feedback::from(Change::ReplaceContent { id: self.id, html })
            }
            _ => self
                .enabled
                .then(|| self.editor.on_event(event, gen))
                .unwrap_or_default(),
        }
    }

    fn value(&self) -> Result<Self::Value, EditorError> {
        if self.enabled {
            Ok(Some(self.editor.value()?))
        } else {
            Ok(None)
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
    id: Id,
    sub_id: Id,
}

impl Row {
    fn new(gen: &mut Generator) -> Self {
        Row {
            id: gen.next(),
            sub_id: gen.next(),
        }
    }

    /// Creates a row consisting of the editor and a button to remove it.
    fn to_html<E>(&self, editor: &E) -> Html
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

    fn add_button(id: Id) -> Html {
        html! {r#"
            <button id="{id}" class="button is-outlined" type="button" onclick="press(this)">
                {Icon::Plus}
            </button>
        "#}
    }
}
