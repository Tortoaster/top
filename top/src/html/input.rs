use serde_json::json;

use crate::html::{AsHtml, Html, CHECKBOX, INPUT, REGISTRY};
use crate::id::Id;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Input<'a> {
    id: Id,
    ty: InputType,
    value: &'a str,
    label: Option<&'a str>,
}

impl<'a> Input<'a> {
    pub const fn new(id: Id) -> Self {
        Input {
            id,
            ty: InputType::Text,
            value: "",
            label: None,
        }
    }

    pub const fn with_type(mut self, ty: InputType) -> Self {
        self.ty = ty;
        self
    }

    pub fn with_value(mut self, value: &'a str) -> Self {
        self.value = value;
        self
    }

    pub fn with_label(mut self, label: Option<&'a str>) -> Self {
        self.label = label;
        self
    }
}

impl AsHtml for Input<'_> {
    fn as_html(&self) -> Html {
        let html = REGISTRY
            .render(
                INPUT,
                &json!({
                    "id": self.id,
                    "type": self.ty.as_html(),
                    "value": self.value,
                    "label": self.label,
                }),
            )
            .expect("failed to render template");

        Html(html)
    }
}

/// Used in [`InputField`] to specify what kind of HTML input type should be used.
///
/// Not all input types are included here, some have dedicated [`AsHtml`] types:
///
/// * `type="button"`: [`Button`] or [`IconButton`]
/// * `type="checkbox"`: [`CheckBox`]
/// * `type="Radio"`: [`RadioGroup`]
///
/// [`Button`]: ../button/struct.Button.html
/// [`IconButton`]: ../button/struct.IconButton.html
/// [`RadioGroup`]: ../tag/struct.RadioGroup.html
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum InputType {
    Color,
    Date,
    DatetimeLocal,
    Email,
    File,
    Hidden,
    Image,
    Month,
    Number,
    Password,
    Range,
    Reset,
    Search,
    Submit,
    Tel,
    Text,
    Time,
    Url,
    Week,
}

impl AsHtml for InputType {
    fn as_html(&self) -> Html {
        match self {
            InputType::Color => Html("color".to_owned()),
            InputType::Date => Html("date".to_owned()),
            InputType::DatetimeLocal => Html("datetime-local".to_owned()),
            InputType::Email => Html("email".to_owned()),
            InputType::File => Html("file".to_owned()),
            InputType::Hidden => Html("hidden".to_owned()),
            InputType::Image => Html("image".to_owned()),
            InputType::Month => Html("month".to_owned()),
            InputType::Number => Html("number".to_owned()),
            InputType::Password => Html("password".to_owned()),
            InputType::Range => Html("range".to_owned()),
            InputType::Reset => Html("reset".to_owned()),
            InputType::Search => Html("search".to_owned()),
            InputType::Submit => Html("submit".to_owned()),
            InputType::Tel => Html("tel".to_owned()),
            InputType::Text => Html("text".to_owned()),
            InputType::Time => Html("time".to_owned()),
            InputType::Url => Html("url".to_owned()),
            InputType::Week => Html("week".to_owned()),
        }
    }
}

impl Default for InputType {
    fn default() -> Self {
        InputType::Text
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CheckBox<'a> {
    id: Id,
    checked: bool,
    label: Option<&'a str>,
}

impl<'a> CheckBox<'a> {
    pub const fn new(id: Id) -> Self {
        CheckBox {
            id,
            checked: false,
            label: None,
        }
    }

    pub const fn with_checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    pub fn with_label(mut self, label: Option<&'a str>) -> Self {
        self.label = label;
        self
    }
}

impl AsHtml for CheckBox<'_> {
    fn as_html(&self) -> Html {
        let html = REGISTRY
            .render(
                CHECKBOX,
                &json!({
                    "id": self.id,
                    "checked": self.checked,
                    "label": self.label,
                }),
            )
            .expect("failed to render template");

        Html(html)
    }
}
