/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */
use crate::prelude::*;

use gtk::subclass::prelude::*;
use gtk::CompositeTemplate;

#[derive(Default, CompositeTemplate)]
#[template(resource = "/io/fortressia/Echidna/sidebar.ui")]
pub struct EchidnaSidebar {
    #[template_child]
    pub settings_button: TemplateChild<gtk::Button>,
}

#[glib::object_subclass]
impl ObjectSubclass for EchidnaSidebar {
    const NAME: &'static str = "EchidnaSidebar";
    type Type = super::EchidnaSidebar;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for EchidnaSidebar {}
impl WidgetImpl for EchidnaSidebar {}
impl BoxImpl for EchidnaSidebar {}
