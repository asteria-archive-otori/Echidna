/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

pub mod imp;
use crate::components::prelude::*;
use glib::clone;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

glib::wrapper! {
    pub struct GettingStartedPage(ObjectSubclass<imp::GettingStartedPage>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl GettingStartedPage {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create 'GettingStartedPage' component.")
    }

    pub fn setup_actions<P: glib::IsA<crate::components::EchidnaWindow>>(&self, window: &P)
    where
        P: FileImplementedEditor,
    {
        let imp_class = imp::GettingStartedPage::from_instance(self);
    }
}
