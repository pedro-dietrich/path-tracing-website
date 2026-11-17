use leptos::prelude::*;
use crate::i18n::Text;
use crate::preferences::tr;

#[component]
pub fn Methodology() -> impl IntoView {
    view! {
        <h1>{tr(Text::Methodology)}</h1>
        <p>{tr(Text::MethodologyDescription)}</p>
    }
}
