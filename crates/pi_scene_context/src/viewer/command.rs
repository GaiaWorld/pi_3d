

use bevy::ecs::entity::Entity;
use pi_scene_shell::prelude::ActionList;

use crate::renderers::prelude::*;

use super::base::*;

pub struct OpsViewerForceInclude(pub(crate) Entity, pub(crate) Entity, pub(crate) bool);
impl OpsViewerForceInclude {
    pub fn ops(viewer: Entity, model: Entity, doadd: bool) -> Self {
        Self(viewer, model, doadd)
    }
}
pub type ActionListViewerForceInclude = ActionList<OpsViewerForceInclude>;

pub struct BundleViewer(
    ViewerViewMatrix,
    ViewerProjectionMatrix,
    ViewerTransformMatrix,
    ViewerGlobalPosition,
    ViewerDirection,
    // ModelList,
    // FlagModelList,
    ModelListAfterCulling,
    // ViewerRenderersInfo,
    DirtyViewerRenderersInfo,
);
