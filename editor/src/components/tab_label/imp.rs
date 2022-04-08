/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */
use crate::prelude::*;
use gtk::subclass::prelude::*;
use gtk::CompositeTemplate;

#[derive(Default, CompositeTemplate)]
#[template(resource = "/io/fortressia/Echidna/tab-label.ui")]
pub struct TabLabel {
    #[template_child]
    pub button: TemplateChild<gtk::Button>,
}

#[glib::object_subclass]
impl ObjectSubclass for TabLabel {
    const NAME: &'static str = "TabLabel";
    type Type = super::TabLabel;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for TabLabel {}
impl WidgetImpl for TabLabel {}
impl BoxImpl for TabLabel {}

impl BuildableImpl for TabLabel {
    fn add_child(
        &self,
        buildable: &Self::Type,
        _builder: &gtk::Builder,
        child: &glib::Object,
        _type_: Option<&str>,
    ) {
        buildable.prepend(child.downcast_ref::<gtk::Widget>().unwrap());
    }
}
