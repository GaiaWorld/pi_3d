
use pi_engine_shell::prelude::*;

use super::{base::*, command::*};

pub fn sys_act_layer_mask(
    mut cmds: ResMut<ActionListLayerMask>,
    mut nodes: Query<&mut LayerMask>,
) {
    cmds.drain().drain(..).for_each(|OpsLayerMask(entity, layermask)| {
        if let Ok(mut node) = nodes.get_mut(entity) {
            if *node != layermask {
                *node = layermask;
            }
        } else {
            cmds.push(OpsLayerMask(entity, layermask));
        }
    });
}


pub struct ActionLayerMask;
impl ActionLayerMask {
    pub fn modify(
        app: &mut App,
        entity: Entity,
        val: LayerMask,
    ) {
        let mut cmds = app.world.get_resource_mut::<ActionListLayerMask>().unwrap();
        cmds.push(OpsLayerMask(entity, val));
    }
}
