use serde_json::json;

use crate::html::{AsHtml, Html, DIV, REGISTRY};
use crate::id::Id;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Div {
    id: Option<Id>,
    children: Vec<Html>,
    div_type: DivType,
}

impl Div {
    pub const fn new(children: Vec<Html>) -> Self {
        Div {
            id: None,
            children,
            div_type: DivType::Column,
        }
    }

    pub const fn with_id(mut self, id: Id) -> Self {
        self.id = Some(id);
        self
    }

    pub const fn with_div_type(mut self, display_as: DivType) -> Self {
        self.div_type = display_as;
        self
    }
}

impl AsHtml for Div {
    fn as_html(&self) -> Html {
        let html = REGISTRY
            .render(
                DIV,
                &json!({
                    "id": self.id,
                    "children": self.children,
                    "class": self.div_type.class()
                }),
            )
            .expect("failed to render template");

        Html(html)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum DivType {
    Column,
    Row,
    Section,
}

impl DivType {
    const fn class(&self) -> &'static str {
        match self {
            DivType::Column => "block",
            DivType::Row => "level",
            DivType::Section => "section",
        }
    }
}

impl Default for DivType {
    fn default() -> Self {
        DivType::Column
    }
}
