use uuid::Uuid;

use crate::html::Html;
use crate::task::TaskValue;

pub trait FromForm: Sized {
    fn from_form(value: String) -> TaskValue<Self>;
}

pub trait IntoForm: Sized {
    fn into_form(value: &TaskValue<Self>, id: &Uuid, label: &str) -> Html;
}

macro_rules! impl_from_form {
    ($($ty:ty),*) => {
        $(
            impl FromForm for $ty {
                fn from_form(value: String) -> TaskValue<Self> {
                    match value.parse::<Self>() {
                        Ok(value) => TaskValue::Unstable(value),
                        Err(error) => TaskValue::Error(error.to_string()),
                    }
                }
            }
        )*
    };
}

impl_from_form!(
    String, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64, bool, char
);

impl IntoForm for String {
    fn into_form(value: &TaskValue<Self>, id: &Uuid, label: &str) -> Html {
        Html(format!(
            r#"
            <label for="{id}" class="label">{label}</label>
            <input id="{id}" class="input" value="{}" oninput="update(this)"/>
        "#,
            value.clone().unwrap_or_default()
        ))
    }
}

macro_rules! impl_into_form_for_number {
    ($($ty:ty),*) => {
        $(
            impl IntoForm for $ty {
                fn into_form(value: &TaskValue<Self>, id: &Uuid, label: &str) -> Html {
                    Html(format!(r#"
                        <label for="{id}" class="label">{label}</label>
                        <input id="{id}" type="number" class="input" value="{}" oninput="update(this)"/>
                    "#,
                        value.as_ref().map(ToString::to_string).unwrap_or_default()
                    ))
                }
            }
        )*
    };
}

impl_into_form_for_number!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64
);

impl IntoForm for bool {
    fn into_form(value: &TaskValue<Self>, id: &Uuid, label: &str) -> Html {
        let checked = value
            .as_ref()
            .map(|checked| checked.then(|| "checked").unwrap_or_default())
            .unwrap_or_default();
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

impl IntoForm for char {
    fn into_form(value: &TaskValue<Self>, id: &Uuid, label: &str) -> Html {
        Html(format!(
            r#"
            <label for="{id}" class="label">{label}</label>
            <input id="{id}" class="input" value="{}" oninput="update(this)" maxlength="1"/>
        "#,
            value.as_ref().map(ToString::to_string).unwrap_or_default()
        ))
    }
}
