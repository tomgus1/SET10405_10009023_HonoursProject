use gpui::{App, Application};
use rustcalc::framework;
use rustcalc::framework::theme::{Theme, ThemeMode};
use rustcalc::CalculatorView;

fn main() {
    Application::new().run(|cx: &mut App| {
        framework::init(cx);
        Theme::set_mode(ThemeMode::Dark, cx);
        framework::window::open_window(cx, "Rust Calculator", 340.0, 480.0, |_window, cx| {
            CalculatorView::new(cx)
        });
    });
}
