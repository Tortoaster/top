use crate::html::{AsHtml, Html, Span};
use crate::viewer::Viewer;

/// Basic viewer for strings.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextViewer(String);

impl AsHtml for TextViewer {
    fn as_html(&self) -> Html {
        Span::new(&self.0).as_html()
    }
}

impl Viewer for TextViewer {
    type Input = String;
    type Output = String;

    fn start(value: Self::Input) -> Self {
        TextViewer(value)
    }

    fn finish(&self) -> Self::Output {
        self.0.clone()
    }
}
