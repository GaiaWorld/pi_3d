
use pi_engine_shell::prelude::*;

use super::{
    ScenePassRenderCfg,
};

pub struct OpsSceneCreation(pub(crate) Entity, pub(crate) ScenePassRenderCfg);
impl OpsSceneCreation {
    pub fn ops(scene: Entity, passes_cfg: ScenePassRenderCfg) -> Self {
        Self(scene, passes_cfg)
    }
}
pub type ActionListSceneCreate = ActionList<OpsSceneCreation>;
