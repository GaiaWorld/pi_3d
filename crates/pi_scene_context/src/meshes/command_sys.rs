
use pi_render::components;
use pi_scene_shell::{add_component, prelude::{pi_world::editor::{self, EntityEditor}, *}};

use crate::{
    cullings::prelude::*, flags::Enable, geometry::{
        instance::{types::{InstanceAttributeAnimated, ModelInstanceAttributes}, DirtyInstanceSourceForSingleBuffer}, prelude::*
    }, layer_mask::prelude::*, object::ActionEntity, pass::*, prelude::{TypeAnimeAssetMgrs, TypeAnimeContexts}, renderers::prelude::*, skeleton::prelude::*, state::{DirtyMeshStates, MeshStates}, transforms::command_sys::{ActionTransformNode, ActionTransformNodeBundle}
};

use super::{
    command::*,
    model::*,
    abstract_mesh::AbstructMesh,
    lighting::*,
};
use crate::prelude::*;

pub fn sys_create_mesh(
    mut cmds: ResMut<ActionListMeshCreate>,
    // mut commands: Commands,
    mut editor: EntityEditor,
    mut allocator: ResMut<ResBindBufferAllocator>,
    empty: Res<SingleEmptyEntity>,
    mut disposereadylist: ResMut<ActionListDisposeReadyForRef>,
    mut _disposecanlist: ResMut<ActionListDisposeCan>,
    lightlimit: Res<ModelLightLimit>,
    commonbindmodel: Res<CommonBindModel>,
) {
    cmds.drain().drain(..).for_each(|OpsMeshCreation(scene, entity, state )| {
        // log::error!("Create Mesh");
        if ActionMesh::init(&mut editor, entity, scene, &mut allocator, &empty, state, &lightlimit.0, &commonbindmodel) == false {
            disposereadylist.push(OpsDisposeReadyForRef::ops(entity));
        }
    });
}

pub fn sys_create_instanced_mesh(
    mut cmds: ResMut<ActionListInstanceMeshCreate>,
    mut editor: EntityEditor,
    mut meshes: Query<(&SceneID, &mut InstanceSourceRefs, &mut DirtyInstanceSourceRefs, &ModelInstanceAttributes)>,
) {
    cmds.drain().drain(..).for_each(|OpsInstanceMeshCreation(source, instance, count)| {
        if let Ok((id_scene, mut instancelist, mut flag, instanceattrs)) = meshes.get_mut(source) {

            let instanceattrs = instanceattrs.clone();

            if  editor.contains_entity(instance) {
                let components: [pi_world::world::ComponentIndex; 3] = [editor.init_component::<ModelInstanceAttributes>(), editor.init_component::<TargetAnimatorableIsRunning>(), editor.init_component::<InstanceAttributeAnimated>(),];
                editor.add_components(instance, &components);
                *editor.get_component_unchecked_mut_by_id(instance, components[0]) = instanceattrs;
                *editor.get_component_unchecked_mut_by_id(instance, components[1]) = TargetAnimatorableIsRunning;
                *editor.get_component_unchecked_mut_by_id(instance, components[2]) = InstanceAttributeAnimated::default();
                // let _ = editor.alter(instance, (instanceattrs, TargetAnimatorableIsRunning, InstanceAttributeAnimated::default()));
                // commands.insert(TargetAnimatorableIsRunning).insert(InstanceAttributeAnimated::default());
                ActionInstanceMesh::init(instance, &mut editor, source, id_scene.0);
    
                instancelist.insert(instance);
                *flag = DirtyInstanceSourceRefs;
            }
            // 
        } else {
            if count < 2 {
                cmds.push(OpsInstanceMeshCreation(source, instance, count + 1))
            }
        }
    });
}

pub fn sys_act_target_animation_attribute(
    mut cmds: ResMut<ActionListTargetAnimationAttribute>,
    mut items: Query<(&mut ModelInstanceAttributes, &mut InstanceAttributeAnimated)>,
    mut insert0: Insert<()>,
    mut animatorablefloat: ResMut<ActionListAnimatorableFloat>,
    mut animatorablevec2s: ResMut<ActionListAnimatorableVec2>,
    mut animatorablevec3s: ResMut<ActionListAnimatorableVec3>,
    mut animatorablevec4s: ResMut<ActionListAnimatorableVec4>,
    mut animatorableuints: ResMut<ActionListAnimatorableUint>,
    mut animatorablesints: ResMut<ActionListAnimatorableSint>,
    anime_assets: TypeAnimeAssetMgrs,
    mut anime_contexts: TypeAnimeContexts,
    mut targetanimations: ResMut<ActionListAddTargetAnime>,
) {
    cmds.drain().drain(..).for_each(|OpsTargetAnimationAttribute(item, attr, group, curve)| {
        if let Ok((mut attributes, mut animated)) = items.get_mut(item) {
            if let Some(offset) = attributes.animator(&attr, item, &mut insert0, &mut animatorablefloat, &mut animatorablevec2s, &mut animatorablevec3s, &mut animatorablevec4s, &mut animatorableuints, &mut animatorablesints) {
                match offset.entity() {
                    Some(target) => {
                        animated.add(&attr);
                        match offset.atype() {
                            EAnimatorableType::Vec4 => if let Some(curve) = anime_assets.vec4s.get(&curve) {
                                let anime = anime_contexts.vec4s.ctx.create_animation(0, AssetTypeFrameCurve::from(curve));
                                targetanimations.push(OpsAddTargetAnimation::ops(group, target, anime));
                            },
                            EAnimatorableType::Vec3 => if let Some(curve) = anime_assets.vec3s.get(&curve) {
                                let anime = anime_contexts.vec3s.ctx.create_animation(0, AssetTypeFrameCurve::from(curve));
                                targetanimations.push(OpsAddTargetAnimation::ops(group, target, anime));
                            },
                            EAnimatorableType::Vec2 => if let Some(curve) = anime_assets.vec2s.get(&curve) {
                                let anime = anime_contexts.vec2s.ctx.create_animation(0, AssetTypeFrameCurve::from(curve));
                                targetanimations.push(OpsAddTargetAnimation::ops(group, target, anime));
                            },
                            EAnimatorableType::Float => if let Some(curve) = anime_assets.float.get(&curve) {
                                let anime = anime_contexts.float.ctx.create_animation(0, AssetTypeFrameCurve::from(curve));
                                targetanimations.push(OpsAddTargetAnimation::ops(group, target, anime));
                            },
                            EAnimatorableType::Uint => if let Some(curve) = anime_assets.uints.get(&curve) {
                                let anime = anime_contexts.uints.ctx.create_animation(0, AssetTypeFrameCurve::from(curve));
                                targetanimations.push(OpsAddTargetAnimation::ops(group, target, anime));
                            },
                            EAnimatorableType::Int => if let Some(curve) = anime_assets._ints.get(&curve) {
                                let anime = anime_contexts._ints.ctx.create_animation(0, AssetTypeFrameCurve::from(curve));
                                targetanimations.push(OpsAddTargetAnimation::ops(group, target, anime));
                            },
                        }
                    },
                    None => { },
                }
            }
        }
    });
}

pub fn sys_act_mesh_modify(
    mut cmds: ResMut<ActionListMeshShadow>,
    mut castshadows: Query<&mut MeshCastShadow>,
    mut receiveshadows: Query<&mut MeshReceiveShadow>,
    mut align_cmds: ResMut<ActionListMeshRenderAlignment>,
    mut align_items: Query<&mut RenderAlignment>,
    mut scalingmode_cmds: ResMut<ActionListAbstructMeshScalingMode>,
    mut scalingode_items: Query<&mut ScalingMode>,
    mut velocity_cmds: ResMut<ActionListAbstructMeshVelocity>,
    mut velocity_items: Query<&mut ModelVelocity>,
    mut indices_cmds: ResMut<ActionListMeshRenderIndiceRange>,
    mut indices_items: Query<(&mut IndiceRenderRange, &mut RecordIndiceRenderRange)>,
    mut vertexrange_cmds: ResMut<ActionListMeshRenderVertexRange>,
    mut vertexrange_items: Query<&mut VertexRenderRange>,
) {
    cmds.drain().drain(..).for_each(|cmd| {
        match cmd {
            OpsMeshShadow::CastShadow(entity, val) => {
                if let Ok(mut castshadow) = castshadows.get_mut(entity) {
                    if val != castshadow.0 {
                        *castshadow = MeshCastShadow(val);
                    }
                }
            },
            OpsMeshShadow::ReceiveShadow(entity, val) => {
                if let Ok(mut receiveshadow) = receiveshadows.get_mut(entity) {
                    if val != receiveshadow.0 {
                        *receiveshadow = MeshReceiveShadow(val);
                    }
                }
            },
        }
    });
    align_cmds.drain().drain(..).for_each(|OpsMeshRenderAlignment(entity, val)| {
        if let Ok(mut item) = align_items.get_mut(entity) {
            // log::warn!("RenderAlignment: {:?}", (val));
            *item = val;
        }
    });
    scalingmode_cmds.drain().drain(..).for_each(|OpsAbstructMeshScalingMode(entity, val)| {
        if let Ok(mut item) = scalingode_items.get_mut(entity) {
            *item = val;
        }
    });
    velocity_cmds.drain().drain(..).for_each(|OpsAbstructMeshVelocity(entity, val)| {
        if let Ok(mut item) = velocity_items.get_mut(entity) {
            *item = val;
        }
    });
    indices_cmds.drain().drain(..).for_each(|OpsMeshRenderIndiceRange(entity, val)| {
        // log::warn!("Range: {:?}", val);
        if let Ok((mut item, mut record)) = indices_items.get_mut(entity) {
            *record = RecordIndiceRenderRange(IndiceRenderRange(val.clone()));
            *item = IndiceRenderRange(val);
        }
    });
    vertexrange_cmds.drain().drain(..).for_each(|OpsMeshRenderVertexRange(entity, val)| {
        // log::warn!("Range: {:?}", val);
        if let Ok(mut item) = vertexrange_items.get_mut(entity) {
            // *record = RecordIndiceRenderRange(IndiceRenderRange(val.clone()));
            *item = VertexRenderRange(val);
        }
    });
}

pub fn sys_act_instance_attribute(
    mut cmdsfloat: ResMut<ActionListInstanceAttr>,
    mut instances: Query<&mut ModelInstanceAttributes>,

    mut animator_vec4: ResMut<ActionListAnimatorableVec4>,
    mut animator_vec3: ResMut<ActionListAnimatorableVec3>,
    mut animator_vec2: ResMut<ActionListAnimatorableVec2>,
    mut animator_float: ResMut<ActionListAnimatorableFloat>,
    mut animator_uint: ResMut<ActionListAnimatorableUint>,
    mut animator_sint: ResMut<ActionListAnimatorableSint>,

    mut pointlight_cmds: ResMut<ActionListMeshForcePointLighting>,
    mut pointlight_items: Query<&mut ModelForcePointLightings>,
    mut spotlight_cmds: ResMut<ActionListMeshForceSpotLighting>,
    mut spotlight_items: Query<&mut ModelForceSpotLightings>,
    mut hemilight_cmds: ResMut<ActionListMeshForceHemiLighting>,
    mut hemilight_items: Query<&mut ModelForceHemiLightings>,
    mut skinoff_cmds: ResMut<ActionListBoneOffset>,
    skinoff_items: Query<&BindModel>,
) {

    cmdsfloat.drain().drain(..).for_each(|OpsInstanceAttr(instance, val, attr)| {
        if let Ok(mut attributes) = instances.get_mut(instance) {
            if let Some(offset) = attributes.offset(&attr) {
                if let Some(target) = offset.entity() {
                    match val {
                        EInstanceAttr::Float(val) => animator_float.push(OpsAnimatorableFloat::ops(target, instance, AnimatorableFloat(val), EAnimatorableEntityType::Attribute)),
                        EInstanceAttr::Int(val) => animator_sint.push(OpsAnimatorableSint::ops(target, instance, AnimatorableSint(val), EAnimatorableEntityType::Attribute)),
                        EInstanceAttr::Uint(val) => animator_uint.push(OpsAnimatorableUint::ops(target, instance, AnimatorableUint(val), EAnimatorableEntityType::Attribute)),
                        EInstanceAttr::Vec4(val) => animator_vec4.push(OpsAnimatorableVec4::ops(target, instance, AnimatorableVec4::from(val.as_slice()), EAnimatorableEntityType::Attribute)),
                        EInstanceAttr::Vec3(val) => animator_vec3.push(OpsAnimatorableVec3::ops(target, instance, AnimatorableVec3::from(val.as_slice()), EAnimatorableEntityType::Attribute)),
                        EInstanceAttr::Vec2(val) => animator_vec2.push(OpsAnimatorableVec2::ops(target, instance, AnimatorableVec2::from(val.as_slice()), EAnimatorableEntityType::Attribute)),
                    }
                } else {
                    let mut offset = offset.offset() as usize;
                    match val {
                        EInstanceAttr::Float(val) => bytemuck::cast_slice(&[val]).iter().for_each(|v| { attributes.bytes_mut()[offset] = *v; offset += 1; }),
                        EInstanceAttr::Uint(val) => bytemuck::cast_slice(&[val]).iter().for_each(|v| { attributes.bytes_mut()[offset] = *v; offset += 1; }),
                        EInstanceAttr::Int(val) => bytemuck::cast_slice(&[val]).iter().for_each(|v| { attributes.bytes_mut()[offset] = *v; offset += 1; }),
                        EInstanceAttr::Vec4(val) => bytemuck::cast_slice(&val).iter().for_each(|v| { attributes.bytes_mut()[offset] = *v; offset += 1; }),
                        EInstanceAttr::Vec3(val) => bytemuck::cast_slice(&val).iter().for_each(|v| { attributes.bytes_mut()[offset] = *v; offset += 1; }),
                        EInstanceAttr::Vec2(val) => bytemuck::cast_slice(&val).iter().for_each(|v| { attributes.bytes_mut()[offset] = *v; offset += 1; }),
                    }
                    ;
                }
            }
        }
    });
    pointlight_cmds.drain().drain(..).for_each(|OpsMeshForcePointLighting(entity, light, isadd)| {
        // log::warn!("Range: {:?}", val);
        if let Ok(mut item) = pointlight_items.get_mut(entity) {
            // *record = RecordIndiceRenderRange(IndiceRenderRange(val.clone()));
            match item.0.binary_search(&light) {
                Ok(idx)  => { if isadd == false { item.0.remove(idx); } },
                Err(idx) => { if isadd == true  { item.0.insert(idx, light); } },
            }
        }
    });
    spotlight_cmds.drain().drain(..).for_each(|OpsMeshForceSpotLighting(entity, light, isadd)| {
        // log::warn!("Range: {:?}", val);
        if let Ok(mut item) = spotlight_items.get_mut(entity) {
            // *record = RecordIndiceRenderRange(IndiceRenderRange(val.clone()));
            match item.0.binary_search(&light) {
                Ok(idx)  => { if isadd == false { item.0.remove(idx); } },
                Err(idx) => { if isadd == true  { item.0.insert(idx, light); } },
            }
        }
    });
    hemilight_cmds.drain().drain(..).for_each(|OpsMeshForceHemiLighting(entity, light, isadd)| {
        // log::warn!("Range: {:?}", val);
        if let Ok(mut item) = hemilight_items.get_mut(entity) {
            // *record = RecordIndiceRenderRange(IndiceRenderRange(val.clone()));
            match item.0.binary_search(&light) {
                Ok(idx)  => { if isadd == false { item.0.remove(idx); } },
                Err(idx) => { if isadd == true  { item.0.insert(idx, light); } },
            }
        }
    });
    skinoff_cmds.drain().drain(..).for_each(|OpsBoneOffset(entity, offset)| {
        if let Ok(bind) = skinoff_items.get(entity) {
            bind.0.as_ref().unwrap().data().write_data(ShaderBindModelAboutMatrix::OFFSET_U32_A as usize, bytemuck::cast_slice(&[offset]));
        }
    });
}

pub type ActionMeshInitBundle = (
    MeshLightingMode,
    ModelLightingIndexs,
    ModelForcePointLightings,
    ModelForceSpotLightings,
    ModelForceHemiLightings,
    MeshStates,
    DirtyMeshStates,
    ModelInstanceAttributes,
    MeshInstanceState,
);

pub type ActionMeshBundle = (
    AbstructMesh, 
    Mesh, 
    GeometryID, 
    RenderGeometryEable, 
    RenderWorldMatrix, 
    RenderWorldMatrixInv,
    RenderMatrixDirty,
    MeshCastShadow,
    MeshReceiveShadow,
    PassDirtyBindEffectValue,
    FlagPassDirtyBindEffectValue,
    PassDirtyBindEffectTextures,
    FlagPassDirtyBindEffectTextures,
    LayerMask,
    AbstructMeshCullingFlag,
    TransparentSortParam,
    BindSkinValue,
    ModelVelocity,
    RenderAlignment,
    ScalingMode,
    IndiceRenderRange,
    RecordIndiceRenderRange,
    VertexRenderRange,
    GeometryBounding,
    GeometryCullingMode,
    InstancedMeshTransparentSortCollection
);
pub struct ActionMesh;
impl ActionMesh {
    pub fn init(
        editor: &mut EntityEditor,
        entity: Entity,
        scene: Entity,
        allocator: &mut ResBindBufferAllocator,
        empty: &SingleEmptyEntity,
        state: MeshInstanceState,
        lightlimit: &LightLimitInfo,
        commonbindmodel: &CommonBindModel,
    ) -> bool {
        let meshinstanceattributes = ModelInstanceAttributes::new(&state.instances, state.instance_matrix);
        if !editor.contains_entity(entity) 
        //  {
        //     cmd
        // } else 
        {
            return false;
        };

        ActionTransformNode::init(entity, editor, scene);
        ActionMesh::as_mesh(entity, editor, empty.id());
        ActionMesh::as_instance_source(entity, editor);
        // ActionMesh::as_instance_source(&mut entitycmd);
        let components = [editor.init_component::<TargetAnimatorableIsRunning>(), editor.init_component::<InstanceAttributeAnimated>()];
        editor.add_components(entity, &components);
        // editor.get_component_unchecked_mut_by_id(entity, components[0]) = TargetAnimatorableIsRunning;
        // editor.add_components(entity, (TargetAnimatorableIsRunning, InstanceAttributeAnimated::default()));

        if meshinstanceattributes.bytes().len() > 0 {
            let components = [editor.init_component::<BindModel>(), editor.init_component::<ModelStatic>()];
            editor.add_components(entity, &components);

            // editor.add_components(entity, (commonbindmodel.0.clone(), ModelStatic));
            *editor.get_component_unchecked_mut_by_id(entity, components[0]) = commonbindmodel.0.clone();
            // entitycmd.insert(ModelStatic);
        } else {
            if let Some(bind) = BindModel::new(allocator) {
                // log::info!("BindModel New");
                add_component(editor, entity, bind);
                // editor.add_components(entity, (bind,));
            }
        }
        let components = [
            editor.init_component::<MeshLightingMode>(),
            editor.init_component::<ModelLightingIndexs>(),
            editor.init_component::<ModelForcePointLightings>(),
            editor.init_component::<ModelForceSpotLightings>(),
            editor.init_component::<ModelForceHemiLightings>(),
            // ModelPointLightingDirty::default());
            // ModelSpotLightingDirty::default());
            // ModelHemiLightingDirty::default());

            editor.init_component::<MeshStates>(),
            editor.init_component::<DirtyMeshStates>(),

            editor.init_component::<ModelInstanceAttributes>(),
            editor.init_component::<MeshInstanceState>(),
        ];
        let _ = editor.add_components(
            entity, &components); 
        // (
        *editor.get_component_unchecked_mut_by_id(entity, components[0]) =   MeshLightingMode::default();
        *editor.get_component_unchecked_mut_by_id(entity, components[1]) =  ModelLightingIndexs::new(allocator, lightlimit);
        *editor.get_component_unchecked_mut_by_id(entity, components[2]) =ModelForcePointLightings::default();
        *editor.get_component_unchecked_mut_by_id(entity, components[3]) =ModelForceSpotLightings::default();
        *editor.get_component_unchecked_mut_by_id(entity, components[4]) =ModelForceHemiLightings::default();
            // ModelPointLightingDirty::default());
            // ModelSpotLightingDirty::default());
            // ModelHemiLightingDirty::default());

        *editor.get_component_unchecked_mut_by_id(entity, components[5]) =   MeshStates::default();
        *editor.get_component_unchecked_mut_by_id(entity, components[6]) =   DirtyMeshStates;

        *editor.get_component_unchecked_mut_by_id(entity, components[7]) =  meshinstanceattributes;
        *editor.get_component_unchecked_mut_by_id(entity, components[8]) =   state;


        let id01 = editor.alloc_entity();
        let id02 = editor.alloc_entity();
        let id03 = editor.alloc_entity();
        let id04 = editor.alloc_entity();
        let id05 = editor.alloc_entity();
        let id06 = editor.alloc_entity();
        let id07 = editor.alloc_entity();
        let id08 = editor.alloc_entity();
        add_component(editor, entity, PassIDs([id01, id02, id03, id04, id05, id06, id07, id08]));
        // let _ = editor.alter(entity, (PassIDs([id01, id02, id03, id04, id05, id06, id07, id08]),));
        create_passobj(editor, id01, entity, scene, empty.id(), PassTag::PASS_TAG_01);
        create_passobj(editor, id02, entity, scene, empty.id(), PassTag::PASS_TAG_02);
        create_passobj(editor, id03, entity, scene, empty.id(), PassTag::PASS_TAG_03);
        create_passobj(editor, id04, entity, scene, empty.id(), PassTag::PASS_TAG_04);
        create_passobj(editor, id05, entity, scene, empty.id(), PassTag::PASS_TAG_05);
        create_passobj(editor, id06, entity, scene, empty.id(), PassTag::PASS_TAG_06);
        create_passobj(editor, id07, entity, scene, empty.id(), PassTag::PASS_TAG_07);
        create_passobj(editor, id08, entity, scene, empty.id(), PassTag::PASS_TAG_08);
        return true;
    }
    pub(crate) fn as_mesh(
        entity: Entity,
        editor: &mut EntityEditor,
        geometry: Entity,
    ) {
        // let mut unclipdepth = false;
        // #[cfg(not(target_arch = "wasm32"))]
        // {
        //     unclipdepth = true;
        // }
        let unclipdepth = false;

        let components = [
            editor.init_component::<AbstructMesh>(),
            editor.init_component::<    Mesh>(),
            editor.init_component::<    GeometryID>(),
            editor.init_component::<    RenderGeometryEable>(),
            editor.init_component::<    RenderWorldMatrix>(),
            editor.init_component::<    RenderWorldMatrixInv>(),
            editor.init_component::<    RenderMatrixDirty>(),
            editor.init_component::<    MeshCastShadow>(),
            editor.init_component::<    MeshReceiveShadow>(),
            editor.init_component::<    PassDirtyBindEffectValue>(),
            editor.init_component::<    FlagPassDirtyBindEffectValue>(),
            editor.init_component::<    PassDirtyBindEffectTextures>(),
            editor.init_component::<    FlagPassDirtyBindEffectTextures>(),
            editor.init_component::<    LayerMask>(),
            editor.init_component::<    AbstructMeshCullingFlag>(),
            editor.init_component::<    TransparentSortParam>(),
    
                // CCullMode(CullMode::Back))
                // CFrontFace(FrontFace::Ccw))
                // CPolygonMode(PolygonMode::Fill))
                // Topology(PrimitiveTopology::TriangleList))
                // CUnClipDepth(unclipdepth))
                // PrimitiveState { cull: CullMode::Back, frontface: FrontFace::Ccw, polygon: PolygonMode::Fill, topology: PrimitiveTopology::TriangleList, unclip_depth: unclipdepth })
    
                // DepthWrite::default())
                // DepthCompare::default())
                // DepthBias::default())
                // StencilFront::default())
                // StencilBack::default())
                // StencilRead::default())
                // StencilWrite::default())
    
                // ModelBlend::default())
    
            editor.init_component::<    BindSkinValue>(),
            editor.init_component::<    ModelVelocity>(),
            editor.init_component::<    RenderAlignment>(),
            editor.init_component::<    ScalingMode>(),
            editor.init_component::<    IndiceRenderRange>(),
            editor.init_component::<    RecordIndiceRenderRange>(),
            editor.init_component::<    VertexRenderRange>(),
            editor.init_component::<    GeometryBounding>(),
            editor.init_component::<    GeometryCullingMode>(),
            editor.init_component::<    InstancedMeshTransparentSortCollection>(),
        ];
        editor.add_components(entity, &components);

        *editor.get_component_unchecked_mut_by_id(entity, components[0]) =    AbstructMesh;
        *editor.get_component_unchecked_mut_by_id(entity, components[1]) =     Mesh;
        *editor.get_component_unchecked_mut_by_id(entity, components[2]) =    GeometryID(geometry);
        *editor.get_component_unchecked_mut_by_id(entity, components[3]) =    RenderGeometryEable(false);
        *editor.get_component_unchecked_mut_by_id(entity, components[4]) =    RenderWorldMatrix(Matrix::identity());
        *editor.get_component_unchecked_mut_by_id(entity, components[5]) =    RenderWorldMatrixInv(Matrix::identity());
        *editor.get_component_unchecked_mut_by_id(entity, components[6]) =    RenderMatrixDirty(true);
        *editor.get_component_unchecked_mut_by_id(entity, components[7]) =    MeshCastShadow(false);
        *editor.get_component_unchecked_mut_by_id(entity, components[8]) =    MeshReceiveShadow(false);
        *editor.get_component_unchecked_mut_by_id(entity, components[9]) =    PassDirtyBindEffectValue(0);
        *editor.get_component_unchecked_mut_by_id(entity, components[10]) =    FlagPassDirtyBindEffectValue;
        *editor.get_component_unchecked_mut_by_id(entity, components[11]) =    PassDirtyBindEffectTextures(0);
        *editor.get_component_unchecked_mut_by_id(entity, components[12]) =    FlagPassDirtyBindEffectTextures;
        *editor.get_component_unchecked_mut_by_id(entity, components[13]) =    LayerMask::default();
        *editor.get_component_unchecked_mut_by_id(entity, components[14]) =    AbstructMeshCullingFlag(false);
        *editor.get_component_unchecked_mut_by_id(entity, components[15]) =    TransparentSortParam::opaque();

            // CCullMode(CullMode::Back))
            // CFrontFace(FrontFace::Ccw))
            // CPolygonMode(PolygonMode::Fill))
            // Topology(PrimitiveTopology::TriangleList))
            // CUnClipDepth(unclipdepth))
            // PrimitiveState { cull: CullMode::Back, frontface: FrontFace::Ccw, polygon: PolygonMode::Fill, topology: PrimitiveTopology::TriangleList, unclip_depth: unclipdepth })

            // DepthWrite::default())
            // DepthCompare::default())
            // DepthBias::default())
            // StencilFront::default())
            // StencilBack::default())
            // StencilRead::default())
            // StencilWrite::default())

            // ModelBlend::default())

           *editor.get_component_unchecked_mut_by_id(entity, components[16]) =    BindSkinValue(None);
           *editor.get_component_unchecked_mut_by_id(entity, components[17]) =    ModelVelocity::default();
           *editor.get_component_unchecked_mut_by_id(entity, components[18]) =    RenderAlignment::default();
           *editor.get_component_unchecked_mut_by_id(entity, components[19]) =    ScalingMode::default();
           *editor.get_component_unchecked_mut_by_id(entity, components[20]) =    IndiceRenderRange::default();
           *editor.get_component_unchecked_mut_by_id(entity, components[21]) =    RecordIndiceRenderRange::default();
           *editor.get_component_unchecked_mut_by_id(entity, components[22]) =    VertexRenderRange::default();
           *editor.get_component_unchecked_mut_by_id(entity, components[23]) =    GeometryBounding::default();
           *editor.get_component_unchecked_mut_by_id(entity, components[24]) =    GeometryCullingMode::default();
           *editor.get_component_unchecked_mut_by_id(entity, components[25]) =    InstancedMeshTransparentSortCollection(vec![]);

    }

    pub fn modify(
        app: &mut App,
        cmd: OpsMeshShadow,
    ) {
        let mut cmds = app.world.get_single_res_mut::<ActionListMeshShadow>().unwrap();
        cmds.push(cmd);
    }

    pub fn as_instance_source(
        entity: Entity,
        editor: &mut EntityEditor,
    ) {
        let components = [
            editor.init_component::<InstanceSourceRefs>(),
            editor.init_component::<DirtyInstanceSourceRefs>(),
            editor.init_component::<DirtyInstanceSourceForSingleBuffer>(),];

        editor.add_components(entity, &components);
        // (
        //     InstanceSourceRefs::default(),
        //     DirtyInstanceSourceRefs::default(),
        //     DirtyInstanceSourceForSingleBuffer::default())
        // );
    }
}

pub type ActionInstanceMeshBundle = (
    AbstructMesh,
    AbstructMeshCullingFlag,
    InstanceTransparentIndex,
    InstanceMesh,
    RenderMatrixDirty,
    RenderWorldMatrix,
    RenderWorldMatrixInv,
    ModelVelocity,
    ScalingMode,
    GeometryBounding,
    GeometryCullingMode
);
pub struct ActionInstanceMesh;
impl ActionInstanceMesh {
    pub fn init(
        // commands: &mut EntityCommands,
        entity: Entity,  
        editor: &mut EntityEditor,
        source: Entity,
        scene: Entity,
    ) {
        ActionTransformNode::init(entity, editor, scene);
        ActionInstanceMesh::as_instance(entity, editor, source);
    }
    pub(crate) fn as_instance(
        entity: Entity,
        editor: &mut EntityEditor,
        source: Entity,
    ) {
        let components = [

            editor.init_component::<AbstructMesh>(),
            editor.init_component::<AbstructMeshCullingFlag>(),
            editor.init_component::<InstanceTransparentIndex>(),
            editor.init_component::<InstanceMesh>(),

            editor.init_component::<RenderMatrixDirty>(),
            editor.init_component::<RenderWorldMatrix>(),
            editor.init_component::<RenderWorldMatrixInv>(),
            editor.init_component::<ModelVelocity>(),
            editor.init_component::<ScalingMode>(),
            editor.init_component::<GeometryBounding>(),
            editor.init_component::<GeometryCullingMode>(),
            ];
        
        let _ = editor.add_components(entity, &components);
        
        
            *editor.get_component_unchecked_mut_by_id(entity, components[0]) = AbstructMesh;
            *editor.get_component_unchecked_mut_by_id(entity, components[0]) = AbstructMeshCullingFlag(false);
            *editor.get_component_unchecked_mut_by_id(entity, components[0]) = InstanceTransparentIndex(0);
            *editor.get_component_unchecked_mut_by_id(entity, components[0]) = InstanceMesh(source);

            *editor.get_component_unchecked_mut_by_id(entity, components[0]) = RenderMatrixDirty(true);
            *editor.get_component_unchecked_mut_by_id(entity, components[0]) = RenderWorldMatrix(Matrix::identity());
            *editor.get_component_unchecked_mut_by_id(entity, components[0]) = RenderWorldMatrixInv(Matrix::identity());
            *editor.get_component_unchecked_mut_by_id(entity, components[0]) = ModelVelocity::default();
            *editor.get_component_unchecked_mut_by_id(entity, components[0]) = ScalingMode::default();
            *editor.get_component_unchecked_mut_by_id(entity, components[0]) = GeometryBounding::default();
            *editor.get_component_unchecked_mut_by_id(entity, components[0]) = GeometryCullingMode::default() ;
        
    }
}

fn create_passobj(
    editor: &mut EntityEditor,
    id: Entity,
    idmodel: Entity,
    scene: Entity,
    empty: Entity,
    tag: PassTag,
) {
    // let mut entitycmd = commands.entity(id);
    ActionEntity::init(id,  editor); ActionPassObject::init(id, editor, empty, idmodel, scene);
    add_component(editor, id, tag);
}