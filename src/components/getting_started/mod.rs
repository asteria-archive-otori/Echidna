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

    pub fn to_imp(&self) -> &imp::GettingStartedPage {
        imp::GettingStartedPage::from_instance(self)
    }
    pub fn setup_actions(&self, window: &crate::components::EchidnaWindow) -> &Self {
        let imp_class = self.to_imp();
        imp_class
            .link_open_file
            .connect_clicked(clone!(@weak window =>
                move |_| {
                window.action_open_file();
            }));

        imp_class
            .link_new_file
            .connect_clicked(clone!(@weak window =>
                move |_| {
                    window.action_new_file();
            }));

        self
    }
}
