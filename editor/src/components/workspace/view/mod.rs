mod imp;
use crate::{components::WorkspaceModel, prelude::*};
glib::wrapper! {
    pub struct WorkspaceView(ObjectSubclass<imp::WorkspaceView>)
        @extends adw::Bin, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl WorkspaceView {
    pub fn new() -> Self {
        glib::Object::new(&[]).expect("Failed to create 'WorkspaceView' component.")
    }

    pub fn model(&self) -> WorkspaceModel {
        self.property("workspace-model")
    }
}
