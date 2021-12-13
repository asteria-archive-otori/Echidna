/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use super::imp::EchidnaEditor;
use gio::Cancellable;
use gio::{File, FileQueryInfoFlags, FileType, SimpleAction};
use glib::clone;
use glib::subclass::types::ObjectSubclassExt;
use glib::types::Type;
use gtk::prelude::*;
use gtk::{ApplicationWindow, FileChooserAction, FileChooserDialog, ResponseType, TreeStore};
use relative_path::RelativePath;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Deserialize, Serialize)]
struct MonacoFolder {
    path: String,
}

#[derive(Deserialize, Serialize)]
struct MonacoWorkspace {
    folders: Vec<MonacoFolder>,
}

trait WorkspaceImplementedEditor {
    fn action_open_workspace(
        &self,
        window: ApplicationWindow,
        app: super::EchidnaEditor,
        _action: &SimpleAction,
        _variant: Option<&glib::Variant>,
    );

    fn open_workspace(&self, file: File);
    fn recursive_add_files_into_tree_store(&self, parent_file: File, tree: &TreeStore);
    fn open_folder(&self, file: File);
}

impl WorkspaceImplementedEditor for EchidnaEditor {
    fn action_open_workspace(
        &self,
        window: ApplicationWindow,
        app: super::EchidnaEditor,
        _action: &SimpleAction,
        _variant: Option<&glib::Variant>,
    ) {
        let dialog: FileChooserDialog = FileChooserDialog::new(
            Some("Open a file"),
            Some(&window),
            FileChooserAction::Open,
            &[
                ("Cancel", ResponseType::Cancel),
                ("Open", ResponseType::Accept),
            ],
        );
        dialog.set_visible(true);
        dialog.connect_response(clone!(@weak window, @weak app =>
            move |dialog, response| {
                if response == ResponseType::Accept {
                    let file_option = dialog.file();
                    match file_option {
                        Some(file) => {
                            dialog.destroy();
                            Self::from_instance(&app).open_workspace(file);
                        },
                        None => {}
                    }
                } else if response == ResponseType::Cancel {
                    dialog.destroy();
                }
            }
        ));
    }
    /**
     *   __Open Workspace__
     *   
     *   Basically, this is just the same as Open Folder, but it's many folders.
     *
     *   - Open a FileChooserDialog, set to only view .code-workspace files.
     *   - If the user pressed cancel, destroy the dialog. If the user opened a .code-workspace file:
     *   - Get the workspace file, load and parse its content, .code-workspace files are in JSON with comments. But JSON only should be fine, for now.
     *   - Iterate over folders listed in the workspace file.
     *   - Find the absolute path of the folder: Relative to the workspace file.
     *   - Create a GFile instance of that path, and call open_folder(file: File) and pass the GFile instance to it.
     *
     */
    fn open_workspace(&self, file: File) {
        let cancellable = Cancellable::new();
        let filepath_raw = &file
            .path()
            .expect("Could not get the file path of the file.");
        let filepath = Path::new(&filepath_raw);
        let info = file
            .query_info("*", gio::FileQueryInfoFlags::NONE, Some(&cancellable))
            .expect(format!(
                "Could not retrieve file information for {:?}",
                filepath
            ));
        let content_type = info
            .content_type()
            .expect(format!("Found no content type for {:?}", filepath));
        println!(
            "Opened {} and found its content type is {}.",
            "file",
            content_type.to_string()
        );
        let content_cancellable = Cancellable::new();
        let content = file
            .load_contents(Some(&content_cancellable))
            .expect("Could not load the file contents for {:?}", filepath);

        let (int_vec, _byte_string) = content;
        let workspace = serde_json::from_slice::<MonacoWorkspace>(&int_vec).expect(format!(
            "Could not parse the workspace file of {:?}",
            filepath
        ));

        for folder in workspace.folders {
            let path = RelativePath::new(&folder.path);
            let folder = File::for_path(path.to_path(filepath));

            // Do something with the folder, perhaps lists its child and .
            self.open_folder(folder);
        }
    }

    /**
     *
     *
     */
    fn recursive_add_files_into_tree_store(&self, parent_file: File, tree: &TreeStore) {
        let child_enumerate_cancellable = Cancellable::new();
        let child_files = parent_file
            .enumerate_children(
                "*",
                FileQueryInfoFlags::NONE,
                Some(&child_enumerate_cancellable),
            )
            .expect(
                format!(
                    "Could not look up the children files of {:?} because:\n{:#?}",
                    filepath
                )
                .as_str(),
            );
        let filepath = &parent_file
            .path()
            .expect("Could not get the file path of the file.");

        for file_iter in files {
            let file_info = file_iter.expect();
            let file = parent_file.child(file_info.name());
            let tree_iter = tree.append(None);
            tree.set_value(&tree_iter, 2, &file_info.name().to_str().to_value());

            if file_info.file_type() == FileType::Directory {
                self.recursive_add_files_into_tree_store(file, tree);
            }
        }
    }

    /*
    Loads a folder into the tree view.

    - Create a new tree
    - Enumerate over child files of 'file'. (PS: In the Unix family of OS-es, directories are files too)
    */
    fn open_folder(&self, file: File) {
        let tree = TreeStore::new(&[gdk::Texture::static_type(), Type::STRING]);
        self.recursive_add_files_into_tree_store(file, &tree);
    }
}
