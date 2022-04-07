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
    color: Color,
}

impl<'a> Span<'a> {
    pub const fn new(content: &'a str) -> Self {
        Span {
            content,
            color: Color::Black,
        }
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Color {
    Black,
    White,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Pink,
    Brown,
}

impl Default for Color {
    fn default() -> Self {
        Color::Black
    }
}

impl AsHtml for Color {
    fn as_html(&self) -> Html {
        match self {
            Color::Black => Html("black".to_owned()),
            Color::White => Html("white".to_owned()),
            Color::Red => Html("red".to_owned()),
            Color::Orange => Html("orange".to_owned()),
            Color::Yellow => Html("yellow".to_owned()),
            Color::Green => Html("green".to_owned()),
            Color::Blue => Html("blue".to_owned()),
            Color::Purple => Html("purple".to_owned()),
            Color::Pink => Html("pink".to_owned()),
            Color::Brown => Html("brown".to_owned()),
        }
    }
}

impl AsHtml for Span<'_> {
    fn as_html(&self) -> Html {
        let html = format!(
            "<span style=\"color: {};\">{}</span>",
            self.color.as_html(),
            self.content
        );

        Html(html)
    }
}
