use handlebars::Handlebars;
use serde_json::json;

pub const INDEX: &str = "index";
const TEXTFIELD: &str = "textfield";

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
    Column {
        children: Vec<Component>,
    },
}

impl Component {
    pub fn registry() -> Handlebars<'static> {
        // TODO: constant
        let mut handlebars = Handlebars::new();
        handlebars
            .register_template_string(INDEX, include_str!("../../../web/dist/index.hbs"))
            .unwrap();
        handlebars
            .register_template_string(
                TEXTFIELD,
                include_str!("../../../web/dist/component/textfield.hbs"),
            )
            .unwrap();
        handlebars
    }

    pub fn html(&self, reg: &Handlebars) -> String {
        match self {
            Component::TextField {
                value,
                label,
                disabled,
            } => reg
                .render(
                    TEXTFIELD,
                    &json!({
                        "value": value,
                        "label": if let Some(label) = label { label } else { "" },
                        "disabled": if *disabled { "disabled" } else { "" },
                    }),
                )
                .unwrap(),
            Component::NumberField {
                value,
                label,
                disabled,
            } => format!(
                "<label>{}<input type='number' value='{}'{}/></label>",
                label.as_ref().unwrap_or(&String::new()),
                value,
                if *disabled { " disabled" } else { "" },
            ),
            Component::Column { children } => format!(
                "<div>{}</div>",
                children
                    .iter()
                    .map(|c| c.html(reg))
                    .collect::<Vec<String>>()
                    .join("<br/>")
            ),
        }
    }
}
