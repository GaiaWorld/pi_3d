use pi_engine_shell::prelude::*;

use super::base::*;


pub struct OpsLayerMask(pub(crate) Entity, pub(crate) LayerMask);
impl OpsLayerMask {
    pub fn ops(transformnode: Entity, mask: u32) -> Self {
        Self(transformnode, LayerMask(mask))
    }
}
pub type ActionListLayerMask = ActionList<OpsLayerMask>;