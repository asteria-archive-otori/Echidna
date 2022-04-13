use std::{error::Error, fs::Metadata, path::PathBuf};

use crate::prelude::*;
pub mod file_info;
pub mod imp;
use file_info::FileInfo;
use gtk::{
    gio::ListStore,
    glib::{clone, Sender},
    TreeListModel,
};
use relative_path::RelativePath;
use serde::{Deserialize, Serialize};
use tokio::fs::{self, DirEntry};

glib::wrapper! {
    /// A list model that supports opening single or multiple folders. Implementing GListModel and TreeListModel.
    pub struct WorkspaceModel(ObjectSubclass<imp::WorkspaceModel>)
        @implements gio::ListModel;
}

#[derive(Deserialize, Serialize)]
struct MonacoFolder {
    path: String,
}

#[derive(Deserialize, Serialize)]
struct MonacoWorkspace {
    folders: Vec<MonacoFolder>,
}
pub struct WorkspaceOpenMessage {
    metadata: Metadata,
    entry: DirEntry,
    parent_path: PathBuf,
}

impl WorkspaceModel {
    pub fn new() -> Self {
        glib::Object::new(&[]).unwrap()
    }

    async fn add_folder_internal(path: &PathBuf, tx: Sender<WorkspaceOpenMessage>) {
        match fs::read_dir(path).await {
            Ok(mut dir) => loop {
                match dir.next_entry().await {
                    Ok(entry) => match entry {
                        Some(entry) => {
                            match entry.metadata().await {
                                Ok(metadata) => {
                                    match tx.send(WorkspaceOpenMessage {
                                        metadata: metadata,
                                        entry,
                                        parent_path: path.to_path_buf(),
                                    }) {
                                        Ok(_) => {}
                                        Err(e) => eprintln!("{:#?}", e),
                                    };
                                }
                                Err(e) => eprintln!("{:#?}", e),
                            };
                        }
                        None => {
                            break;
                        }
                    },
                    Err(_) => break,
                }
            },
            Err(err) => eprintln!("{err:#?}"),
        };
    }

    pub fn model(&self) -> TreeListModel {
        self.property::<TreeListModel>("model")
    }
    ///
    /// Read a workspace config file and add the workspace's folders and all their children recursively.
    ///
    /// The function will take the config_path parameter. If you want to use it later on, you should pass a clone of the PathBuf or create another PathBuf with the same path.
    ///
    fn open_folder_attacher(message: WorkspaceOpenMessage, store: &gio::ListStore) {
        let info = FileInfo::new(
            &message.entry.file_name().to_string_lossy().to_string(),
            &String::from(
                message
                    .parent_path
                    .to_str()
                    .expect("Parent path is not unicode"),
            ),
            &message.metadata.is_dir(),
        );

        if store.item_type() == FileInfo::static_type() {
            store.append(&info);
        } else {
            panic!(
                "Store item type is not FileInfo, it's {:?}",
                store.item_type()
            );
        };
    }

    ///
    /// Add a single folder to the workspace
    ///
    pub fn add_folder(&self, path: PathBuf) -> Result<(), Box<dyn Error>> {
        let (tx, rx) = glib::MainContext::channel::<WorkspaceOpenMessage>(glib::PRIORITY_DEFAULT);
        let model = self.model().model();
        let store = model.downcast_ref::<ListStore>().unwrap();
        rx.attach(
            None,
            clone!(@strong tx, @strong store => move |message| {
                Self::open_folder_attacher(message, &store);
                Continue(true)
            }),
        );
        tokio::spawn(async move {
            Self::add_folder_internal(&path, tx).await;
        });
        Ok(())
    }

    ///
    /// Reads a workspace file (.code-workspace format) and add the folders listed in the file to the model.
    ///
    pub fn add_workspace(&self, config_path: PathBuf) -> Result<(), Box<dyn Error>> {
        // glib::MainContext is like std::mpsc, but its receivers don't block the current thread.
        let (tx, rx) = glib::MainContext::channel::<WorkspaceOpenMessage>(glib::PRIORITY_DEFAULT);

        rx.attach(
            None,
            glib::clone!(@strong tx, @strong self as workspace => move |message| {
                Self::open_folder_attacher(message, &workspace.model().model().downcast_ref::<ListStore>().unwrap());
                Continue(true)
            }),
        );
        #[allow(unused_braces)]
        tokio::spawn(clone!(@strong tx =>
        async move {
            let workspace = match fs::read_to_string(&config_path).await {
                Ok(content) => content,
                Err(error) => return Err(error.to_string()),
            };

            let workspace = match serde_json::from_str::<MonacoWorkspace>(&workspace) {
                Ok(workspace) => workspace,
                Err(error) => return Err(error.to_string()),
            };

            for folder in workspace.folders {
                let path = RelativePath::new(&folder.path).to_path(&config_path.parent().expect("Can't retrieve the parent of the workspace configuration. Seems that the configuration is in the root (in Unix/MacOS) or the drive (Windows)."));

                Self::add_folder_internal(&path, tx.clone()).await;
            }

            Ok(())

        }));

        Ok(())
    }
}
