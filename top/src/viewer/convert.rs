use std::fmt::Display;

use crate::html::{AsHtml, Html, Span};
use crate::viewer::Viewer;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DisplayViewer<T>(T);

impl<T> AsHtml for DisplayViewer<T>
where
    T: Display,
{
    fn as_html(&self) -> Html {
        Span::new(&self.0.to_string()).as_html()
    }
}

impl<T> Viewer for DisplayViewer<T>
where
    T: Clone + Display,
{
    type Input = T;
    type Output = T;

    fn start(value: Self::Input) -> Self {
        DisplayViewer(value)
    }

    fn finish(&self) -> Self::Output {
        self.0.clone()
    }
}
