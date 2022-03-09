use handlebars::Handlebars;
use lazy_static::lazy_static;
use serde_json::json;

use crate::component::{Component, Widget};

const INDEX: &str = "index";
const INPUT: &str = "input";
const BUTTON: &str = "button";

lazy_static! {
    static ref REGISTRY: Handlebars<'static> = {
        let mut reg = Handlebars::new();
        #[cfg(debug_assertions)]
        reg.set_dev_mode(true);
        // TODO: Improve paths
        reg.register_template_file(INDEX, "../../web/dist/index.hbs")
            .unwrap();
        reg.register_template_file(INPUT, "../../web/dist/component/input.hbs").unwrap();
        reg.register_template_file(BUTTON, "../../web/dist/component/button.hbs").unwrap();
        reg
    };
}

impl Component {
    fn value(&self) -> Option<String> {
        match &self.widget {
            Widget::TextField { value, .. } => Some(value.clone()),
            Widget::NumberField { value, .. } => Some(value.to_string()),
            _ => None,
        }
    }

    fn input_type(&self) -> Option<&'static str> {
        match &self.widget {
            Widget::TextField { .. } => Some("text"),
            Widget::NumberField { .. } => Some("number"),
            _ => None,
        }
    }

    /// Generate an HTML representation of this component.
    pub fn html(&self) -> String {
        match &self.widget {
            Widget::TextField {
                label, disabled, ..
            }
            | Widget::NumberField {
                label, disabled, ..
            } => REGISTRY
                .render(
                    INPUT,
                    &json!({
                        "id": self.id(),
                        "type": self.input_type().expect("no input type"),
                        "value": self.value().expect("no value"),
                        "label": label.as_ref().unwrap_or(&String::new()),
                        "disabled": *disabled,
                    }),
                )
                .unwrap(),
            Widget::Button { text, disabled } => REGISTRY
                .render(
                    BUTTON,
                    &json!({
                        "id": self.id(),
                        "text": text,
                        "disabled": *disabled,
                    }),
                )
                .unwrap(),
            Widget::Row(children) | Widget::Column(children) => format!(
                "<div id=\"{}\">{}</div>",
                self.id(),
                children
                    .iter()
                    .map(|c| c.html())
                    .collect::<Vec<String>>()
                    .join("<br/>")
            ),
        }
    }

    /// Generate a wrapper webpage to contain other components.
    pub fn html_wrapper(title: &str) -> String {
        REGISTRY
            .render(INDEX, &json!({ "title": title }))
            .expect("failed to render template")
    }
}
