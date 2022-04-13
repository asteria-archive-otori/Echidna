use super::file_info;
use super::WorkspaceOpenMessage;
use crate::prelude::*;
use file_info::FileInfo;
use gtk::{
    gio::{ListModel, ListStore},
    glib::{clone, ParamFlags, ParamSpec, ParamSpecObject, Value},
    TreeListModel, TreeStore,
};
use once_cell::sync::Lazy;
use relative_path::RelativePath;
use std::cell::RefCell;

#[derive(Debug, Default)]

pub struct WorkspaceModel {
    model: RefCell<Option<TreeListModel>>,
}

#[glib::object_subclass]
impl ObjectSubclass for WorkspaceModel {
    const NAME: &'static str = "EchWorkspaceModel";
    type Type = super::WorkspaceModel;
    type ParentType = glib::Object;

    type Interfaces = (ListModel,);
}

impl ObjectImpl for WorkspaceModel {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![ParamSpecObject::new(
                "model",
                "model",
                "the tree list model of the workspace",
                TreeStore::static_type(),
                ParamFlags::READABLE,
            )]
        });

        PROPERTIES.as_ref()
    }

    fn property(&self, _obj: &Self::Type, _id: usize, spec: &ParamSpec) -> Value {
        match spec.name() {
            "model" => self.model.borrow().to_value(),
            _ => unimplemented!(),
        }
    }

    fn constructed(&self, workspace: &Self::Type) {
        self.model.replace(Some(TreeListModel::new(
            &ListStore::new(file_info::FileInfo::static_type()),
            false,
            true,
            clone!(@strong workspace =>
                move |obj| {
                    let (tx, rx) = glib::MainContext::channel::<WorkspaceOpenMessage>(glib::PRIORITY_DEFAULT);
                    
                    let store = ListStore::new(FileInfo::static_type());
                    rx.attach(
                        None,
                        glib::clone!(@strong tx, @strong workspace, @strong store => move |message| {
                            Self::Type::open_folder_attacher(message, &store);
                  
                            Continue(true)
                        }),
                    );
                    let info = obj.downcast_ref::<file_info::FileInfo>().expect("not a file info");
                    let path = RelativePath::new(&info.name()).to_path(info.parent_path());
                    if info.folder() {
                    tokio::spawn( async move {
                  
                        Self::Type::add_folder_internal(&path, tx.clone()).await;
                    });
                }

                    Some(store.upcast::<ListModel>()) }
                    ),
        )));

        println!(
            "{:?}",
            workspace
                .model()
                .model()
                .downcast::<ListStore>()
                .expect("not a list store")
                .item_type()
        );
    }
}

impl ListModelImpl for WorkspaceModel {
    fn item_type(&self, list_model: &Self::Type) -> glib::Type {
        list_model
            .model()
            .model()
            .downcast::<ListStore>()
            .unwrap()
            .item_type()
    }

    fn n_items(&self, list_model: &Self::Type) -> u32 {
        list_model
            .model()
            .model()
            .downcast::<ListStore>()
            .unwrap()
            .n_items()
    }

    fn item(&self, list_model: &Self::Type, position: u32) -> Option<glib::Object> {
        list_model
            .model()
            .model()
            .downcast::<ListStore>()
            .unwrap()
            .item(position)
    }
}
