/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod components;
pub mod lib;
use app::EchidnaEditor;
use components::app;
use gtk::prelude::ApplicationExtManual;

fn main() {
    let app = EchidnaEditor::new("land.echidna.editor");

    std::process::exit(app.run());
}

pub mod prelude {
    pub use super::components::prelude::*;
    pub use super::lib::prelude::*;
}
