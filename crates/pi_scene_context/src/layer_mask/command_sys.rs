
use pi_scene_shell::prelude::*;

use crate::prelude::FlagMeshNeedRecheckForView;

use super::{base::*, command::*};

pub fn sys_act_layer_mask(
    mut cmds: ResMut<ActionListLayerMask>,
    mut nodes: Query<&mut LayerMask>,
    mut meshes: Query<&mut FlagMeshNeedRecheckForView>,
    // mut performance: ResMut<Performance>,
) {
    // performance.systems.push(String::from("sys_act_layer_mask"));
    cmds.drain().for_each(|OpsLayerMask(entity, layermask)| {
        if let Ok(mut node) = nodes.get_mut(entity) {
            if *node != layermask {
                *node = layermask;
                if let Ok(mut flag) = meshes.get_mut(entity) {
                    *flag = FlagMeshNeedRecheckForView;
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
        let cmds = app.world.get_resource_mut::<ActionListLayerMask>().unwrap();
        cmds.push(OpsLayerMask(entity, val));
    }
}
