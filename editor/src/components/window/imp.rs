/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * tab-view, You can obtain one at https://mozilla.org/MPL/2.0/. */
use crate::prelude::*;
pub use adw::subclass::prelude::*;
use gtk::glib::ParamFlags;
use gtk::glib::ParamSpec;
use gtk::glib::ParamSpecObject;
use gtk::glib::Value;
use gtk::glib::clone;
use gtk::subclass::prelude::*;
use gtk::CompositeTemplate;
use gtk::Inhibit;
use once_cell::sync::Lazy;
use std::cell::RefCell;
use adw::TabView;
#[derive(Debug, Default, CompositeTemplate)]
#[template(resource = "/io/fortressia/Echidna/components/window/window.ui")]
pub struct EchidnaWindow {
    #[template_child]
    pub tab_view: TemplateChild<adw::TabView>,
    pub settings: RefCell<Option<gio::Settings>>,
    pub dialogs: RefCell<Vec<gtk::NativeDialog>>,
    #[template_child]
    open_button: TemplateChild<gtk::Button>,
    #[template_child]
    pub settings_button: TemplateChild<gtk::Button>,
    #[template_child]
    pub explorer: TemplateChild<gtk::Box>
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
        let settings = gio::Settings::new("io.fortressia.Echidna");

        self.settings_button.connect_clicked(clone!( @weak settings =>
            move |_| {
                let builder = gtk::Builder::from_resource("/io/fortressia/Echidna/components/window/preferences.ui");

               let pwin: adw::PreferencesWindow = builder.object("window").expect("no window");
               adw_gschema_auto::from_gsettings(&settings);
               pwin.show();
        }));

        self.settings.replace(Some(settings));
    }

    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![ParamSpecObject::new(
                "tab-view",
                "tab-view",
                "the tab-view of the editor",
                TabView::static_type(),
                ParamFlags::READABLE,
            ),
  
            ]
        });

        PROPERTIES.as_ref()
    }



    fn property(&self, _obj: &Self::Type, _id: usize, spec: &ParamSpec) -> Value {
        match spec.name() {
            "tab-view" => self.tab_view.to_value(),
            _ => unimplemented!(),
        }
    }
}

impl WidgetImpl for EchidnaWindow {}

impl WindowImpl for EchidnaWindow {
    fn close_request(&self, window: &Self::Type) -> Inhibit {
        println!("window ref count: {} ", window.ref_count());

        Inhibit(true)
    }
}

impl ApplicationWindowImpl for EchidnaWindow {}

impl AdwApplicationWindowImpl for EchidnaWindow {}

impl BuildableImpl for EchidnaWindow {}
