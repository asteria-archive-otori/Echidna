/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * model, You can obtain one at https://mozilla.org/MPL/2.0/. */

pub mod imp {

    use crate::glib::ParamSpecObject;
    use crate::prelude::*;
    use gtk::gio::File;
    use gtk::glib::{ParamFlags, ParamSpec, Value};
    use gtk::subclass::prelude::*;
    use gtk::{CompositeTemplate, DirectoryList, TreeListModel};
    use once_cell::sync::Lazy;
    use std::cell::RefCell;

    #[derive(Default, CompositeTemplate)]
    #[template(resource = "/io/fortressia/Echidna/components/sidebar/sidebar.ui")]
    pub struct EchidnaWorkspace {
        model: RefCell<TreeListModel>,
        file: RefCell<Option<File>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for EchidnaWorkspace {
        const NAME: &'static str = "EchidnaWorkspace";
        type Type = super::EchidnaWorkspace;
        type ParentType = gtk::Box;

        #[inline]
        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        #[inline]
        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for EchidnaWorkspace {
        fn constructed(&self, obj: &Self::Type) {
            static ATTRIBUTES: std::option::Option<&str> =
                Some("standard::name,standard::icon,standard::display-name,standard::content_type");
            self.model.replace(TreeListModel::new(
                &DirectoryList::new(ATTRIBUTES, obj.file().as_ref()),
                false,
                false,
                |item| {
                    let file = item
                        .clone()
                        .downcast::<gio::FileInfo>()
                        .unwrap()
                        .attribute_object("standard::file")
                        .unwrap()
                        .downcast::<gio::File>()
                        .unwrap();

                    Some(DirectoryList::new(ATTRIBUTES, Some(&file)).upcast())
                },
            ));
        }

        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![
                    ParamSpecObject::new(
                        "model",
                        "model",
                        "the model of the view",
                        TreeListModel::static_type(),
                        ParamFlags::READABLE,
                    ),
                    ParamSpecObject::new(
                        "file",
                        "file",
                        "the file of the view",
                        File::static_type(),
                        ParamFlags::READWRITE,
                    ),
                ]
            });

            PROPERTIES.as_ref()
        }

        fn property(&self, _obj: &Self::Type, _id: usize, spec: &ParamSpec) -> Value {
            match spec.name() {
                "model" => self.model.borrow().to_value(),
                "file" => self.file.borrow().to_value(),
                _ => unimplemented!(),
            }
        }

        fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, spec: &ParamSpec) {
            match spec.name() {
                "file" => {
                    let file: Option<File> = value.get().expect("The file needs to be gio::File");
                    self.file.replace(file);
                }
                _ => unimplemented!(),
            }
        }
    }
    impl WidgetImpl for EchidnaWorkspace {}
    impl BoxImpl for EchidnaWorkspace {}
}

use gtk::gio::File;

use crate::prelude::*;
glib::wrapper! {
    pub struct EchidnaWorkspace(ObjectSubclass<imp::EchidnaWorkspace>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl EchidnaWorkspace {
    pub fn new(file: Option<&File>) -> Self {
        glib::Object::new(&[("file", &file) ]).expect("Failed to create 'EchidnaWorkspace' component.")
    }

    pub fn file(&self) -> Option<File> {
        self.property("file")
    }
}

impl Default for EchidnaWorkspace {
    #[inline]
    fn default() -> Self {
        Self::new(None)
    }
}
