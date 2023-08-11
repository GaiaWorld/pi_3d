use pi_engine_shell::prelude::*;


pub struct OpsTrail(pub(crate) Entity, pub(crate) Entity, pub(crate) Entity, pub(crate) Entity);
impl OpsTrail {
    pub fn ops(id_scene: Entity, id_linked_transform: Entity, id_material: Entity, entity: Entity) -> Self {
        Self(id_scene, id_linked_transform, id_material, entity)
    }
}
pub type ActionListTrial = ActionList<OpsTrail>;

#[derive(SystemParam)]
pub struct ActionSetTrail<'w> {
    pub create: ResMut<'w, ActionListTrial>,
}