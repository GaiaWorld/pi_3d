use pi_engine_shell::prelude::*;


pub struct OpsTrailMeshGeometry(pub(crate) Entity, pub(crate) Entity, pub(crate) Entity, pub(crate) u8);
impl OpsTrailMeshGeometry {
    pub fn ops(id_scene: Entity, id_mesh: Entity, geo: Entity) -> Self {
        Self(id_scene, id_mesh, geo, 0)
    }
}
pub type ActionListTrialMeshGeometry = ActionList<OpsTrailMeshGeometry>;