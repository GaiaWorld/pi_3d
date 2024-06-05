use std::{hash::Hasher, sync::Arc};

use pi_scene_shell::prelude::*;
use pi_scene_context::{geometry::instance::instanced_buffer::InstancedInfoComp, prelude::*};
use pi_scene_math::Vector4;

use crate::{base::*, command::*, ResTrailBuffer};

pub type BundleTrail = ((DisposeReady, DisposeCan), SceneID, TrailParam, TrailGeometry, TrailBase, TrailPoints, TrailRandom);

pub fn sys_create_trail_mesh(
    mut cmds: ResMut<ActionListTrail>,
    mut commands: Commands,
    trailbuffer: Res<ResTrailBuffer>,
    mut allocator: ResMut<ResBindBufferAllocator>,
    empty: Res<SingleEmptyEntity>,
    // mut matuse: ResMut<ActionListMaterialUse>,
    lightlimit: Res<ModelLightLimit>,
    commonbindmodel: Res<CommonBindModel>,
    mut meshprimitivestate: ResMut<ActionListPrimitiveState>,
    // mut altermodel: Alter<(), (), BundleModel, ()>,
    // mut altergeo: Alter<(), (), BundleGeometry, ()>,
    // mut altertrail: Alter<(), (), BundleTrail, ()>,
) {
    if let Some(trailbuffer) = &trailbuffer.0 {

        cmds.drain().drain(..).for_each(|OpsTrail(id_scene, id_linked, entity)| {

            let id_mesh = entity;
            let id_geo = commands.spawn_empty().id();

            // matuse.push(OpsMaterialUse::ops(id_mesh, id_mat));

            // meshcreate.push(OpsMeshCreation::ops(id_scene, id_mesh, String::from("")));
            ActionMesh::init(&mut commands, id_mesh, id_scene, &mut allocator, &empty, MeshInstanceState::default(), &lightlimit.0, &commonbindmodel);
            meshprimitivestate.push(OpsPrimitiveState::ops(id_mesh, PassTag::PASS_TAG_01, EPrimitiveState::Topology(PrimitiveTopology::TriangleStrip)));
            meshprimitivestate.push(OpsPrimitiveState::ops(id_mesh, PassTag::PASS_TAG_02, EPrimitiveState::Topology(PrimitiveTopology::TriangleStrip)));
            meshprimitivestate.push(OpsPrimitiveState::ops(id_mesh, PassTag::PASS_TAG_03, EPrimitiveState::Topology(PrimitiveTopology::TriangleStrip)));
            meshprimitivestate.push(OpsPrimitiveState::ops(id_mesh, PassTag::PASS_TAG_04, EPrimitiveState::Topology(PrimitiveTopology::TriangleStrip)));
            meshprimitivestate.push(OpsPrimitiveState::ops(id_mesh, PassTag::PASS_TAG_05, EPrimitiveState::Topology(PrimitiveTopology::TriangleStrip)));
            meshprimitivestate.push(OpsPrimitiveState::ops(id_mesh, PassTag::PASS_TAG_06, EPrimitiveState::Topology(PrimitiveTopology::TriangleStrip)));
            meshprimitivestate.push(OpsPrimitiveState::ops(id_mesh, PassTag::PASS_TAG_07, EPrimitiveState::Topology(PrimitiveTopology::TriangleStrip)));
            meshprimitivestate.push(OpsPrimitiveState::ops(id_mesh, PassTag::PASS_TAG_08, EPrimitiveState::Topology(PrimitiveTopology::TriangleStrip)));
            meshprimitivestate.push(OpsPrimitiveState::ops(id_mesh, PassTag::PASS_TAG_01, EPrimitiveState::CCullMode(CullMode::Off)));
            meshprimitivestate.push(OpsPrimitiveState::ops(id_mesh, PassTag::PASS_TAG_02, EPrimitiveState::CCullMode(CullMode::Off)));
            meshprimitivestate.push(OpsPrimitiveState::ops(id_mesh, PassTag::PASS_TAG_03, EPrimitiveState::CCullMode(CullMode::Off)));
            meshprimitivestate.push(OpsPrimitiveState::ops(id_mesh, PassTag::PASS_TAG_04, EPrimitiveState::CCullMode(CullMode::Off)));
            meshprimitivestate.push(OpsPrimitiveState::ops(id_mesh, PassTag::PASS_TAG_05, EPrimitiveState::CCullMode(CullMode::Off)));
            meshprimitivestate.push(OpsPrimitiveState::ops(id_mesh, PassTag::PASS_TAG_06, EPrimitiveState::CCullMode(CullMode::Off)));
            meshprimitivestate.push(OpsPrimitiveState::ops(id_mesh, PassTag::PASS_TAG_07, EPrimitiveState::CCullMode(CullMode::Off)));
            meshprimitivestate.push(OpsPrimitiveState::ops(id_mesh, PassTag::PASS_TAG_08, EPrimitiveState::CCullMode(CullMode::Off)));

            if let Some(mut cmd) = commands.get_entity(id_mesh) {
                // log::warn!("Mesh Ok");
                cmd.insert((
                    GeometryID(id_geo),
                    // 显式重置为默认
                    commonbindmodel.0.clone(),
                    ModelStatic,
                ));
            }

            if let Some(mut geocommands) = commands.get_entity(id_geo) {
                // log::warn!("Geometry Ok");
                let vertex_desc = vec![trailbuffer.buffer_desc()];
                
                let (comp1, comp2, comp3, comp4, comp5, comp6) = ActionGeometry::init(&vertex_desc, None, id_mesh);

                let mut desclist = AssetDescVBSlots::default();
                let mut keyslist = LoadedKeyVBSlots::default();
                let mut datalist = AssetResVBSlots::default();
                // let mut verticescode = EVerticeExtendCodeComp::default();
                // verticescode.0.0 += EVerticeExtendCode::TRIAL;
                let slot = AssetDescVBSlot::from(vertex_desc[0].clone());
                let geo_desc = GeometryDesc { list: vertex_desc };
                let buffer = AssetResVBSlot::from(EVerticesBufferUsage::EVBRange(Arc::new(EVertexBufferRange::NotUpdatable(trailbuffer.buffer(), 0, 0))));
                keyslist[0] = Some(slot.key().clone());
                desclist[0] = Some(slot);
                datalist[0] = Some(buffer);
    
                let mut hasher = DefaultHasher::default();
                geo_desc.hash_resource(&mut hasher);
                let bundle = (
                    comp1,
                    geo_desc,
                    (comp2, comp3, comp4, comp5, comp6, desclist, keyslist, datalist),
                    AssetResBufferIndicesComp(None),
                    InstancedInfoComp(None),
                    GeometryResourceHash(hasher.finish()),
                );
                geocommands.insert( bundle );
                // altergeo.alter(id_geo, bundle);
            }
            
            if let Some(mut cmd) = commands.get_entity(entity) {
                let bundle: BundleTrail = (
                    ActionEntity::init(),
                    SceneID(id_scene),
                    TrailParam {
                        size: 1.,
                        color: Vector4::new(1., 1., 1., 1.),
                        age_control: 200,
                        linked: id_linked,
                        world_place: true,
                        minimun_vertex_distance: 0.01,
                        wildth_over_trail: FloatInterpolation::new(1.),
                        color_over_trail: Color4Gradient::default(),
                    },
                    TrailGeometry(id_geo),
                    TrailBase::new(u32::MAX),
                    TrailPoints::default(),
                    TrailRandom(pi_wy_rng::WyRng::default()),
                );
                cmd.insert(bundle);
                // altertrail.alter(entity, bundle);
            }
        });
    }
}

pub fn sys_act_trail_age(
    mut cmds: ResMut<ActionListTrailAge>,
    mut items: Query<&mut TrailParam>,
) {
    cmds.drain().drain(..).for_each(|OpsTrailAgeControl(entity, ms, count)| {
        if let Ok(mut item) = items.get_mut(entity) {
            item.age_control = ms;
        } else if count < 2 {
            cmds.push(OpsTrailAgeControl(entity, ms, count+1))
        }
    });
}

pub fn act_update_trail_geometry_buffer(
    id_geo: Entity,
    items: &mut Query<&mut AssetResVBSlots>,
    data: (Arc<NotUpdatableBufferRange>, u32, u32),
) {
    if let Ok(mut buffer) = items.get_mut(id_geo) {
        buffer.0[0] = Some(AssetResVBSlot::from(EVerticesBufferUsage::EVBRange(Arc::new(EVertexBufferRange::NotUpdatable(data.0, data.1, data.2)))));
    }
}