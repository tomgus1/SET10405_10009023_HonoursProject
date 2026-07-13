use gpui::{App, AppContext, Context, Entity, Subscription};

/// A single reactive value. Each `Signal<T>` is backed by its own GPUI
/// `Entity`, so observers that only `watch()` this signal are notified only
/// when THIS value changes — not when unrelated state elsewhere in the
/// view model changes. Composing a view model out of several independent
/// `Signal`s (rather than one struct with many fields behind one `Entity`)
/// is what gives the fine-grained "propagate only to dependent components"
/// behaviour described by the framework's reactive state model.
pub struct Signal<T>(Entity<T>);

impl<T: 'static> Signal<T> {
    pub fn new(cx: &mut App, value: T) -> Self {
        Self(cx.new(|_cx| value))
    }

    pub fn read<'a>(&self, cx: &'a App) -> &'a T {
        self.0.read(cx)
    }

    pub fn set(&self, cx: &mut App, value: T) {
        self.0.update(cx, |current, cx| {
            *current = value;
            cx.notify();
        });
    }

    /// Re-render `observer` whenever this signal's value changes.
    pub fn watch<O: 'static>(&self, cx: &mut Context<O>) -> Subscription {
        cx.observe(&self.0, |_observer, _signal, cx| cx.notify())
    }
}

impl<T> Clone for Signal<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
