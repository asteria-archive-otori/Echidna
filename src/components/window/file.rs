/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::components::editor::EchidnaCoreEditor;
use crate::lib::prelude::*;
use glib::clone;
use gtk::{
    prelude::*, subclass::prelude::*, FileChooserAction, FileChooserNative, Label, ResponseType,
};
use sourceview::{File, FileExt as SourceFileExt};

pub trait FileImplementedEditor {
    fn action_open_file(&self);
    fn open_file(notebook: &gtk::Notebook, file: gio::File);
    fn action_save_file_as(&self);
    fn action_new_file(&self);
    fn action_save_file(&self);
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
        let dialog = FileChooserNative::new(
            Some("Open a file"),
            Some(self),
            FileChooserAction::Open,
            Some("Open"),
            Some("Cancel"),
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
                file_location
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
        let dialog = FileChooserNative::new(
            Some("Save File As"),
            Some(self),
            FileChooserAction::Save,
            Some("Open"),
            Some("Cancel"),
        );

        dialog.set_current_name("untitled");

        dialog.show();

        dialog.connect_response(clone!( @weak self as window, =>
        move |dialog, response| {
            if response == ResponseType::Accept {
                let file = dialog.file().expect("");
                let tab: EchidnaCoreEditor = window.get_current_tab().expect("error");
                tab.save_file(Some(&file));
            }

                dialog.destroy();

            }));
    }

    fn action_new_file(&self) {
        let editor_page = EchidnaCoreEditor::new(None);

        self.to_imp()
            .notebook
            .prepend_closable_page(&editor_page, Some(&gtk::Label::new(Some(&"Untitled"))));
    }

    fn action_save_file(&self) {
        let page: EchidnaCoreEditor = self
            .get_current_tab()
            .expect("Can't find the current tab because there are no tabs.");
        match page.file().location() {
            Some(_) => {
                page.save_file(None);
            }
            None => self.action_save_file_as(),
        }
    }
}
