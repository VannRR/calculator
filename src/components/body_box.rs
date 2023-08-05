use super::{buttons_grid, screen_textview};

use gtk::prelude::*;
use gtk::Box;
use leptos::{create_signal, Scope};

const MARGIN: i32 = 5;

pub fn new(cx: Scope, window_width: i32) -> Box {
    let (output, set_output) = create_signal(cx, Vec::<String>::new());

    let screen_textview = screen_textview::new(cx, output, window_width, MARGIN);
    let buttons_grid = buttons_grid::new(set_output, window_width, MARGIN);

    let vbox = gtk::Box::builder()
        .orientation(gtk::Orientation::Vertical)
        .margin_top(MARGIN)
        .margin_bottom(MARGIN)
        .margin_start(MARGIN)
        .margin_end(MARGIN)
        .build();

    vbox.append(&screen_textview);
    vbox.append(&buttons_grid);

    vbox
}
