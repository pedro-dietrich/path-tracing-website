use leptos::prelude::*;
use web_sys::window;

use crate::i18n::{translate, Language, Text};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Light,
    Dark
}

impl Theme {
    pub fn code(self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark"
        }
    }
}

#[derive(Clone, Copy)]
pub struct Preferences {
    pub language: ReadSignal<Language>,
    pub set_language: WriteSignal<Language>,
    pub theme: ReadSignal<Theme>,
    pub set_theme: WriteSignal<Theme>
}

fn load(key: &str) -> Option<String> {
    window()?
        .local_storage().ok()??
        .get_item(key).ok()?
}

fn save(key: &str, value: &str) {
    if let Some(storage) = window().and_then(|w| w.local_storage().ok().flatten())
    {
        let _ = storage.set_item(key, value);
    }
}

pub fn init_preferences() {
    let initial_language = load("language")
        .map(|s| Language::from_code(&s))
        .unwrap_or(Language::En);

    let initial_theme = match load("theme").as_deref() {
        Some("light") => Theme::Light,
        _ => Theme::Dark
    };

    let (language, set_language) = signal(initial_language);
    let (theme, set_theme) = signal(initial_theme);

    provide_context(Preferences {
        language,
        set_language,
        theme,
        set_theme
    });

    Effect::new(move |_| {
        let lang = language.get();
        save("language", lang.code());

        if let Some(document) = window().and_then(|w| w.document()) {
            if let Some(html) = document.document_element() {
                let _ = html.set_attribute("lang", lang.code());
            }
        }
    });

    Effect::new(move |_| {
        let value = theme.get().code();
        save("theme", value);

        if let Some(document) = window().and_then(|w| w.document()) {
            if let Some(html) = document.document_element() {
                let _ = html.set_attribute("data-theme", value);
            }
        }
    });
}

pub fn tr(key: Text) -> impl Fn() -> &'static str {
    let prefs = use_context::<Preferences>().expect("Preferences context missing");

    move || translate(prefs.language.get(), key)
}
