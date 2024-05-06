
use pi_scene_shell::prelude::*;

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
    mut allocator: ResMut<ResBindBufferAllocator>,
    empty: Res<SingleEmptyEntity>,
    mut disposereadylist: ResMut<ActionListDisposeReadyForRef>,
    mut _disposecanlist: ResMut<ActionListDisposeCan>,
    lightlimit: Res<ModelLightLimit>,
    commonbindmodel: Res<CommonBindModel>,
) {
    cmds.drain().drain(..).for_each(|OpsMeshCreation(scene, entity, state )| {
        // log::error!("Create Mesh");
        if ActionMesh::init(&mut insert, &mut alter1, &mut alter2, &mut alter3, &mut alter4, &mut alter6, &mut alter7, &mut alter8, &mut alter9, &mut alter10, &mut alter11, &mut alter12, &mut alter13, &mut alter14, &mut alter15, entity, scene, &mut allocator, &empty, state, &lightlimit.0, &commonbindmodel) == false {
            disposereadylist.push(OpsDisposeReadyForRef::ops(entity));
        }
    });
}

pub fn sys_create_instanced_mesh(
    mut cmds: ResMut<ActionListInstanceMeshCreate>,
    mut commands: Alter<(), (), (ModelInstanceAttributes, TargetAnimatorableIsRunning, InstanceAttributeAnimated), ()>,
    mut alter1: Alter<(), (), (DisposeReady, DisposeCan)>,
    mut alter2: Alter<(), (), (SceneID, )>,
    mut alter3: Alter<(), (), (Down, Up, Layer, Enable, RecordEnable, GlobalEnable)>,
    mut alter4: Alter<(), (), ActionTransformNodeBundle>,
    mut alter5: Alter<(), (), ActionInstanceMeshBundle>,
    mut meshes: Query<(&SceneID, &mut InstanceSourceRefs, &mut DirtyInstanceSourceRefs, &ModelInstanceAttributes)>,
) {
    cmds.drain().drain(..).for_each(|OpsInstanceMeshCreation(source, instance, count)| {
        if let Ok((id_scene, mut instancelist, mut flag, instanceattrs)) = meshes.get_mut(source) {

            let instanceattrs = instanceattrs.clone();

            if  commands.get(instance).is_ok() {
                let _ = commands.alter(instance, (instanceattrs, TargetAnimatorableIsRunning, InstanceAttributeAnimated::default()));
                // commands.insert(TargetAnimatorableIsRunning).insert(InstanceAttributeAnimated::default());
                ActionInstanceMesh::init(instance, &mut alter1, &mut alter2, &mut alter3, &mut alter4, &mut alter5, source, id_scene.0);
    
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
            bind.0.data().write_data(ShaderBindModelAboutMatrix::OFFSET_U32_A as usize, bytemuck::cast_slice(&[offset]));
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
        // commands: &mut Commands,
        insert: &mut Insert<()>,
        alter1: &mut Alter<(), (), (DisposeReady, DisposeCan),>,
        alter2: &mut Alter<(), (), (SceneID,)>,
        alter3: &mut Alter<(), (), (Down, Up, Layer, Enable, RecordEnable, GlobalEnable)>,
        alter4: &mut Alter<(), (), ActionTransformNodeBundle>,
        alter6: &mut Alter<(), (), (InstanceSourceRefs, DirtyInstanceSourceRefs, DirtyInstanceSourceForSingleBuffer)>,
        alter7: &mut Alter<(), (), (TargetAnimatorableIsRunning, InstanceAttributeAnimated)>,
        alter8: &mut Alter<(), (), (BindModel, ModelStatic)>,
        alter9: &mut Alter<(), (), (BindModel,)>,
        alter10: &mut Alter<(), (), ActionMeshInitBundle>,
        alter11: &mut Alter<(), (), (DisposeReady, DisposeCan)>,
        alter12: &mut Alter<(), (), ActionPassObjectInitBundle>, 
        alter13: &mut Alter<(), (), (PassTag,), ()>,
        alter14: &mut Alter<(), (), ActionMeshBundle>,
        alter15: &mut Alter<(), (), (PassIDs,), ()>,
        entity: Entity,
        scene: Entity,
        allocator: &mut ResBindBufferAllocator,
        empty: &SingleEmptyEntity,
        state: MeshInstanceState,
        lightlimit: &LightLimitInfo,
        commonbindmodel: &CommonBindModel,
    ) -> bool {
        let meshinstanceattributes = ModelInstanceAttributes::new(&state.instances, state.instance_matrix);
        if alter1.get(entity).is_err() 
        //  {
        //     cmd
        // } else 
        {
            return false;
        };

        ActionTransformNode::init(entity, alter1, alter2, alter3, alter4, scene);
        ActionMesh::as_mesh(entity, alter14, empty.id());
        ActionMesh::as_instance_source(entity, alter6);
        // ActionMesh::as_instance_source(&mut entitycmd);
        alter7.alter(entity, (TargetAnimatorableIsRunning, InstanceAttributeAnimated::default()));

        if meshinstanceattributes.bytes().len() > 0 {
            alter8.alter(entity, (commonbindmodel.0.clone(), ModelStatic));
            // entitycmd.insert(ModelStatic);
        } else {
            if let Some(bind) = BindModel::new(allocator) {
                // log::info!("BindModel New");
                alter9.alter(entity, (bind,));
            }
        }
        let _ = alter10.alter(
            entity, 
        (
            MeshLightingMode::default(),
            ModelLightingIndexs::new(allocator, lightlimit),
            ModelForcePointLightings::default(),
            ModelForceSpotLightings::default(),
            ModelForceHemiLightings::default(),
            // ModelPointLightingDirty::default());
            // ModelSpotLightingDirty::default());
            // ModelHemiLightingDirty::default());

            MeshStates::default(),
            DirtyMeshStates,

            meshinstanceattributes,
            state
        ));

        let id01 = insert.insert(());
        let id02 = insert.insert(());
        let id03 = insert.insert(());
        let id04 = insert.insert(());
        let id05 = insert.insert(());
        let id06 = insert.insert(());
        let id07 = insert.insert(());
        let id08 = insert.insert(());
        let _ = alter15.alter(entity, (PassIDs([id01, id02, id03, id04, id05, id06, id07, id08]),));
        create_passobj(alter11, alter12, alter13, id01, entity, scene, empty.id(), PassTag::PASS_TAG_01);
        create_passobj(alter11, alter12, alter13, id02, entity, scene, empty.id(), PassTag::PASS_TAG_02);
        create_passobj(alter11, alter12, alter13, id03, entity, scene, empty.id(), PassTag::PASS_TAG_03);
        create_passobj(alter11, alter12, alter13, id04, entity, scene, empty.id(), PassTag::PASS_TAG_04);
        create_passobj(alter11, alter12, alter13, id05, entity, scene, empty.id(), PassTag::PASS_TAG_05);
        create_passobj(alter11, alter12, alter13, id06, entity, scene, empty.id(), PassTag::PASS_TAG_06);
        create_passobj(alter11, alter12, alter13, id07, entity, scene, empty.id(), PassTag::PASS_TAG_07);
        create_passobj(alter11, alter12, alter13, id08, entity, scene, empty.id(), PassTag::PASS_TAG_08);
        return true;
    }
    pub(crate) fn as_mesh(
        entity: Entity,
        commands: &mut Alter<(), (), ActionMeshBundle>,
        geometry: Entity,
    ) {
        // let mut unclipdepth = false;
        // #[cfg(not(target_arch = "wasm32"))]
        // {
        //     unclipdepth = true;
        // }
        let unclipdepth = false;

        commands.alter(entity, 
            (AbstructMesh,
            Mesh,
            GeometryID(geometry),
            RenderGeometryEable(false),
            RenderWorldMatrix(Matrix::identity()),
            RenderWorldMatrixInv(Matrix::identity()),
            RenderMatrixDirty(true),
            MeshCastShadow(false),
            MeshReceiveShadow(false),
            PassDirtyBindEffectValue(0),
            FlagPassDirtyBindEffectValue,
            PassDirtyBindEffectTextures(0),
            FlagPassDirtyBindEffectTextures,
            LayerMask::default(),
            AbstructMeshCullingFlag(false),
            TransparentSortParam::opaque(),

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

            BindSkinValue(None),
            ModelVelocity::default(),
            RenderAlignment::default(),
            ScalingMode::default(),
            IndiceRenderRange::default(),
            RecordIndiceRenderRange::default(),
            VertexRenderRange::default(),
            GeometryBounding::default(),
            GeometryCullingMode::default(),
            InstancedMeshTransparentSortCollection(vec![]))
            );
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
        commands: &mut Alter<(), (), (InstanceSourceRefs, DirtyInstanceSourceRefs, DirtyInstanceSourceForSingleBuffer), ()>,
    ) {
        commands.alter(entity, 
           (InstanceSourceRefs::default(),
            DirtyInstanceSourceRefs::default(),
            DirtyInstanceSourceForSingleBuffer::default())
        );
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
        alter1: &mut Alter<(), (), (DisposeReady, DisposeCan)>,
        alter2: &mut Alter<(), (), (SceneID, )>,
        alter3: &mut Alter<(), (), (Down, Up, Layer, Enable, RecordEnable, GlobalEnable)>,
        alter4: &mut Alter<(), (), ActionTransformNodeBundle>,
        alter5: &mut Alter<(), (), ActionInstanceMeshBundle>,
        source: Entity,
        scene: Entity,
    ) {
        ActionTransformNode::init(entity, alter1, alter2, alter3, alter4, scene);
        ActionInstanceMesh::as_instance(entity, alter5, source);
    }
    pub(crate) fn as_instance(
        entity: Entity,
        commands: &mut Alter<(), (), ActionInstanceMeshBundle, ()>,
        source: Entity,
    ) {
        let _ = commands.alter(entity, 
        (
            AbstructMesh,
            AbstructMeshCullingFlag(false),
            InstanceTransparentIndex(0),
            InstanceMesh(source),

            RenderMatrixDirty(true),
            RenderWorldMatrix(Matrix::identity()),
            RenderWorldMatrixInv(Matrix::identity()),
            ModelVelocity::default(),
            ScalingMode::default(),
            GeometryBounding::default(),
            GeometryCullingMode::default() )
        );
    }
}

fn create_passobj(
    alter0: &mut Alter<(), (), (DisposeReady, DisposeCan), ()>,
    alter1: &mut Alter<(), (), ActionPassObjectInitBundle, ()>, 
    alter2: &mut Alter<(), (), (PassTag,), ()>,
    id: Entity,
    idmodel: Entity,
    scene: Entity,
    empty: Entity,
    tag: PassTag,
) {
    // let mut entitycmd = commands.entity(id);
    ActionEntity::init(id,  alter0); ActionPassObject::init(id, alter1, empty, idmodel, scene);
    alter2.alter(id, (tag,));
}