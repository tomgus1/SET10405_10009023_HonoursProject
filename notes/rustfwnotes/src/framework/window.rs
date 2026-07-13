use gpui::{App, AppContext, Bounds, Entity, Render, WindowBounds, WindowOptions, px, size};

/// Opens a centered, resizable application window at the given logical size
/// and mounts `build_root`'s view as its content. GPUI's `WindowOptions`/
/// `cx.open_window` already abstract macOS/Linux/Windows differences fully —
/// this wrapper just gives application code one call instead of hand-writing
/// `Bounds`/`WindowBounds` boilerplate per app.
pub fn open_window<V: Render>(
    cx: &mut App,
    width: f32,
    height: f32,
    build_root: impl FnOnce(&mut gpui::Window, &mut gpui::Context<V>) -> V + 'static,
) {
    let bounds = Bounds::centered(None, size(px(width), px(height)), cx);
    cx.open_window(
        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            ..Default::default()
        },
        |window, cx| -> Entity<V> { cx.new(|cx| build_root(window, cx)) },
    )
    .expect("failed to open window");
}
