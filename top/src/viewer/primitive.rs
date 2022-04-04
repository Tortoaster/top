use crate::component::{Component, Widget};
use crate::id::Id;
use crate::viewer::Viewer;

/// Basic viewer for strings.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextViewer(String);

impl Viewer for TextViewer {
    type Input = String;
    type Output = String;

    fn start(value: Self::Input) -> Self {
        TextViewer(value)
    }

    fn component(&self) -> Component {
        Component::new(Id::INVALID, Widget::Text(self.0.clone()))
    }

    fn read(&self) -> Self::Output {
        self.0.clone()
    }
}
