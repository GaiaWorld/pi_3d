
use pi_scene_shell::prelude::{ActionList, Entity};

pub struct OpsViewerForceInclude(pub(crate) Entity, pub(crate) Entity, pub(crate) bool);
impl OpsViewerForceInclude {
    pub fn ops(viewer: Entity, model: Entity, doadd: bool) -> Self {
        Self(viewer, model, doadd)
    }
}
pub type ActionListViewerForceInclude = ActionList<OpsViewerForceInclude>;
