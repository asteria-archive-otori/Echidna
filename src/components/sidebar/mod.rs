pub mod imp;

glib::wrapper! {
    pub struct EchidnaSidebar(ObjectSubclass<imp::EchidnaSidebar>)
    @extends gtk::Box, gtk::Widget,
    @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl EchidnaSidebar {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create 'EchidnaSidebar' component.")
    }
}
