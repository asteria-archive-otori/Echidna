/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

pub mod imp;

use gtk::subclass::prelude::ObjectSubclassExt;

use crate::prelude::*;
glib::wrapper! {
    pub struct EchidnaSidebar(ObjectSubclass<imp::EchidnaSidebar>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl EchidnaSidebar {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create 'EchidnaSidebar' component.")
    }
    pub fn to_imp(&self) -> &imp::EchidnaSidebar {
        imp::EchidnaSidebar::from_instance(&self)
    }
}

impl Default for EchidnaSidebar {
    fn default() -> Self {
        Self::new()
    }
}
