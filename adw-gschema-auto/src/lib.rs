use adw::gio;
use adw::gio::SettingsSchemaKey;
use adw::glib;
use adw::gtk;
use adw::prelude::*;
use glib::VariantTy;
use gtk::Widget;

fn match_type(
    value_type: &VariantTy,
    key: &SettingsSchemaKey,
    settings: &gio::Settings,
) -> Option<Widget> {
    if value_type == VariantTy::BOOLEAN {
        let switch = gtk::Switch::new();
        settings.bind(&key.name(), &switch, "state").build();
        Some(switch.upcast::<Widget>())
    } else if key.value_type() == VariantTy::STRING {
        let view = gtk::TextView::new();

        settings.bind(&key.name(), &view.buffer(), "text").build();
        Some(view.upcast::<Widget>())
    } else {
        None
    }
}

///
/// Generates an adw::PreferencesPage. See the module level documentation.
///
pub fn from_gsettings(settings: &gio::Settings) -> Vec<adw::PreferencesPage> {
    let pages = Vec::new();

    let schema = settings
        .settings_schema()
        .expect("Settings don't have a schema");

    schema.list_children();

    for key_id in schema.list_keys() {
        let key = schema.key(&key_id);

        let row = adw::PreferencesRow::new();
        if key.summary().is_some() {
            row.set_title(&key.summary().unwrap());
        }
        let child = gtk::Box::new(gtk::Orientation::Horizontal, 15);
        if key.description().is_some() {
            child.append(&gtk::Label::new(Some(&key.description().unwrap())));
        }
        let value_type = key.value_type();
        if value_type.is_tuple() {
            for value_type in value_type.tuple_types() {
                match_type(&value_type, &key, &settings);
            }
        } else {
            let widget = match_type(&value_type, &key, &settings);

            if widget.is_some() {
                child.append(&widget.unwrap());
            }
        }
        row.set_child(Some(&child));
    }

    pages
}
