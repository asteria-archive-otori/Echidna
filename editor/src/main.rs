/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */
pub mod prelude {
    pub use super::components::prelude::*;
    pub use super::lib::prelude::*;
    pub use gtk::gdk;
    pub use gtk::gio;
    pub use gtk::glib;
    pub use gtk::prelude::*;
}

mod components;
pub mod lib;
use app::EchidnaEditor;
use components::app;
use gtk::prelude::ApplicationExtManual;

fn main() {
    let app = EchidnaEditor::default();

    std::process::exit(app.run());
}
