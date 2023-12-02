use bevy::prelude::{ResMut, Query};

use super::{command::{ActionListMeshBounding, OpsMeshBounding, ActionListMeshBoundingCullingMode, OpsMeshBoundingCullingMode}, base::{GeometryBounding, GeometryCullingMode}};


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