mod framework;
mod model;
mod repository;
mod service;
mod ui;
mod viewmodel;

use gpui::{App, Application};

fn main() {
    Application::new().run(|cx: &mut App| {
        framework::init(cx);
        framework::window::open_window(cx, 1080., 680., |_window, cx| ui::NotesApp::new(cx));
        cx.activate(true);
    });
}
