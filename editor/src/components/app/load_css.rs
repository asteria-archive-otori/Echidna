use crate::prelude::*;
use gdk::Display;
use gtk::{CssProvider, StyleContext};

pub fn load_css() {
    let css = CssProvider::new();

    css.load_from_data(include_bytes!("../../styles/get-started.css"));

    StyleContext::add_provider_for_display(
        &Display::default().expect("Cannot connect to a display"),
        &css,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}
