/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use std::str::from_utf8;
use sourceview::{View, Buffer, LanguageManager, };
use sourceview::prelude::*;
use gtk::subclass::prelude::*;
use gtk::prelude::*;
use std::rc::Rc;
use super::menubar::MenubarImplementedEditor;
use gtk::{
        ApplicationWindow, 
        Application, 
        FileChooserDialog, 
        FileChooserAction,
        ResponseType,
        Label

};

use gio::{
        MenuModel, 
        SimpleAction, 
        Cancellable
};
//use super::workspace;
use glib::clone;

#[derive(Debug, Default)]
pub struct EchidnaEditor {
    pub name: &'static str,
    pub app_id: &'static str
}

pub trait EchidnaEditorExt {
        fn action_open_file(window: ApplicationWindow, app: super::EchidnaEditor, action: &SimpleAction, variant: Option<&glib::Variant>, notebook: gtk::Notebook);
        fn open_file(notebook: &gtk::Notebook, file: gio::File);
}



#[glib::object_subclass]
impl ObjectSubclass for EchidnaEditor {
    const NAME: &'static str = "EchidnaEditorApplication";
    type Type = super::EchidnaEditor;
    type ParentType = Application;

    fn new() -> Self {
            Self {
                    name: "Echidna Code Editor",
                    app_id: "land.echidna.editor"
                }
    }
}



impl EchidnaEditorExt for EchidnaEditor {

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
        fn action_open_file(window: ApplicationWindow, app: super::EchidnaEditor, _action: &SimpleAction, variant: Option<&glib::Variant>, notebook: gtk::Notebook){

                let dialog: FileChooserDialog = FileChooserDialog::new(Some("Open a file"), 
                                                                        Some(&window), 
                                                                        FileChooserAction::Open, 
                                                                        &[("Cancel", ResponseType::Cancel),
                                                                      ("Open", ResponseType::Accept)        ]);


                dialog.set_visible(true);
    
                                                   

                // TODO: Somehow inserts self to this function.
                // This function sets the callback function as 'static, which for some reasons ban cloning self into it. Idk why.
                dialog.connect_response(clone!(@weak window =>
        move |dialog, response| {
    
            if response == ResponseType::Accept {
    
                let file_option = dialog.file();
               
                match file_option {
                        Some(file) => {
                                dialog.destroy();   
                                Self::open_file(file);

                        },
                        None => {
    
                        },                        
                }                                                      
               } else if response == ResponseType::Cancel {
                       dialog.destroy();
             
               }  }));                                                              
        
        }
    
        fn open_file(notebook: &gtk::Notebook, file: gio::File){
                let cancellable = Cancellable::new();
                let filepath = file.path();
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
                            let (int_vec, text_option) = content;
                                println!("The int vector of {:?} is {:?}", filepath, int_vec);
                            match text_option {
                                Some(text) => println!("Opened {:?} and its content is:\n{}", filepath, text.as_str()),
                                None => println!("No value for {:?}.", filepath),
                            }
                        }
                        Err(e) => println!("Could not open {:?} because:\n{:#?}", filepath, e),
                }
                },
                None => println!("It does not seem like {:?} has a type", filepath),
    
                }
    
                },
                Err(e) => println!("Could not retrieve file information for {:?} because:\n{}", filepath, e),
                }
                
    
        }


}

impl ObjectImpl for EchidnaEditor {



}

impl ApplicationImpl for EchidnaEditor {
     
        fn activate(&self, app: &Self::Type){

         let builder = gtk::Builder::from_string(include_str!("../../ui/window.ui"));
         let window: ApplicationWindow = builder
          .object("window")
          .expect("Could not get object 'window' from builder.");

         let menubuilder = gtk::Builder::from_string(include_str!("../../ui/menu.ui"));
         let menubar: MenuModel = menubuilder
          .object("menu")
          .expect("Could not get object 'menu' from builder.");

         self.setup_menubar(app, window, builder);
         let notebook: gtk::Notebook = builder
                .object("echidna-notebook")
                .expect("Could not get 'echidna-notebook' from builder.");
         window.set_application(Some(app));
        
         window.present();
        
        }

}

impl GtkApplicationImpl for EchidnaEditor {}

