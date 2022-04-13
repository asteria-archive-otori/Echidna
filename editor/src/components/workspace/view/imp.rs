use super::super::model::WorkspaceModel;
use crate::{components::workspace::model::file_info::FileInfo, prelude::*};
use gtk::{
    glib::{ParamFlags, ParamSpec, ParamSpecObject, Value},
    CompositeTemplate,
};
use once_cell::sync::Lazy;

#[derive(Debug, Default, CompositeTemplate)]
#[template(resource = "/io/fortressia/Echidna/workspace.ui")]
pub struct WorkspaceView {
    #[template_child]
    view: TemplateChild<gtk::ListView>,
    #[template_child]
    model: TemplateChild<WorkspaceModel>,
}

#[glib::object_subclass]
impl ObjectSubclass for WorkspaceView {
    const NAME: &'static str = "EchWorkspaceView";
    type Type = super::WorkspaceView;
    type ParentType = adw::Bin;

    fn class_init(klass: &mut Self::Class) {
        WorkspaceModel::ensure_type();

        Self::bind_template(klass);
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ObjectImpl for WorkspaceView {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![ParamSpecObject::new(
                "workspace-model",
                "workspace-model",
                "the workspace workspace_model of the view",
                WorkspaceModel::static_type(),
                ParamFlags::READABLE,
            )]
        });

        PROPERTIES.as_ref()
    }

    fn property(&self, _obj: &Self::Type, _id: usize, spec: &ParamSpec) -> Value {
        match spec.name() {
            "workspace-model" => self.model.to_value(),

            _ => unimplemented!(),
        }
    }
}

impl WidgetImpl for WorkspaceView {}

impl BinImpl for WorkspaceView {}
