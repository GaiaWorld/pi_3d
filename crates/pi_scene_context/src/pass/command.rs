use pi_engine_shell::prelude::*;



pub struct OpsPassObject(pub(crate) Entity, pub(crate) Entity, pub(crate) PassTag);
impl OpsPassObject {
    pub fn ops(model: Entity, material: Entity, pass: PassTag) -> Self {
        Self(model, material, pass)
    }
}
pub type ActionListPassObject = ActionList<OpsPassObject>;