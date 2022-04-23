mod imp;

use adw::TabView;

use crate::internal::prelude::*;

glib::wrapper! {
    ///
    /// Model for VerticalTabBar
    ///
    /// This model already implements the selection model.
    ///
    pub struct VerticalBarModel(ObjectSubclass<imp::VerticalBarModel>)
    @implements gio::ListModel, gtk::SelectionModel;
}

impl VerticalBarModel {
    pub fn new(tab_view: &TabView) -> Self {
        glib::Object::new(&[("tab-view", &tab_view)]).expect("Can't make EchVerticalBarModel")
    }
    pub fn tab_view(&self) -> TabView {
        self.property("tab-view")
    }
}

impl Default for VerticalBarModel {
    fn default() -> Self {
        Self::new(&Default::default())
    }
}
