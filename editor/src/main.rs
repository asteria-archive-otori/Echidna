/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */
pub mod prelude {
    pub use super::components::prelude::*;
    pub use super::lib::prelude::*;
    pub use adw::prelude::*;
    pub use gtk::gdk;
    pub use gtk::gio;
    pub use gtk::glib;
    pub use gtk::prelude::*;
}

mod components;
pub mod lib;

use gtk::prelude::ApplicationExtManual;
use prelude::*;

fn main() {
    // Register and include resources
    gio::resources_register_include!("echidna.gresource").expect("Failed to register resources.");
    libchidna::init();
    let app = adw::Application::new(
        Some("io.fortressia.Echidna"),
        gio::ApplicationFlags::FLAGS_NONE,
    );

    let style = app.style_manager();

    style.set_color_scheme(adw::ColorScheme::PreferDark);

    app.connect_activate(|app| {
        let window = components::EchidnaWindow::new(app);

        window.setup_menubar();
        window.set_application(Some(app));

        window.show();
    });

    std::process::exit(app.run());
}
