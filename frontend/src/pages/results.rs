use leptos::prelude::*;
use crate::i18n::Text;
use crate::preferences::tr;

#[component]
pub fn Results() -> impl IntoView {
    view! {
        <h1>{tr(Text::Results)}</h1>
        <p>{tr(Text::ResultsDescription)}</p>
    }
}
