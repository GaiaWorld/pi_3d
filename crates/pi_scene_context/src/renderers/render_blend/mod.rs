
use pi_scene_shell::prelude::*;
pub use pi_scene_shell::prelude::ModelBlend;

#[derive(Clone, Copy)]
pub enum OpsRenderBlend {
    Disable(Entity),
    Blend(Entity, PassTag, ModelBlend),
}
impl OpsRenderBlend {
    pub fn ops(mesh: Entity, pass: PassTag, mode: ModelBlend) -> Self {
        Self::Blend(mesh, pass, mode)
    }
}

pub type ActionListBlend = ActionList<OpsRenderBlend>;
