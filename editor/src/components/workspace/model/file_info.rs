use crate::prelude::*;

pub mod imp {
    use crate::prelude::*;
    use gtk::glib::{ParamFlags, ParamSpec, ParamSpecBoolean, ParamSpecString, Value};
    use once_cell::sync::Lazy;
    use std::cell::RefCell;

    #[derive(Debug, Default)]
    pub struct FileInfo {
        parent_path: RefCell<String>,
        name: RefCell<String>,
        folder: RefCell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for FileInfo {
        const NAME: &'static str = "EchFileInfo";
        type Type = super::FileInfo;

        type ParentType = glib::Object;
    }

    impl ObjectImpl for FileInfo {
        fn properties() -> &'static [ParamSpec] {
            static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
                vec![
                    ParamSpecString::new(
                        "parent-path",
                        "parent-path",
                        "the path of the file's parent",
                        None,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecString::new(
                        "file-name",
                        "file-name",
                        "the file name of the file",
                        None,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecBoolean::new(
                        "is-folder",
                        "is-folder",
                        "whether the file is a folder",
                        false,
                        ParamFlags::READWRITE,
                    ),
                ]
            });

            PROPERTIES.as_ref()
        }

        fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, spec: &ParamSpec) {
            match spec.name() {
                "parent-path" => {
                    let parent_path: String =
                        value.get().expect("The parent path needs to be String");
                    self.parent_path.replace(parent_path);
                }
                "file-name" => {
                    let name: String = value.get().expect("The file name needs to be String");
                    self.name.replace(name);
                }
                "is-folder" => {
                    let folder: bool = value.get().expect("The folder property needs to be bool");
                    self.folder.replace(folder);
                }

                _ => unimplemented!(),
            }
        }

        fn property(&self, _obj: &Self::Type, _id: usize, spec: &ParamSpec) -> Value {
            match spec.name() {
                "parent-path" => self.parent_path.borrow().to_value(),
                "file-name" => self.name.borrow().to_value(),
                "is-folder" => self.folder.borrow().to_value(),
                _ => unimplemented!(),
            }
        }
    }
}

glib::wrapper! {
    pub struct FileInfo(ObjectSubclass<imp::FileInfo>);
}

impl FileInfo {
    pub fn new(file_name: &String, parent_path: &String, is_folder: &bool) -> Self {
        glib::Object::new(&[
            ("file-name", file_name),
            ("parent-path", parent_path),
            ("is-folder", is_folder),
        ])
        .unwrap()
    }

    pub fn name(&self) -> String {
        self.property("file-name")
    }

    pub fn parent_path(&self) -> String {
        self.property("parent-path")
    }

    pub fn folder(&self) -> bool {
        self.property("is-folder")
    }
}
