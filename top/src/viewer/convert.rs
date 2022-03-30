use std::fmt::Display;

use crate::component::id::ComponentCreator;
use crate::component::{Component, Widget};
use crate::viewer::Viewer;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DisplayViewer<T>(T);

impl<T> Viewer for DisplayViewer<T>
where
    T: Clone + Display,
{
    type Input = T;
    type Output = T;

    fn start(value: Self::Input) -> Self {
        DisplayViewer(value)
    }

    fn component(&self, ctx: &mut ComponentCreator) -> Component {
        ctx.create(Widget::Text(self.0.to_string()))
    }

    fn read(&self) -> Self::Output {
        self.0.clone()
    }
}
