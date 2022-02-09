/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use super::{WorkspaceConnectionType, WorkspaceOpenMessage};
use crate::prelude::*;
use gio::{Cancellable, File, FileQueryInfoFlags, FileType};

fn to_duplicate_option_string(option_string: &Option<String>) -> Option<String> {
    match option_string {
        Some(path) => Some(String::from(path)),
        None => None,
    }
}

pub async fn open_folder(
    parent_file: File,
    tx: glib::Sender<WorkspaceOpenMessage>,
    parent_iter_path: Option<String>,
) {
    let child_enumerate_cancellable = Cancellable::new();
    let parent_path = parent_file.path().expect("No path for the parent file");
    println!(
        "Adding file {:?} and if it's a folder, with its children, to the tree store.",
        parent_path
    );
    let child_files = parent_file
        .enumerate_children(
            "*",
            FileQueryInfoFlags::NONE,
            Some(&child_enumerate_cancellable),
        )
        .expect(
            format!(
                "Could not look up the children files of {:?} because:",
                parent_path
            )
            .as_str(),
        );
    for file_iter in child_files {
        let info = file_iter.expect("Could not get the file info");

        println!("Found child {:?} of {:?}", info.name(), parent_file.path());
        match tx.send(WorkspaceOpenMessage {
            connection_type: WorkspaceConnectionType::AttachFile,
            parent: parent_path.clone(),
            child: info.name(),
            parent_path: to_duplicate_option_string(&parent_iter_path),
        }) {
            Ok(_) => {}
            Err(err) => {
                eprintln!("Cannot send message to the main thread as it has been disconnected. Message: {:#?}", err);
            }
        };
        // let tree_iter = tree.append(parent_iter);
        // tree.set_value(&tree_iter, 1, &file_info.name().to_str().to_value());
        if info.file_type() == FileType::Directory {
            //    open_folder(&file, tx).await

            match tx.send(WorkspaceOpenMessage {
                connection_type: WorkspaceConnectionType::AttachFolder,
                parent: parent_path.clone(),
                child: info.name(),
                parent_path: to_duplicate_option_string(&parent_iter_path),
            }) {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("Cannot send message to the main thread as it has been disconnected. Message: {:#?}", err);
                }
            }
        }
    }
}
