use leptos::prelude::*;
use leptos::html::button;


#[component]
pub fn ThemeToggle() -> impl IntoView {
    let is_dark = RwSignal::new(false);
    
    // Initialize theme from localStorage on hydration
    Effect::new(move |_| {
        if cfg!(feature = "hydrate") {
            if let Some(window) = leptos::web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(body) = document.body() {
                        // Check localStorage for saved theme
                        if let Some(storage) = window.local_storage().ok().flatten() {
                            if let Ok(Some(theme)) = storage.get_item("theme-dark") {
                                if theme == "true" {
                                    is_dark.set(true);
                                    let _ = body.class_list().add_1("dark-theme");
                                } else {
                                    let _ = body.class_list().remove_1("dark-theme");
                                }
                            }
                        }
                    }
                }
            }
        }
    });
    
    // Update DOM when theme changes
    Effect::new(move |_| {
        if cfg!(feature = "hydrate") {
            if let Some(window) = leptos::web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(body) = document.body() {
                        if is_dark.get() {
                            let _ = body.class_list().add_1("dark-theme");
                            // Save to localStorage
                            if let Some(storage) = window.local_storage().ok().flatten() {
                                let _ = storage.set_item("theme-dark", "true");
                            }
                        } else {
                            let _ = body.class_list().remove_1("dark-theme");
                            // Save to localStorage
                            if let Some(storage) = window.local_storage().ok().flatten() {
                                let _ = storage.set_item("theme-dark", "false");
                            }
                        }
                    }
                }
            }
        }
    });
    
    let toggle_theme = move || {
        is_dark.update(|dark| *dark = !*dark);
    };

    button()
        .id("theme-toggle")
        .class("theme-toggle")
        .attr("aria-label", "Toggle dark mode")
        .onclick(toggle_theme)
        .child(move || if is_dark.get() { "☀️" } else { "🌙" })
}