use std::sync::Arc;

use pi_engine_shell::prelude::*;
use pi_scene_context::prelude::*;

use crate::{base::*, command::*, ResTrailBuffer};


pub fn sys_act_trail_mesh_geometry(
    mut cmds: ResMut<ActionListTrial>,
    mut commands: Commands,
    trailbuffer: Res<ResTrailBuffer>,
    mut tree: ResMut<ActionListTransformNodeParent>,
    mut allocator: ResMut<ResBindBufferAllocator>,
    device: Res<PiRenderDevice>,
    empty: Res<SingleEmptyEntity>,
    mut matuse: ResMut<ActionListMaterialUse>,
    mut meshcreate: ResMut<ActionListMeshCreate>,
    mut meshtopology: ResMut<ActionListTopology>,
) {
    if let Some(trailbuffer) = &trailbuffer.0 {
        cmds.drain().drain(..).for_each(|OpsTrail(id_scene, id_linked, id_mat, entity)| {

            let id_mesh = commands.spawn_empty().id();
            let id_geo = commands.spawn_empty().id();

            matuse.push(OpsMaterialUse::ops(id_mesh, id_mat));

            // meshcreate.push(OpsMeshCreation::ops(id_scene, id_mesh, String::from("")));
            ActionMesh::init(&mut commands, id_mesh, id_scene, &mut tree, &mut allocator, &device, &empty);

            if let Some(mut cmd) = commands.get_entity(id_mesh) {
                log::warn!("Mesh Ok");
                // meshtopology.push(OpsTopology::ops(id_mesh, PrimitiveTopology::TriangleStrip));
                cmd.insert(Topology(PrimitiveTopology::TriangleStrip));
                cmd.insert(GeometryID(id_geo));
            }

            if let Some(mut cmd) = commands.get_entity(id_geo) {
                log::warn!("Geometry Ok");
                let mut verticescode = EVerticeExtendCodeComp(EVerticeExtendCode(EVerticeExtendCode::NONE));
                verticescode.0.0 += EVerticeExtendCode::TRIAL;
                let vertex_desc = vec![trailbuffer.buffer_desc()];
                let vblayout = VertexBufferLayoutsComp(VertexBufferLayouts::from(&vertex_desc));
                let slot = AssetDescVBSlot01::from(vertex_desc[0].clone());
                let geo_desc = GeometryDesc { list: vertex_desc };
                let buffer = AssetResVBSlot01::from(EVerticesBufferUsage::EVBRange(Arc::new(EVertexBufferRange::NotUpdatable(trailbuffer.buffer(), 0, 0))));
    
                ActionEntity::init(&mut cmd);
                cmd
                    .insert(MeshID(id_mesh))
                    .insert(vblayout)
                    .insert(geo_desc)
                    .insert(slot)
                    .insert(buffer)
                    .insert(verticescode)
                    ;
            }
            
            if let Some(mut cmd) = commands.get_entity(entity) {
                ActionEntity::init(&mut cmd);
                cmd
                    .insert(SceneID(id_scene))
                    .insert(TrailLinkedTransform(id_linked))
                    .insert(TrailMesh(id_geo))
                    .insert(TrailGeometry(id_mesh))
                    .insert(TrailBase::new(u32::MAX))
                    .insert(TrailWorldPlace(false))
                    .insert(TrailPoints::default())
                    .insert(ColorOverTrail(Color4Gradient::default()))
                    .insert(TrailMinimunVertexDistance(0.01))
                    .insert(WidthOverTrail(FloatInterpolation::new(1.)))
                    .insert(TrailAgeControl(1000))
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