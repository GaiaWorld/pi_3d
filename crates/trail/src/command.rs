use pi_engine_shell::prelude::*;


pub struct OpsTrail(pub(crate) Entity, pub(crate) Entity, pub(crate) Entity);
impl OpsTrail {
    pub fn ops(id_scene: Entity, id_linked_transform: Entity, entity: Entity) -> Self {
        Self(id_scene, id_linked_transform, entity)
    }
}
pub type ActionListTrail = ActionList<OpsTrail>;

pub struct OpsTrailAgeControl(pub(crate) Entity, pub(crate) u32, pub(crate) u8);
impl OpsTrailAgeControl {
    pub fn ops(entity: Entity, ms: u32) -> Self {
        Self(entity, ms, 0)
    }
}
pub type ActionListTrailAge = ActionList<OpsTrailAgeControl>;
