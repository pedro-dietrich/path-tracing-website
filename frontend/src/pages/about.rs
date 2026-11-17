use leptos::prelude::*;
use crate::i18n::Text;
use crate::preferences::tr;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <h1>{tr(Text::About)}</h1>
        <p>{tr(Text::AboutDescription)}</p>
    }
}
