
use pi_scene_shell::prelude::*;

use crate::{
    cullings::prelude::*, geometry::{
        instance::{types::{InstanceAttributeAnimated, ModelInstanceAttributes}, DirtyInstanceSourceForSingleBuffer}, prelude::*
    }, layer_mask::prelude::*, object::ActionEntity, pass::*, prelude::{TypeAnimeAssetMgrs, TypeAnimeContexts}, renderers::prelude::*, skeleton::prelude::*, state::{DirtyMeshStates, MeshStates}, transforms::command_sys::{ActionTransformNode, TransformNodeBundle}
};

use super::{
    command::*,
    model::*,
    abstract_mesh::AbstructMesh,
    lighting::*,
};


pub type BundleModelStatic = (GeometryID, ModelStatic, BindModel);
pub type BundleModel = (
    TransformNodeBundle,
    BundleMesh,
    BundleInstanceSource,
    TargetAnimatorableIsRunning, InstanceAttributeAnimated,
    BundleMeshLighting,
    MeshStates, DirtyMeshStates, ModelInstanceAttributes, MeshInstanceState
);

pub type BundleMesh = (
    (
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
    ),
    (
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
        InstancedMeshTransparentSortCollection,
        SkeletonID,
    )
);

pub type BundleInstanceSource = (
    InstanceSourceRefs,
    DirtyInstanceSourceRefs,
    DirtyInstanceSourceForSingleBuffer,
);

pub type BundleInstance = (
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
    GeometryCullingMode,
);

pub type BundleMeshLighting = (
    MeshLightingMode,
    ModelLightingIndexs,
    ModelForcePointLightings,
    ModelForceSpotLightings,
    ModelForceHemiLightings,
);

pub fn sys_create_mesh(
    mut cmds: ResMut<ActionListMeshCreate>,
    mut commands: Commands,
    mut allocator: ResMut<ResBindBufferAllocator>,
    empty: Res<SingleEmptyEntity>,
    mut disposereadylist: ResMut<ActionListDisposeReadyForRef>,
    mut _disposecanlist: ResMut<ActionListDisposeCan>,
    lightlimit: Res<ModelLightLimit>,
    commonbindmodel: Res<CommonBindModel>,
    mut instancecmds: ResMut<ActionListInstanceMeshCreate>,
    // mut altermodel: Alter<(), (), BundleModel, ()>,
    // mut insert: Insert<PassObjBundle>,
) {
    cmds.drain().drain(..).for_each(|OpsMeshCreation(scene, entity, state )| {
        // log::error!("Create Mesh");
        if ActionMesh::init(&mut commands, entity, scene, &mut allocator, &empty, state, &lightlimit.0, &commonbindmodel) == false {
            disposereadylist.push(OpsDisposeReadyForRef::ops(entity));
        }
        // instancecmds.push(OpsInstanceMeshCreation::ops(entity, entity));
    });
}

pub fn sys_create_instanced_mesh(
    mut cmds: ResMut<ActionListInstanceMeshCreate>,
    mut commands: Commands,
    mut meshes: Query<(&SceneID, &mut InstanceSourceRefs, &mut DirtyInstanceSourceRefs, &ModelInstanceAttributes)>,
    // mut alter: Alter<(), (), (ModelInstanceAttributes, TargetAnimatorableIsRunning, InstanceAttributeAnimated, (TransformNodeBundle, InstanceBundle)), ()>,
) {
    cmds.drain().drain(..).for_each(|OpsInstanceMeshCreation(source, instance, count)| {
        if let Ok((id_scene, mut instancelist, mut flag, instanceattrs)) = meshes.get_mut(source) {

            let instanceattrs = instanceattrs.clone();

            if let Some(mut commands) = commands.get_entity(instance) {
                let bundle = (
                    instanceattrs,
                    TargetAnimatorableIsRunning,
                    InstanceAttributeAnimated::default(),
                    ActionInstanceMesh::init(source, id_scene.0),
                );
                commands.insert(bundle);
                // alter.alter(instance, bundle);
    
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
    mut command: Commands,
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
            if let Some(offset) = attributes.animator(&attr, item, &mut command, &mut animatorablefloat, &mut animatorablevec2s, &mut animatorablevec3s, &mut animatorablevec4s, &mut animatorableuints, &mut animatorablesints) {
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
    mut cmds: ResMut<ActionListMeshStateModify>,
    mut value_cmds: ResMut<ActionListAbstructMeshValueStateModify>,
    mut castshadows: Query<&mut MeshCastShadow>,
    mut receiveshadows: Query<&mut MeshReceiveShadow>,
    mut align_items: Query<&mut RenderAlignment>,
    mut scalingode_items: Query<&mut ScalingMode>,
    mut velocity_items: Query<&mut ModelVelocity>,
    mut indices_items: Query<(&mut IndiceRenderRange, &mut RecordIndiceRenderRange)>,
    mut vertexrange_items: Query<&mut VertexRenderRange>,
    mut culling_items: Query<&mut GeometryCullingMode>,
    skinoff_items: Query<&BindModel>,
) {
    cmds.drain().drain(..).for_each(|OpsMeshStateModify(entity, cmd)| {
        match cmd {
            EMeshStateModify::Alignment(val) => if let Ok(mut item) = align_items.get_mut(entity) {
                // log::warn!("RenderAlignment: {:?}", (val));
                *item = RenderAlignment(val);
            },
            EMeshStateModify::ScalingMode(val) => if let Ok(mut item) = scalingode_items.get_mut(entity) {
                *item = ScalingMode(val);
            },
            EMeshStateModify::CastShadow(val) => if let Ok(mut castshadow) = castshadows.get_mut(entity) {
                if val != castshadow.0 {
                    *castshadow = MeshCastShadow(val);
                }
            },
            EMeshStateModify::ReceiveShadow(val) => if let Ok(mut receiveshadow) = receiveshadows.get_mut(entity) {
                if val != receiveshadow.0 {
                    *receiveshadow = MeshReceiveShadow(val);
                }
            },
            EMeshStateModify::BoundingCullingMode(val) => if let Ok(mut cullingmode) = culling_items.get_mut(entity) {
                if val != cullingmode.0 {
                    cullingmode.0 = val;
                }
            },
        }
    });
    value_cmds.drain().drain(..).for_each(|OpsAbstructMeshValueStateModify(entity, val)| {
        match val {
            EMeshValueStateModify::BoneOffset(val) => if let Ok(bind) = skinoff_items.get(entity) {
                bind.0.as_ref().unwrap().data().write_data(ShaderBindModelAboutMatrix::OFFSET_U32_A as usize, bytemuck::cast_slice(&[val]));
            },
            EMeshValueStateModify::IndiceRange(val) => if let Ok((mut item, mut record)) = indices_items.get_mut(entity) {
                *record = RecordIndiceRenderRange(IndiceRenderRange::new(val.clone()));
                *item = IndiceRenderRange::new(val);
            },
            EMeshValueStateModify::VertexRange(val) => if let Ok(mut item) = vertexrange_items.get_mut(entity) {
                // *record = RecordIndiceRenderRange(IndiceRenderRange(val.clone()));
                *item = VertexRenderRange(val);
            },
            EMeshValueStateModify::Velocity(x, y, z) => if let Ok(mut item) = velocity_items.get_mut(entity) {
                *item = ModelVelocity(Vector3::new(x, y, z));
            },
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

    mut forcelight_cmds: ResMut<ActionListMeshForceLighting>,
    mut pointlight_items: Query<&mut ModelForcePointLightings>,
    mut spotlight_items: Query<&mut ModelForceSpotLightings>,
    mut hemilight_items: Query<&mut ModelForceHemiLightings>,
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

    forcelight_cmds.drain().drain(..).for_each(|OpsMeshForceLighting(entity, light, isadd)| {
        // log::warn!("Range: {:?}", val);
        match isadd {
            EMeshForceLighting::ForcePointLighting(isadd) => if let Ok(mut item) = pointlight_items.get_mut(entity) {
                // *record = RecordIndiceRenderRange(IndiceRenderRange(val.clone()));
                match item.0.binary_search(&light) {
                    Ok(idx)  => { if isadd == false { item.0.remove(idx); } },
                    Err(idx) => { if isadd == true  { item.0.insert(idx, light); } },
                }
            },
            EMeshForceLighting::ForceSpotLighting(isadd) => if let Ok(mut item) = spotlight_items.get_mut(entity) {
                // *record = RecordIndiceRenderRange(IndiceRenderRange(val.clone()));
                match item.0.binary_search(&light) {
                    Ok(idx)  => { if isadd == false { item.0.remove(idx); } },
                    Err(idx) => { if isadd == true  { item.0.insert(idx, light); } },
                }
            },
            EMeshForceLighting::ForceHemiLighting(isadd) => if let Ok(mut item) = hemilight_items.get_mut(entity) {
                // *record = RecordIndiceRenderRange(IndiceRenderRange(val.clone()));
                match item.0.binary_search(&light) {
                    Ok(idx)  => { if isadd == false { item.0.remove(idx); } },
                    Err(idx) => { if isadd == true  { item.0.insert(idx, light); } },
                }
            },
        }
    });
}

pub struct ActionMesh;
impl ActionMesh {
    pub fn init(
        commands: &mut Commands,
        entity: Entity,
        scene: Entity,
        allocator: &mut ResBindBufferAllocator,
        empty: &SingleEmptyEntity,
        mut state: MeshInstanceState,
        lightlimit: &LightLimitInfo,
        commonbindmodel: &CommonBindModel,
        // altermodel: &mut Alter<(), (), BundleModel, ()>,
    ) -> bool {
        // state.instance_matrix = true;
        // state.instances.push(
        //     CustomVertexAttribute::new(
        //         Atom::from(EBuildinVertexAtribute::TextureIDs.var_code()),
        //         Atom::from(String::from(ShaderVarVarying::TEXTURE_IDS) + "=" + EBuildinVertexAtribute::TextureIDs.var_code() + ";\n"),
        //         ECustomVertexType::UVec4, None
        //     )
        // );
        // state.instances.push(EVertexAttribute::Buildin(EBuildinVertexAtribute::ModelMaterialSkin));
        
        let meshinstanceattributes = ModelInstanceAttributes::new(&state.instances, state.instance_matrix);
        if commands.get_entity(entity).is_none() {
            return false;
        };

        let id01 = commands.spawn(create_passobj(entity, scene, empty.id(), PassTag::PASS_TAG_01)).id();
        let id02 = commands.spawn(create_passobj(entity, scene, empty.id(), PassTag::PASS_TAG_02)).id();
        let id03 = commands.spawn(create_passobj(entity, scene, empty.id(), PassTag::PASS_TAG_03)).id();
        let id04 = commands.spawn(create_passobj(entity, scene, empty.id(), PassTag::PASS_TAG_04)).id();
        let id05 = commands.spawn(create_passobj(entity, scene, empty.id(), PassTag::PASS_TAG_05)).id();
        let id06 = commands.spawn(create_passobj(entity, scene, empty.id(), PassTag::PASS_TAG_06)).id();
        let id07 = commands.spawn(create_passobj(entity, scene, empty.id(), PassTag::PASS_TAG_07)).id();
        let id08 = commands.spawn(create_passobj(entity, scene, empty.id(), PassTag::PASS_TAG_08)).id();
        let passids = PassIDs([id01, id02, id03, id04, id05, id06, id07, id08]);

        let mut entitycmd = commands.get_entity(entity).unwrap();
        let instanceattr = meshinstanceattributes.bytes().len() > 0;

        let modellightidx = ModelLightingIndexs::new(allocator, lightlimit);
        let lightbundle = (
            MeshLightingMode::default(),
            modellightidx,
            ModelForcePointLightings::default(),
            ModelForceSpotLightings::default(),
            ModelForceHemiLightings::default(),
        );
        let bundle: BundleModel = (
            ActionTransformNode::init(scene),
            ActionMesh::as_mesh(empty.id()),
            ActionMesh::as_instance_source(),
            TargetAnimatorableIsRunning, InstanceAttributeAnimated::default(),
            lightbundle,
            MeshStates::default(),
            DirtyMeshStates,
            meshinstanceattributes,
            state,
        );

        if instanceattr {
            entitycmd.insert((bundle, commonbindmodel.0.clone(), ModelStatic, passids));
        } else {
            if let Some(bind) = BindModel::new(allocator) {
                entitycmd.insert((bundle, bind, passids));
            }
        }

        // let meshinstanceattributes = ModelInstanceAttributes::new(&state.instances, state.instance_matrix);
        // let mut entitycmd = if let Some(cmd) = commands.get_entity(entity) {
        //     cmd
        // } else {
        //     return false;
        // };
        // let modellightidx = ModelLightingIndexs::new(allocator, lightlimit);

        // if meshinstanceattributes.bytes().len() > 0 {
        //     entitycmd.insert((commonbindmodel.0.clone(), ModelStatic));
        //     log::warn!(">>>>>>>>>>> sys_create_mesh 1");
        // } else {
        //     if let Some(bind) = BindModel::new(allocator) {
        //         // log::info!("BindModel New");
        //         entitycmd.insert((bind,));
        //         log::warn!(">>>>>>>>>>> sys_create_mesh 2");
        //     }
        // }
        // let lightbundle = (
        //     MeshLightingMode::default(),
        //     modellightidx,
        //     ModelForcePointLightings::default(),
        //     ModelForceSpotLightings::default(),
        //     ModelForceHemiLightings::default(),
        // );
        // let bundle: BundleModel = (
        //     ActionTransformNode::init(scene),
        //     ActionMesh::as_mesh(empty.id()),
        //     ActionMesh::as_instance_source(),
        //     TargetAnimatorableIsRunning, InstanceAttributeAnimated::default(),
        //     lightbundle,
        //     MeshStates::default(),
        //     DirtyMeshStates,
        //     meshinstanceattributes,
        //     state,
        // );

        // log::warn!(">>>>>>>>>>> sys_create_mesh 3");
        // entitycmd.insert(bundle);
        // // altermodel.alter(entity, bundle);
        
        // log::warn!(">>>>>>>>>>> sys_create_mesh OKKK");

        // // entitycmd.insert(ModelPointLightingDirty::default());
        // // entitycmd.insert(ModelSpotLightingDirty::default());
        // // entitycmd.insert(ModelHemiLightingDirty::default());

        // let id01 = commands.spawn(create_passobj(entity, scene, empty.id(), PassTag::PASS_TAG_01)).id();
        // let id02 = commands.spawn(create_passobj(entity, scene, empty.id(), PassTag::PASS_TAG_02)).id();
        // let id03 = commands.spawn(create_passobj(entity, scene, empty.id(), PassTag::PASS_TAG_03)).id();
        // let id04 = commands.spawn(create_passobj(entity, scene, empty.id(), PassTag::PASS_TAG_04)).id();
        // let id05 = commands.spawn(create_passobj(entity, scene, empty.id(), PassTag::PASS_TAG_05)).id();
        // let id06 = commands.spawn(create_passobj(entity, scene, empty.id(), PassTag::PASS_TAG_06)).id();
        // let id07 = commands.spawn(create_passobj(entity, scene, empty.id(), PassTag::PASS_TAG_07)).id();
        // let id08 = commands.spawn(create_passobj(entity, scene, empty.id(), PassTag::PASS_TAG_08)).id();
        // commands.entity(entity).insert((PassIDs([id01, id02, id03, id04, id05, id06, id07, id08]),));

        return true;
    }
    pub(crate) fn as_mesh(
        geometry: Entity,
    ) -> BundleMesh {
        // let mut unclipdepth = false;
        // #[cfg(not(target_arch = "wasm32"))]
        // {
        //     unclipdepth = true;
        // }
        let unclipdepth = false;
        ((
            AbstructMesh,
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
        ),(
            TransparentSortParam::opaque(),
            BindSkinValue(None),
            ModelVelocity::default(),
            RenderAlignment::default(),
            ScalingMode::default(),
            IndiceRenderRange::default(),
            RecordIndiceRenderRange::default(),
            VertexRenderRange::default(),
            GeometryBounding::default(),
            GeometryCullingMode::default(),
            InstancedMeshTransparentSortCollection(vec![]),
            SkeletonID(None),
        ))
    }

    pub fn as_instance_source() -> BundleInstanceSource {
        (
            InstanceSourceRefs::default(),
            DirtyInstanceSourceRefs::default(),
            DirtyInstanceSourceForSingleBuffer::default(),
        )
    }
}
pub struct ActionInstanceMesh;
impl ActionInstanceMesh {
    pub fn init(
        source: Entity,
        scene: Entity,
    ) -> (TransformNodeBundle, BundleInstance) {
        (
            ActionTransformNode::init(scene),
            ActionInstanceMesh::as_instance(source)
        )
    }
    pub(crate) fn as_instance(
        source: Entity,
    ) -> BundleInstance {
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
            GeometryCullingMode::default(),
        )
    }
}

fn create_passobj(
    idmodel: Entity,
    scene: Entity,
    empty: Entity,
    tag: PassTag,
) -> (BundleEntity, PassObjInitBundle, PassTag) {
    (
        ActionEntity::init(),
        ActionPassObject::init(empty, idmodel, scene),
        tag
    )
}