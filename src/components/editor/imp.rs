/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use glib::{ParamFlags, ParamSpec, Value};
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::CompositeTemplate;
use once_cell::sync::Lazy;
use std::cell::RefCell;

#[derive(Default, CompositeTemplate)]
#[template(file = "./editor.ui")]
pub struct EchidnaCoreEditor {
    #[template_child]
    pub minimap: TemplateChild<sourceview::Map>,
    #[template_child]
    pub sourceview: TemplateChild<sourceview::View>,
    pub file: RefCell<sourceview::File>,
}

#[glib::object_subclass]
impl ObjectSubclass for EchidnaCoreEditor {
    const NAME: &'static str = "EchidnaCoreEditor";
    type Type = super::EchidnaCoreEditor;
    type ParentType = gtk::Box;

    fn class_init(class: &mut Self::Class) {
        Self::bind_template(class);
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for EchidnaCoreEditor {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![ParamSpec::new_object(
                "file",
                "file",
                "the file of the editor",
                sourceview::File::static_type(),
                ParamFlags::READWRITE,
            )]
        });

        PROPERTIES.as_ref()
    }

    fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, spec: &ParamSpec) {
        match spec.name() {
            "file" => {
                let file: sourceview::File = value
                    .get()
                    .expect("The file needs to be a sourceview::File");
                self.file.replace(file);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, spec: &ParamSpec) -> Value {
        match spec.name() {
            "file" => self.file.borrow().to_value(),
            _ => unimplemented!(),
        }
    }
}
impl WidgetImpl for EchidnaCoreEditor {}
impl BoxImpl for EchidnaCoreEditor {}
