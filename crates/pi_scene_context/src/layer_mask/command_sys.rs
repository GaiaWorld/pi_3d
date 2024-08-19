
use pi_scene_shell::prelude::*;

use crate::prelude::FlagAbstructMeshForView;

use super::{base::*, command::*};

pub fn sys_act_layer_mask(
    mut cmds: ResMut<ActionListLayerMask>,
    mut nodes: Query<&mut LayerMask>,
    mut meshes: Query<&mut FlagAbstructMeshForView>,
) {
    cmds.drain().for_each(|OpsLayerMask(entity, layermask)| {
        if let Ok(mut node) = nodes.get_mut(entity) {
            if *node != layermask {
                *node = layermask;
                if let Ok(mut flag) = meshes.get_mut(entity) {
                    *flag = FlagAbstructMeshForView;
                }
            }
        // } else {
        //     cmds.push(OpsLayerMask(entity, layermask));
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
