
use pi_scene_shell::prelude::*;
use pi_scene_math::*;

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
) {
    cmds.drain().drain(..).for_each(|OpsMeshCreation(scene, entity, state )| {
        // log::error!("Create Mesh");
        if ActionMesh::init(&mut commands, entity, scene, &mut allocator, &empty, state, &lightlimit.0) == false {
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
    // mut meshes: Query<>,
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
}

pub fn sys_act_instance_attribute(
    mut cmdsfloat: ResMut<ActionListInstanceFloat>,
    mut cmdsvec4s: ResMut<ActionListInstanceVec4>,
    mut cmdsvec3s: ResMut<ActionListInstanceVec3>,
    mut cmdsvec2s: ResMut<ActionListInstanceVec2>,
    mut cmdsuints: ResMut<ActionListInstanceUint>,
    mut cmdssints: ResMut<ActionListInstanceSint>,
    mut instances: Query<&mut ModelInstanceAttributes>,

    mut animator_vec4: ResMut<ActionListAnimatorableVec4>,
    mut animator_vec3: ResMut<ActionListAnimatorableVec3>,
    mut animator_vec2: ResMut<ActionListAnimatorableVec2>,
    mut animator_float: ResMut<ActionListAnimatorableFloat>,
    mut animator_uint: ResMut<ActionListAnimatorableUint>,
    mut animator_sint: ResMut<ActionListAnimatorableSint>,
) {

    cmdsfloat.drain().drain(..).for_each(|OpsInstanceFloat(instance, val, attr)| {
        if let Ok(mut attributes) = instances.get_mut(instance) {
            if let Some(offset) = attributes.offset(&attr) {
                if let Some(target) = offset.entity() {
                    animator_float.push(OpsAnimatorableFloat::ops(target, instance, AnimatorableFloat(val), EAnimatorableEntityType::Attribute));
                } else {
                    let mut offset = offset.offset() as usize;
                    bytemuck::cast_slice(&[offset]).iter().for_each(|v| { attributes.bytes_mut()[offset] = *v; offset += 1; });
                }
            }
        }
    });
    cmdsvec4s.drain().drain(..).for_each(|OpsInstanceVec4(instance, val, attr)| {
        if let Ok(mut attributes) = instances.get_mut(instance) {
            if let Some(offset) = attributes.offset(&attr) {
                if let Some(target) = offset.entity() {
                    animator_vec4.push(OpsAnimatorableVec4::ops(target, instance, AnimatorableVec4::from(val.as_slice()), EAnimatorableEntityType::Attribute));
                } else {
                    let mut offset = offset.offset() as usize;
                    bytemuck::cast_slice(&val).iter().for_each(|v| { attributes.bytes_mut()[offset] = *v; offset += 1; });
                }
            }
        }
    });
    cmdsvec3s.drain().drain(..).for_each(|OpsInstanceVec3(instance, val, attr)| {
        if let Ok(mut attributes) = instances.get_mut(instance) {
            if let Some(offset) = attributes.offset(&attr) {
                if let Some(target) = offset.entity() {
                    animator_vec3.push(OpsAnimatorableVec3::ops(target, instance, AnimatorableVec3::from(val.as_slice()), EAnimatorableEntityType::Attribute));
                } else {
                    let mut offset = offset.offset() as usize;
                    bytemuck::cast_slice(&val).iter().for_each(|v| { attributes.bytes_mut()[offset] = *v; offset += 1; });
                }
            }
        }
    });
    cmdsvec2s.drain().drain(..).for_each(|OpsInstanceVec2(instance, val, attr)| {
        if let Ok(mut attributes) = instances.get_mut(instance) {
            if let Some(offset) = attributes.offset(&attr) {
                if let Some(target) = offset.entity() {
                    animator_vec2.push(OpsAnimatorableVec2::ops(target, instance, AnimatorableVec2::from(val.as_slice()), EAnimatorableEntityType::Attribute));
                } else {
                    let mut offset = offset.offset() as usize;
                    bytemuck::cast_slice(&val).iter().for_each(|v| { attributes.bytes_mut()[offset] = *v; offset += 1; });
                }
            }
        }
    });
    cmdsuints.drain().drain(..).for_each(|OpsInstanceUint(instance, val, attr)| {
        if let Ok(mut attributes) = instances.get_mut(instance) {
            if let Some(offset) = attributes.offset(&attr) {
                if let Some(target) = offset.entity() {
                    animator_uint.push(OpsAnimatorableUint::ops(target, instance, AnimatorableUint(val), EAnimatorableEntityType::Attribute));
                } else {
                    let mut offset = offset.offset() as usize;
                    bytemuck::cast_slice(&[val]).iter().for_each(|v| { attributes.bytes_mut()[offset] = *v; offset += 1; });
                }
            }
        }
    });
    cmdssints.drain().drain(..).for_each(|OpsInstanceSint(instance, val, attr)| {
        if let Ok(mut attributes) = instances.get_mut(instance) {
            if let Some(offset) = attributes.offset(&attr) {
                if let Some(target) = offset.entity() {
                    animator_sint.push(OpsAnimatorableSint::ops(target, instance, AnimatorableSint(val), EAnimatorableEntityType::Attribute));
                } else {
                    let mut offset = offset.offset() as usize;
                    bytemuck::cast_slice(&[val]).iter().for_each(|v| { attributes.bytes_mut()[offset] = *v; offset += 1; });
                }
            }
        }
    });
}

pub fn sys_act_abstruct_mesh_render_alignment(
    mut cmds: ResMut<ActionListMeshRenderAlignment>,
    mut items: Query<&mut RenderAlignment>,
) {
    cmds.drain().drain(..).for_each(|OpsMeshRenderAlignment(entity, val)| {
        if let Ok(mut item) = items.get_mut(entity) {
            // log::warn!("RenderAlignment: {:?}", (val));
            *item = val;
        } else {
            // if count < 2 {
            //     cmds.push(OpsMeshRenderAlignment(entity, val, count + 1));
            // }
        }
    });
}

pub fn sys_act_abstruct_mesh_scaling_mode(
    mut cmds: ResMut<ActionListAbstructMeshScalingMode>,
    mut items: Query<&mut ScalingMode>,
) {
    cmds.drain().drain(..).for_each(|OpsAbstructMeshScalingMode(entity, val)| {
        if let Ok(mut item) = items.get_mut(entity) {
            *item = val;
        } else {
            // if count < 2 {
            //     cmds.push(OpsAbstructMeshScalingMode(entity, val, count + 1));
            // }
        }
    });
}

pub fn sys_act_abstruct_mesh_velocity(
    mut cmds: ResMut<ActionListAbstructMeshVelocity>,
    mut items: Query<&mut ModelVelocity>,
) {
    cmds.drain().drain(..).for_each(|OpsAbstructMeshVelocity(entity, val)| {
        if let Ok(mut item) = items.get_mut(entity) {
            *item = val;
        } else {
            // if count < 2 {
            //     cmds.push(OpsAbstructMeshVelocity(entity, val, count + 1));
            // }
        }
    });
}

pub fn sys_act_mesh_render_indice(
    mut cmds: ResMut<ActionListMeshRenderIndiceRange>,
    mut items: Query<(&mut IndiceRenderRange, &mut RecordIndiceRenderRange)>,
) {
    cmds.drain().drain(..).for_each(|OpsMeshRenderIndiceRange(entity, val)| {
        // log::warn!("Range: {:?}", val);
        if let Ok((mut item, mut record)) = items.get_mut(entity) {
            *record = RecordIndiceRenderRange(IndiceRenderRange(val.clone()));
            *item = IndiceRenderRange(val);
        } else {
            // if count < 2 {
            //     cmds.push(OpsMeshRenderIndiceRange(entity, val, count + 1));
            // }
        }
    });
}

pub fn sys_act_mesh_render_vertex_range(
    mut cmds: ResMut<ActionListMeshRenderVertexRange>,
    mut items: Query<&mut VertexRenderRange>,
    // mut items: Query<(&mut VertexRenderRange, &mut RecordVertexRenderRange)>,
) {
    cmds.drain().drain(..).for_each(|OpsMeshRenderVertexRange(entity, val)| {
        // log::warn!("Range: {:?}", val);
        if let Ok(mut item) = items.get_mut(entity) {
            // *record = RecordIndiceRenderRange(IndiceRenderRange(val.clone()));
            *item = VertexRenderRange(val);
        } else {
            // if count < 2 {
            //     cmds.push(OpsMeshRenderVertexRange(entity, val, count + 1));
            // }
        }
    });
}

pub fn sys_act_mesh_force_point_lighting(
    mut cmds: ResMut<ActionListMeshForcePointLighting>,
    mut items: Query<&mut ModelForcePointLightings>,
    // mut items: Query<(&mut VertexRenderRange, &mut RecordVertexRenderRange)>,
) {
    cmds.drain().drain(..).for_each(|OpsMeshForcePointLighting(entity, light, isadd)| {
        // log::warn!("Range: {:?}", val);
        if let Ok(mut item) = items.get_mut(entity) {
            // *record = RecordIndiceRenderRange(IndiceRenderRange(val.clone()));
            match item.0.binary_search(&light) {
                Ok(idx)  => { if isadd == false { item.0.remove(idx); } },
                Err(idx) => { if isadd == true  { item.0.insert(idx, light); } },
            }
        } else {
            // if count < 2 {
            //     cmds.push(OpsMeshForcePointLighting(entity, light, isadd, count + 1));
            // }
        }
    });
}

pub fn sys_act_mesh_force_spot_lighting(
    mut cmds: ResMut<ActionListMeshForceSpotLighting>,
    mut items: Query<&mut ModelForceSpotLightings>,
    // mut items: Query<(&mut VertexRenderRange, &mut RecordVertexRenderRange)>,
) {
    cmds.drain().drain(..).for_each(|OpsMeshForceSpotLighting(entity, light, isadd)| {
        // log::warn!("Range: {:?}", val);
        if let Ok(mut item) = items.get_mut(entity) {
            // *record = RecordIndiceRenderRange(IndiceRenderRange(val.clone()));
            match item.0.binary_search(&light) {
                Ok(idx)  => { if isadd == false { item.0.remove(idx); } },
                Err(idx) => { if isadd == true  { item.0.insert(idx, light); } },
            }
        } else {
            // if count < 2 {
            //     cmds.push(OpsMeshForceSpotLighting(entity, light, isadd, count + 1));
            // }
        }
    });
}

pub fn sys_act_mesh_force_hemi_lighting(
    mut cmds: ResMut<ActionListMeshForceHemiLighting>,
    mut items: Query<&mut ModelForceHemiLightings>,
    // mut items: Query<(&mut VertexRenderRange, &mut RecordVertexRenderRange)>,
) {
    cmds.drain().drain(..).for_each(|OpsMeshForceHemiLighting(entity, light, isadd)| {
        // log::warn!("Range: {:?}", val);
        if let Ok(mut item) = items.get_mut(entity) {
            // *record = RecordIndiceRenderRange(IndiceRenderRange(val.clone()));
            match item.0.binary_search(&light) {
                Ok(idx)  => { if isadd == false { item.0.remove(idx); } },
                Err(idx) => { if isadd == true  { item.0.insert(idx, light); } },
            }
        } else {
            // if count < 2 {
            //     cmds.push(OpsMeshForceHemiLighting(entity, light, isadd, count + 1));
            // }
        }
    });
}

pub fn sys_act_model_skinoffset(
    mut cmds: ResMut<ActionListBoneOffset>,
    items: Query<&BindModel>,
) {
    cmds.drain().drain(..).for_each(|OpsBoneOffset(entity, offset)| {
        if let Ok(bind) = items.get(entity) {
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

        if let Some(bind) = BindModel::new(allocator) {
            // log::info!("BindModel New");
            entitycmd.insert(bind);
        }

        entitycmd.insert(MeshLightingMode::default());
        entitycmd.insert(ModelLightingIndexs::new(allocator, lightlimit));
        entitycmd.insert(ModelForcePointLightings::default());
        entitycmd.insert(ModelForceSpotLightings::default());
        entitycmd.insert(ModelForceHemiLightings::default());
        entitycmd.insert(ModelPointLightingDirty::default());
        entitycmd.insert(ModelSpotLightingDirty::default());
        entitycmd.insert(ModelHemiLightingDirty::default());

        entitycmd.insert(MeshStates::default());
        entitycmd.insert(DirtyMeshStates);

        entitycmd.insert(meshinstanceattributes);
        entitycmd.insert(state);

        create_passobj::<Pass01, PassID01>(entity, commands, empty.id(), scene );
        create_passobj::<Pass02, PassID02>(entity, commands, empty.id(), scene );
        create_passobj::<Pass03, PassID03>(entity, commands, empty.id(), scene );
        create_passobj::<Pass04, PassID04>(entity, commands, empty.id(), scene );
        create_passobj::<Pass05, PassID05>(entity, commands, empty.id(), scene );
        create_passobj::<Pass06, PassID06>(entity, commands, empty.id(), scene );
        create_passobj::<Pass07, PassID07>(entity, commands, empty.id(), scene );
        create_passobj::<Pass08, PassID08>(entity, commands, empty.id(), scene );
        // create_passobj::<Pass09, PassID09>(entity, commands, empty.id(), scene );
        // create_passobj::<Pass10, PassID10>(entity, commands, empty.id(), scene );
        // create_passobj::<Pass11, PassID11>(entity, commands, empty.id(), scene );
        // create_passobj::<Pass12, PassID12>(entity, commands, empty.id(), scene );

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
            .insert(CCullMode(CullMode::Back))
            .insert(CFrontFace(FrontFace::Ccw))
            .insert(CPolygonMode(PolygonMode::Fill))
            .insert(Topology(PrimitiveTopology::TriangleList))
            .insert(CUnClipDepth(unclipdepth))
            .insert(DepthWrite::default())
            .insert(DepthCompare::default())
            .insert(DepthBias::default())
            .insert(StencilFront::default())
            .insert(StencilBack::default())
            .insert(StencilRead::default())
            .insert(StencilWrite::default())
            .insert(ModelBlend::default())
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
    // pub fn create(
    //     app: &mut App,
    //     scene: Entity,
    // ) -> Entity {
    //     let mut queue = CommandQueue::default();
    //     let mut commands = Commands::new(&mut queue, &app.world);

    //     let entity = commands.spawn_empty().id();
    //     queue.apply(&mut app.world);

    //     let mut cmds = app.world.get_resource_mut::<ActionListMeshCreate>().unwrap();
    //     cmds.push(OpsMeshCreation(scene, entity));

    //     entity
    // }

    // pub fn use_geometry(
    //     app: &mut App,
    //     id_mesh: Entity,
    //     vertex_desc: Vec<VertexBufferDesc>,
    //     indices_desc: Option<IndicesBufferDesc>,
    // ) {
    //     let mut queue = CommandQueue::default();
    //     let mut commands = Commands::new(&mut queue, &app.world);

    //     let id_geo = commands.spawn_empty().id();

    //     let mut cmds = app.world.get_resource_mut::<ActionListGeometryCreate>().unwrap();
    //     ActionGeometry::create(&mut cmds, id_geo, id_mesh, vertex_desc, indices_desc);
    // }

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

fn create_passobj<T: TPass + Component, T2: TPassID + Component>(
    model: Entity,
    commands: &mut Commands,
    empty: Entity,
    scene: Entity,
) -> ObjectID {
    let id = commands.spawn_empty().id();

    let passid = T2::new(id);
    commands.entity(model).insert(passid);
    // log::warn!("Model Pass: {:?}", (model, T2::TAG, id, scene));

    let mut entitycmd = commands.entity(id);
    ActionEntity::init(&mut entitycmd);
    ActionPassObject::init(&mut entitycmd, empty, model, scene);
    entitycmd.insert(T::default());
    entitycmd.insert(T::TAG);


    id
}
