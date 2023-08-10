use gtk::prelude::*;
use gtk::{TextBuffer, TextView, WrapMode};
use leptos::{create_effect, ReadSignal, Scope};

pub fn new(cx: Scope, output: ReadSignal<Vec<String>>, window_width: i32, margin: i32) -> TextView {
    let text_buffer = TextBuffer::new(None);
    text_buffer.set_text("");

    let text_view_height: i32 = (window_width as f64 / 2.75) as i32 - (margin * 2);
    let text_view = TextView::new();
    text_view.set_height_request(text_view_height);
    text_view.set_margin_top(margin);
    text_view.set_margin_bottom(margin);
    text_view.set_margin_start(margin);
    text_view.set_margin_end(margin);
    text_view.set_editable(false);
    text_view.set_cursor_visible(false);
    text_view.set_monospace(true);
    text_view.set_wrap_mode(WrapMode::Char);
    text_view.set_buffer(Some(&text_buffer));

    create_effect(cx, {
        move |_| {
            let text = output().join(" ");
            text_buffer.set_text(&text);
        }
    });

    text_view
}
