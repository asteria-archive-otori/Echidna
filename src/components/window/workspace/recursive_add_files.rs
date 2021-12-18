/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use async_recursion::async_recursion;
use gio::{Cancellable, File, FileQueryInfoFlags, FileType};
use gtk::{prelude::*, TreeIter, TreeStore};
#[async_recursion(?Send)]
pub async fn recursive_add_files_into_tree_store(
    parent_file: &File,
    tree: &TreeStore,
    parent_iter: Option<&'async_recursion TreeIter>,
) {
    let child_enumerate_cancellable = Cancellable::new();

    println!(
        "Adding file {:?} and if it's a folder, with its children, to the tree store.",
        parent_file.path()
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
                parent_file.path().expect("No path for the parent file")
            )
            .as_str(),
        );
    for file_iter in child_files {
        let file_info = file_iter.expect("Could not get the file info");
        let file = parent_file.child(file_info.name());

        println!(
            "Found child {:?} of {:?}",
            file_info.name(),
            parent_file.path()
        );

        let tree_iter = tree.append(parent_iter);
        tree.set_value(&tree_iter, 1, &file_info.name().to_str().to_value());
        if file_info.file_type() == FileType::Directory {
            recursive_add_files_into_tree_store(&file, tree, Some(&tree_iter)).await;
        }
    }
}
