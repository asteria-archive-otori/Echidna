/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::lib::prelude::*;

use crate::components::editor::EchidnaCoreEditor;
use gio::Cancellable;
use glib::{clone, Priority};
use gtk::{
    prelude::*, subclass::prelude::*, FileChooserAction, FileChooserDialog, Label, ResponseType,
};
use sourceview::{prelude::*, Buffer, File, FileSaver};

pub trait FileImplementedEditor {
    fn action_open_file(&self);
    fn open_file(notebook: &gtk::Notebook, file: gio::File);
    fn action_save_file_as(&self);
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
    fn action_open_file(&self) {
        let dialog: FileChooserDialog = FileChooserDialog::new(
            Some("Open a file"),
            Some(self),
            FileChooserAction::Open,
            &[
                ("Cancel", ResponseType::Cancel),
                ("Open", ResponseType::Accept),
            ],
        );

        dialog.set_visible(true);

        // TODO: Somehow inserts self to this function.
        // This function sets the callback function as 'static, which for some reasons ban cloning self into it. Idk why.
        dialog.connect_response(clone!( @weak self as window, =>
        move |dialog, response| {
            if response == ResponseType::Accept {
                let file = dialog.file().expect("");
                Self::open_file(&super::imp::EchidnaWindow::from_instance(&window).notebook, file);

            }

            dialog.destroy();

        }));
    }

    fn open_file(notebook: &gtk::Notebook, file_location: gio::File) {
        let file = File::builder().location(&file_location).build();
        let editor_page = EchidnaCoreEditor::new(Some(file));
        notebook.prepend_closable_page(
            &editor_page,
            Some(&Label::new(Some(
                &file_location
                    .path()
                    .expect("The file's path is missing")
                    .file_name()
                    .expect("Could not get the file name, as it ends with ..")
                    .to_str()
                    .expect("Could not parse the file name, as it is not a valid Unicode."),
            ))),
        );
    }

    fn action_save_file_as(&self) {
        let dialog = FileChooserDialog::new(
            Some("Save File As"),
            Some(self),
            FileChooserAction::Save,
            &[
                ("Cancel", ResponseType::Cancel),
                ("Save", ResponseType::Accept),
            ],
        );

        dialog.set_current_name("untitled");

        dialog.show();

        dialog.connect_response(clone!( @weak self as window, =>
        move |dialog, response| {
            if response == ResponseType::Accept {
                let file = dialog.file().expect("");
                let window_imp = window.to_imp();
                let page: EchidnaCoreEditor;

                match window_imp.notebook
                    .nth_page(
                        Some(window_imp.notebook
                            .current_page()
                            .expect(
                                "No tabs is the current tab, probably all tabs closed. No files to save"
                            )
                        )
                    ).expect(
                        "Couldn't get the page of the current index. Try again."
                    ).downcast::<EchidnaCoreEditor>() {
                    Ok(res) => page = res,
                    Err(e) => panic!("We got an error when trying to downcast the current tab page into EchidnaCoreEditor:\n{}", e)
                }

                let buffer: Buffer = page.to_imp().sourceview.buffer().downcast().expect("Could not downcast the editor's buffer to GtkSourceBuffer.");
                let cancellable = Cancellable::new();

                let file_saver = FileSaver::with_target(
                    &buffer,
                     &page.file(), &file);
                file_saver.save_async(
                    Priority::default(),
                    Some(&cancellable),
                    |_, _| {},
                    |result| {
                        if result.is_err() {
                            panic!("Found an error while saving the file:\n{}", result.err().expect("No error"))
                        }
                    });

                }

                dialog.destroy();

            }));
    }
}
