/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */
use crate::prelude::*;
pub mod imp;
mod load_css;

glib::wrapper! {
    pub struct EchidnaEditor(ObjectSubclass<imp::EchidnaEditor>)
        @extends gio::Application, gtk::Application,
        @implements gio::ActionGroup, gio::ActionMap;

}

impl Default for EchidnaEditor {
    fn default() -> Self {
        Self::new("land.echidna.editor").expect("Can't create EchidnaEditor object")
    }
}

impl EchidnaEditor {
    pub fn new(app_id: &'static str) -> Result<Self, glib::BoolError> {
        glib::Object::new(&[
            ("application-id", &app_id),
            ("flags", &gio::ApplicationFlags::empty()),
        ])
    }
}
