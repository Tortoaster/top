use crate::component::id::ComponentCreator;
use crate::component::{Component, Widget};
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

    fn component(&self, ctx: &mut ComponentCreator) -> Component {
        ctx.create(Widget::Text(self.0.clone()))
    }

    fn read(&self) -> Self::Output {
        self.0.clone()
    }
}
