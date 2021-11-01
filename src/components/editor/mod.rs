/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

pub mod imp;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use sourceview::{prelude::*, Buffer, FileExt as SourceFileExt, FileLoader, LanguageManager};

glib::wrapper! {
    pub struct EchidnaCoreEditor(ObjectSubclass<imp::EchidnaCoreEditor>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl EchidnaCoreEditor {
    pub fn new(file: Option<sourceview::File>) -> Self {
        let this: Self =
            glib::Object::new(&[]).expect("Failed to create 'EchidnaCoreEditor' component.");
        let this_imp = this.to_imp();
        // Without cloning it, for some reasons the Rust compiler complains about &this.to_imp().sourceview not being IsA<sourceview::View>
        this_imp.minimap.set_view(&this_imp.sourceview.clone());

        if file.is_some() {
            let file = file.unwrap();
            let file_location = file
                .location()
                .expect("file is required to have a location");

            this.set_property("file", &file)
                .expect("Could not set the 'file' property of EchidnaCoreEditor");

            let cancellable = gio::Cancellable::new();
            let filepath = file_location.path().expect("No filepath");
            let info = file_location
                .query_info("*", gio::FileQueryInfoFlags::NONE, Some(&cancellable))
                .expect("Could not query the info for file");

            let content_type = info
                .content_type()
                .expect(format!("It does not seem like {:?} has a type", filepath).as_str());
            {
                println!(
                    "Opened {} and found its content type is {}.",
                    "file",
                    content_type.to_string()
                );
                let buffer = this_imp.sourceview.buffer().downcast::<Buffer>().expect("Cannot downcast the sourceview's buffer. Maybe the sourceview's buffer is not IsA<sourceview::Buffer>.");
                let language_manager = LanguageManager::new();
                let language = language_manager.guess_language(
                    Some(&info.name().to_str().expect(
                        "Could not open the file because its name is not supported by Unicode.",
                    )),
                    None,
                );

                match language {
                    Some(lang) => buffer.set_language(Some(&lang)),
                    None => {}
                }

                let file_loader: FileLoader = FileLoader::new(&buffer, &file);

                file_loader.load_async(
                    glib::Priority::default(),
                    Some(&cancellable),
                    |_, _| {},
                    |result| {
                        if result.is_err() {
                            panic!(result.err());
                        }
                    },
                );
            }
        }
        this
    }

    pub fn to_imp(&self) -> &imp::EchidnaCoreEditor {
        imp::EchidnaCoreEditor::from_instance(self)
    }

    pub fn file(&self) -> sourceview::File {
        self.property("file").expect("Could not get property 'file' of EchidnaCoreEditor").get::<sourceview::File>().expect("Could not get property 'file' of EchidnaCoreEditor because its type is not IsA<sourceview::File>")
    }
}
