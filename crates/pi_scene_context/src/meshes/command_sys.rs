
use pi_scene_shell::prelude::*;

use crate::{
    cullings::prelude::*, geometry::{
        instance::{types::{InstanceAttributeAnimated, ModelInstanceAttributes}, DirtyInstanceSourceForSingleBuffer}, prelude::*
    }, layer_mask::prelude::*, object::ActionEntity, pass::*, prelude::{TypeAnimeAssetMgrs, TypeAnimeContexts}, renderers::prelude::*, skeleton::prelude::*, state::{DirtyMeshStates, MeshStates}, transforms::command_sys::ActionTransformNode
};

use super::{
    command::*,
    model::*,
    abstract_mesh::AbstructMesh,
    lighting::*,
};


pub fn sys_create_mesh(
    mut cmds: ResMut<ActionListMeshCreate>,
    mut commands: Commands,
    mut allocator: ResMut<ResBindBufferAllocator>,
    empty: Res<SingleEmptyEntity>,
    mut disposereadylist: ResMut<ActionListDisposeReadyForRef>,
    mut _disposecanlist: ResMut<ActionListDisposeCan>,
    lightlimit: Res<ModelLightLimit>,
    commonbindmodel: Res<CommonBindModel>,
) {
    cmds.drain().drain(..).for_each(|OpsMeshCreation(scene, entity, state )| {
        // log::error!("Create Mesh");
        if ActionMesh::init(&mut commands, entity, scene, &mut allocator, &empty, state, &lightlimit.0, &commonbindmodel) == false {
            disposereadylist.push(OpsDisposeReadyForRef::ops(entity));
        }
    });
}

pub fn sys_create_instanced_mesh(
    mut cmds: ResMut<ActionListInstanceMeshCreate>,
    mut commands: Commands,
    mut meshes: Query<(&SceneID, &mut InstanceSourceRefs, &mut DirtyInstanceSourceRefs, &ModelInstanceAttributes)>,
) {
    cmds.drain().drain(..).for_each(|OpsInstanceMeshCreation(source, instance, count)| {
        if let Ok((id_scene, mut instancelist, mut flag, instanceattrs)) = meshes.get_mut(source) {

            let instanceattrs = instanceattrs.clone();

            if let Some(mut commands) = commands.get_entity(instance) {
                commands.insert(instanceattrs);
                commands.insert(TargetAnimatorableIsRunning).insert(InstanceAttributeAnimated::default());
                ActionInstanceMesh::init(&mut commands, source, id_scene.0);
    
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

pub struct ActionMesh;
impl ActionMesh {
    pub fn init(
        commands: &mut Commands,
        entity: Entity,
        scene: Entity,
        allocator: &mut ResBindBufferAllocator,
        empty: &SingleEmptyEntity,
        state: MeshInstanceState,
        lightlimit: &LightLimitInfo,
        commonbindmodel: &CommonBindModel,
    ) -> bool {
        let meshinstanceattributes = ModelInstanceAttributes::new(&state.instances, state.instance_matrix);
        let mut entitycmd = if let Some(cmd) = commands.get_entity(entity) {
            cmd
        } else {
            return false;
        };

        ActionTransformNode::init(&mut entitycmd, scene);
        ActionMesh::as_mesh(&mut entitycmd, empty.id());
        ActionMesh::as_instance_source(&mut entitycmd);
        // ActionMesh::as_instance_source(&mut entitycmd);
        entitycmd.insert(TargetAnimatorableIsRunning).insert(InstanceAttributeAnimated::default());

        if meshinstanceattributes.bytes().len() > 0 {
            entitycmd.insert(commonbindmodel.0.clone());
            entitycmd.insert(ModelStatic);
        } else {
            if let Some(bind) = BindModel::new(allocator) {
                // log::info!("BindModel New");
                entitycmd.insert(bind);
            }
        }

        entitycmd.insert(MeshLightingMode::default());
        entitycmd.insert(ModelLightingIndexs::new(allocator, lightlimit));
        entitycmd.insert(ModelForcePointLightings::default());
        entitycmd.insert(ModelForceSpotLightings::default());
        entitycmd.insert(ModelForceHemiLightings::default());
        // entitycmd.insert(ModelPointLightingDirty::default());
        // entitycmd.insert(ModelSpotLightingDirty::default());
        // entitycmd.insert(ModelHemiLightingDirty::default());

        entitycmd.insert(MeshStates::default());
        entitycmd.insert(DirtyMeshStates);

        entitycmd.insert(meshinstanceattributes);
        entitycmd.insert(state);

        let id01 = commands.spawn_empty().id();
        let id02 = commands.spawn_empty().id();
        let id03 = commands.spawn_empty().id();
        let id04 = commands.spawn_empty().id();
        let id05 = commands.spawn_empty().id();
        let id06 = commands.spawn_empty().id();
        let id07 = commands.spawn_empty().id();
        let id08 = commands.spawn_empty().id();
        commands.entity(entity).insert(PassIDs([id01, id02, id03, id04, id05, id06, id07, id08]));
        create_passobj(commands, id01, entity, scene, empty.id(), PassTag::PASS_TAG_01);
        create_passobj(commands, id02, entity, scene, empty.id(), PassTag::PASS_TAG_02);
        create_passobj(commands, id03, entity, scene, empty.id(), PassTag::PASS_TAG_03);
        create_passobj(commands, id04, entity, scene, empty.id(), PassTag::PASS_TAG_04);
        create_passobj(commands, id05, entity, scene, empty.id(), PassTag::PASS_TAG_05);
        create_passobj(commands, id06, entity, scene, empty.id(), PassTag::PASS_TAG_06);
        create_passobj(commands, id07, entity, scene, empty.id(), PassTag::PASS_TAG_07);
        create_passobj(commands, id08, entity, scene, empty.id(), PassTag::PASS_TAG_08);
        return true;
    }
    pub(crate) fn as_mesh(
        commands: &mut EntityCommands,
        geometry: Entity,
    ) {
        // let mut unclipdepth = false;
        // #[cfg(not(target_arch = "wasm32"))]
        // {
        //     unclipdepth = true;
        // }
        let unclipdepth = false;

        commands
            .insert(AbstructMesh)
            .insert(Mesh)
            .insert(GeometryID(geometry))
            .insert(RenderGeometryEable(false))
            .insert(RenderWorldMatrix(Matrix::identity()))
            .insert(RenderWorldMatrixInv(Matrix::identity()))
            .insert(RenderMatrixDirty(true))
            .insert(MeshCastShadow(false))
            .insert(MeshReceiveShadow(false))
            .insert(PassDirtyBindEffectValue(0))
            .insert(FlagPassDirtyBindEffectValue)
            .insert(PassDirtyBindEffectTextures(0))
            .insert(FlagPassDirtyBindEffectTextures)
            .insert(LayerMask::default())
            .insert(AbstructMeshCullingFlag(false))
            .insert(TransparentSortParam::opaque())

            // .insert(CCullMode(CullMode::Back))
            // .insert(CFrontFace(FrontFace::Ccw))
            // .insert(CPolygonMode(PolygonMode::Fill))
            // .insert(Topology(PrimitiveTopology::TriangleList))
            // .insert(CUnClipDepth(unclipdepth))
            // .insert(PrimitiveState { cull: CullMode::Back, frontface: FrontFace::Ccw, polygon: PolygonMode::Fill, topology: PrimitiveTopology::TriangleList, unclip_depth: unclipdepth })

            // .insert(DepthWrite::default())
            // .insert(DepthCompare::default())
            // .insert(DepthBias::default())
            // .insert(StencilFront::default())
            // .insert(StencilBack::default())
            // .insert(StencilRead::default())
            // .insert(StencilWrite::default())

            // .insert(ModelBlend::default())

            .insert(BindSkinValue(None))
            .insert(ModelVelocity::default())
            .insert(RenderAlignment::default())
            .insert(ScalingMode::default())
            .insert(IndiceRenderRange::default())
            .insert(RecordIndiceRenderRange::default())
            .insert(VertexRenderRange::default())
            .insert(GeometryBounding::default())
            .insert(GeometryCullingMode::default())
            .insert(InstancedMeshTransparentSortCollection(vec![]))
            ;
    }

    pub fn modify(
        app: &mut App,
        cmd: OpsMeshShadow,
    ) {
        let mut cmds = app.world.get_resource_mut::<ActionListMeshShadow>().unwrap();
        cmds.push(cmd);
    }

    pub fn as_instance_source(
        commands: &mut EntityCommands,
    ) {
        commands
            .insert(InstanceSourceRefs::default())
            .insert(DirtyInstanceSourceRefs::default())
            .insert(DirtyInstanceSourceForSingleBuffer::default())
            ;
    }
}
pub struct ActionInstanceMesh;
impl ActionInstanceMesh {
    pub fn init(
        commands: &mut EntityCommands,
        source: Entity,
        scene: Entity,
    ) {
        ActionTransformNode::init(commands, scene);
        ActionInstanceMesh::as_instance(commands, source);
    }
    pub(crate) fn as_instance(
        commands: &mut EntityCommands,
        source: Entity,
    ) {
        commands.insert(AbstructMesh);
        commands.insert(AbstructMeshCullingFlag(false));
        commands.insert(InstanceTransparentIndex(0));
        commands.insert(InstanceMesh(source));

        commands.insert(RenderMatrixDirty(true));
        commands.insert(RenderWorldMatrix(Matrix::identity()));
        commands.insert(RenderWorldMatrixInv(Matrix::identity()));
        commands.insert(ModelVelocity::default());
        commands.insert(ScalingMode::default());
        commands.insert(GeometryBounding::default());
        commands.insert(GeometryCullingMode::default());
    }
}

fn create_passobj(
    commands: &mut Commands,
    id: Entity,
    idmodel: Entity,
    scene: Entity,
    empty: Entity,
    tag: PassTag,
) {
    let mut entitycmd = commands.entity(id);
    ActionEntity::init(&mut entitycmd); ActionPassObject::init(&mut entitycmd, empty, idmodel, scene);
    entitycmd.insert(tag);
}