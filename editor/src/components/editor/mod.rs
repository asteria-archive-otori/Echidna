/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

pub mod imp;
use crate::prelude::*;
use adw::Application;
use gio::Cancellable;
use gtk::{ subclass::prelude::*};
use sourceview::{ Buffer, FileSaver};
use std::{error::Error, fmt};

glib::wrapper! {
    pub struct EchidnaCoreEditor(ObjectSubclass<imp::EchidnaCoreEditor>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl EchidnaCoreEditor {
    pub fn new(
        file: Option<&sourceview::File>,
        app: Option<&Application>,
    ) -> Result<Self, glib::BoolError> {
       glib::Object::new(&[("file", &file), ("app", &app)])
     
    }
    pub fn app(&self) -> Option<Application> {
        self.property("app")
    }
    pub fn to_imp(&self) -> &imp::EchidnaCoreEditor {
        imp::EchidnaCoreEditor::from_instance(self)
    }

    pub fn file(&self) -> Option<sourceview::File> {
        self.property("file")
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
