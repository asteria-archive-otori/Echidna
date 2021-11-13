/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::CompositeTemplate;

#[derive(Default, CompositeTemplate)]
#[template(file = "./getting-started.ui")]
pub struct GettingStartedPage {
    #[template_child]
    pub link_new_file: TemplateChild<gtk::LinkButton>,
    #[template_child]
    pub link_open_file: TemplateChild<gtk::Button>,
}

#[glib::object_subclass]
impl ObjectSubclass for GettingStartedPage {
    const NAME: &'static str = "GettingStartedPage";
    type Type = super::GettingStartedPage;
    type ParentType = gtk::Box;

    fn class_init(class: &mut Self::Class) {
        Self::bind_template(class);
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for GettingStartedPage {}
impl WidgetImpl for GettingStartedPage {}
impl BoxImpl for GettingStartedPage {}
