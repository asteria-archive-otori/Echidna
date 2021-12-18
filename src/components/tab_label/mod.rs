/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

pub mod imp;
use glib::IsA;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

glib::wrapper! {
    pub struct TabLabel(ObjectSubclass<imp::TabLabel>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl TabLabel {
    pub fn new<U: IsA<gtk::Widget>>(tab_label: Option<&U>) -> Self {
        let this: Self = glib::Object::new(&[]).expect("Failed to create 'TabLabel' component.");

        if tab_label.is_some() {
            this.prepend(tab_label.unwrap());
        }
        this
    }

    pub fn to_imp(&self) -> &imp::TabLabel {
        imp::TabLabel::from_instance(self)
    }
}
