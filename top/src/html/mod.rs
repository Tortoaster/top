//! This module contains functionality for generating user interfaces for tasks.

use std::fmt::{Display, Formatter};

use handlebars::Handlebars;
use lazy_static::lazy_static;
use serde::Serialize;
use serde_json::json;

pub use button::{Button, Icon, IconButton};
pub use div::{Div, DivType};
pub use input::{CheckBox, Input, InputType};
pub use radio::RadioGroup;
pub use span::{Color, Span};

mod button;
mod div;
mod input;
mod radio;
mod span;

const INDEX: &str = "index";

const INPUT: &str = "input";
const CHECKBOX: &str = "checkbox";
const BUTTON: &str = "button";
const ICON_BUTTON: &str = "icon_button";
const DIV: &str = "div";
const RADIO_GROUP: &str = "radio_group";

lazy_static! {
    static ref REGISTRY: Handlebars<'static> = {
        let mut reg = Handlebars::new();

        #[cfg(debug_assertions)]
        reg.set_dev_mode(true);

        // TODO: Improve paths
        reg.register_template_file(INDEX, "../../web/dist/template/index.hbs").unwrap();

        reg.register_template_file(INPUT, "../../web/dist/template/input.hbs").unwrap();
        reg.register_template_file(CHECKBOX, "../../web/dist/template/checkbox.hbs").unwrap();
        reg.register_template_file(BUTTON, "../../web/dist/template/button.hbs").unwrap();
        reg.register_template_file(ICON_BUTTON, "../../web/dist/template/icon_button.hbs").unwrap();
        reg.register_template_file(DIV, "../../web/dist/template/div.hbs").unwrap();
        reg.register_template_file(RADIO_GROUP, "../../web/dist/template/radio_group.hbs").unwrap();

        reg
    };
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(transparent)]
pub struct Html(String);

impl Html {
    pub const fn empty() -> Self {
        Html(String::new())
    }

    pub fn wrapper(title: &str) -> Html {
        let html = REGISTRY
            .render(INDEX, &json!({ "title": title }))
            .expect("failed to render template");

        Html(html)
    }
}

impl Display for Html {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub trait AsHtml {
    fn as_html(&self) -> Html;
}
