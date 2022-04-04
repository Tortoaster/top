#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FieldTuner {
    pub label: Option<String>,
}

impl FieldTuner {
    pub fn with_label(mut self, label: String) -> Self {
        self.label = Some(label);
        self
    }
}

pub trait Tune {
    type Tuner;

    fn tune_with(&mut self, tuner: Self::Tuner);
}
