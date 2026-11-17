use leptos::prelude::*;
use crate::i18n::Text;
use crate::preferences::tr;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <h1>{tr(Text::HomeTitle)}</h1>
        <p>{tr(Text::HomeDescription)}</p>
    }
}
