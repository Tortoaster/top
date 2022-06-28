use async_trait::async_trait;
use uuid::Uuid;

use top_derive::html;

use crate::html::{Html, ToHtml};
use crate::prelude::TaskValue;
use crate::share::guard::ShareGuard;

#[async_trait]
pub trait Form: Sized {
    async fn form(
        value: ShareGuard<'_, TaskValue<Self>>,
        id: &Uuid,
        label: &Option<String>,
    ) -> Html;
}

#[async_trait]
impl Form for String {
    async fn form(
        value: ShareGuard<'_, TaskValue<Self>>,
        id: &Uuid,
        label: &Option<String>,
    ) -> Html {
        html! {r#"
            <label for="{id}" class="label">{label}</label>
            <input id="{id}" class="input" value="{value}" oninput="update(this)"/>
        "#}
    }
}

macro_rules! impl_to_html_for_number {
    ($($ty:ty),*) => {
        $(
            #[async_trait]
            impl Form for $ty {
                async fn form(value: ShareGuard<'_, TaskValue<Self>>, id: &Uuid, label: &Option<String>) -> Html {
                    html! {r#"
                        <label for="{id}" class="label">{label}</label>
                        <input id="{id}" type="number" class="input" value="{value}" oninput="update(this)"/>
                    "#}
                }
            }
        )*
    };
}

impl_to_html_for_number!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

#[async_trait]
impl Form for bool {
    async fn form(
        value: ShareGuard<'_, TaskValue<Self>>,
        id: &Uuid,
        label: &Option<String>,
    ) -> Html {
        let checked = value.as_ref().map(|x| x.then(|| " checked"));
        html! {r#"
            <label class="checkbox">
                <input id="{id}" type="checkbox" onclick="update(this, this.checked.toString())"{checked}/>
                {label}
            </label>
        "#}
    }
}

#[async_trait]
impl Form for char {
    async fn form(
        value: ShareGuard<'_, TaskValue<Self>>,
        id: &Uuid,
        label: &Option<String>,
    ) -> Html {
        html! {r#"
            <label for="{id}" class="label">{label}</label>
            <input id="{id}" class="input" value="{value}" oninput="update(this)" maxlength="1"/>
        "#}
    }
}
