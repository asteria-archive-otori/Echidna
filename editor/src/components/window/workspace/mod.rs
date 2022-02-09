/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

mod open_folder;
use super::EchidnaWindow;
use crate::prelude::*;
use gio::{Cancellable, File};
use glib::{clone, Type};
use gtk::{ResponseType, TreePath, TreeStore, TreeView};
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

enum WorkspaceConnectionType {
    AttachFile,
    AttachFolder,
}

pub trait WorkspaceImplementedEditor {
    fn action_open_workspace(&self) -> Result<(), String>;

    fn open_workspace(&self, file: File);
}

async fn open_root_folder(
    folder_path: &MonacoFolder,
    filepath: &PathBuf,
    tx: glib::Sender<WorkspaceOpenMessage>,
) {
    let path = RelativePath::new(&folder_path.path);
    let folder = File::for_path(path.to_path(filepath.parent().expect(
        "Could not get the parent of 'filepath'. 'filepath' terminates in a root or prefix.",
    )));

    println!("Pushing {:?} into the futures for opening.", folder.path());

    // Do something with the folder, perhaps lists its child and .
    open_folder(folder.clone(), tx, None).await;
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

        let filepath = file
            .path()
            .expect("Could not get the file path of the file.");
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
        let tree_view = TreeView::new();

        // TODO: Implement file icons
        let tree = TreeStore::new(&[/*gdk::Texture::static_type(), */ Type::STRING]);
        tree_view.set_model(Some(&tree));

        // glib::MainContext is like std::mpsc, but its receivers don't block the current thread.
        let (tx, rx) = glib::MainContext::channel::<WorkspaceOpenMessage>(glib::PRIORITY_DEFAULT);

        explorer_box.prepend(&tree_view);
        rx.attach(
            None,
            clone!(@strong tx =>
                move |message| {
                match message.connection_type {
                    WorkspaceConnectionType::AttachFile => {

                        // TODO: Attach a file into the TreeStore

                        let parent_iter = match message.parent_path {
                            Some(parent_path) => {
                                match TreePath::from_string(&parent_path) {
                                    Some(tree_path) => {

                                        tree.iter(&tree_path)
                                    },
                                    None => {
                                        eprintln!("The processing of {:?}/{:?} does not have a valid iter. Attaching as root.", 
                                                    message.parent, message.child);
                                        None
                                    }
                                }

                            },
                            None => None
                        };

                        let iter = tree.append(parent_iter.as_ref());

                        match message.child.to_str() {
                            Some(name) => {
                                println!("Adding {:?}", &name);
                                tree.set(&iter, &[(0, &name)]);
                            },
                            None => eprintln!("Name is not Unicode")
                        };
                    }
                    WorkspaceConnectionType::AttachFolder => {
                        let file = File::for_path(&message.parent.join(&message.child));
                        // TODO: Attach a folder into the TreeStore


                        let parent_iter = match &message.parent_path {
                            Some(parent_path) => {

                                match &TreePath::from_string(parent_path) {
                                    Some(parent_path) => {
                                     tree.iter(&parent_path)
                                    },
                                    None => None
                                }

                            },
                            None => None
                        };

                        let iter = tree.append(parent_iter.as_ref());

                        match message.child.to_str() {
                            Some(name) => {
                                println!("Adding {:?}", &name);
                                tree.set(&iter, &[(0, &name)]);
                            },
                            None => eprintln!("Name is not Unicode")
                        };

                        // Borrows message.parent_path to be moved into the async closure.
                        let parent_path = message.parent_path;

                        #[allow(unused_braces)]
                        tokio::spawn(clone!(@strong tx =>
                            async move { open_folder(file, tx.clone(), parent_path).await }));
                    }
                };
                Continue(true)
            }),
        );

        for folder in workspace.folders {
            #[allow(unused_braces)]
            tokio::spawn(clone!(@strong tx, @strong filepath =>
                async move { open_root_folder(&folder, &filepath, tx.clone()).await }));
        }
    }
}
