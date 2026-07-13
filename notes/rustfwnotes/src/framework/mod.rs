//! The Rust GUI framework layer described in Section 3.2 of the interim
//! report: component abstraction, reactive state, layout, theming, and
//! window management, built directly on `gpui`. Application code (`ui`,
//! `viewmodel`) depends on this module; this module never depends on it.

pub mod layout;
pub mod state;
pub mod theme;
pub mod widgets;
pub mod window;

/// Installs the framework's global state (theme, text-input keybindings).
/// Call once at application startup, before opening any window.
pub fn init(cx: &mut gpui::App) {
    theme::Theme::install(cx);
    widgets::install_keybindings(cx);
}
