use top_derive::html;

use crate::html::{Html, ToHtml};
use crate::task::tune::{StringTuner, Tune};
use crate::viewer::Viewer;

/// Basic viewer for strings.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StringViewer {
    value: String,
    tuner: StringTuner,
}

impl StringViewer {
    pub fn new(value: String) -> Self {
        StringViewer {
            value,
            tuner: StringTuner::default(),
        }
    }
}

impl ToHtml for StringViewer {
    fn to_html(&self) -> Html {
        html! {r#"
            <span style="color: {self.tuner.color};">{self.value}</span>
        "#}
    }
}

impl Viewer for StringViewer {
    type Value = String;

    fn finish(&self) -> Self::Value {
        self.value.clone()
    }
}

impl Tune for StringViewer {
    type Tuner = StringTuner;

    fn tune(&mut self, tuner: Self::Tuner) {
        self.tuner = tuner;
    }
}
