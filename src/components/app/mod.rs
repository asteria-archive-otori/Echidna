/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

pub mod imp;

use glib::wrapper;

wrapper! {
    pub struct EchidnaEditor(ObjectSubclass<imp::EchidnaEditor>) @extends gio::Application, gtk::Application, @implements gio::ActionGroup, gio::ActionMap;

}

impl Default for EchidnaEditor {
    fn default() -> Self {
        Self::new("land.echidna.editor")
    }
}

impl EchidnaEditor {
    pub fn new(app_id: &'static str) -> Self {
        let object = glib::Object::new(&[
            ("application-id", &app_id),
            ("flags", &gio::ApplicationFlags::empty()),
        ]);

        match object {
            Ok(o) => o,
            Err(e) => panic!("Error in making EchidnaApplication {}", e),
        }
    }
}
