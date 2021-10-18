/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */



use gtk::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{
        ApplicationWindow, 
        Application, 
        FileChooserDialog, 
        FileChooserAction,
        AboutDialog, 
        ResponseType
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



impl EchidnaEditor {

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
        fn action_open_file(window: ApplicationWindow, app: &Self, action: &SimpleAction, variant: Option<&glib::Variant>){

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
    
        fn open_file(file: gio::File){
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
        
                app.set_menubar(Some(&menubar));
        
                let act_exit: SimpleAction = SimpleAction::new("exit", None);
                app.add_action(&act_exit);
        
                act_exit.connect_activate(clone!(@weak app =>
                        move |_action, _value| {
                                app.quit();
                        }
                ));
        
        
                let act_about: SimpleAction = SimpleAction::new("about", None);
                app.add_action(&act_about);
                act_about.connect_activate( |_action, _value| {
        
                                        let about_dialog: AboutDialog = AboutDialog::new();
        
                             
                                        about_dialog.set_license_type(gtk::License::Mpl20);
                                        about_dialog.set_program_name(Some("Echidna Code Editor"));
                                        about_dialog.set_website(Some("https://gitlab.com/EchidnaHQ/Echidna"));
                                        about_dialog.set_authors(&["FortressValkriye"]);
                                        about_dialog.set_copyright(Some("Made with by ❤️ Echidna contributors"));
                                        about_dialog.set_visible(true);
                });
        

                let action_open_file: SimpleAction = SimpleAction::new("open-file", None);

                window.add_action(&action_open_file);
                action_open_file.connect_activate(clone!(@weak window, @weak app =>
                        move |action, variant| {
                        Self::action_open_file(window, Self::from_instance(&app), action, variant);
                }));
                window.set_application(Some(app));
        
                window.present();
        
            }


     

}

impl GtkApplicationImpl for EchidnaEditor {}

