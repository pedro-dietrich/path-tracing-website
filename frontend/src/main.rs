mod components;
mod i18n;
mod pages;
mod preferences;

use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path
};

use components::navbar::Navbar;
use pages::{
    about::About,
    home::Home,
    implementation::Implementation,
    methodology::Methodology,
    results::Results
};

use preferences::init_preferences;

#[component]
fn App() -> impl IntoView {
    init_preferences();

    view! {
        <Router>
            <Navbar />
            <main class="content">
                <Routes fallback=|| view! {
                    <h1>"404 - Page not found"</h1>
                }>
                    <Route path=path!("/") view=Home />
                    <Route path=path!("/methodology") view=Methodology />
                    <Route path=path!("/results") view=Results />
                    <Route path=path!("/implementation") view=Implementation />
                    <Route path=path!("/about") view=About />
                </Routes>
            </main>
        </Router>
    }
}

fn main() {
    leptos::mount::mount_to_body(App);
}
