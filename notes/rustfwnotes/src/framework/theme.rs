use gpui::{App, Global, Hsla, hsla, rgb};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Light,
    Dark,
}

impl ThemeMode {
    pub fn is_dark(self) -> bool {
        matches!(self, ThemeMode::Dark)
    }
}

/// Central design tokens for the framework: colors, spacing, typography, radius.
/// Application code never hard-codes a color/size; it reads these instead.
#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub mode: ThemeMode,
    pub background: Hsla,
    pub surface: Hsla,
    pub foreground: Hsla,
    pub muted_foreground: Hsla,
    pub border: Hsla,
    pub primary: Hsla,
    pub primary_foreground: Hsla,
    pub danger: Hsla,
    pub danger_foreground: Hsla,
    pub selection: Hsla,
    pub cursor: Hsla,
    pub radius: gpui::Pixels,
}

impl Theme {
    pub fn light() -> Self {
        Self {
            mode: ThemeMode::Light,
            background: rgb(0xffffff).into(),
            surface: rgb(0xf4f4f5).into(),
            foreground: rgb(0x18181b).into(),
            muted_foreground: rgb(0x71717a).into(),
            border: rgb(0xe4e4e7).into(),
            primary: rgb(0x2563eb).into(),
            primary_foreground: rgb(0xffffff).into(),
            danger: rgb(0xdc2626).into(),
            danger_foreground: rgb(0xffffff).into(),
            selection: hsla(217. / 360., 0.91, 0.60, 0.35),
            cursor: rgb(0x2563eb).into(),
            radius: gpui::px(6.),
        }
    }

    pub fn dark() -> Self {
        Self {
            mode: ThemeMode::Dark,
            background: rgb(0x18181b).into(),
            surface: rgb(0x27272a).into(),
            foreground: rgb(0xfafafa).into(),
            muted_foreground: rgb(0xa1a1aa).into(),
            border: rgb(0x3f3f46).into(),
            primary: rgb(0x3b82f6).into(),
            primary_foreground: rgb(0xffffff).into(),
            danger: rgb(0xef4444).into(),
            danger_foreground: rgb(0xffffff).into(),
            selection: hsla(217. / 360., 0.91, 0.60, 0.35),
            cursor: rgb(0x60a5fa).into(),
            radius: gpui::px(6.),
        }
    }

    pub fn for_mode(mode: ThemeMode) -> Self {
        match mode {
            ThemeMode::Light => Self::light(),
            ThemeMode::Dark => Self::dark(),
        }
    }

    /// Install the default (light) theme as a global. Call once at app startup.
    pub fn install(cx: &mut App) {
        cx.set_global(ThemeGlobal(Self::light()));
    }

    /// Swap the active theme mode and refresh all windows so the new colors paint.
    pub fn set_mode(mode: ThemeMode, cx: &mut App) {
        cx.set_global(ThemeGlobal(Self::for_mode(mode)));
        cx.refresh_windows();
    }
}

struct ThemeGlobal(Theme);

impl Global for ThemeGlobal {}

/// Read access to the active theme from anywhere a `App`/`Context` is available.
pub trait ActiveTheme {
    fn theme(&self) -> &Theme;
}

impl ActiveTheme for App {
    fn theme(&self) -> &Theme {
        &self.global::<ThemeGlobal>().0
    }
}

impl<T> ActiveTheme for gpui::Context<'_, T> {
    fn theme(&self) -> &Theme {
        &self.global::<ThemeGlobal>().0
    }
}
