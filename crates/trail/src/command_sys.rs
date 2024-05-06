use std::{hash::Hasher, sync::Arc};

use pi_scene_shell::prelude::*;
use pi_scene_context::{geometry::instance::{types::InstanceAttributeAnimated, DirtyInstanceSourceForSingleBuffer}, prelude::*, transforms::command_sys::ActionTransformNodeBundle};
use pi_scene_math::Vector4;

use crate::{base::*, command::*, ResTrailBuffer};

pub type TrailMeshBundle = (
    SceneID,
    TrailLinkedTransform,
    // TrailMesh,
    TrailGeometry,
    TrailBase,
    TrailWorldPlace,
    TrailPoints,
    ColorOverTrail,
    TrailMinimunVertexDistance,
    WidthOverTrail,
    TrailAgeControl,
    TrailSize,
    TrailColor,
    TrailRandom,
);

pub fn sys_create_trail_mesh(
    mut cmds: ResMut<ActionListTrail>,
    // mut commands: Commands,
    mut insert: Insert<()>,
    mut alter1: Alter<(), (), (DisposeReady, DisposeCan),>,
    mut alter2: Alter<(), (), (SceneID,)>,
    mut alter3: Alter<(), (), (Down, Up, Layer, Enable, RecordEnable, GlobalEnable)>,
    mut alter4: Alter<(), (), ActionTransformNodeBundle>,
    mut alter6: Alter<(), (), (InstanceSourceRefs, DirtyInstanceSourceRefs, DirtyInstanceSourceForSingleBuffer)>,
    mut alter7: Alter<(), (), (TargetAnimatorableIsRunning, InstanceAttributeAnimated)>,
    mut alter8: Alter<(), (), (BindModel, ModelStatic)>,
    mut alter9: Alter<(), (), (BindModel,)>,
    mut alter10: Alter<(), (), ActionMeshInitBundle>,
    mut alter11: Alter<(), (), (DisposeReady, DisposeCan)>,
    mut alter12: Alter<(), (), ActionPassObjectInitBundle>, 
    mut alter13: Alter<(), (), (PassTag,), ()>,
    mut alter14: Alter<(), (), ActionMeshBundle>,
    mut alter15: Alter<(), (), (PassIDs,), ()>,
    mut alter16: Alter<(), (), (GeometryID, BindModel, ModelStatic), ()>,
    mut alter17: Alter<(), (), (DisposeReady, DisposeCan)>,
    mut alter18: Alter<(), (), (VertexBufferLayoutsComp, MeshID, RenderGeometryComp)>,
    mut alter19: Alter<(), (), (IndicesBufferDesc,)>,
    mut alter20: Alter<(), (), (), (IndicesBufferDesc,)>,
    mut alter21: Alter<(), (), (GeometryResourceHash, GeometryDesc, AssetDescVBSlot01, AssetResVBSlot01), ()>,
    mut alter22: Alter<(), (), (DisposeReady, DisposeCan), ()>,
    mut alter23: Alter<(), (), TrailMeshBundle, ()>,
    trailbuffer: Res<ResTrailBuffer>,
    mut allocator: ResMut<ResBindBufferAllocator>,
    empty: Res<SingleEmptyEntity>,
    // mut matuse: ResMut<ActionListMaterialUse>,
    lightlimit: Res<ModelLightLimit>,
    commonbindmodel: Res<CommonBindModel>,
    mut meshprimitivestate: ResMut<ActionListPrimitiveState>,
) {
    if let Some(trailbuffer) = &trailbuffer.0 {

        cmds.drain().drain(..).for_each(|OpsTrail(id_scene, id_linked, entity)| {

            let id_mesh = entity;
            let id_geo = insert.insert(()); //commands.spawn_empty().id();

            // matuse.push(OpsMaterialUse::ops(id_mesh, id_mat));

            // meshcreate.push(OpsMeshCreation::ops(id_scene, id_mesh, String::from("")));
            ActionMesh::init(&mut insert, &mut alter1, &mut alter2, &mut alter3, &mut alter4, &mut alter6, &mut alter7, &mut alter8, &mut alter9, &mut alter10, &mut alter11, &mut alter12, &mut alter13, &mut alter14, &mut alter15, id_mesh, id_scene, &mut allocator, &empty, MeshInstanceState::default(), &lightlimit.0, &commonbindmodel);
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

            if  alter16.get(id_mesh).is_ok() {
                // log::warn!("Mesh Ok");
                // meshtopology.push(OpsTopology::ops(id_mesh, PrimitiveTopology::TriangleStrip));
                // cmd.insert(Topology(PrimitiveTopology::TriangleStrip));
                // cmd.insert(CCullMode(CullMode::Off));
                alter16.alter(id_mesh, (GeometryID(id_geo), commonbindmodel.0.clone(), ModelStatic));
                // 显式重置为默认
                // cmd.insert(commonbindmodel.0.clone());
                // cmd.insert(ModelStatic);
            }

            if alter17.get(id_geo).is_ok() {
                // log::warn!("Geometry Ok");
                let vertex_desc = vec![trailbuffer.buffer_desc()];
                ActionGeometry::init(id_geo, &mut alter17, &mut alter18, &mut alter19, &mut alter20, &vertex_desc, None, id_mesh);

                // let mut verticescode = EVerticeExtendCodeComp::default();
                // verticescode.0.0 += EVerticeExtendCode::TRIAL;
                let slot = AssetDescVBSlot01::from(vertex_desc[0].clone());
                let geo_desc = GeometryDesc { list: vertex_desc };
                let buffer = AssetResVBSlot01::from(EVerticesBufferUsage::EVBRange(Arc::new(EVertexBufferRange::NotUpdatable(trailbuffer.buffer(), 0, 0))));
    
                let mut hasher = DefaultHasher::default();
                geo_desc.hash_resource(&mut hasher);
                // cmd.insert(GeometryResourceHash(hasher.finish()));

                // cmd
                //     .insert(geo_desc)
                //     .insert(slot)
                //     .insert(buffer)
                //     // .insert(verticescode)
                //     ;
                    alter21.alter(id_geo, (GeometryResourceHash(hasher.finish()), geo_desc, slot, buffer));
            }
            
            if alter22.get(entity).is_ok() {
                ActionEntity::init(entity, &mut alter22);
                alter23.alter(entity, 
                    (SceneID(id_scene),
                    TrailLinkedTransform(id_linked),
                    // TrailMesh(id_mesh),
                    TrailGeometry(id_geo),
                    TrailBase::new(u32::MAX),
                    TrailWorldPlace(true),
                    TrailPoints::default(),
                    ColorOverTrail(Color4Gradient::default()),
                    TrailMinimunVertexDistance(0.01),
                    WidthOverTrail(FloatInterpolation::new(1.)),
                    TrailAgeControl(200),
                    TrailSize(1.),
                    TrailColor(Vector4::new(1., 1., 1., 1.)),
                    TrailRandom(pi_wy_rng::WyRng::default()),
                ));
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