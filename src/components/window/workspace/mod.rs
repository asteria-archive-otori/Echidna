/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod open_folder;

pub use open_folder::*;
mod recursive_add_files;
use super::EchidnaWindow;
use core::pin::Pin;
use futures::stream::FuturesUnordered;
use gio::{Cancellable, File};
use glib::clone;
use gtk::{prelude::*, FileChooserAction, FileChooserNative, ResponseType};
use relative_path::RelativePath;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::runtime;

#[derive(Deserialize, Serialize)]
struct MonacoFolder {
    path: String,
}

#[derive(Deserialize, Serialize)]
struct MonacoWorkspace {
    folders: Vec<MonacoFolder>,
}

pub trait WorkspaceImplementedEditor {
    fn action_open_workspace(&self);

    fn open_workspace(&self, file: File);
}

impl WorkspaceImplementedEditor for EchidnaWindow {
    fn action_open_workspace(&self) {
        let dialog: FileChooserNative = FileChooserNative::new(
            Some("Open a file"),
            Some(self),
            FileChooserAction::Open,
            Some("Open"),
            Some("Cancel"),
        );
        dialog.set_visible(true);
        dialog.connect_response(clone!(@weak self as win =>
            move |dialog, response| {
                if response == ResponseType::Accept {
                    let file_option = dialog.file();
                    match file_option {
                        Some(file) => {
                            dialog.destroy();
                            win.open_workspace(file);
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
     *   - Open a FileChooserNative, set to only view .code-workspace files.
     *   - If the user pressed cancel, destroy the dialog. If the user opened a .code-workspace file:
     *   - Get the workspace file, load and parse its content, .code-workspace files are - - Create a GFile instance of that path, and call open_folder(file: File) and pass the GFile instance to it.
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
            .expect(format!("Could not retrieve file information for {:?}", filepath).as_str());
        let content_type = info
            .content_type()
            .expect(format!("Found no content type for {:?}", filepath).as_str());
        println!(
            "Opened {} and found its content type is {}.",
            "file",
            content_type.to_string()
        );
        let content_cancellable = Cancellable::new();
        let (content, _) = file
            .load_contents(Some(&content_cancellable))
            .expect(format!("Could not load the file contents for {:?}", filepath).as_str());

        let workspace = serde_json::from_slice::<MonacoWorkspace>(&content)
            .expect(format!("Could not parse the workspace file of {:?}", filepath).as_str());
        let explorer_box: &gtk::Box = &self
            .to_imp()
            .sidebar
            .to_imp()
            .explorer
            .child()
            .downcast()
            .expect("Could not downcast the Explorer activity tab's child widget to GtkBox.");
        let folder_futures_unordered: FuturesUnordered<_> = workspace.folders.iter().map(|folder_path|{
            let path = RelativePath::new(&folder_path.path);
            let folder = File::for_path(
                path.to_path(
                    filepath
                    .parent()
                    .expect("Could not get the parent of 'filepath'. 'filepath' terminates in a root or prefix.")
                )
            );

            println!("Pushing {:?} into the futures for opening.", folder.path());

            // Do something with the folder, perhaps lists its child and .
            open_folder(&explorer_box, folder)
        }).collect();

        let rt = runtime::Runtime::new().unwrap();

        for future in Pin::new(&folder_futures_unordered).iter_pin_ref() {

            /*
                 TODO: Implement a connection between the main thread and the Tokio runtime.

                 GTK4 is not thread-safe thus it does not implement Sync on its objects, which is a requirement for tokio Runtime::spawn().

                 Or maybe using Tokio is stupid for this and we should just use another runtime that doesn't require Send, which idk if there are any atm.
            */

            // rt.spawn(future);
        }
    }
}
