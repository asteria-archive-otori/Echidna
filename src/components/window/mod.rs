mod file;

mod imp;
pub mod menubar;

use glib::{
  

    object::IsA,

};

glib::wrapper! {
    pub struct EchidnaWindow(ObjectSubclass<imp::EchidnaWindow>)
    @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, 
    @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;  
}

impl EchidnaWindow {
    pub fn new<P: IsA<gtk::Application>>(application: &P) -> Self {
        
        let object = glib::Object::new(&[
            ("application", &application)
            ]);
            
            match object {
                Ok(o) => o,
                Err(e) => panic!("Error in making EchidnaApplication {}", e),
            }
    }


       

} 

