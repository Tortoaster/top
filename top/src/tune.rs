#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct InputTuner {
    pub label: Option<String>,
}

impl InputTuner {
    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }
}

pub trait Tune {
    type Tuner;

    fn tune(&mut self, tuner: Self::Tuner);
}
