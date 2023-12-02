use std::sync::Arc;

use pi_engine_shell::prelude::*;
use pi_scene_context::prelude::*;
use pi_scene_math::Vector4;

use crate::{base::*, command::*, ResTrailBuffer};


pub fn sys_create_trail_mesh(
    mut cmds: ResMut<ActionListTrail>,
    mut commands: Commands,
    trailbuffer: Res<ResTrailBuffer>,
    mut allocator: ResMut<ResBindBufferAllocator>,
    empty: Res<SingleEmptyEntity>,
    // mut matuse: ResMut<ActionListMaterialUse>,
    lightlimit: Res<ModelLightLimit>,
) {
    if let Some(trailbuffer) = &trailbuffer.0 {
        cmds.drain().drain(..).for_each(|OpsTrail(id_scene, id_linked, entity)| {

            let id_mesh = entity;
            let id_geo = commands.spawn_empty().id();

            // matuse.push(OpsMaterialUse::ops(id_mesh, id_mat));

            // meshcreate.push(OpsMeshCreation::ops(id_scene, id_mesh, String::from("")));
            ActionMesh::init(&mut commands, id_mesh, id_scene, &mut allocator, &empty, MeshInstanceState::default(), &lightlimit.0);

            if let Some(mut cmd) = commands.get_entity(id_mesh) {
                // log::warn!("Mesh Ok");
                // meshtopology.push(OpsTopology::ops(id_mesh, PrimitiveTopology::TriangleStrip));
                cmd.insert(Topology(PrimitiveTopology::TriangleStrip));
                cmd.insert(CCullMode(CullMode::Off));
                cmd.insert(GeometryID(id_geo));
            }

            if let Some(mut cmd) = commands.get_entity(id_geo) {
                // log::warn!("Geometry Ok");
                let vertex_desc = vec![trailbuffer.buffer_desc()];
                ActionGeometry::init(&mut cmd, &vertex_desc, None, id_mesh);

                let mut verticescode = EVerticeExtendCodeComp::default();
                verticescode.0.0 += EVerticeExtendCode::TRIAL;
                let slot = AssetDescVBSlot01::from(vertex_desc[0].clone());
                let geo_desc = GeometryDesc { list: vertex_desc };
                let buffer = AssetResVBSlot01::from(EVerticesBufferUsage::EVBRange(Arc::new(EVertexBufferRange::NotUpdatable(trailbuffer.buffer(), 0, 0))));
    
                cmd
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
                    // .insert(TrailMesh(id_mesh))
                    .insert(TrailGeometry(id_geo))
                    .insert(TrailBase::new(u32::MAX))
                    .insert(TrailWorldPlace(true))
                    .insert(TrailPoints::default())
                    .insert(ColorOverTrail(Color4Gradient::default()))
                    .insert(TrailMinimunVertexDistance(0.01))
                    .insert(WidthOverTrail(FloatInterpolation::new(1.)))
                    .insert(TrailAgeControl(200))
                    .insert(TrailSize(1.))
                    .insert(TrailColor(Vector4::new(1., 1., 1., 1.)))
                    .insert(TrailRandom(pi_wy_rng::WyRng::default()))
                    ;
            }
        });
    }
}

pub fn sys_act_trail_age(
    mut cmds: ResMut<ActionListTrailAge>,
    mut items: Query<&mut TrailAgeControl>,
) {
    cmds.drain().drain(..).for_each(|OpsTrailAgeControl(entity, ms, count)| {
        if let Ok(mut item) = items.get_mut(entity) {
            *item = TrailAgeControl(ms);
        } else if count < 2 {
            cmds.push(OpsTrailAgeControl(entity, ms, count+1))
        }
    });
}

pub fn act_update_trail_geometry_buffer(
    id_geo: Entity,
    items: &mut Query<&mut AssetResVBSlot01>,
    data: (Arc<NotUpdatableBufferRange>, u32, u32),
) {
    if let Ok(mut buffer) = items.get_mut(id_geo) {
        *buffer = AssetResVBSlot01::from(EVerticesBufferUsage::EVBRange(Arc::new(EVertexBufferRange::NotUpdatable(data.0, data.1, data.2))));
    }
}