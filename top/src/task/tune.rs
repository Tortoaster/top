pub trait Tune {
    type Tuner;

    fn tune(&mut self, tuner: Self::Tuner);
}

pub trait ContentTune {
    type ContentTuner;

    fn tune_content(&mut self, tuner: Self::ContentTuner);
}
