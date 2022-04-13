use std::fmt::{Debug, Display};

use crate::html::{AsHtml, Html};
use crate::tune::Tune;
use crate::viewer::primitive::StringViewer;
use crate::viewer::Viewer;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DisplayViewer<T> {
    viewer: StringViewer,
    value: T,
}

impl<T> DisplayViewer<T> {
    pub fn new(value: T) -> Self
    where
        T: Display,
    {
        DisplayViewer {
            viewer: StringViewer::new(value.to_string()),
            value,
        }
    }
}

impl<T> AsHtml for DisplayViewer<T> {
    fn as_html(&self) -> Html {
        self.viewer.as_html()
    }
}

impl<T> Viewer for DisplayViewer<T>
where
    T: Clone,
{
    type Value = T;

    fn finish(&self) -> Self::Value {
        self.value.clone()
    }
}

impl<T> Tune for DisplayViewer<T> {
    type Tuner = <StringViewer as Tune>::Tuner;

    fn tune(&mut self, tuner: Self::Tuner) {
        self.viewer.tune(tuner);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DebugViewer<T> {
    viewer: StringViewer,
    value: T,
}

impl<T> DebugViewer<T> {
    pub fn new(value: T) -> Self
    where
        T: Debug,
    {
        DebugViewer {
            viewer: StringViewer::new(format!("{:?}", value)),
            value,
        }
    }
}

impl<T> AsHtml for DebugViewer<T> {
    fn as_html(&self) -> Html {
        self.viewer.as_html()
    }
}

impl<T> Viewer for DebugViewer<T>
where
    T: Clone,
{
    type Value = T;

    fn finish(&self) -> Self::Value {
        self.value.clone()
    }
}

impl<T> Tune for DebugViewer<T> {
    type Tuner = <StringViewer as Tune>::Tuner;

    fn tune(&mut self, tuner: Self::Tuner) {
        self.viewer.tune(tuner);
    }
}
