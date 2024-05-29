use std::{hash::Hasher, sync::Arc};

use pi_scene_shell::prelude::*;
use pi_scene_context::{geometry::instance::{types::InstanceAttributeAnimated, DirtyInstanceSourceForSingleBuffer}, pass::pi_world::editor::EntityEditor, prelude::*, transforms::command_sys::ActionTransformNodeBundle};
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
    mut editor: EntityEditor,
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
            let id_geo = editor.alloc_entity(); //commands.spawn_empty().id();

            // matuse.push(OpsMaterialUse::ops(id_mesh, id_mat));

            // meshcreate.push(OpsMeshCreation::ops(id_scene, id_mesh, String::from("")));
            ActionMesh::init(&mut editor, id_mesh, id_scene, &mut allocator, &empty, MeshInstanceState::default(), &lightlimit.0, &commonbindmodel);
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

            if  editor.contains_entity(id_mesh) {
                // log::warn!("Mesh Ok");
                // meshtopology.push(OpsTopology::ops(id_mesh, PrimitiveTopology::TriangleStrip));
                // cmd.insert(Topology(PrimitiveTopology::TriangleStrip));
                // cmd.insert(CCullMode(CullMode::Off));
                let components = [
                    editor.init_component::<GeometryID>(),
                    editor.init_component::<BindModel>(),
                    editor.init_component::<ModelStatic>(),
                ];
                editor.add_components(entity, &components);
        
                *editor.get_component_unchecked_mut_by_id(entity, components[0]) =GeometryID(id_geo);
                *editor.get_component_unchecked_mut_by_id(entity, components[1]) = commonbindmodel.0.clone();
                *editor.get_component_unchecked_mut_by_id(entity, components[2]) = ModelStatic;
                // alter16.alter(id_mesh, (GeometryID(id_geo), commonbindmodel.0.clone(), ModelStatic));
                // 显式重置为默认
                // cmd.insert(commonbindmodel.0.clone());
                // cmd.insert(ModelStatic);
            }

            if editor.contains_entity(id_geo) {
                // log::warn!("Geometry Ok");
                let vertex_desc = vec![trailbuffer.buffer_desc()];
                ActionGeometry::init(id_geo, &mut editor, &vertex_desc, None, id_mesh);

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
                let components = [
                    editor.init_component::<GeometryResourceHash>(),
                    editor.init_component::<GeometryDesc>(),
                    editor.init_component::<AssetDescVBSlot01>(),
                    editor.init_component::<AssetResVBSlot01>(),
                ];
                let _ = editor.add_components(entity, &components);
        
                *editor.get_component_unchecked_mut_by_id(entity, components[0]) = GeometryResourceHash(hasher.finish());
                *editor.get_component_unchecked_mut_by_id(entity, components[1]) = geo_desc;
                *editor.get_component_unchecked_mut_by_id(entity, components[2]) = slot;
                *editor.get_component_unchecked_mut_by_id(entity, components[3]) = buffer;
                    // alter21.alter(id_geo, (GeometryResourceHash(hasher.finish()), geo_desc, slot, buffer));
            }
            
            if editor.contains_entity(entity) {
                ActionEntity::init(entity, &mut editor);
                let components = [
                     editor.init_component::<SceneID>(),
                     editor.init_component::<TrailLinkedTransform>(),
                    //  editor.init_component::<// TrailMesh(id_mesh),
                     editor.init_component::<TrailGeometry>(),
                     editor.init_component::<TrailBase>(),
                     editor.init_component::<TrailWorldPlace>(),
                     editor.init_component::<TrailPoints>(),
                     editor.init_component::<ColorOverTrail>(),
                     editor.init_component::<TrailMinimunVertexDistance>(),
                     editor.init_component::<WidthOverTrail>(),
                     editor.init_component::<TrailAgeControl>(),
                     editor.init_component::<TrailSize>(),
                     editor.init_component::<TrailColor>(),
                     editor.init_component::<TrailRandom>(),
                ];
                editor.add_components(entity, &components).unwrap();

                // alter23.alter(entity, 
                    *editor.get_component_unchecked_mut_by_id(entity, components[0]) =SceneID(id_scene);
                    *editor.get_component_unchecked_mut_by_id(entity, components[1]) =TrailLinkedTransform(id_linked);
                    // *editor.get_component_unchecked_mut_by_id(entity, components[2]) =TrailMesh(id_mesh),
                    *editor.get_component_unchecked_mut_by_id(entity, components[2]) =TrailGeometry(id_geo);
                    *editor.get_component_unchecked_mut_by_id(entity, components[3]) =TrailBase::new(u32::MAX);
                    *editor.get_component_unchecked_mut_by_id(entity, components[4]) =TrailWorldPlace(true);
                    *editor.get_component_unchecked_mut_by_id(entity, components[5]) =TrailPoints::default();
                    *editor.get_component_unchecked_mut_by_id(entity, components[6]) =ColorOverTrail(Color4Gradient::default());
                    *editor.get_component_unchecked_mut_by_id(entity, components[7]) =TrailMinimunVertexDistance(0.01);
                    *editor.get_component_unchecked_mut_by_id(entity, components[8]) =WidthOverTrail(FloatInterpolation::new(1.));
                    *editor.get_component_unchecked_mut_by_id(entity, components[9]) =TrailAgeControl(200);
                    *editor.get_component_unchecked_mut_by_id(entity, components[10]) =TrailSize(1.);
                    *editor.get_component_unchecked_mut_by_id(entity, components[11]) =TrailColor(Vector4::new(1., 1., 1., 1.));
                    *editor.get_component_unchecked_mut_by_id(entity, components[12]) =TrailRandom(pi_wy_rng::WyRng::default());
                // ));
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