/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use super::recursive_add_files::*;
use gio::File;
use glib::Type;
use gtk::{prelude::*, TreeStore, TreeView};

pub async fn open_folder(explorer_box: &gtk::Box, file: File) {
    let tree = TreeStore::new(&[gdk::Texture::static_type(), Type::STRING]);
    println!("Opening folder {:?}", file.path());
    recursive_add_files_into_tree_store(&file, &tree, None).await;

    let tree_view = TreeView::new();

    tree_view.set_model(Some(&tree));

    explorer_box.prepend(&tree_view);
}
