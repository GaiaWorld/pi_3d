use bevy::{prelude::{ResMut, Query}, ecs::system::Res};

use crate::materials::prelude::{SingleIDBaseDefaultMaterial, ActionListMaterialUse, OpsMaterialUse};

use super::{command::{ActionListMeshBounding, OpsMeshBounding, ActionListMeshBoundingCullingMode, OpsMeshBoundingCullingMode, ActionListBoundingBoxDisplay, OpsBoundingBoxDisplay}, base::{GeometryBounding, GeometryCullingMode, BoundingBoxDisplay}};


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

pub fn sys_act_mesh_bounding_culling(
    mut cmds: ResMut<ActionListMeshBoundingCullingMode>,
    mut items: Query<&mut GeometryCullingMode>,
) {
    cmds.drain().drain(..).for_each(|OpsMeshBoundingCullingMode(entity, mode)| {
        if let Ok(mut item) = items.get_mut(entity) {
            item.0 = mode;
        // } else if count < 2 {
        //     cmds.push(OpsMeshBoundingCullingMode(entity, mode, count + 1))
        }
    });
}

pub fn sys_act_mesh_bounding_culling_display(
    mut cmds: ResMut<ActionListBoundingBoxDisplay>,
    mut scenes: Query<&mut BoundingBoxDisplay>,
    mut matuse: ResMut<ActionListMaterialUse>,
    deafultmat: Res<SingleIDBaseDefaultMaterial>,
) {
    cmds.drain().drain(..).for_each(|OpsBoundingBoxDisplay(entity, mode, pass)| {
        if let Ok(mut item) = scenes.get_mut(entity) {
            item.display = mode;
            if mode {
                matuse.push(OpsMaterialUse::ops(item.mesh, deafultmat.0, pass));
            }
        }
    });
}