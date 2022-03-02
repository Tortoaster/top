use handlebars::Handlebars;
use lazy_static::lazy_static;
use serde_json::json;

#[derive(Debug)]
pub enum Component {
    TextField {
        value: String,
        label: Option<String>,
        disabled: bool,
    },
    NumberField {
        value: i32,
        label: Option<String>,
        disabled: bool,
    },
    Button {
        text: String,
        disabled: bool,
    },
    Row(Vec<Component>),
    Column(Vec<Component>),
}

impl Component {
    pub fn value(&self) -> Option<String> {
        match self {
            Component::TextField { value, .. } => Some(value.clone()),
            Component::NumberField { value, .. } => Some(value.to_string()),
            _ => None,
        }
    }

    pub fn input_type(&self) -> Option<&'static str> {
        match self {
            Component::TextField { .. } => Some("text"),
            Component::NumberField { .. } => Some("number"),
            _ => None,
        }
    }

    pub fn render(&self) -> String {
        match &self {
            Component::TextField {
                label, disabled, ..
            }
            | Component::NumberField {
                label, disabled, ..
            } => REGISTRY
                .render(
                    INPUT,
                    &json!({
                        "type": self.input_type().expect("no input type"),
                        "value": self.value().expect("no value"),
                        "label": label.as_ref().unwrap_or(&String::new()),
                        "disabled": *disabled,
                    }),
                )
                .unwrap(),
            Component::Button { text, disabled } => REGISTRY
                .render(
                    BUTTON,
                    &json!({
                        "text": text,
                        "disabled": *disabled,
                    }),
                )
                .unwrap(),
            Component::Row(children) | Component::Column(children) => format!(
                "<div>{}</div>",
                children
                    .iter()
                    .map(|c| c.render())
                    .collect::<Vec<String>>()
                    .join("<br/>")
            ),
        }
    }

    pub fn render_page(&self, title: &str) -> String {
        REGISTRY
            .render(INDEX, &json!({ "title": title, "content": self.render() }))
            .expect("failed to render template")
    }
}

const INDEX: &'static str = "index";
const INPUT: &'static str = "input";
const BUTTON: &'static str = "button";

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
