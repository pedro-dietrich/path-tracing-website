use leptos::prelude::*;
use crate::i18n::Text;
use crate::preferences::tr;

#[component]
pub fn Implementation() -> impl IntoView {
    view! {
        <h1>{tr(Text::Implementation)}</h1>
        <p>{tr(Text::ImplementationDescription)}</p>
    }
}
