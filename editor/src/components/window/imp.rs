/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */
use crate::components::getting_started::GettingStartedPage;
use crate::prelude::*;
pub use adw::subclass::prelude::*;
use gtk::glib::clone;
use gtk::subclass::prelude::*;
use gtk::CompositeTemplate;
use std::cell::RefCell;

#[derive(Debug, Default, CompositeTemplate)]
#[template(resource = "/io/fortressia/Echidna/window.ui")]
pub struct EchidnaWindow {
    #[template_child]
    pub tab_bar: TemplateChild<adw::TabBar>,
    #[template_child]
    pub tab_view: TemplateChild<adw::TabView>,
    #[template_child]
    pub sidebar: TemplateChild<super::super::sidebar::EchidnaSidebar>,
    #[template_child]
    pub editor_stack: TemplateChild<gtk::Stack>,
    #[template_child]
    getting_started: TemplateChild<GettingStartedPage>,
    pub dialogs: RefCell<Vec<gtk::NativeDialog>>,
}

#[glib::object_subclass]
impl ObjectSubclass for EchidnaWindow {
    const NAME: &'static str = "EchidnaWindow";
    type Type = super::EchidnaWindow;
    type ParentType = adw::ApplicationWindow;

    fn class_init(class: &mut Self::Class) {
        Self::bind_template(class);
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for EchidnaWindow {
    fn constructed(&self, win: &Self::Type) {
        self.editor_stack.set_visible_child_name("getting-started");
        self.tab_view.connect_selected_page_notify(
            clone!(@strong self.editor_stack as editor_stack =>
                move |tab_view|{
                    if tab_view.selected_page().is_none() {
                        editor_stack.set_visible_child_name("getting-started");
                    } else {
                        editor_stack.set_visible_child_name("editor");
                    }
            }),
        );
    }
}

impl WidgetImpl for EchidnaWindow {}

impl WindowImpl for EchidnaWindow {}

impl ApplicationWindowImpl for EchidnaWindow {}

impl AdwApplicationWindowImpl for EchidnaWindow {}

impl BuildableImpl for EchidnaWindow {}
