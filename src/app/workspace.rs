/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */
 
 use glib::subclass::types::ObjectSubclassExt;
 use super::imp::EchidnaEditor; 
 use std::path::Path;
 use serde::{Deserialize, Serialize};
 use relative_path::RelativePath;
 use gio::Cancellable;
 use gio::{
     SimpleAction, 
     File,
     FileQueryInfoFlags,
     FileType
};
 use gtk::prelude::*;
 use glib::clone;
 use gtk::{
    ApplicationWindow, 
    FileChooserDialog, 
    FileChooserAction,
    ResponseType,
    TreeStore,
};
use glib::types::Type;

#[derive(Deserialize, Serialize)]
struct MonacoFolder {
    path: String
}

#[derive(Deserialize, Serialize)]
struct MonacoWorkspace {
     folders: Vec<MonacoFolder>
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
 
     // TODO: Somehow inserts self to this function.
     // This function sets the callback function as 'static, which for some reasons ban cloning self into it. Idk why.
     dialog.connect_response(clone!(@weak window, @weak app =>
         move |dialog, response| {
 
             if response == ResponseType::Accept {
 
                 let file_option = dialog.file();
 
                 match file_option {
                     Some(file) => {
                         dialog.destroy();   
                         Self::from_instance(&app).open_workspace(file);
 },
 None => {
 
 },                        
 }                                                      
 } else if response == ResponseType::Cancel {
 dialog.destroy();
 
 }  }));
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
fn open_workspace(&self, file: File){
    let cancellable = Cancellable::new();
    let filepath_raw = &file.path().expect("Could not get the file path of the file.");
    let filepath = Path::new(&filepath_raw);
    let file_info_result = file.query_info(
     "*",
    gio::FileQueryInfoFlags::NONE,
    Some(&cancellable));

    match file_info_result {
                    Ok(info) => {
    match info.content_type() {
    Some(content_type) => { 
    println!("Opened {} and found its content type is {}.", "file", content_type.to_string());
    let content_cancellable = Cancellable::new();
    let file_content = file.load_contents(Some(&content_cancellable));
    match file_content {
            Ok(content) => {
                let (int_vec, _byte_string) = content;
             
                match serde_json::from_slice::<MonacoWorkspace>(&int_vec) {
                    Ok(workspace) => for folder in workspace.folders {
                            let path = RelativePath::new(&folder.path);

                            let folder = File::for_path(path.to_path(filepath));

                            // Do something with the folder, perhaps lists its child and .
                            self.open_folder(folder);
                        },
                    Err(e) => println!("Could not parse {:#?} because of:\n{}", filepath, e),
                }
            }
            Err(e) => println!("Could not open {:?} because:\n{}", filepath, e),
    }
    },
    None => println!("It does not seem like {:?} has a type", filepath),

    }

    },
    Err(e) => println!("Could not retrieve file information for {:?} because:\n{}", filepath, e),
    }
    
 }



/**
 * 
 * 
 */
fn recursive_add_files_into_tree_store(&self, parent_file: File, tree: &TreeStore){
    let child_enumerate_cancellable = Cancellable::new();
    let child_files = parent_file.enumerate_children("*", FileQueryInfoFlags::NONE, Some(&child_enumerate_cancellable));
    let filepath = &parent_file.path().expect("Could not get the file path of the file.");
    match child_files {
        Ok(files) => {
            for file_iter in files {
                match file_iter {
                    Ok(file_info) => {
                        let file = parent_file.child(file_info.name());
                        let tree_iter = tree.append(None);
                        tree.set_value(&tree_iter, 2, &file_info.name().to_str().to_value());
                        
                        if file_info.file_type() == FileType::Directory {
                            self.recursive_add_files_into_tree_store(file, tree);

                        }

                    },
                    Err(e) => println!("Could not get information on file because of:\n{}", e)
                }
               
            }

        },
        Err(e) => println!("Could not look up the children files of {:?} because:\n{:#?}", filepath, e)
    }
}

/*
 Loads a folder into the tree view.
 
 - Create a new tree
 - Enumerate over child files of 'file'. (PS: In the Unix family of OS-es, directories are files too)
 */
fn open_folder(&self, file: File){
    let tree = TreeStore::new(&[gdk::Texture::static_type(), Type::STRING]);
    self.recursive_add_files_into_tree_store(file, &tree);
   


    }
}