use std::ops::Range;

use gpui::{
    App, Bounds, ClipboardItem, Context, CursorStyle, Element, ElementId, ElementInputHandler,
    Entity, EntityInputHandler, FocusHandle, Focusable, GlobalElementId, InteractiveElement,
    IntoElement, KeyBinding, LayoutId, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent,
    ParentElement, PaintQuad, Pixels, Point, Render, RenderOnce, ShapedLine, SharedString, Style,
    Styled, TextRun, UTF16Selection, Window, actions, div, fill, point, px, relative, size,
};
use unicode_segmentation::UnicodeSegmentation;

use crate::framework::theme::ActiveTheme;

actions!(
    framework_text_input,
    [
        TextInputBackspace,
        TextInputDelete,
        TextInputLeft,
        TextInputRight,
        TextInputSelectLeft,
        TextInputSelectRight,
        TextInputSelectAll,
        TextInputHome,
        TextInputEnd,
        TextInputNewline,
        TextInputPaste,
        TextInputCut,
        TextInputCopy,
    ]
);

/// Registers the key bindings the text field's actions rely on. Call once
/// at application startup (`framework::init`).
pub fn install_keybindings(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("backspace", TextInputBackspace, Some("TextInput")),
        KeyBinding::new("delete", TextInputDelete, Some("TextInput")),
        KeyBinding::new("left", TextInputLeft, Some("TextInput")),
        KeyBinding::new("right", TextInputRight, Some("TextInput")),
        KeyBinding::new("shift-left", TextInputSelectLeft, Some("TextInput")),
        KeyBinding::new("shift-right", TextInputSelectRight, Some("TextInput")),
        KeyBinding::new("cmd-a", TextInputSelectAll, Some("TextInput")),
        KeyBinding::new("ctrl-a", TextInputSelectAll, Some("TextInput")),
        KeyBinding::new("home", TextInputHome, Some("TextInput")),
        KeyBinding::new("end", TextInputEnd, Some("TextInput")),
        KeyBinding::new("enter", TextInputNewline, Some("TextInput")),
        KeyBinding::new("cmd-v", TextInputPaste, Some("TextInput")),
        KeyBinding::new("ctrl-v", TextInputPaste, Some("TextInput")),
        KeyBinding::new("cmd-c", TextInputCopy, Some("TextInput")),
        KeyBinding::new("ctrl-c", TextInputCopy, Some("TextInput")),
        KeyBinding::new("cmd-x", TextInputCut, Some("TextInput")),
        KeyBinding::new("ctrl-x", TextInputCut, Some("TextInput")),
    ]);
}

/// The state behind an editable text field: cursor position, selection,
/// IME composition range, and the raw content. GPUI provides no built-in
/// text-editing widget, only the low-level `EntityInputHandler` hook that
/// platform IME/keyboard input is wired through (see `examples/input.rs`
/// in the gpui crate, which this implementation follows closely).
///
/// Known limitation (v1): Up/Down arrow navigation between lines in
/// multi-line mode is not implemented; Left/Right/Home/End and mouse
/// selection are. This is a deliberate scope cut, not an oversight.
pub struct TextInputState {
    focus_handle: FocusHandle,
    content: SharedString,
    placeholder: SharedString,
    selected_range: Range<usize>,
    selection_reversed: bool,
    marked_range: Option<Range<usize>>,
    last_lines: Vec<ShapedLine>,
    last_line_starts: Vec<usize>,
    last_bounds: Option<Bounds<Pixels>>,
    is_selecting: bool,
    multi_line: bool,
    rows: usize,
}

impl TextInputState {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            focus_handle: cx.focus_handle(),
            content: SharedString::default(),
            placeholder: SharedString::default(),
            selected_range: 0..0,
            selection_reversed: false,
            marked_range: None,
            last_lines: Vec::new(),
            last_line_starts: Vec::new(),
            last_bounds: None,
            is_selecting: false,
            multi_line: false,
            rows: 1,
        }
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn multi_line(mut self, multi_line: bool) -> Self {
        self.multi_line = multi_line;
        self
    }

    pub fn rows(mut self, rows: usize) -> Self {
        self.rows = rows.max(1);
        self
    }

    pub fn value(&self) -> &SharedString {
        &self.content
    }

    pub fn set_value(&mut self, value: impl Into<SharedString>, cx: &mut Context<Self>) {
        self.content = value.into();
        self.selected_range = 0..0;
        self.marked_range = None;
        cx.notify();
    }

    fn cursor_offset(&self) -> usize {
        if self.selection_reversed {
            self.selected_range.start
        } else {
            self.selected_range.end
        }
    }

    fn move_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        self.selected_range = offset..offset;
        cx.notify();
    }

    fn select_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        if self.selection_reversed {
            self.selected_range.start = offset;
        } else {
            self.selected_range.end = offset;
        }
        if self.selected_range.end < self.selected_range.start {
            self.selection_reversed = !self.selection_reversed;
            self.selected_range = self.selected_range.end..self.selected_range.start;
        }
        cx.notify();
    }

    fn previous_boundary(&self, offset: usize) -> usize {
        self.content
            .grapheme_indices(true)
            .rev()
            .find_map(|(idx, _)| (idx < offset).then_some(idx))
            .unwrap_or(0)
    }

    fn next_boundary(&self, offset: usize) -> usize {
        self.content
            .grapheme_indices(true)
            .find_map(|(idx, _)| (idx > offset).then_some(idx))
            .unwrap_or(self.content.len())
    }

    /// Byte offset of the start of the line containing `offset`.
    fn line_start(&self, offset: usize) -> usize {
        self.content[..offset].rfind('\n').map_or(0, |idx| idx + 1)
    }

    /// Byte offset of the end of the line containing `offset` (exclusive of the newline).
    fn line_end(&self, offset: usize) -> usize {
        self.content[offset..]
            .find('\n')
            .map_or(self.content.len(), |idx| offset + idx)
    }

    fn offset_from_utf16(&self, offset: usize) -> usize {
        let mut utf8_offset = 0;
        let mut utf16_count = 0;
        for ch in self.content.chars() {
            if utf16_count >= offset {
                break;
            }
            utf16_count += ch.len_utf16();
            utf8_offset += ch.len_utf8();
        }
        utf8_offset
    }

    fn offset_to_utf16(&self, offset: usize) -> usize {
        let mut utf16_offset = 0;
        let mut utf8_count = 0;
        for ch in self.content.chars() {
            if utf8_count >= offset {
                break;
            }
            utf8_count += ch.len_utf8();
            utf16_offset += ch.len_utf16();
        }
        utf16_offset
    }

    fn range_to_utf16(&self, range: &Range<usize>) -> Range<usize> {
        self.offset_to_utf16(range.start)..self.offset_to_utf16(range.end)
    }

    fn range_from_utf16(&self, range_utf16: &Range<usize>) -> Range<usize> {
        self.offset_from_utf16(range_utf16.start)..self.offset_from_utf16(range_utf16.end)
    }

    /// Finds the byte offset closest to `position` using the last painted layout.
    fn index_for_mouse_position(&self, position: Point<Pixels>) -> usize {
        if self.content.is_empty() {
            return 0;
        }
        let Some(bounds) = self.last_bounds.as_ref() else {
            return 0;
        };
        let line_height = bounds.size.height / self.last_lines.len().max(1) as f32;
        let relative_y = (position.y - bounds.top()).max(px(0.));
        let row = ((relative_y / line_height) as usize).min(self.last_lines.len().saturating_sub(1));

        let (Some(line), Some(&line_start)) =
            (self.last_lines.get(row), self.last_line_starts.get(row))
        else {
            return self.content.len();
        };
        line_start + line.closest_index_for_x(position.x - bounds.left())
    }

    fn backspace(&mut self, _: &TextInputBackspace, window: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.select_to(self.previous_boundary(self.cursor_offset()), cx);
        }
        self.replace_text_in_range(None, "", window, cx);
    }

    fn delete(&mut self, _: &TextInputDelete, window: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.select_to(self.next_boundary(self.cursor_offset()), cx);
        }
        self.replace_text_in_range(None, "", window, cx);
    }

    fn left(&mut self, _: &TextInputLeft, _window: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.move_to(self.previous_boundary(self.cursor_offset()), cx);
        } else {
            self.move_to(self.selected_range.start, cx);
        }
    }

    fn right(&mut self, _: &TextInputRight, _window: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            self.move_to(self.next_boundary(self.selected_range.end), cx);
        } else {
            self.move_to(self.selected_range.end, cx);
        }
    }

    fn select_left(&mut self, _: &TextInputSelectLeft, _window: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.previous_boundary(self.cursor_offset()), cx);
    }

    fn select_right(&mut self, _: &TextInputSelectRight, _window: &mut Window, cx: &mut Context<Self>) {
        self.select_to(self.next_boundary(self.cursor_offset()), cx);
    }

    fn select_all(&mut self, _: &TextInputSelectAll, _window: &mut Window, cx: &mut Context<Self>) {
        self.move_to(0, cx);
        self.select_to(self.content.len(), cx);
    }

    fn home(&mut self, _: &TextInputHome, _window: &mut Window, cx: &mut Context<Self>) {
        let start = self.line_start(self.cursor_offset());
        self.move_to(start, cx);
    }

    fn end(&mut self, _: &TextInputEnd, _window: &mut Window, cx: &mut Context<Self>) {
        let end = self.line_end(self.cursor_offset());
        self.move_to(end, cx);
    }

    fn newline(&mut self, _: &TextInputNewline, window: &mut Window, cx: &mut Context<Self>) {
        if self.multi_line {
            self.replace_text_in_range(None, "\n", window, cx);
        }
    }

    fn paste(&mut self, _: &TextInputPaste, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(text) = cx.read_from_clipboard().and_then(|item| item.text()) {
            let text = if self.multi_line {
                text
            } else {
                text.replace('\n', " ")
            };
            self.replace_text_in_range(None, &text, window, cx);
        }
    }

    fn copy(&mut self, _: &TextInputCopy, _window: &mut Window, cx: &mut Context<Self>) {
        if !self.selected_range.is_empty() {
            cx.write_to_clipboard(ClipboardItem::new_string(
                self.content[self.selected_range.clone()].to_string(),
            ));
        }
    }

    fn cut(&mut self, _: &TextInputCut, window: &mut Window, cx: &mut Context<Self>) {
        if !self.selected_range.is_empty() {
            cx.write_to_clipboard(ClipboardItem::new_string(
                self.content[self.selected_range.clone()].to_string(),
            ));
            self.replace_text_in_range(None, "", window, cx);
        }
    }

    fn on_mouse_down(&mut self, event: &MouseDownEvent, _window: &mut Window, cx: &mut Context<Self>) {
        self.is_selecting = true;
        if event.modifiers.shift {
            self.select_to(self.index_for_mouse_position(event.position), cx);
        } else {
            self.move_to(self.index_for_mouse_position(event.position), cx);
        }
    }

    fn on_mouse_up(&mut self, _: &MouseUpEvent, _window: &mut Window, _cx: &mut Context<Self>) {
        self.is_selecting = false;
    }

    fn on_mouse_move(&mut self, event: &MouseMoveEvent, _window: &mut Window, cx: &mut Context<Self>) {
        if self.is_selecting {
            self.select_to(self.index_for_mouse_position(event.position), cx);
        }
    }
}

impl EntityInputHandler for TextInputState {
    fn text_for_range(
        &mut self,
        range_utf16: Range<usize>,
        actual_range: &mut Option<Range<usize>>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<String> {
        let range = self.range_from_utf16(&range_utf16);
        actual_range.replace(self.range_to_utf16(&range));
        Some(self.content[range].to_string())
    }

    fn selected_text_range(
        &mut self,
        _ignore_disabled_input: bool,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        Some(UTF16Selection {
            range: self.range_to_utf16(&self.selected_range),
            reversed: self.selection_reversed,
        })
    }

    fn marked_text_range(&self, _window: &mut Window, _cx: &mut Context<Self>) -> Option<Range<usize>> {
        self.marked_range.as_ref().map(|range| self.range_to_utf16(range))
    }

    fn unmark_text(&mut self, _window: &mut Window, _cx: &mut Context<Self>) {
        self.marked_range = None;
    }

    fn replace_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let range = range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .or(self.marked_range.clone())
            .unwrap_or(self.selected_range.clone());

        self.content =
            (self.content[0..range.start].to_owned() + new_text + &self.content[range.end..]).into();
        self.selected_range = range.start + new_text.len()..range.start + new_text.len();
        self.marked_range.take();
        cx.notify();
    }

    fn replace_and_mark_text_in_range(
        &mut self,
        range_utf16: Option<Range<usize>>,
        new_text: &str,
        new_selected_range_utf16: Option<Range<usize>>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let range = range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .or(self.marked_range.clone())
            .unwrap_or(self.selected_range.clone());

        self.content =
            (self.content[0..range.start].to_owned() + new_text + &self.content[range.end..]).into();
        self.marked_range = if !new_text.is_empty() {
            Some(range.start..range.start + new_text.len())
        } else {
            None
        };
        self.selected_range = new_selected_range_utf16
            .as_ref()
            .map(|range_utf16| self.range_from_utf16(range_utf16))
            .map(|new_range| new_range.start + range.start..new_range.end + range.end)
            .unwrap_or_else(|| range.start + new_text.len()..range.start + new_text.len());

        cx.notify();
    }

    fn bounds_for_range(
        &mut self,
        range_utf16: Range<usize>,
        bounds: Bounds<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<Bounds<Pixels>> {
        let range = self.range_from_utf16(&range_utf16);
        let row = self.content[..range.start].matches('\n').count();
        let line = self.last_lines.get(row)?;
        let line_start = *self.last_line_starts.get(row)?;
        let row_height = bounds.size.height / self.last_lines.len().max(1) as f32;
        let top = bounds.top() + row_height * row as f32;
        Some(Bounds::from_corners(
            point(bounds.left() + line.x_for_index(range.start - line_start), top),
            point(
                bounds.left() + line.x_for_index(range.end - line_start),
                top + row_height,
            ),
        ))
    }

    fn character_index_for_point(
        &mut self,
        point: Point<Pixels>,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> Option<usize> {
        Some(self.offset_to_utf16(self.index_for_mouse_position(point)))
    }
}

impl Focusable for TextInputState {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

struct TextElement {
    state: Entity<TextInputState>,
}

struct PrepaintState {
    lines: Vec<ShapedLine>,
    line_starts: Vec<usize>,
    cursor: Option<PaintQuad>,
    selection: Vec<PaintQuad>,
}

impl IntoElement for TextElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for TextElement {
    type RequestLayoutState = ();
    type PrepaintState = PrepaintState;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let input = self.state.read(cx);
        let line_count = if input.multi_line {
            input.content.matches('\n').count() + 1
        } else {
            1
        };
        let visible_rows = line_count.max(input.rows);

        let mut style = Style::default();
        style.size.width = relative(1.).into();
        style.size.height = (window.line_height() * visible_rows as f32).into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        let input = self.state.read(cx);
        let content = input.content.clone();
        let selected_range = input.selected_range.clone();
        let cursor = input.cursor_offset();
        let theme = *cx.theme();
        let style = window.text_style();
        let font_size = style.font_size.to_pixels(window.rem_size());
        let line_height = window.line_height();

        let is_empty = content.is_empty();
        let display_lines: Vec<SharedString> = if is_empty {
            vec![input.placeholder.clone()]
        } else {
            content.split('\n').map(|s| SharedString::from(s.to_string())).collect()
        };
        let text_color = if is_empty { theme.muted_foreground } else { style.color };

        let mut line_starts = Vec::with_capacity(display_lines.len());
        {
            let mut offset = 0;
            for line in &display_lines {
                line_starts.push(offset);
                offset += line.len() + 1;
            }
        }

        let lines: Vec<ShapedLine> = display_lines
            .iter()
            .map(|line| {
                let run = TextRun {
                    len: line.len(),
                    font: style.font(),
                    color: text_color,
                    background_color: None,
                    underline: None,
                    strikethrough: None,
                };
                window.text_system().shape_line(line.clone(), font_size, &[run], None)
            })
            .collect();

        let cursor_row = content[..cursor].matches('\n').count();
        let cursor_line_start = line_starts.get(cursor_row).copied().unwrap_or(0);

        let (selection, cursor_quad) = if selected_range.is_empty() {
            let row_top = bounds.top() + line_height * cursor_row as f32;
            let cursor_x = lines
                .get(cursor_row)
                .map(|l| l.x_for_index(cursor - cursor_line_start))
                .unwrap_or(px(0.));
            (
                Vec::new(),
                Some(fill(
                    Bounds::new(point(bounds.left() + cursor_x, row_top), size(px(2.), line_height)),
                    theme.cursor,
                )),
            )
        } else {
            let mut quads = Vec::new();
            for (row, line) in lines.iter().enumerate() {
                let row_start = line_starts[row];
                let row_end = row_start + display_lines[row].len();
                let sel_start = selected_range.start.max(row_start).min(row_end);
                let sel_end = selected_range.end.max(row_start).min(row_end);
                if sel_start >= sel_end {
                    continue;
                }
                let row_top = bounds.top() + line_height * row as f32;
                quads.push(fill(
                    Bounds::from_corners(
                        point(bounds.left() + line.x_for_index(sel_start - row_start), row_top),
                        point(
                            bounds.left() + line.x_for_index(sel_end - row_start),
                            row_top + line_height,
                        ),
                    ),
                    theme.selection,
                ));
            }
            (quads, None)
        };

        PrepaintState {
            lines,
            line_starts,
            cursor: cursor_quad,
            selection,
        }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let focus_handle = self.state.read(cx).focus_handle.clone();
        window.handle_input(&focus_handle, ElementInputHandler::new(bounds, self.state.clone()), cx);

        for selection in prepaint.selection.drain(..) {
            window.paint_quad(selection);
        }

        let line_height = window.line_height();
        for (row, line) in prepaint.lines.iter().enumerate() {
            let origin = point(bounds.left(), bounds.top() + line_height * row as f32);
            line.paint(origin, line_height, window, cx).ok();
        }

        if focus_handle.is_focused(window) {
            if let Some(cursor) = prepaint.cursor.take() {
                window.paint_quad(cursor);
            }
        }

        let lines = std::mem::take(&mut prepaint.lines);
        let line_starts = std::mem::take(&mut prepaint.line_starts);
        self.state.update(cx, |input, _cx| {
            input.last_lines = lines;
            input.last_line_starts = line_starts;
            input.last_bounds = Some(bounds);
        });
    }
}

impl Render for TextInputState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = *cx.theme();
        let min_height = window.line_height() * self.rows.max(1) as f32;

        div()
            .flex()
            .key_context("TextInput")
            .track_focus(&self.focus_handle(cx))
            .cursor(CursorStyle::IBeam)
            .on_action(cx.listener(Self::backspace))
            .on_action(cx.listener(Self::delete))
            .on_action(cx.listener(Self::left))
            .on_action(cx.listener(Self::right))
            .on_action(cx.listener(Self::select_left))
            .on_action(cx.listener(Self::select_right))
            .on_action(cx.listener(Self::select_all))
            .on_action(cx.listener(Self::home))
            .on_action(cx.listener(Self::end))
            .on_action(cx.listener(Self::newline))
            .on_action(cx.listener(Self::paste))
            .on_action(cx.listener(Self::cut))
            .on_action(cx.listener(Self::copy))
            .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::on_mouse_up))
            .on_mouse_up_out(MouseButton::Left, cx.listener(Self::on_mouse_up))
            .on_mouse_move(cx.listener(Self::on_mouse_move))
            .w_full()
            .min_h(min_height)
            .px_2()
            .py_1()
            .rounded(theme.radius)
            .border_1()
            .border_color(theme.border)
            .bg(theme.background)
            .text_color(theme.foreground)
            .text_sm()
            .child(TextElement { state: cx.entity() })
    }
}

/// A configured, renderable text field bound to a `TextInputState` entity.
/// Thin composition wrapper so application code can write
/// `TextInput::new(&state)` the same way it would reach for any other
/// framework widget.
#[derive(IntoElement)]
pub struct TextInput {
    state: Entity<TextInputState>,
}

impl TextInput {
    pub fn new(state: &Entity<TextInputState>) -> Self {
        Self { state: state.clone() }
    }
}

impl RenderOnce for TextInput {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        self.state
    }
}
