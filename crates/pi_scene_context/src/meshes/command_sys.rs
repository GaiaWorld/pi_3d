
use pi_scene_shell::prelude::*;

use crate::{
    cullings::prelude::*, geometry::{
        instance::{types::{InstanceAttributeAnimated, ModelInstanceAttributes}, DirtyInstanceSourceForSingleBuffer, EInstanceSortMode}, prelude::*
    },
    layer_mask::prelude::*,
    object::ActionEntity,
    pass::*,
    prelude::{TypeAnimeAssetMgrs, TypeAnimeContexts},
    renderers::prelude::*,
    skeleton::prelude::*,
    transforms::command_sys::{ActionTransformNode, TransformNodeBundle}
};

use super::{
    abstract_mesh::AbstructMesh, command::*, lighting::*, model::*, prelude::FlagMeshNeedRecheckForView
};


pub type BundleModelStatic = (GeometryID, ModelStatic, BindModel);
pub type BundleModel = (
    TransformNodeBundle,
    BundleMesh,
    BundleInstanceSource,
    TargetAnimatorableIsRunning, InstanceAttributeAnimated,
    BundleMeshLighting,
    ModelInstanceAttributes, MeshInstanceState
);

pub type BundleMesh = (
    (
        AbstructMesh,
        FlagMeshNeedRecheckForView,
        Mesh,
        GeometryID,
        RenderGeometryEable,
        RenderWorldMatrix,
        FlagRenderWorldMatrix,
        MeshCastShadow,
        MeshReceiveShadow,
        LayerMask,
        AbstructMeshCullingFlag,
        EInstanceSortMode,
        RenderPoseMatrix,
    ),
    (
        RenderQueueSortParam,
        BindSkinValue,
        ModelVelocity,
        RenderAlignment,
        ScalingMode,
        IndiceRenderRange,
        VertexRenderRange,
        GeometryBounding,
        GeometryCullingMode,
        ItemCullingDirty,
        InstancedMeshTransparentSortCollection,
        SkeletonID,
    )
);

pub type BundleInstanceSource = (
    InstanceSourceRefs,
    DirtyInstanceSourceForSingleBuffer,
);

pub type BundleInstance = (
    AbstructMesh,
    FlagMeshNeedRecheckForView,
    AbstructMeshCullingFlag,
    RenderQueueSortParam,
    InstanceMesh,
    RenderWorldMatrix,
    FlagRenderWorldMatrix,
    ModelVelocity,
    ScalingMode,
    ItemCullingDirty,
    RenderPoseMatrix,
);

pub type BundleMeshLighting = (
    MeshLightingMode,
    ModelLightingIndexs,
    ModelForceLightings,
);

pub fn sys_create_mesh(
    mut cmds: ResMut<ActionListMeshCreate>,
    mut allocator: ResMut<ResBindBufferAllocator>,
    empty: Res<SingleEmptyEntity>,
    mut commands: Commands,
    mut disposereadylist: ResMut<ActionListDisposeReadyForRef>,
    mut _disposecanlist: ResMut<ActionListDisposeCan>,
    lightlimit: Res<ModelLightLimit>,
    commonbindmodel: Res<CommonBindModel>,
    mut altermodel: Alter<(), (), (BundleModel, BindModel, PassIDs, ModelStatic), ()>,
    mut passinsert: Insert<(BundleEntity, PassObjInitBundle, PassTag)>,
    // mut insert: Insert<PassObjBundle>,
) {
    // let time1 = pi_time::Instant::now();
    let mut count = 0;
    cmds.drain().for_each(|OpsMeshCreation(scene, entity, state )| {
        // log::error!("Create Mesh");
        // if ActionMesh::init(&mut commands, entity, scene, &mut allocator, &empty, state, &lightlimit.0, &commonbindmodel) == false {
        if ActionMesh::init(
            entity, &mut commands, scene, &mut allocator, &empty, state, &lightlimit.0, &commonbindmodel,
            &mut altermodel, &mut passinsert 
        ) == false {
            disposereadylist.push(OpsDisposeReadyForRef::ops(entity));
        }
        count += 1;
        // instancecmds.push(OpsInstanceMeshCreation::ops(entity, entity));
    });

    if count > 0 {
        // log::error!("Creat Mesh Count {:?}, Time: {:?}", count, pi_time::Instant::now() - time1);
    }
}

pub fn sys_create_instanced_mesh(
    mut cmds: ResMut<ActionListInstanceMeshCreate>,
    // mut commands: Commands,
    mut meshes: Query<(&SceneID, &mut InstanceSourceRefs, &ModelInstanceAttributes, &mut FlagMeshNeedRecheckForView)>,
    mut alter: Alter<(), (), (ModelInstanceAttributes, TargetAnimatorableIsRunning, InstanceAttributeAnimated, (TransformNodeBundle, BundleInstance)), ()>,
) {
    cmds.drain().for_each(|OpsInstanceMeshCreation(source, instance, count)| {
        if let Ok((id_scene, mut instancelist, instanceattrs, mut flagview)) = meshes.get_mut(source) {

            let instanceattrs = instanceattrs.clone();

            let bundle = (
                instanceattrs,
                TargetAnimatorableIsRunning,
                InstanceAttributeAnimated::default(),
                ActionInstanceMesh::init(source, id_scene.0),
            );
            // commands.get_entity(instance).unwrap().insert(bundle);
            alter.alter(instance, bundle);

            instancelist.insert(instance);
            *flagview = FlagMeshNeedRecheckForView;
            // 
        } else {
            // if count < 2 {
            //     cmds.push(OpsInstanceMeshCreation(source, instance, count + 1))
            // }
        }
    });
}

pub fn sys_create_abstract_posematrix(
    mut cmds: ResMut<ActionListAbstractMeshPose>,
    mut flagrendermatrix: Query<&mut FlagRenderWorldMatrix>,
    mut commands: Commands,
) {
    cmds.drain().for_each(|OpsAbstractMeshPose(entity, matrix)| {
        if let (Some(mut entitycmd), Ok(mut flag)) = (commands.get_entity(entity), flagrendermatrix.get_mut(entity)) {
            entitycmd.insert(RenderPoseMatrix(matrix));
            *flag = FlagRenderWorldMatrix;
        }
    })
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
    mut targetanimations: ResMut<ActionListAnimationGroupAction>,
    instances: Query<&InstanceMesh>,
    mut meshes: Query<&mut InstanceSourceRefs>,
) {
    cmds.drain().for_each(|OpsTargetAnimationAttribute(item, attr, group, curve)| {
        let mut mesh = item;
        if let Ok(instance) = instances.get(item) {
            mesh = instance.0;
        }
        if let Ok((mut attributes, mut animated)) = items.get_mut(item) {
            if let Some(offset) = attributes.animator(&attr, item, &mut command, &mut animatorablefloat, &mut animatorablevec2s, &mut animatorablevec3s, &mut animatorablevec4s, &mut animatorableuints, &mut animatorablesints) {
                match offset.entity() {
                    Some(target) => {
                        animated.add(&attr);
                        if let Some(atype) = offset.atype() {
                            match atype {
                                EAnimatorableType::Vec4 => if let Some(curve) = anime_assets.vec4s.get(&curve) {
                                    let anime = anime_contexts.vec4s.ctx.create_animation(0, AssetTypeFrameCurve::from(curve));
                                    targetanimations.push(OpsAnimationGroupAction::addtarget(group, target, anime));
                                },
                                EAnimatorableType::Vec3 => if let Some(curve) = anime_assets.vec3s.get(&curve) {
                                    let anime = anime_contexts.vec3s.ctx.create_animation(0, AssetTypeFrameCurve::from(curve));
                                    targetanimations.push(OpsAnimationGroupAction::addtarget(group, target, anime));
                                },
                                EAnimatorableType::Vec2 => if let Some(curve) = anime_assets.vec2s.get(&curve) {
                                    let anime = anime_contexts.vec2s.ctx.create_animation(0, AssetTypeFrameCurve::from(curve));
                                    targetanimations.push(OpsAnimationGroupAction::addtarget(group, target, anime));
                                },
                                EAnimatorableType::Float => if let Some(curve) = anime_assets.float.get(&curve) {
                                    let anime = anime_contexts.float.ctx.create_animation(0, AssetTypeFrameCurve::from(curve));
                                    targetanimations.push(OpsAnimationGroupAction::addtarget(group, target, anime));
                                },
                                EAnimatorableType::Uint => if let Some(curve) = anime_assets.uints.get(&curve) {
                                    let anime = anime_contexts.uints.ctx.create_animation(0, AssetTypeFrameCurve::from(curve));
                                    targetanimations.push(OpsAnimationGroupAction::addtarget(group, target, anime));
                                },
                                EAnimatorableType::Int => if let Some(curve) = anime_assets._ints.get(&curve) {
                                    let anime = anime_contexts._ints.ctx.create_animation(0, AssetTypeFrameCurve::from(curve));
                                    targetanimations.push(OpsAnimationGroupAction::addtarget(group, target, anime));
                                },
                            }
                        }
                    },
                    None => { },
                }
                
            }
            if let Ok(mut flag) = meshes.get_mut(mesh) {
                flag.dirty = true;
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
    mut indices_items: Query<&mut IndiceRenderRange>,
    mut vertexrange_items: Query<&mut VertexRenderRange>,
    mut instance_sortmodes: Query<&mut EInstanceSortMode>,
    mut culling_items: Query<(&mut GeometryCullingMode, &mut ItemCullingDirty)>,
    mut flagrendermatrix: Query<&mut FlagRenderWorldMatrix>,
    mut records: ResMut<AnimeTargetRecordValues<IndiceRenderRange>>,
    skinoff_items: Query<&BindModel>,
) {
    cmds.drain().for_each(|OpsMeshStateModify(entity, cmd)| {
        match cmd {
            EMeshStateModify::Alignment(val) => if let Ok(mut item) = align_items.get_mut(entity) {
                // log::warn!("RenderAlignment: {:?}", (val));
                *item = RenderAlignment(val);
                if let Ok(mut flag) = flagrendermatrix.get_mut(entity) {
                    *flag = FlagRenderWorldMatrix;
                }
            },
            EMeshStateModify::ScalingMode(val) => if let Ok(mut item) = scalingode_items.get_mut(entity) {
                *item = ScalingMode(val);
                if let Ok(mut flag) = flagrendermatrix.get_mut(entity) {
                    *flag = FlagRenderWorldMatrix;
                }
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
            EMeshStateModify::BoundingCullingMode(val) => if let Ok((mut cullingmode, mut flag)) = culling_items.get_mut(entity) {
                if val != cullingmode.0 {
                    cullingmode.0 = val;
                    *flag = ItemCullingDirty;
                } else {
                    // log::error!("BoundingCullingMode Same. {:?}", entity);
                }
            } else {
                // log::error!("BoundingCullingMode Not Found. {:?}", entity);
            },
            EMeshStateModify::InstanceSortMode(val) => if let Ok(mut mode) = instance_sortmodes.get_mut(entity) {
                if val != *mode {
                    *mode = val;
                }
            },
        }
    });
    value_cmds.drain().for_each(|OpsAbstructMeshValueStateModify(entity, val)| {
        match val {
            EMeshValueStateModify::BoneOffset(val) => if let Ok(bind) = skinoff_items.get(entity) {
                bind.0.as_ref().unwrap().data().write_data(ShaderBindModelAboutMatrix::OFFSET_U32_A as usize, bytemuck::cast_slice(&[val]));
            },
            EMeshValueStateModify::IndiceRange(val) => if let Ok(mut item) = indices_items.get_mut(entity) {
                records.insert(entity, IndiceRenderRange::new(val.clone()));
                *item = IndiceRenderRange::new(val);
            },
            EMeshValueStateModify::VertexRange(val) => if let Ok(mut item) = vertexrange_items.get_mut(entity) {
                // *record = RecordIndiceRenderRange(IndiceRenderRange(val.clone()));
                *item = VertexRenderRange::new(val);
            },
            EMeshValueStateModify::Velocity(x, y, z) => if let Ok(mut item) = velocity_items.get_mut(entity) {
                *item = ModelVelocity(Vector3::new(x, y, z));
                if let Ok(mut flag) = flagrendermatrix.get_mut(entity) {
                    *flag = FlagRenderWorldMatrix;
                }
            },
        }
    });
}

pub fn sys_act_instance_attribute(
    mut cmdsfloat: ResMut<ActionListInstanceAttr>,
    mut instances: Query<(&InstanceMesh, &mut ModelInstanceAttributes)>,

    mut animator_vec4: ResMut<ActionListAnimatorableVec4>,
    mut animator_vec3: ResMut<ActionListAnimatorableVec3>,
    mut animator_vec2: ResMut<ActionListAnimatorableVec2>,
    mut animator_float: ResMut<ActionListAnimatorableFloat>,
    mut animator_uint: ResMut<ActionListAnimatorableUint>,
    mut animator_sint: ResMut<ActionListAnimatorableSint>,

    mut forcelight_cmds: ResMut<ActionListMeshForceLighting>,
    mut light_items: Query<&mut ModelForceLightings>,
    mut meshes: Query<&mut InstanceSourceRefs>,
) {

    cmdsfloat.drain().for_each(|OpsInstanceAttr(instance, val, attr)| {
        if let Ok((inssource, mut attributes)) = instances.get_mut(instance) {
            if let Some(info) = attributes.offset(&attr) {
                let mut offset = info.offset() as usize;
                if let Some(target) = info.entity() {
                    // log::error!("Push 。。。。");
                    match val {
                        EInstanceAttr::Float(val)   => { animator_float.push(OpsAnimatorableFloat::ops(target, instance, AnimatorableFloat(val), EAnimatorableEntityType::Attribute)); },
                        EInstanceAttr::Int (val)    => { animator_sint.push(OpsAnimatorableSint::ops(target, instance, AnimatorableSint(val), EAnimatorableEntityType::Attribute)); },
                        EInstanceAttr::Uint(val)    => { animator_uint.push(OpsAnimatorableUint::ops(target, instance, AnimatorableUint(val), EAnimatorableEntityType::Attribute)); },
                        EInstanceAttr::Vec4(val) => { animator_vec4.push(OpsAnimatorableVec4::ops(target, instance, AnimatorableVec4::from(val.as_slice()), EAnimatorableEntityType::Attribute)); },
                        EInstanceAttr::Vec3(val) => { animator_vec3.push(OpsAnimatorableVec3::ops(target, instance, AnimatorableVec3::from(val.as_slice()), EAnimatorableEntityType::Attribute)); },
                        EInstanceAttr::Vec2(val) => { animator_vec2.push(OpsAnimatorableVec2::ops(target, instance, AnimatorableVec2::from(val.as_slice()), EAnimatorableEntityType::Attribute)); },
                        EInstanceAttr::U8x4(val)   => bytemuck::cast_slice(&val).iter().for_each(|v| { attributes.bytes_mut()[offset] = *v; offset += 1; }),
                        EInstanceAttr::U16x4(val) => bytemuck::cast_slice(&val).iter().for_each(|v| { attributes.bytes_mut()[offset] = *v; offset += 1; }),
                        EInstanceAttr::U16x2(val) => bytemuck::cast_slice(&val).iter().for_each(|v| { attributes.bytes_mut()[offset] = *v; offset += 1; }),
                        EInstanceAttr::IVec4(val) => bytemuck::cast_slice(&val).iter().for_each(|v| { attributes.bytes_mut()[offset] = *v; offset += 1; }),
                    };
                } else {
                    match val {
                        EInstanceAttr::Float(val) => bytemuck::cast_slice(&[val]).iter().for_each(|v| { attributes.bytes_mut()[offset] = *v; offset += 1; }),
                        EInstanceAttr::Uint(val) => bytemuck::cast_slice(&[val]).iter().for_each(|v| { attributes.bytes_mut()[offset] = *v; offset += 1; }),
                        EInstanceAttr::Int(val) => bytemuck::cast_slice(&[val]).iter().for_each(|v| { attributes.bytes_mut()[offset] = *v; offset += 1; }),
                        EInstanceAttr::Vec4(val) => bytemuck::cast_slice(&val).iter().for_each(|v| { attributes.bytes_mut()[offset] = *v; offset += 1; }),
                        EInstanceAttr::Vec3(val) => bytemuck::cast_slice(&val).iter().for_each(|v| { attributes.bytes_mut()[offset] = *v; offset += 1; }),
                        EInstanceAttr::Vec2(val) => bytemuck::cast_slice(&val).iter().for_each(|v| { attributes.bytes_mut()[offset] = *v; offset += 1; }),
                        EInstanceAttr::U8x4(val)   => bytemuck::cast_slice(&val).iter().for_each(|v| { attributes.bytes_mut()[offset] = *v; offset += 1; }),
                        EInstanceAttr::U16x4(val) => bytemuck::cast_slice(&val).iter().for_each(|v| { attributes.bytes_mut()[offset] = *v; offset += 1; }),
                        EInstanceAttr::U16x2(val) => bytemuck::cast_slice(&val).iter().for_each(|v| { attributes.bytes_mut()[offset] = *v; offset += 1; }),
                        EInstanceAttr::IVec4(val) => bytemuck::cast_slice(&val).iter().for_each(|v| { attributes.bytes_mut()[offset] = *v; offset += 1; }),
                    }
                    ;
                }
                
                if let Ok(mut flag) = meshes.get_mut(inssource.0) {
                    flag.dirty = true;
                }
            }
        }
    });

    forcelight_cmds.drain().for_each(|OpsMeshForceLighting(entity, light, isadd)| {
        // log::warn!("Range: {:?}", val);
        match isadd {
            EMeshForceLighting::ForcePointLighting(isadd) => if let Ok(mut item) = light_items.get_mut(entity) {
                // *record = RecordIndiceRenderRange(IndiceRenderRange(val.clone()));
                match item.point.binary_search(&light) {
                    Ok(idx)  => { if isadd == false { item.point.remove(idx); } },
                    Err(idx) => { if isadd == true  { item.point.insert(idx, light); } },
                }
            },
            EMeshForceLighting::ForceSpotLighting(isadd) => if let Ok(mut item) = light_items.get_mut(entity) {
                // *record = RecordIndiceRenderRange(IndiceRenderRange(val.clone()));
                match item.spot.binary_search(&light) {
                    Ok(idx)  => { if isadd == false { item.spot.remove(idx); } },
                    Err(idx) => { if isadd == true  { item.spot.insert(idx, light); } },
                }
            },
            EMeshForceLighting::ForceHemiLighting(isadd) => if let Ok(mut item) = light_items.get_mut(entity) {
                // *record = RecordIndiceRenderRange(IndiceRenderRange(val.clone()));
                match item.hemi.binary_search(&light) {
                    Ok(idx)  => { if isadd == false { item.hemi.remove(idx); } },
                    Err(idx) => { if isadd == true  { item.hemi.insert(idx, light); } },
                }
            },
        }
    });
}

pub struct ActionMesh;
impl ActionMesh {
    pub fn init(
        entity: Entity,
        _commands: &mut Commands,
        scene: Entity,
        allocator: &mut ResBindBufferAllocator,
        empty: &SingleEmptyEntity,
        mut state: MeshInstanceState,
        lightlimit: &LightLimitInfo,
        commonbindmodel: &CommonBindModel,
        altermodel: &mut Alter<(), (), (BundleModel, BindModel, PassIDs, ModelStatic), ()>,
        passinsert: &mut Insert<(BundleEntity, PassObjInitBundle, PassTag)>,
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

        // let passids = PassIDs([entity, entity, entity, entity, entity, entity, entity, entity]);
        let id01 = passinsert.insert(create_passobj(entity, scene, empty.id(), PassTag::PASS_TAG_01));
        let id02 = passinsert.insert(create_passobj(entity, scene, empty.id(), PassTag::PASS_TAG_02));
        let id03 = passinsert.insert(create_passobj(entity, scene, empty.id(), PassTag::PASS_TAG_03));
        let id04 = passinsert.insert(create_passobj(entity, scene, empty.id(), PassTag::PASS_TAG_04));
        let id05 = passinsert.insert(create_passobj(entity, scene, empty.id(), PassTag::PASS_TAG_05));
        let id06 = passinsert.insert(create_passobj(entity, scene, empty.id(), PassTag::PASS_TAG_06));
        let id07 = passinsert.insert(create_passobj(entity, scene, empty.id(), PassTag::PASS_TAG_07));
        let id08 = passinsert.insert(create_passobj(entity, scene, empty.id(), PassTag::PASS_TAG_08));
        // let id01 = commands.spawn(create_passobj(entity, scene, empty.id(), PassTag::PASS_TAG_01)).id();
        // let id02 = commands.spawn(create_passobj(entity, scene, empty.id(), PassTag::PASS_TAG_02)).id();
        // let id03 = commands.spawn(create_passobj(entity, scene, empty.id(), PassTag::PASS_TAG_03)).id();
        // let id04 = commands.spawn(create_passobj(entity, scene, empty.id(), PassTag::PASS_TAG_04)).id();
        // let id05 = commands.spawn(create_passobj(entity, scene, empty.id(), PassTag::PASS_TAG_05)).id();
        // let id06 = commands.spawn(create_passobj(entity, scene, empty.id(), PassTag::PASS_TAG_06)).id();
        // let id07 = commands.spawn(create_passobj(entity, scene, empty.id(), PassTag::PASS_TAG_07)).id();
        // let id08 = commands.spawn(create_passobj(entity, scene, empty.id(), PassTag::PASS_TAG_08)).id();
        let passids = PassIDs([id01, id02, id03, id04, id05, id06, id07, id08]);

        // let mut entitycmd = commands.get_entity(entity).unwrap();
        let instanceattr = meshinstanceattributes.bytes().len() > 0;

        let modellightidx = ModelLightingIndexs::new(allocator, lightlimit);
        let lightbundle = (
            MeshLightingMode::default(),
            modellightidx,
            ModelForceLightings::default(),
        );
        let bundle: BundleModel = (
            ActionTransformNode::init(scene),
            ActionMesh::as_mesh(empty.id()),
            ActionMesh::as_instance_source(),
            TargetAnimatorableIsRunning, InstanceAttributeAnimated::default(),
            lightbundle,
            // MeshStates::default(),
            // DirtyMeshStates,
            meshinstanceattributes,
            state,
        );

        if instanceattr {
            let _ = altermodel.alter(entity, (bundle, commonbindmodel.0.clone(), passids, ModelStatic(true)));
        } else {
            if let Some(bind) = BindModel::new(allocator) {
                // entitycmd.insert((bundle, bind, passids));
                let _ = altermodel.alter(entity, (bundle, bind, passids, ModelStatic(false)));
            }
        }

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
            FlagMeshNeedRecheckForView,
            Mesh,
            GeometryID(geometry),
            RenderGeometryEable(false),
            RenderWorldMatrix(Matrix::identity()),
            FlagRenderWorldMatrix,

            // RenderMatrixDirty(true),
            MeshCastShadow(false),
            MeshReceiveShadow(false),
            // PassDirtyBindEffectValue(0),
            // FlagPassDirtyBindEffectValue,
            // PassDirtyBindEffectTextures(0),
            // FlagPassDirtyBindEffectTextures,
            LayerMask::default(),
            AbstructMeshCullingFlag(false),
            EInstanceSortMode::default(),
            RenderPoseMatrix::default(),
        ),(
            RenderQueueSortParam::opaque(),
            BindSkinValue(None),
            ModelVelocity::default(),
            RenderAlignment::default(),
            ScalingMode::default(),
            IndiceRenderRange::default(),
            VertexRenderRange::default(),
            GeometryBounding::default(),
            GeometryCullingMode::default(),
            ItemCullingDirty::default(),
            InstancedMeshTransparentSortCollection::default(),
            SkeletonID(None),
        ))
    }

    pub fn as_instance_source() -> BundleInstanceSource {
        (
            InstanceSourceRefs::default(),
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
            FlagMeshNeedRecheckForView,
            AbstructMeshCullingFlag(false),
            RenderQueueSortParam::default(),
            InstanceMesh(source),
            // RenderMatrixDirty(true),
            RenderWorldMatrix(Matrix::identity()),
            FlagRenderWorldMatrix,

            ModelVelocity::default(),
            ScalingMode::default(),
            ItemCullingDirty::default(),
            RenderPoseMatrix::default(),
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