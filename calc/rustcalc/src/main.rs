use gpui::*;
use rustcalc::CalculatorView;

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(340.0), px(480.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some("Rust Calculator".into()),
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_window, cx| cx.new(CalculatorView::new),
        )
        .unwrap();
    });
}
