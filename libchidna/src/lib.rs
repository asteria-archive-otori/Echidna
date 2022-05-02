/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * Copyright 2022 to Echidna Code contributors. All Rights Reserved
 *
 * List of authors:
 * Nefo Fortressia <nefothingy@hotmail.com>
 *
 */

use gtk::gio;

pub mod iterables;
pub mod vertical_tab_bar;
mod internal {
    pub mod prelude {

        pub use adw::gio;
        pub use adw::glib;
        pub use adw::prelude::*;
        pub use gtk::prelude::*;
    }
    pub mod subclass {
        pub mod prelude {
            pub use adw::subclass::prelude::*;
            pub use gtk::subclass::prelude::*;
        }
    }
}

pub fn init() {
    // Register and include resources
    gio::resources_register_include!("com.fortressia.libchidna.gresource")
        .expect("Failed to register resources.");
}
