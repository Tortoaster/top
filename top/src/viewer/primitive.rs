use crate::html::{AsHtml, Html, Span};
use crate::tune::{StringTuner, Tune};
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

impl AsHtml for StringViewer {
    fn as_html(&self) -> Html {
        Span::new(&self.value)
            .with_color(self.tuner.color)
            .as_html()
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
