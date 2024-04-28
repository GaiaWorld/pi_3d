use pi_scene_shell::prelude::{ResMut, Query, Res};

use crate::materials::prelude::{SingleIDBaseDefaultMaterial, ActionListMaterialUse, OpsMaterialUse};

use super::{command::{ActionListMeshBounding, OpsMeshBounding, ActionListBoundingBoxDisplay, OpsBoundingBoxDisplay}, base::{GeometryBounding, BoundingBoxDisplay}};


pub fn sys_act_mesh_bounding(
    mut cmds: ResMut<ActionListMeshBounding>,
    mut items: Query<&mut GeometryBounding>,
) {
    cmds.drain().drain(..).for_each(|OpsMeshBounding(entity, min, max)| {
        if let Ok(mut item) = items.get_mut(entity) {
            item.minimum.copy_from(&min);
            item.maximum.copy_from(&max);
        // } else if count < 2 {
        //     cmds.push(OpsMeshBounding(entity, min, max, count + 1))
        }
    });
}

pub fn sys_act_mesh_bounding_culling_display(
    mut display_cmds: ResMut<ActionListBoundingBoxDisplay>,
    mut display_scenes: Query<&mut BoundingBoxDisplay>,
    mut display_matuse: ResMut<ActionListMaterialUse>,
    deafultmat: Res<SingleIDBaseDefaultMaterial>,
) {
    display_cmds.drain().drain(..).for_each(|OpsBoundingBoxDisplay(entity, mode, pass)| {
        if let Ok(mut item) = display_scenes.get_mut(entity) {
            item.display = mode;
            if mode {
                display_matuse.push(OpsMaterialUse::ops(item.mesh, deafultmat.0, pass));
            }
        }
    });
}