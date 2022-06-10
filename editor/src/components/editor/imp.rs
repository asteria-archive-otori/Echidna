/* obj Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with obj
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::prelude::*;
use glib::{ParamFlags, ParamSpec, ParamSpecObject, Value, clone};

use gtk::{subclass::prelude::*, CompositeTemplate};
use once_cell::sync::Lazy;
use sourceview::{traits::*, Buffer, LanguageManager, FileLoader};
use std::{cell::RefCell};
fn set_scheme(buffer: &sourceview::Buffer, is_dark: bool) {
    let style_manager = sourceview::StyleSchemeManager::default();
    // Set the scheme to Adwaita by default.
    let default_scheme = match is_dark {
        true => "Adwaita-dark",
        false => "Adwaita",
    };
    buffer.set_style_scheme(style_manager.scheme(default_scheme).as_ref());
}
#[derive(Default, CompositeTemplate)]
#[template(resource = "/io/fortressia/Echidna/components/editor/editor.ui")]
pub struct EchidnaCoreEditor {
    #[template_child]
    pub minimap: TemplateChild<sourceview::Map>,
    #[template_child]
    pub sourceview: TemplateChild<sourceview::View>,
    file: RefCell<Option<sourceview::File>>,
    app: RefCell<Option<adw::Application>>,
}

#[glib::object_subclass]
impl ObjectSubclass for EchidnaCoreEditor {
    const NAME: &'static str = "EchidnaCoreEditor";
    type Type = super::EchidnaCoreEditor;
    type ParentType = gtk::Box;

    fn class_init(class: &mut Self::Class) {
        Self::bind_template(class);
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for EchidnaCoreEditor {

    fn constructed(&self, obj: &Self::Type) {
           // Without cloning it, for some reasons the Rust compiler complains about &obj.to_imp().sourceview not being IsA<sourceview::View>
           self.minimap.set_view(&self.sourceview.clone());
           let buffer = self.sourceview.buffer().downcast::<Buffer>().expect("Cannot downcast the sourceview's buffer. Maybe the sourceview's buffer is not IsA<sourceview::Buffer>.");
           
           if let Some(f) = obj.file() {
               let file_location = f.location();
               obj.set_property("file", &f);
   
               let cancellable = gio::Cancellable::new();
               let filepath = file_location.path().expect("No filepath");
               let info = file_location
                   .query_info("*", gio::FileQueryInfoFlags::NONE, Some(&cancellable))
                   .expect("Could not query the info for file");
   
               let content_type = info.content_type().expect(
                   format!("It does not seem like {:?} has a type", filepath).as_str(),
               );
               {
                   println!(
                       "Opened {} and found its content type is {}.",
                       "file",
                       content_type.to_string()
                   );
   
                   let language_manager = LanguageManager::new();
                   let language = language_manager.guess_language(
                       Some(&info.name().to_str().expect(
                           "Could not open the file because its name is not supported by Unicode.",
                       )),
                       None,
                   );
   
                   if let Some(lang) = language {
                       buffer.set_language(Some(&lang));
                   }
   
                   let file_loader: FileLoader = FileLoader::new(&buffer, &f);
   
                   file_loader.load_async(
                   glib::Priority::default(),
                   Some(&cancellable),
   
                   |result| {
                       if result.is_err() {
                           panic!("Found an error when loading the file into the text editor's buffer. {:#?}", result.err());
                       }
                   },
                   );
               }
           }
   
           if let Some(a) = obj.app() {
               let manager = a.style_manager();
               set_scheme(&buffer, manager.is_dark());
               manager.connect_dark_notify(clone!(@weak buffer =>
                   move |manager|{
                   set_scheme(&buffer, manager.is_dark());
               }));
           }
           
           
    }
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![ParamSpecObject::new(
                "file",
                "file",
                "the file of the editor",
                sourceview::File::static_type(),
                ParamFlags::READWRITE,
            ),
            ParamSpecObject::new(
                "file",
                "file",
                "the application",
                adw::Application::static_type(),
                ParamFlags::READWRITE
            )
            
            ]
        });

        PROPERTIES.as_ref()
    }

    fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, spec: &ParamSpec) {
        match spec.name() {
            "file" => {
                let file: Option<sourceview::File> =
                    value.get().expect("The file needs to be sourceview::File");
                self.file.replace(file);
            }
            "app" => {
                let app: Option<adw::Application> =
                    value.get().expect("The file needs to be adw::Application");
                self.app.replace(app);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, spec: &ParamSpec) -> Value {
        match spec.name() {
            "file" => self.file.borrow().to_value(),
            "app" => self.app.borrow().to_value(),
            _ => unimplemented!(),
        }
    }
}

impl WidgetImpl for EchidnaCoreEditor {}
impl BoxImpl for EchidnaCoreEditor {}
