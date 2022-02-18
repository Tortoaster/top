pub mod generic;
pub mod primitive;

pub type Html = String;

pub trait Editor {
    type Read;
    type Write;
    type Error;

    fn html(&self) -> Html;

    fn read_value(&self) -> Self::Read;

    fn write_value(&mut self, value: Self::Write) -> Result<(), Self::Error>;
}

#[derive(Debug, Default)]
pub struct FormatParams {
    label: Option<String>,
    disabled: bool,
    sync: bool,
}

impl FormatParams {
    pub const fn new() -> Self {
        FormatParams {
            label: None,
            disabled: false,
            sync: false,
        }
    }

    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }

    pub const fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    pub const fn synchronized(mut self) -> Self {
        self.sync = true;
        self
    }
}
