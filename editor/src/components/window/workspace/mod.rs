/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod open_folder;
use super::EchidnaWindow;
use crate::prelude::*;
use gio::{Cancellable, File};
use glib::clone;
use gtk::{ResponseType, TreePath, TreeStore};
use open_folder::open_folder;
use relative_path::RelativePath;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Deserialize, Serialize)]
struct MonacoFolder {
    path: String,
}

#[derive(Deserialize, Serialize)]
struct MonacoWorkspace {
    folders: Vec<MonacoFolder>,
}

pub struct WorkspaceOpenMessage {
    connection_type: WorkspaceConnectionType,
    parent: PathBuf,
    parent_path: Option<String>,
    child: PathBuf,
}

#[derive(PartialEq)]
enum WorkspaceConnectionType {
    AttachFile,
    AttachFolder,
}

pub trait WorkspaceImplementedEditor {
    fn action_open_workspace(&self) -> Result<(), String>;

    fn open_workspace(&self, file: File);
}

impl WorkspaceImplementedEditor for EchidnaWindow {
    fn action_open_workspace(&self) -> Result<(), String> {
        /*
           Borrows self.to_imp()'s dialog Vector mutably, create a new dialog , and push that dialog into the vector.

           This is required because we own the dialog and thus the dialog will be destroyed when this function has completed.
        */
        match self.to_imp().dialogs.try_borrow_mut() {
            Ok(mut dialogs) => {
                let dialog = gtk::FileChooserNative::new(
                    Some("Open Workspace"),
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

                        match dialog.file() {
                            Some(file) => window.open_workspace(file),
                            None => eprintln!("The dialog does not select any files.")
                        };

                    } else {
                        println!("{:?}", response);
                    }
                    dialog.destroy();

                }));
                dialog_clone.show();

                Ok(())
            }
            Err(e) => Err(format!("{:#?}", e)),
        }
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

        let workspace_config = file
            .path()
            .expect("Could not get the file path of the file.");
        let info = file
            .query_info("*", gio::FileQueryInfoFlags::NONE, Some(&cancellable))
            .expect(
                format!(
                    "Could not retrieve file information for {:?}",
                    workspace_config
                )
                .as_str(),
            );
        let content_type = info
            .content_type()
            .expect(format!("Found no content type for {:?}", workspace_config).as_str());
        println!(
            "Opened {} and found its content type is {}.",
            "file",
            content_type.to_string()
        );
        let content_cancellable = Cancellable::new();
        let (content, _) = file.load_contents(Some(&content_cancellable)).expect(
            format!(
                "Could not load the file contents for {:?}",
                workspace_config
            )
            .as_str(),
        );

        let workspace = serde_json::from_slice::<MonacoWorkspace>(&content).expect(
            format!(
                "Could not parse the workspace file of {:?}",
                workspace_config
            )
            .as_str(),
        );

        let tree_view = &self.to_imp().sidebar.to_imp().tree;

        // TODO: Implement file icons
        let tree: TreeStore = tree_view
            .model()
            .expect("No model in tree view")
            .downcast()
            .expect("Cannot downcast to GtkTreeStore");

        // glib::MainContext is like std::mpsc, but its receivers don't block the current thread.
        let (tx, rx) = glib::MainContext::channel::<WorkspaceOpenMessage>(glib::PRIORITY_DEFAULT);

        rx.attach(
            None,
            clone!(@strong tx, @strong tree =>
                move |message| {
                    let file = File::for_path(&message.parent.join(&message.child));

                    let parent_iter = match &message.parent_path {
                        Some(parent_path) => {
                            println!("{parent_path}");

                            match &TreePath::from_string(parent_path) {
                                Some(parent_path) => {
                                 tree.iter(&parent_path)
                                },
                                None => None
                            }


                        },
                        None => None
                    };


                    match parent_iter {
                        Some(iter) => {let path = tree.path(&iter).to_str();
                        println!("{parent_iter:?} with {path:?}");
                        },
                        None => {}
                    };

                    let iter = tree.append(parent_iter.as_ref());

                    match message.child.to_str() {
                        Some(name) => {
                            println!("Adding {:?}", &name);
                            tree.set(&iter, &[(0, &name)]);
                            println!(
                                "CLIENT: Adding file {:?} and if it's a folder, with its children, to the tree store.",
                                &name
                            );
                        },
                        None => eprintln!("Name is not Unicode")
                    };
                if message.connection_type == WorkspaceConnectionType::AttachFolder  {

                        let parent_path = tree.path(&iter).to_str().expect("No iter in tree");

                        #[allow(unused_braces)]
                        tokio::spawn(clone!(@strong tx =>
                            async move { open_folder(file, tx.clone(), Some(parent_path.to_string())).await }));

                };
                Continue(true)
            }),
        );

        let parent = workspace_config.parent().expect("Can't retrieve the parent of the workspace configuration. Seems that the configuration is in the root (in Unix/MacOS) or the drive (Windows).");
        for folder in workspace.folders {
            println!(
                "CLIENT: Adding file {:?} and if it's a folder, with its children, to the tree store.",
                &folder.path
            );

            let path = RelativePath::new(&folder.path).to_path(&workspace_config.parent().expect("Can't retrieve the parent of the workspace configuration. Seems that the configuration is in the root (in Unix/MacOS) or the drive (Windows)."));
            let folder = File::for_path(&path);
            let iter = tree.append(None);
            let iter_path = match tree.path(&iter).to_str() {
                Some(path) => Some(path.to_string()),
                None => {
                    eprintln!("Path is invalid for {path:?}");

                    None
                }
            };
            tree.set(
                &iter,
                &[(
                    0,
                    &path
                        .file_name()
                        .expect(&format!(
                            "CLIENT: path {path:?} {:?} ends with .. parent {parent:?}",
                            &folder.path()
                        ))
                        .to_str()
                        .expect("CLIENT: No unicode in filename"),
                )],
            );

            println!("CLIENT: Pushing {path:?} into the futures for opening.");

            #[allow(unused_braces)]
            tokio::spawn(clone!(@strong tx =>
            async move {
                // Do something with the folder, perhaps lists its child and .
                open_folder(folder.clone(), tx, iter_path).await;
            }));
        }
    }
}
