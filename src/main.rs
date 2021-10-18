/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod app;

use gtk::prelude::ApplicationExtManual;
use app::EchidnaEditor;

fn main() {
    let app = EchidnaEditor::new(
        "land.echidna.editor",
    );

    std::process::exit(app.run());
}