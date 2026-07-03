use std::cell::RefCell;
use std::rc::Rc;

use gpui::{
    Context, Div, ElementId, InteractiveElement, IntoElement, MouseButton, MouseDownEvent,
    MouseMoveEvent, MouseUpEvent, ParentElement, Pixels, ScrollHandle, Stateful,
    StatefulInteractiveElement, Styled, canvas, div, point, px,
};

use crate::framework::theme::ActiveTheme;

const THUMB_WIDTH: Pixels = px(8.);
const THUMB_MIN_HEIGHT: Pixels = px(24.);

/// Backing state for a vertical scrollbar: the `ScrollHandle` that GPUI's
/// own `overflow_y_scroll`/`track_scroll` machinery reads and writes, plus
/// the in-progress drag offset (if the thumb is currently being dragged).
/// Owned by whichever entity renders the scrollable list.
#[derive(Clone)]
pub struct ScrollbarState {
    handle: ScrollHandle,
    drag_offset: Rc<RefCell<Option<Pixels>>>,
}

impl Default for ScrollbarState {
    fn default() -> Self {
        Self::new()
    }
}

impl ScrollbarState {
    pub fn new() -> Self {
        Self {
            handle: ScrollHandle::new(),
            drag_offset: Rc::new(RefCell::new(None)),
        }
    }
}

/// Wraps `content` in a scrollable, `min-height`-safe container bound to
/// `state`'s handle. Callers must place this inside a bounded-height flex
/// item; a flex item's default `min-height: auto` otherwise stops it from
/// ever shrinking enough to overflow, which is why this helper sets
/// `min_h(0)` for callers rather than leaving it to be forgotten.
pub fn scroll_container(id: impl Into<ElementId>, state: &ScrollbarState) -> Stateful<Div> {
    div()
        .id(id)
        .flex_1()
        .min_h(px(0.))
        .overflow_y_scroll()
        .track_scroll(&state.handle)
}

/// Renders the draggable thumb for `state`. Place as a sibling of the
/// element returned by `scroll_container`, inside a `.relative()` parent.
pub fn scrollbar<V: 'static>(state: &ScrollbarState, cx: &Context<V>) -> impl IntoElement {
    let theme = *cx.theme();
    let entity_id = cx.entity_id();
    let handle = state.handle.clone();
    let drag_offset = state.drag_offset.clone();

    let bounds = handle.bounds();
    let max_offset = handle.max_offset().height;
    let track_height = bounds.size.height;

    if max_offset <= px(0.) || track_height <= px(0.) {
        return div().id("scrollbar-empty");
    }

    let visible_fraction = (track_height / (track_height + max_offset)).clamp(0., 1.);
    let thumb_height = (track_height * visible_fraction).max(THUMB_MIN_HEIGHT).min(track_height);
    let scroll_fraction = (-handle.offset().y / max_offset).clamp(0., 1.);
    let thumb_top = scroll_fraction * (track_height - thumb_height);

    div()
        .id("scrollbar")
        .absolute()
        .top(thumb_top)
        .right_1()
        .w(THUMB_WIDTH)
        .h(thumb_height)
        .rounded(theme.radius)
        .bg(theme.border)
        .hover(|style| style.bg(theme.muted_foreground))
        .child(canvas(
            |_, _, _| (),
            move |thumb_bounds, _, window, _cx| {
                let handle = handle.clone();
                let drag_offset = drag_offset.clone();

                window.on_mouse_event({
                    let drag_offset = drag_offset.clone();
                    move |event: &MouseDownEvent, _phase, _window, cx| {
                        if event.button == MouseButton::Left && thumb_bounds.contains(&event.position) {
                            *drag_offset.borrow_mut() = Some(event.position.y - thumb_bounds.origin.y);
                        }
                        let _ = cx;
                    }
                });

                window.on_mouse_event({
                    let drag_offset = drag_offset.clone();
                    move |_event: &MouseUpEvent, _phase, _window, _cx| {
                        *drag_offset.borrow_mut() = None;
                    }
                });

                window.on_mouse_event(move |event: &MouseMoveEvent, _phase, _window, cx| {
                    if !event.dragging() {
                        return;
                    }
                    let Some(inside_offset) = *drag_offset.borrow() else {
                        return;
                    };
                    let track_height = bounds.size.height;
                    let usable = (track_height - thumb_height).max(px(1.));
                    let fraction =
                        ((event.position.y - bounds.top() - inside_offset) / usable).clamp(0., 1.);
                    handle.set_offset(point(px(0.), -fraction * max_offset));
                    cx.notify(entity_id);
                });
            },
        ))
}
