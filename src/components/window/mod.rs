/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod file;

mod imp;
pub mod menubar;

use glib::object::IsA;
use gtk::subclass::prelude::*;

glib::wrapper! {
    pub struct EchidnaWindow(ObjectSubclass<imp::EchidnaWindow>)
    @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,
    @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl EchidnaWindow {
    pub fn new<P: IsA<gtk::Application>>(application: &P) -> Self {
        let object = glib::Object::new(&[("application", &application)]);

        match object {
            Ok(o) => o,
            Err(e) => panic!("Error in making EchidnaApplication {}", e),
        }
    }

    pub fn to_imp(&self) -> &imp::EchidnaWindow {
        imp::EchidnaWindow::from_instance(self)
    }
}
