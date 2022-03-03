/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

pub mod imp;
use crate::prelude::*;
use gio::Cancellable;
use gtk::subclass::prelude::*;
use sourceview::{prelude::*, Buffer, FileLoader, FileSaver, LanguageManager};
use std::{error::Error, fmt};

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
                        let file_location = file.location();
                        this.set_property("file", &file);

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

    pub fn file(&self) -> Option<sourceview::File> {
        self.property::<Option<sourceview::File>>("file")
    }

    pub fn save_file(&self, save_as: Option<&gio::File>) -> Result<(), Box<dyn Error>> {
        let buffer = self.to_imp().sourceview.buffer().downcast::<Buffer>();

        match buffer {
            Ok(buffer) => match self.file() {
                Some(file) => {
                    let cancellable = Cancellable::new();

                    let file_saver: Option<FileSaver> = match save_as {
                        Some(save_as_file) => {
                            Some(FileSaver::with_target(&buffer, &file, save_as_file))
                        }
                        None => Some(FileSaver::new(&buffer, &file)),
                    };

                    match file_saver {
                        Some(file_saver) => {
                            file_saver.save_async(
                                glib::Priority::default(),
                                Some(&cancellable),
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
                        None => todo!(),
                    }
                }
                None => todo!(),
            },
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
