use serde_json::json;

use crate::html::{AsHtml, Html, GROUP, RADIO_GROUP, REGISTRY};
use crate::id::Id;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Div {
    id: Option<Id>,
    layout: Layout,
    children: Vec<Html>,
}

impl Div {
    pub const fn new(children: Vec<Html>) -> Self {
        Div {
            id: None,
            layout: Layout::Column,
            children,
        }
    }

    pub const fn with_id(mut self, id: Id) -> Self {
        self.id = Some(id);
        self
    }

    pub const fn with_layout(mut self, layout: Layout) -> Self {
        self.layout = layout;
        self
    }
}

impl AsHtml for Div {
    fn as_html(&self) -> Html {
        let html = REGISTRY
            .render(
                GROUP,
                &json!({
                    "id": self.id,
                    "horizontal": self.layout == Layout::Row,
                    "children": self.children,
                }),
            )
            .expect("failed to render template");

        Html(html)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Layout {
    Row,
    Column,
}

impl Default for Layout {
    fn default() -> Self {
        Layout::Column
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct RadioGroup {
    id: Id,
    children: Vec<Html>,
}

impl RadioGroup {
    pub const fn new(id: Id, children: Vec<Html>) -> Self {
        RadioGroup { id, children }
    }
}

impl AsHtml for RadioGroup {
    fn as_html(&self) -> Html {
        let html = REGISTRY
            .render(
                RADIO_GROUP,
                &json!({
                    "id": self.id,
                    "options": self.children,
                }),
            )
            .expect("failed to render template");

        Html(html)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Span<'a> {
    content: &'a str,
}

impl<'a> Span<'a> {
    pub fn new(content: &'a str) -> Self {
        Span { content }
    }
}

impl AsHtml for Span<'_> {
    fn as_html(&self) -> Html {
        let html = format!("<span>{}</span>", self.content);

        Html(html)
    }
}
