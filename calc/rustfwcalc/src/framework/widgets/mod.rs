mod button;
mod dialog;
mod list;
mod scrollbar;
mod text_input;

/// Shared boxed-closure alias for `Button`/`ListItem` click handlers, kept
/// in one place so clippy's `type_complexity` lint doesn't get triggered
/// three times over for what is really one shape.
pub(crate) type ClickHandler = Box<dyn Fn(&gpui::ClickEvent, &mut gpui::Window, &mut gpui::App)>;

pub use button::Button;
pub use dialog::{confirm_dialog, render_dialog_layer};
pub use list::ListItem;
pub use scrollbar::{ScrollbarState, scroll_container, scrollbar};
pub use text_input::{TextInput, TextInputState, install_keybindings};
