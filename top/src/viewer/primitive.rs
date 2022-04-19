use top_derive::html;

use crate::html::{Html, ToHtml};
use crate::task::tune::{OutputTuner, Tune};
use crate::viewer::Viewer;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct OutputViewer<T> {
    pub(in crate::viewer) value: T,
    pub tuner: OutputTuner,
}

impl<T> OutputViewer<T> {
    pub fn new(value: T) -> Self {
        OutputViewer {
            value,
            tuner: OutputTuner::default(),
        }
    }
}

impl<T> Viewer for OutputViewer<T>
where
    T: Clone,
{
    type Value = T;

    fn finish(&self) -> Self::Value {
        self.value.clone()
    }
}

impl<T> Tune for OutputViewer<T> {
    type Tuner = OutputTuner;

    fn tune(&mut self, tuner: Self::Tuner) {
        self.tuner = tuner;
    }
}

macro_rules! impl_to_html {
    ($($ty:ty),*) => {
        $(
            impl ToHtml for OutputViewer<$ty> {
                fn to_html(&self) -> Html {
                    html! {r#"
                        <span style="color: {self.tuner.color};">{self.value}</span>
                    "#}
                }
            }
        )*
    };
}

impl_to_html!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, bool, char, &str,
    String
);

// impl ToHtml for OutputViewer<bool> {
//     fn to_html(&self) -> Html {
//         let checked = self
//             .value
//             .as_ref()
//             .copied()
//             .unwrap_or_default()
//             .then(|| "checked");
//         html! {r#"
//             <label class="checkbox">
//                 <input id="{self.id}" type="checkbox" onclick="update(this, this.checked.toString())" {checked}>
//                 {self.tuner.label}
//             </label>
//         "#}
//     }
// }
