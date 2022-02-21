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
    pub fn html(&self) -> String {
        match self {
            Component::TextField {
                value,
                label,
                disabled,
            } => format!(
                "<label>{}<input type='text' value='{}'{}/></label>",
                label.as_ref().unwrap_or(&String::new()),
                value,
                if *disabled { " disabled" } else { "" },
            ),
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
                    .map(|c| c.html())
                    .collect::<Vec<String>>()
                    .join("<br/>")
            ),
        }
    }
}
