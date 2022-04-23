use crate::internal::prelude::*;
use adw::subclass::prelude::*;
use gio::ListStore;
use glib::{ParamFlags, ParamSpec, ParamSpecObject, Value};
use gtk::{
    subclass::prelude::*, CompositeTemplate, CustomFilter, FilterListModel, MultiSelection,
    TreeListModel, TreeModel,
};
use once_cell::sync::Lazy;

use std::cell::RefCell;

#[derive(Default)]
pub struct VerticalBarModel {
    tab_view: RefCell<adw::TabView>,
    selection_model: RefCell<Option<MultiSelection>>,
    filter_model: RefCell<Option<FilterListModel>>,
    tree_model: RefCell<Option<gtk::TreeListModel>>,
}

#[glib::object_subclass]
impl ObjectSubclass for VerticalBarModel {
    const NAME: &'static str = "EchVerticalBarModel";
    type Type = super::VerticalBarModel;
    type ParentType = adw::Bin;
    type Interfaces = (gio::ListModel, gtk::SelectionModel);
}

impl VerticalBarModel {
    fn selection_model(&self) -> MultiSelection {
        self.selection_model.borrow().clone().unwrap()
    }
}

impl ObjectImpl for VerticalBarModel {
    fn constructed(&self, model: &Self::Type) {
        let filter = CustomFilter::new(move |obj| true);
        let internal = model.tab_view().pages();
        let tree_model = TreeListModel::new(&internal, false, true, |item| {
            Some(ListStore::new(adw::TabPage::static_type()).upcast())
        });

        let filter_model = FilterListModel::new(Some(&tree_model), Some(&filter));
        self.selection_model
            .replace(Some(MultiSelection::new(Some(&filter_model))));
    }

    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![ParamSpecObject::new(
                "tab-view",
                "tab-view",
                "the tab-view of the editor",
                adw::TabView::static_type(),
                ParamFlags::READWRITE,
            )]
        });

        PROPERTIES.as_ref()
    }

    fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, spec: &ParamSpec) {
        match spec.name() {
            "tab-view" => {
                let tab_view: adw::TabView =
                    value.get().expect("The tab_view needs to be adw::TabView");
                self.tab_view.replace(tab_view);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, spec: &ParamSpec) -> Value {
        match spec.name() {
            "tab-view" => self.tab_view.borrow().to_value(),
            _ => unimplemented!(),
        }
    }
}

impl WidgetImpl for VerticalBarModel {}

impl BinImpl for VerticalBarModel {}

impl ListModelImpl for VerticalBarModel {
    fn item_type(&self, _obj: &Self::Type) -> glib::Type {
        self.selection_model().item_type()
    }

    fn n_items(&self, _obj: &Self::Type) -> u32 {
        self.selection_model().n_items()
    }

    fn item(&self, _obj: &Self::Type, position: u32) -> Option<glib::Object> {
        self.selection_model().item(position)
    }
}

impl gtk::subclass::prelude::SelectionModelImpl for VerticalBarModel {}
