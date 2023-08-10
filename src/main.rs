mod components;
mod logic;
mod types;

use components::body_box;

use gtk::gdk::Display;
use gtk::prelude::*;
use gtk::{style_context_add_provider_for_display, Application, ApplicationWindow, CssProvider};
use leptos::{create_runtime, create_scope, Scope};

const APP_ID: &str = "dev.vannrr.calculator";

const WINDOW_TITLE: &str = "Calculator";
const WINDOW_WIDTH: i32 = 300;

fn main() {
    _ = create_scope(create_runtime(), |cx| {
        let app = Application::builder().application_id(APP_ID).build();

        app.connect_startup(|_| load_css());
        app.connect_activate(move |app| build_ui(cx, app));

        app.set_accels_for_action("window.close", &["<Ctrl>q", "Escape"]);

        app.run();
    });
}

fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_data(include_str!("style.css"));

    style_context_add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(cx: Scope, app: &Application) {
    let body_box = body_box::new(cx, WINDOW_WIDTH);

    let window = ApplicationWindow::builder()
        .application(app)
        .title(WINDOW_TITLE)
        .default_width(WINDOW_WIDTH)
        .resizable(false)
        .child(&body_box)
        .build();

    window.present();
}
