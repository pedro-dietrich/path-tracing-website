use leptos::prelude::*;
use leptos_router::components::A;

use crate::i18n::{Language, Text};
use crate::preferences::{tr, Preferences, Theme};

#[component]
pub fn Navbar() -> impl IntoView {
    let prefs = use_context::<Preferences>().expect("Preferences context missing");
    let toggle_theme = move |_| {
        prefs.set_theme.update(|theme| {
            *theme = match *theme {
                Theme::Dark => Theme::Light,
                Theme::Light => Theme::Dark
            };
        });
    };

    view! {
        <header class="topbar">
            <div class="topbar-inner">
                <A href="/" exact=true attr:class="brand">
                    <span class="brand-title">{tr(Text::NavbarTitle)}</span>
                    <span class="brand-subtitle">{tr(Text::NavbarSubtitle)}</span>
                </A>

                <nav class="nav-links" aria-label="Main navigation">
                    <A href="/" exact=true>{tr(Text::Home)}</A>
                    <A href="/methodology">{tr(Text::Methodology)}</A>
                    <A href="/results">{tr(Text::Results)}</A>
                    <A href="/implementation">{tr(Text::Implementation)}</A>
                    <A href="/about">{tr(Text::About)}</A>
                </nav>

                <div class="nav-settings">
                    <button
                        type="button"
                        class="theme-button"
                        title=tr(Text::ThemeToggle)
                        aria-label=tr(Text::ThemeToggle)
                        on:click=toggle_theme
                    >
                        {move || match prefs.theme.get() {
                            Theme::Dark => view! {
                                <svg xmlns="http://www.w3.org/2000/svg"
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    stroke="currentColor"
                                    stroke-width="2"
                                    stroke-linecap="round"
                                    stroke-linejoin="round">
                                    <circle cx="12" cy="12" r="4"/>
                                    <path d="M12 2v2M12 20v2M4.93 4.93l1.42 1.42 M17.65 17.65l1.42 1.42 M2 12h2M20 12h2 M6.35 17.65l-1.42 1.42 M19.07 4.93l-1.42 1.42"/>
                                </svg>
                            }.into_any(),

                            Theme::Light => view! {
                                <svg xmlns="http://www.w3.org/2000/svg"
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    stroke="currentColor"
                                    stroke-width="2"
                                    stroke-linecap="round"
                                    stroke-linejoin="round">
                                    <path d="M20.985 12.486A9 9 0 0 1 11.514 3.015 9 9 0 1 0 20.985 12.486Z"/>
                                </svg>
                            }.into_any()
                        }}
                    </button>

                    <select
                        aria-label="Language"
                        prop:value=move || prefs.language.get().code()
                        on:change=move |ev| {
                            let value = event_target_value(&ev);
                            prefs.set_language.set(Language::from_code(&value));
                        }
                    >
                        <option value="en">"English"</option>
                        <option value="pt-BR">"Português"</option>
                    </select>
                </div>
            </div>
        </header>
    }
}
