/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::components::editor::EchidnaCoreEditor;
use crate::prelude::*;
use std::{error::Error, fmt};

use glib::clone;
use gtk::{subclass::prelude::*, FileChooserAction, FileChooserNative, Label, ResponseType};
use sourceview::{prelude::*, File};

pub trait FileImplementedEditor {
    fn action_open_file(&self) -> Result<(), Box<dyn Error>>;
    fn open_file(
        tab_bar: &adw::TabBar,
        file_location: gio::File,
        app: Option<&adw::Application>,
    ) -> Result<(), Box<dyn Error>>;
    fn action_save_file_as(&self) -> Result<(), Box<dyn Error>>;
    fn action_new_file(&self) -> Result<adw::TabPage, Box<dyn Error>>;
    fn action_save_file(&self) -> Result<(), Box<dyn Error>>;
}

impl FileImplementedEditor for super::EchidnaWindow {
    /*
    Open a file and put it in an editor and the opened files bar.

    - Open a file chooser dialog.
    - Connect a signal to it and get the file choosen.
    - Read th file's information and
    - TODO: if it's a plain text,
    - TODO: Load the file's content
    - TODO: Create an editor
    - TODO: Set the editor's content to the file's content.
    - TODO: Somehow keep track of what file belongs to what editor/opened file bar widget.
    - TODO: If the user enables Autosave, listen to the editor's changes and automatically save the editor's content.
    - TODO: Close the editor and the tab widget when the file is closed (triggered by the X button on them).

    Perhaps some of the last points should not be implemented in this function but rather in another function that keeps track of every files.
    */
    fn action_open_file(&self) -> Result<(), Box<dyn Error>> {
        /*
           Borrows self.to_imp()'s dialog Vector mutably, create a new dialog , and push that dialog into the vector.

           This is required because we own the dialog and thus the dialog will be destroyed when this function has completed.
        */
        match self.to_imp().dialogs.try_borrow_mut() {
            Ok(mut dialogs) => {
                let dialog = gtk::FileChooserNative::new(
                    Some("Open File"),
                    Some(self),
                    gtk::FileChooserAction::Open,
                    Some("Open"),
                    Some("Cancel"),
                );
                let dialog_clone = dialog.clone();
                // The upcast() function moves the dialog variable, so we need to get a reference to the dialog trough dialog_clone
                dialogs.push(dialog.upcast::<gtk::NativeDialog>());

                dialog_clone.connect_response(clone!( @weak self as window, =>
                move |dialog, response| {

                    if response == ResponseType::Accept {
                        let file = dialog.file().expect("");
                        Self::open_file(
                            &super::imp::EchidnaWindow::from_instance(&window).tab_bar, file,
                            Some(
                                &window.application().expect("No application in window")
                                    .downcast::<adw::Application>()
                                    .expect("Application is not AdwApplication"),
                            ),
                        );

                    } else {
                        println!("{:?}", response);
                    }
                    dialog.destroy();

                }));
                dialog_clone.show();

                Ok(())
            }
            Err(e) => Err(Box::new(e)),
        }
    }

    fn open_file(
        tab_bar: &adw::TabBar,
        file_location: gio::File,
        app: Option<&adw::Application>,
    ) -> Result<(), Box<dyn Error>> {
        let file = File::builder().location(&file_location).build();
        let editor_page = EchidnaCoreEditor::new(Some(file), app);
        match editor_page {
            Ok(editor_page) => {
                let view = tab_bar.view().expect("No view in tab bar");

                let page = view.prepend(&editor_page);
                page.set_title(
                    file_location
                        .path()
                        .expect("The file's path is missing")
                        .file_name()
                        .expect("Could not get the file name, as it ends with ..")
                        .to_str()
                        .expect("Could not parse the file name, as it is not a valid Unicode."),
                );
                view.set_selected_page(&page);

                Ok(())
            }
            Err(e) => Err(Box::new(e)),
        }
    }
    fn action_save_file_as(&self) -> Result<(), Box<dyn Error>> {
        /*
           Borrows self.to_imp()'s dialog Vector mutably, create a new dialog , and push that dialog into the vector.

           This is required because we own the dialog and thus the dialog will be destroyed when this function has completed.
        */

        match self.to_imp().dialogs.try_borrow_mut() {
            Ok(mut dialogs) => {
                let dialog = FileChooserNative::new(
                    Some("Save File As"),
                    gtk::Window::NONE,
                    FileChooserAction::Save,
                    Some("_Save"),
                    Some("_Cancel"),
                );

                let dialog_clone = dialog.clone();
                // The upcast() function moves the dialog variable, so we need to get a reference to the dialog trough dialog_clone
                dialogs.push(dialog.upcast::<gtk::NativeDialog>());

                dialog_clone.connect_response(clone!( @weak self as window, =>
                move |dialog, response| {
                    if response == ResponseType::Accept {
                        let file = dialog.file().expect("No file in response");
                        let tab: EchidnaCoreEditor = window.get_current_tab().expect("error");
                        match tab.save_file(Some(&file)) {
                            Ok(_) => {},
                            Err(e) => eprintln!("{}", e)
                        };
                    }

                        dialog.destroy();

                    }));
                dialog_clone.show();

                Ok(())
            }
            Err(e) => Err(Box::new(e)),
        }
    }

    fn action_new_file(&self) -> Result<adw::TabPage, Box<dyn Error>> {
        match EchidnaCoreEditor::new(
            None,
            Some(
                &self
                    .application()
                    .expect("No application in window")
                    .downcast::<adw::Application>()
                    .expect("Application is not AdwApplication"),
            ),
        ) {
            Ok(editor) => {
                let tab_bar = &self.to_imp().tab_bar;
                let view = tab_bar.view().expect("No view in tab bar");
                let page = view.prepend(&editor);

                view.set_selected_page(&page);

                Ok(page)
            }
            Err(e) => Err(Box::new(e)),
        }
    }

    fn action_save_file(&self) -> Result<(), Box<dyn Error>> {
        match self.get_current_tab() {
            Ok(page) => {
                let page: EchidnaCoreEditor = page;
                match page.file() {
                    Some(file) => {
                        let location = file.location();
                        match page.save_file(None) {
                            Ok(_) => Ok(()),
                            Err(e) => Err(e),
                        }
                    }

                    None => self.action_save_file_as(),
                }
            }
            Err(e) => Err(e),
        }
    }
}
