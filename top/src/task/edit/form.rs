use uuid::Uuid;

use crate::html::Html;

pub trait Form {
    fn form(&self, id: &Uuid, label: &str) -> Html;
}

impl Form for String {
    fn form(&self, id: &Uuid, label: &str) -> Html {
        Html(format!(
            r#"
            <label for="{id}" class="label">{label}</label>
            <input id="{id}" class="input" value="{self}" oninput="update(this)"/>
        "#
        ))
    }
}

macro_rules! impl_to_html_for_number {
    ($($ty:ty),*) => {
        $(
            impl Form for $ty {
                fn form(&self, id: &Uuid, label: &str) -> Html {
                    Html(format!(r#"
                        <label for="{id}" class="label">{label}</label>
                        <input id="{id}" type="number" class="input" value="{self}" oninput="update(this)"/>
                    "#))
                }
            }
        )*
    };
}

impl_to_html_for_number!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

impl Form for bool {
    fn form(&self, id: &Uuid, label: &str) -> Html {
        let checked = self.then("checked").unwrap_or_default();
        Html(format!(
            r#"
            <label class="checkbox">
                <input id="{id}" type="checkbox" onclick="update(this, this.checked.toString())" {checked}/>
                {label}
            </label>
        "#
        ))
    }
}

impl Form for char {
    fn form(&self, id: &Uuid, label: &str) -> Html {
        Html(format!(
            r#"
            <label for="{id}" class="label">{label}</label>
            <input id="{id}" class="input" value="{self}" oninput="update(this)" maxlength="1"/>
        "#
        ))
    }
}
