use std::fmt::{Debug, Display};

use crate::html::{AsHtml, Html};
use crate::tune::Tune;
use crate::viewer::primitive::StringViewer;
use crate::viewer::Viewer;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DisplayViewer {
    viewer: StringViewer,
}

impl DisplayViewer {
    pub fn new<T>(value: T) -> Self
    where
        T: Display,
    {
        DisplayViewer {
            viewer: StringViewer::new(value.to_string()),
        }
    }
}

impl AsHtml for DisplayViewer {
    fn as_html(&self) -> Html {
        self.viewer.as_html()
    }
}

impl Viewer for DisplayViewer {
    type Value = <StringViewer as Viewer>::Value;

    fn finish(&self) -> Self::Value {
        self.viewer.finish()
    }
}

impl Tune for DisplayViewer {
    type Tuner = <StringViewer as Tune>::Tuner;

    fn tune(&mut self, tuner: Self::Tuner) {
        self.viewer.tune(tuner);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DebugViewer {
    viewer: StringViewer,
}

impl DebugViewer {
    pub fn new<T>(value: T) -> Self
    where
        T: Debug,
    {
        DebugViewer {
            viewer: StringViewer::new(format!("{:?}", value)),
        }
    }
}

impl AsHtml for DebugViewer {
    fn as_html(&self) -> Html {
        self.viewer.as_html()
    }
}

impl Viewer for DebugViewer {
    type Value = <StringViewer as Viewer>::Value;

    fn finish(&self) -> Self::Value {
        self.viewer.finish()
    }
}

impl Tune for DebugViewer {
    type Tuner = <StringViewer as Tune>::Tuner;

    fn tune(&mut self, tuner: Self::Tuner) {
        self.viewer.tune(tuner);
    }
}
