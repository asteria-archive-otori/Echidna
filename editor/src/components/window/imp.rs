/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */
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
    pub settings: RefCell<Option<gio::Settings>>,
    #[template_child]
    pub sidebar: TemplateChild<super::super::sidebar::EchidnaSidebar>,
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
        let view = self.tab_bar.view().unwrap();
        let settings = gio::Settings::new("io.fortressia.Echidna");

        self.sidebar.to_imp().settings_button.connect_clicked(clone!(@weak view, @weak settings =>
            move |_| {
                let builder = gtk::Builder::from_resource("/io/fortressia/Echidna/preferences.ui");

               let pwin: adw::PreferencesWindow = builder.object("window").expect("no window");
               adw_gschema_auto::from_gsettings(&settings);
               pwin.show();
        }));

        self.settings.replace(Some(settings));
    }
}

impl WidgetImpl for EchidnaWindow {}

impl WindowImpl for EchidnaWindow {}

impl ApplicationWindowImpl for EchidnaWindow {}

impl AdwApplicationWindowImpl for EchidnaWindow {}

impl BuildableImpl for EchidnaWindow {}
