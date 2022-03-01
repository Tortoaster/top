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
    Column(Vec<Component>),
}

impl Component {
    pub fn render(&self) -> String {
        match &self {
            Component::TextField {
                value,
                label,
                disabled,
            } => REGISTRY
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
            Component::Button { text, disabled } => format!(
                "<input type='submit' value='{}'{}/>",
                text,
                if *disabled { " disabled" } else { "" },
            ),
            Component::Column(children) => format!(
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
const TEXTFIELD: &'static str = "textfield";

lazy_static! {
    static ref REGISTRY: Handlebars<'static> = {
        let mut reg = Handlebars::new();
        #[cfg(debug_assertions)]
        reg.set_dev_mode(true);
        // TODO: Improve paths
        reg.register_template_file(INDEX, "../../web/dist/index.hbs")
            .unwrap();
        reg.register_template_file(TEXTFIELD, "../../web/dist/component/textfield.hbs")
            .unwrap();
        reg
    };
}
