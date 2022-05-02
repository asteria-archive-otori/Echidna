/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * tab_view, You can obtain one at https://mozilla.org/MPL/2.0/.
 *
 * Copyright 2022 to Echidna Code contributors. All Rights Reserved
 *
 * List of authors:
 * Nefo Fortressia <nefothingy@hotmail.com>
 *
 */

use crate::internal::prelude::*;
use adw::{subclass::prelude::*, TabView};
use gtk::ListView;
use gtk::{
    glib::{ParamFlags, ParamSpec, ParamSpecObject, Value},
    subclass::prelude::*,
    CompositeTemplate,
};
use once_cell::sync::Lazy;
use std::cell::RefCell;

use super::model::VerticalBarModel;
#[derive(Default, CompositeTemplate)]
#[template(resource = "/com/fortressia/Libchidna/vtb.ui")]
pub struct VerticalTabBar {
    tab_view: RefCell<TabView>,
    #[template_child]
    list_view: TemplateChild<ListView>,
}

#[glib::object_subclass]
impl ObjectSubclass for VerticalTabBar {
    const NAME: &'static str = "EchVerticalTabBar";
    type Type = super::VerticalTabBar;
    type ParentType = adw::Bin;

    fn class_init(klass: &mut Self::Class) {
        Self::bind_template(klass);
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for VerticalTabBar {
    fn constructed(&self, obj: &Self::Type) {
        self.list_view
            .set_model(Some(&VerticalBarModel::new(&obj.tab_view())));
    }

    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![ParamSpecObject::new(
                "tab-view",
                "tab-view",
                "the tab view of the editor",
                TabView::static_type(),
                ParamFlags::READWRITE,
            )]
        });

        PROPERTIES.as_ref()
    }

    fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, spec: &ParamSpec) {
        match spec.name() {
            "tab-view" => {
                let view: TabView = value.get().expect("The tab_view needs to be AdwTabView");
                self.tab_view.replace(view);
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

impl WidgetImpl for VerticalTabBar {}

impl BinImpl for VerticalTabBar {}
