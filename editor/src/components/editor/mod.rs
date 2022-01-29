/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

pub mod imp;
use std::{error::Error, fmt};

use gio::Cancellable;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use sourceview::{
    prelude::{FileExt as SourceViewExt, *},
    Buffer, FileLoader, FileSaver, LanguageManager,
};

glib::wrapper! {
    pub struct EchidnaCoreEditor(ObjectSubclass<imp::EchidnaCoreEditor>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl EchidnaCoreEditor {
    pub fn new(file: Option<sourceview::File>) -> Result<Self, glib::BoolError> {
        let this: Result<Self, glib::BoolError> = glib::Object::new(&[]);

        match this {
            Ok(this) => {
                let this_imp = this.to_imp();
                // Without cloning it, for some reasons the Rust compiler complains about &this.to_imp().sourceview not being IsA<sourceview::View>
                this_imp.minimap.set_view(&this_imp.sourceview.clone());

                match file {
                    Some(file) => {
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

                        let content_type = info.content_type().expect(
                            format!("It does not seem like {:?} has a type", filepath).as_str(),
                        );
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
                                    panic!("Found an error when loading the file into the text editor's buffer. {:#?}", result.err());
                                }
                            },
                        );
                        }
                    }
                    None => {}
                }
                Ok(this)
            }
            Err(e) => Err(e),
        }
    }

    pub fn to_imp(&self) -> &imp::EchidnaCoreEditor {
        imp::EchidnaCoreEditor::from_instance(self)
    }

    pub fn file(&self) -> sourceview::File {
        self.property("file").expect("Could not get property 'file' of EchidnaCoreEditor").get::<sourceview::File>().expect("Could not get property 'file' of EchidnaCoreEditor because its type is not IsA<sourceview::File>")
    }

    pub fn save_file(&self, save_as: Option<&gio::File>) -> Result<(), Box<dyn Error>> {
        let window_imp = self.to_imp();
        let buffer = self.to_imp().sourceview.buffer().downcast::<Buffer>();

        match buffer {
            Ok(buffer) => {
                let cancellable = Cancellable::new();
                let mut file_saver: Option<FileSaver> = None;
                let result: Result<(), Box<dyn Error>> = match save_as {
                    Some(file) => {
                        file_saver = Some(FileSaver::with_target(&buffer, &self.file(), file));
                        Ok(())
                    }
                    None => match self.file().location() {
                        Some(_) => {
                            file_saver = Some(FileSaver::new(&buffer, &self.file()));
                            Ok(())
                        }
                        None => {
                            #[derive(Debug)]
                            struct Error {}

                            impl fmt::Display for Error {
                                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                                    write!(f, "The file location must exist. Please do \"Save As\"")
                                }
                            }

                            impl std::error::Error for Error {}
                            Err(Box::new(Error {}))
                        }
                    },
                };

                match result {
                    Err(result) => Err(result),
                    Ok(_) => {
                        file_saver.unwrap().save_async(
                            glib::Priority::default(),
                            Some(&cancellable),
                            |_, _| {},
                            |result| {
                                if result.is_err() {
                                    panic!(
                                        "Found an error while saving the file:\n{}",
                                        result.err().expect("No error")
                                    )
                                }
                            },
                        );
                        Ok(())
                    }
                }
            }
            Err(_) => {
                #[derive(Debug)]
                struct Error {}

                impl fmt::Display for Error {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "Can't downcast the buffer to GtkSourceBuffer.")
                    }
                }

                impl std::error::Error for Error {}

                Err(Box::new(Error {}))
            }
        }
    }
}
