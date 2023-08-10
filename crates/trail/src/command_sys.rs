use std::sync::Arc;

use pi_engine_shell::prelude::*;
use pi_scene_context::prelude::*;

use crate::{base::TrialBuffer, command::{ActionListTrialMeshGeometry, OpsTrailMeshGeometry}, ResTrialBuffer};


pub fn sys_act_trail_mesh_geometry(
    mut cmds: ResMut<ActionListTrialMeshGeometry>,
    mut commands: Commands,
    trailbuffer: Res<ResTrialBuffer>,
    mut tree: ResMut<ActionListTransformNodeParent>,
    mut allocator: ResMut<ResBindBufferAllocator>,
    device: Res<PiRenderDevice>,
    empty: Res<SingleEmptyEntity>,
) {
    if let Some(trailbuffer) = &trailbuffer.0 {
        cmds.drain().drain(..).for_each(|OpsTrailMeshGeometry(id_scene, id_mesh, entity, count)| {

            ActionMesh::init(&mut commands, entity, id_scene, &mut tree, &mut allocator, &device, &empty);

            if let Some(mut cmd) = commands.get_entity(id_mesh) {
                cmd.insert(GeometryID(entity));
            }
            if let Some(mut cmd) = commands.get_entity(entity) {
                let mut verticescode = EVerticeExtendCodeComp(EVerticeExtendCode(EVerticeExtendCode::NONE));
                verticescode.0.0 += EVerticeExtendCode::TRIAL;
                let vertex_desc = vec![trailbuffer.buffer_desc()];
                let vblayout = VertexBufferLayoutsComp(VertexBufferLayouts::from(&vertex_desc));
                let slot = AssetDescVBSlot01::from(vertex_desc[0].clone());
                let geo_desc = GeometryDesc { list: vertex_desc };
                let buffer = AssetResVBSlot01::from(EVerticesBufferUsage::EVBRange(Arc::new(EVertexBufferRange::NotUpdatable(trailbuffer.buffer(), 0, 0))));
    
                cmd
                    .insert(vblayout)
                    .insert(geo_desc)
                    .insert(slot)
                    .insert(buffer)
                    .insert(verticescode)
                    ;
            }
        });
    }
}

pub fn act_update_trial_geometry_buffer(
    id_geo: Entity,
    items: &mut Query<&mut AssetResVBSlot01>,
    data: (Arc<NotUpdatableBufferRange>, u32, u32),
) {
    if let Ok(mut buffer) = items.get_mut(id_geo) {
        *buffer = AssetResVBSlot01::from(EVerticesBufferUsage::EVBRange(Arc::new(EVertexBufferRange::NotUpdatable(data.0, data.1, data.2))));
    }
}