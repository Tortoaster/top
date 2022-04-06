use serde_json::json;

use crate::html::{AsHtml, Html, BUTTON, ICON_BUTTON, MINUS, PLUS, REGISTRY};
use crate::id::Id;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Button<'a> {
    id: Id,
    label: &'a str,
}

impl<'a> Button<'a> {
    pub const fn new(id: Id, label: &'a str) -> Self {
        Button { id, label }
    }
}

impl AsHtml for Button<'_> {
    fn as_html(&self) -> Html {
        let html = REGISTRY
            .render(
                BUTTON,
                &json!({
                    "id": self.id,
                    "text": self.label,
                }),
            )
            .expect("failed to render template");

        Html(html)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct IconButton {
    id: Id,
    icon: Icon,
}

impl IconButton {
    pub const fn new(id: Id, icon: Icon) -> Self {
        IconButton { id, icon }
    }
}

impl AsHtml for IconButton {
    fn as_html(&self) -> Html {
        let html = REGISTRY
            .render(
                ICON_BUTTON,
                &json!({
                    "id": self.id,
                    "icon": self.icon.as_html(),
                }),
            )
            .expect("failed to render template");

        Html(html)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Icon {
    Plus,
    Minus,
}

impl AsHtml for Icon {
    fn as_html(&self) -> Html {
        let html = match self {
            Icon::Plus => REGISTRY.render(PLUS, &()),
            Icon::Minus => REGISTRY.render(MINUS, &()),
        }
        .expect("failed to render template");

        Html(html)
    }
}
