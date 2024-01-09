
use pi_engine_shell::prelude::*;
use pi_scene_math::{Vector3, Number};

use super::base::ECullingStrategy;

pub struct OpsMeshBounding(pub(crate) Entity, pub(crate) Vector3, pub(crate) Vector3);
impl OpsMeshBounding {
    pub fn ops(entity: Entity, min: (Number, Number, Number), max: (Number, Number, Number)) -> Self {
        Self(entity, Vector3::new(min.0, min.1, min.2), Vector3::new(max.0, max.1, max.2))
    }
}
pub type ActionListMeshBounding = ActionList<OpsMeshBounding>;


pub struct OpsMeshBoundingCullingMode(pub(crate) Entity, pub(crate) ECullingStrategy);
impl OpsMeshBoundingCullingMode {
    pub fn ops(entity: Entity, mode: ECullingStrategy) -> Self {
        Self(entity, mode)
    }
}
pub type ActionListMeshBoundingCullingMode = ActionList<OpsMeshBoundingCullingMode>;

pub struct OpsBoundingBoxDisplay(pub(crate) Entity, pub(crate) bool, pub(crate) PassTag);
impl OpsBoundingBoxDisplay {
    pub fn ops(scene: Entity, mode: bool, pass: PassTag) -> Self {
        Self(scene, mode, pass)
    }
}
pub type ActionListBoundingBoxDisplay = ActionList<OpsBoundingBoxDisplay>;

