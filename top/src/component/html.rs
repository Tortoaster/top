use handlebars::Handlebars;
use lazy_static::lazy_static;
use serde_json::json;

use crate::component::icon::Icon;
use crate::component::{Component, Widget};

const INDEX: &str = "index";

const INPUT: &str = "input";
const CHECKBOX: &str = "checkbox";
const BUTTON: &str = "button";
const ICON_BUTTON: &str = "icon_button";
const GROUP: &str = "group";
const RADIO_GROUP: &str = "radio_group";

const PLUS: &str = "plus";
const MINUS: &str = "minus";

lazy_static! {
    static ref REGISTRY: Handlebars<'static> = {
        let mut reg = Handlebars::new();

        #[cfg(debug_assertions)]
        reg.set_dev_mode(true);

        // TODO: Improve paths
        reg.register_template_file(INDEX, "../../web/dist/index.hbs").unwrap();

        reg.register_template_file(INPUT, "../../web/dist/component/input.hbs").unwrap();
        reg.register_template_file(CHECKBOX, "../../web/dist/component/checkbox.hbs").unwrap();
        reg.register_template_file(BUTTON, "../../web/dist/component/button.hbs").unwrap();
        reg.register_template_file(ICON_BUTTON, "../../web/dist/component/icon_button.hbs").unwrap();
        reg.register_template_file(GROUP, "../../web/dist/component/group.hbs").unwrap();
        reg.register_template_file(RADIO_GROUP, "../../web/dist/component/radio_group.hbs").unwrap();

        reg.register_template_file(PLUS, "../../web/dist/icon/plus.hbs").unwrap();
        reg.register_template_file(MINUS, "../../web/dist/icon/minus.hbs").unwrap();

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
            Widget::Checkbox {
                checked,
                label,
                disabled,
            } => REGISTRY
                .render(
                    CHECKBOX,
                    &json!({
                        "id": self.id(),
                        "checked": *checked,
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
            Widget::IconButton { icon, disabled } => REGISTRY
                .render(
                    ICON_BUTTON,
                    &json!({
                        "id": self.id(),
                        "icon": icon.html(),
                        "disabled": *disabled,
                    }),
                )
                .unwrap(),
            Widget::Group {
                children,
                horizontal,
            } => REGISTRY
                .render(
                    GROUP,
                    &json!({
                        "id": self.id(),
                        "horizontal": *horizontal,
                        "content": children
                            .iter()
                            .map(|child| child.html())
                            .collect::<Vec<_>>()
                            .join("<br/>"),
                    }),
                )
                .unwrap(),
            Widget::RadioGroup { options } => REGISTRY
                .render(
                    RADIO_GROUP,
                    &json!({
                        "id": self.id(),
                        "options": options.iter().map(|option| option.html()).collect::<Vec<_>>()
                    }),
                )
                .unwrap(),

            Widget::Text(text) => format!("<span>{text}</span>"),
        }
    }

    /// Generate a wrapper webpage to contain other components.
    pub fn html_wrapper(title: &str) -> String {
        REGISTRY
            .render(INDEX, &json!({ "title": title }))
            .expect("failed to render template")
    }
}

impl Icon {
    pub fn html(&self) -> String {
        match self {
            Icon::Plus => REGISTRY.render(PLUS, &()).unwrap(),
            Icon::Minus => REGISTRY.render(MINUS, &()).unwrap(),
        }
    }
}
