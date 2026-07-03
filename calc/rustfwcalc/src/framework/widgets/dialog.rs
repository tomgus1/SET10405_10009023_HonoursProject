use gpui::{
    AnyElement, App, FocusHandle, Global, InteractiveElement, IntoElement, KeyDownEvent,
    ParentElement, SharedString, Styled, Window, deferred, div,
};

use crate::framework::layout::{column, row};
use crate::framework::theme::ActiveTheme;
use crate::framework::widgets::button::Button;

type ConfirmHandler = Box<dyn FnOnce(&mut Window, &mut App)>;

/// A single pending confirm/cancel dialog. GPUI only provides the raw
/// `deferred()` layering primitive; the modal/backdrop/focus-trap/escape
/// behaviour here is the framework's own contribution on top of it.
struct DialogRequest {
    title: SharedString,
    body: SharedString,
    focus_handle: FocusHandle,
    on_confirm: ConfirmHandler,
}

#[derive(Default)]
struct DialogGlobal(Option<DialogRequest>);

impl Global for DialogGlobal {}

/// Open a confirm/cancel dialog. Replaces any dialog already open.
pub fn confirm_dialog(
    window: &mut Window,
    cx: &mut App,
    title: impl Into<SharedString>,
    body: impl Into<SharedString>,
    on_confirm: impl FnOnce(&mut Window, &mut App) + 'static,
) {
    let focus_handle = cx.focus_handle();
    window.focus(&focus_handle);

    cx.set_global(DialogGlobal(Some(DialogRequest {
        title: title.into(),
        body: body.into(),
        focus_handle,
        on_confirm: Box::new(on_confirm),
    })));
    cx.refresh_windows();
}

fn close_dialog(cx: &mut App) {
    cx.set_global(DialogGlobal(None));
    cx.refresh_windows();
}

/// Renders the currently open dialog (if any) as a top-level overlay.
/// The root view of the application should call this once, at the end of
/// its own render, and add the returned element as a child.
pub fn render_dialog_layer(window: &mut Window, cx: &mut App) -> Option<AnyElement> {
    let request = cx.try_global::<DialogGlobal>()?.0.as_ref()?;
    let theme = *cx.theme();
    let title = request.title.clone();
    let body = request.body.clone();
    let focus_handle = request.focus_handle.clone();

    window.focus(&focus_handle);

    let backdrop = div()
        .id("dialog-backdrop")
        .absolute()
        .inset_0()
        .bg(gpui::hsla(0., 0., 0., 0.4))
        .track_focus(&focus_handle)
        .on_key_down(|event: &KeyDownEvent, _window, cx| {
            if event.keystroke.key == "escape" {
                close_dialog(cx);
            }
        })
        .on_mouse_down(gpui::MouseButton::Left, |_event, _window, cx| {
            close_dialog(cx);
        })
        .child(
            div()
                .id("dialog-box")
                .absolute()
                .top(gpui::relative(0.35))
                .left(gpui::relative(0.5))
                .w(gpui::px(360.))
                .ml(gpui::px(-180.))
                .rounded(theme.radius)
                .border_1()
                .border_color(theme.border)
                .bg(theme.surface)
                .text_color(theme.foreground)
                .p_4()
                .on_mouse_down(gpui::MouseButton::Left, |_event, _window, _cx| {
                    // Swallow clicks inside the dialog so they don't bubble to the backdrop.
                })
                .child(
                    column()
                        .gap_3()
                        .child(div().font_weight(gpui::FontWeight::BOLD).child(title))
                        .child(div().text_sm().child(body))
                        .child(
                            row()
                                .gap_2()
                                .justify_end()
                                .child(Button::new("dialog-cancel", "Cancel").on_click(
                                    |_event, _window, cx| {
                                        close_dialog(cx);
                                    },
                                ))
                                .child(Button::new("dialog-confirm", "Confirm").danger().on_click(
                                    |_event, window, cx| {
                                        let Some(request) =
                                            cx.global_mut::<DialogGlobal>().0.take()
                                        else {
                                            return;
                                        };
                                        (request.on_confirm)(window, cx);
                                        close_dialog(cx);
                                    },
                                )),
                        ),
                ),
        );

    Some(deferred(backdrop).with_priority(1).into_any_element())
}
