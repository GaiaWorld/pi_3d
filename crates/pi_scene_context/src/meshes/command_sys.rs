
use pi_engine_shell::prelude::*;
use pi_scene_math::{Vector4, Matrix};

use crate::{
    geometry::{
        prelude::*,
        instance::{instance_boneoffset::*, instance_vec4::*, instance_float::EInstanceFloatType}
    },
    pass::*,
    renderers::prelude::*,
    state::{MeshStates, DirtyMeshStates},
    layer_mask::prelude::*,
    transforms::command_sys::ActionTransformNode,
    skeleton::prelude::*,
    materials::prelude::*,
    object::ActionEntity,
    cullings::prelude::*,
    commands::*,
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
    mut disposereadylist: ResMut<ActionListDisposeReady>,
    mut _disposecanlist: ResMut<ActionListDisposeCan>,
    lightlimit: Res<ModelLightLimit>,
) {
    cmds.drain().drain(..).for_each(|OpsMeshCreation(scene, entity, state )| {
        // log::error!("Create Mesh");
        if ActionMesh::init(&mut commands, entity, scene, &mut allocator, &empty, state, &lightlimit.0) == false {
            disposereadylist.push(OpsDisposeReady::ops(entity));
        }
    });
}

pub fn sys_act_instanced_mesh_create(
    mut cmds: ResMut<ActionListInstanceMeshCreate>,
    mut commands: Commands,
    mut meshes: Query<(&SceneID, &mut InstanceSourceRefs, &mut DirtyInstanceSourceRefs)>,
) {
    cmds.drain().drain(..).for_each(|OpsInstanceMeshCreation(source, instance, count)| {
        if let Ok((id_scene, mut instancelist, mut flag)) = meshes.get_mut(source) {
            if let Some(mut cmd) = commands.get_entity(source) {
                cmd
                    .insert(InstanceColorDirty(true))
                    .insert(InstanceTillOffDirty(true))
                    .insert(InstanceWorldMatrixDirty(true))
                    .insert(InstanceVec4ADirty(true))
                    .insert(InstanceVec4BDirty(true))
                    .insert(InstanceVec4CDirty(true))
                    .insert(InstanceVec4DDirty(true))
                    ;
            } else {
                return;
            };

            let mut ins_cmds = if let Some(cmd) = commands.get_entity(instance) {
                cmd
            } else {
                return;
            };

            // 
            ActionInstanceMesh::init(&mut ins_cmds, source, id_scene.0);
            ActionAnime::as_anime_group_target(&mut ins_cmds);

            instancelist.insert(instance);
            *flag = DirtyInstanceSourceRefs;
        } else {
            if count < 2 {
                cmds.push(OpsInstanceMeshCreation(source, instance, count + 1))
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

pub fn sys_act_instance_color_alpha(
    mut cmds: ResMut<ActionListInstanceColorAlpha>,
    entities: Query<Entity>,
    mut instances: Query<(&InstanceMesh, &mut InstanceColor)>,
) {
    cmds.drain().drain(..).for_each(|OpsInstanceColorAlpha(instance, r, g, b, a, count)| {
        if entities.contains(instance) {
            if let Ok((_source, mut instance_data)) = instances.get_mut(instance) {
                *instance_data = InstanceColor(Vector4::new(r, g, b, a));
            } else {
                if count < 2 {
                    cmds.push(OpsInstanceColorAlpha(instance, r, g, b, a, count + 1));
                }
            }
        }
    });
}

pub fn sys_act_instance_color(
    mut cmds: ResMut<ActionListInstanceColor>,
    entities: Query<Entity>,
    mut instances: Query<(&InstanceMesh, &mut InstanceRGB)>,
) {
    cmds.drain().drain(..).for_each(|OpsInstanceColor(instance, r, g, b, count)| {
        if entities.contains(instance) {
            if let Ok((_source, mut instance_data)) = instances.get_mut(instance) {
                *instance_data = InstanceRGB(r, g, b);
            } else {
                if count < 2 {
                    cmds.push(OpsInstanceColor(instance, r, g, b, count + 1));
                }
            }
        }
    });
}

pub fn sys_act_instance_alpha(
    mut cmds: ResMut<ActionListInstanceAlpha>,
    entities: Query<Entity>,
    mut instances: Query<(&InstanceMesh, &mut InstanceAlpha)>,
) {
    cmds.drain().drain(..).for_each(|OpsInstanceAlpha(instance, val, count)| {
        if entities.contains(instance) {
            if let Ok((_source, mut instance_data)) = instances.get_mut(instance) {
                *instance_data = InstanceAlpha(val);
            } else {
                if count < 2 {
                    cmds.push(OpsInstanceAlpha(instance, val, count + 1));
                }
            }
        }
    });
}

pub fn sys_act_instance_tilloff(
    mut cmds: ResMut<ActionListInstanceTillOff>,
    entities: Query<Entity>,
    mut instances: Query<(&InstanceMesh, &mut InstanceTillOff)>,
    mut source_colors: Query<&mut InstanceTillOffDirty>,
) {
    cmds.drain().drain(..).for_each(|OpsInstanceTillOff(instance, val, count)| {
        if entities.contains(instance) {
            if let Ok((source, mut instance_data)) = instances.get_mut(instance) {
                *instance_data = InstanceTillOff(val);
                if let Ok(mut flag) = source_colors.get_mut(source.0) {
                    *flag = InstanceTillOffDirty(true);
                }
            } else {
                if count < 2 {
                    cmds.push(OpsInstanceTillOff(instance, val, count + 1));
                }
            }
        }
    });
}


pub fn sys_act_instance_float(
    mut cmds: ResMut<ActionListInstanceFloat>,
    entities: Query<Entity>,
    mut instances: Query<(&InstanceMesh, &mut InstanceVec4A, &mut InstanceVec4B, &mut InstanceVec4C, &mut InstanceVec4D)>,
    mut sources: Query<(&mut InstanceVec4ADirty, &mut InstanceVec4BDirty, &mut InstanceVec4CDirty, &mut InstanceVec4DDirty)>,
) {
    cmds.drain().drain(..).for_each(|OpsInstanceFloat(instance, val, usetype)| {
        if entities.contains(instance) {
            // log::warn!("Instance Float 1");
            if let Ok((source, mut a_data, mut b_data, mut c_data, mut d_data)) = instances.get_mut(instance) {
                // log::warn!("Instance Float 2");
                if let Ok((mut adirty, mut bdirty, mut cdirty, mut ddirty)) = sources.get_mut(source.0) {
                    // log::warn!("Instance Float 3");
                    match usetype {
                        EInstanceFloatType::F00 => { a_data.0 = val; *adirty = InstanceVec4ADirty(true); },
                        EInstanceFloatType::F01 => { a_data.1 = val; *adirty = InstanceVec4ADirty(true); },
                        EInstanceFloatType::F02 => { a_data.2 = val; *adirty = InstanceVec4ADirty(true); },
                        EInstanceFloatType::F03 => { a_data.3 = val; *adirty = InstanceVec4ADirty(true); },
                        EInstanceFloatType::F04 => { b_data.0 = val; *bdirty = InstanceVec4BDirty(true); },
                        EInstanceFloatType::F05 => { b_data.1 = val; *bdirty = InstanceVec4BDirty(true); },
                        EInstanceFloatType::F06 => { b_data.2 = val; *bdirty = InstanceVec4BDirty(true); },
                        EInstanceFloatType::F07 => { b_data.3 = val; *bdirty = InstanceVec4BDirty(true); },
                        EInstanceFloatType::F08 => { c_data.0 = val; *cdirty = InstanceVec4CDirty(true); },
                        EInstanceFloatType::F09 => { c_data.1 = val; *cdirty = InstanceVec4CDirty(true); },
                        EInstanceFloatType::F10 => { c_data.2 = val; *cdirty = InstanceVec4CDirty(true); },
                        EInstanceFloatType::F11 => { c_data.3 = val; *cdirty = InstanceVec4CDirty(true); },
                        EInstanceFloatType::F12 => { d_data.0 = val; *ddirty = InstanceVec4DDirty(true); },
                        EInstanceFloatType::F13 => { d_data.1 = val; *ddirty = InstanceVec4DDirty(true); },
                        EInstanceFloatType::F14 => { d_data.2 = val; *ddirty = InstanceVec4DDirty(true); },
                        EInstanceFloatType::F15 => { d_data.3 = val; *ddirty = InstanceVec4DDirty(true); },
                        _ => { 
                            // log::warn!("Instance Float 4");
                        }
                    }
                }
            }
        }
    });
}


pub fn sys_act_bone_offset(
    mut cmds: ResMut<ActionListBoneOffset>,
    mut instances: Query<(&mut InstanceBoneoffset, &mut RecordInstanceBoneoffset)>,
) {
    cmds.drain().drain(..).for_each(|OpsBoneOffset(entity, val, count)| {
        if let Ok((mut instance, mut record)) = instances.get_mut(entity) {
            *record = RecordInstanceBoneoffset(InstanceBoneoffset(val));
            *instance = InstanceBoneoffset(val);
        } else if count < 2 {
            cmds.push(OpsBoneOffset(entity, val, count + 1))
        }
    });
}

pub fn sys_act_abstruct_mesh_render_alignment(
    mut cmds: ResMut<ActionListMeshRenderAlignment>,
    mut items: Query<&mut RenderAlignment>,
) {
    cmds.drain().drain(..).for_each(|OpsMeshRenderAlignment(entity, val, count)| {
        if let Ok(mut item) = items.get_mut(entity) {
            // log::warn!("RenderAlignment: {:?}", (val));
            *item = val;
        } else {
            if count < 2 {
                cmds.push(OpsMeshRenderAlignment(entity, val, count + 1));
            }
        }
    });
}

pub fn sys_act_abstruct_mesh_scaling_mode(
    mut cmds: ResMut<ActionListAbstructMeshScalingMode>,
    mut items: Query<&mut ScalingMode>,
) {
    cmds.drain().drain(..).for_each(|OpsAbstructMeshScalingMode(entity, val, count)| {
        if let Ok(mut item) = items.get_mut(entity) {
            *item = val;
        } else {
            if count < 2 {
                cmds.push(OpsAbstructMeshScalingMode(entity, val, count + 1));
            }
        }
    });
}

pub fn sys_act_abstruct_mesh_velocity(
    mut cmds: ResMut<ActionListAbstructMeshVelocity>,
    mut items: Query<&mut ModelVelocity>,
) {
    cmds.drain().drain(..).for_each(|OpsAbstructMeshVelocity(entity, val, count)| {
        if let Ok(mut item) = items.get_mut(entity) {
            *item = val;
        } else {
            if count < 2 {
                cmds.push(OpsAbstructMeshVelocity(entity, val, count + 1));
            }
        }
    });
}

pub fn sys_act_mesh_render_indice(
    mut cmds: ResMut<ActionListMeshRenderIndiceRange>,
    mut items: Query<(&mut IndiceRenderRange, &mut RecordIndiceRenderRange)>,
) {
    cmds.drain().drain(..).for_each(|OpsMeshRenderIndiceRange(entity, val, count)| {
        // log::warn!("Range: {:?}", val);
        if let Ok((mut item, mut record)) = items.get_mut(entity) {
            *record = RecordIndiceRenderRange(IndiceRenderRange(val.clone()));
            *item = IndiceRenderRange(val);
        } else {
            if count < 2 {
                cmds.push(OpsMeshRenderIndiceRange(entity, val, count + 1));
            }
        }
    });
}

pub fn sys_act_mesh_render_vertex_range(
    mut cmds: ResMut<ActionListMeshRenderVertexRange>,
    mut items: Query<&mut VertexRenderRange>,
    // mut items: Query<(&mut VertexRenderRange, &mut RecordVertexRenderRange)>,
) {
    cmds.drain().drain(..).for_each(|OpsMeshRenderVertexRange(entity, val, count)| {
        // log::warn!("Range: {:?}", val);
        if let Ok(mut item) = items.get_mut(entity) {
            // *record = RecordIndiceRenderRange(IndiceRenderRange(val.clone()));
            *item = VertexRenderRange(val);
        } else {
            if count < 2 {
                cmds.push(OpsMeshRenderVertexRange(entity, val, count + 1));
            }
        }
    });
}

pub fn sys_act_mesh_force_point_lighting(
    mut cmds: ResMut<ActionListMeshForcePointLighting>,
    mut items: Query<&mut ModelForcePointLightings>,
    // mut items: Query<(&mut VertexRenderRange, &mut RecordVertexRenderRange)>,
) {
    cmds.drain().drain(..).for_each(|OpsMeshForcePointLighting(entity, light, isadd, count)| {
        // log::warn!("Range: {:?}", val);
        if let Ok(mut item) = items.get_mut(entity) {
            // *record = RecordIndiceRenderRange(IndiceRenderRange(val.clone()));
            match item.0.binary_search(&light) {
                Ok(idx)  => { if isadd == false { item.0.remove(idx); } },
                Err(idx) => { if isadd == true  { item.0.insert(idx, light); } },
            }
        } else {
            if count < 2 {
                cmds.push(OpsMeshForcePointLighting(entity, light, isadd, count + 1));
            }
        }
    });
}

pub fn sys_act_mesh_force_spot_lighting(
    mut cmds: ResMut<ActionListMeshForceSpotLighting>,
    mut items: Query<&mut ModelForceSpotLightings>,
    // mut items: Query<(&mut VertexRenderRange, &mut RecordVertexRenderRange)>,
) {
    cmds.drain().drain(..).for_each(|OpsMeshForceSpotLighting(entity, light, isadd, count)| {
        // log::warn!("Range: {:?}", val);
        if let Ok(mut item) = items.get_mut(entity) {
            // *record = RecordIndiceRenderRange(IndiceRenderRange(val.clone()));
            match item.0.binary_search(&light) {
                Ok(idx)  => { if isadd == false { item.0.remove(idx); } },
                Err(idx) => { if isadd == true  { item.0.insert(idx, light); } },
            }
        } else {
            if count < 2 {
                cmds.push(OpsMeshForceSpotLighting(entity, light, isadd, count + 1));
            }
        }
    });
}

pub fn sys_act_mesh_force_hemi_lighting(
    mut cmds: ResMut<ActionListMeshForceHemiLighting>,
    mut items: Query<&mut ModelForceHemiLightings>,
    // mut items: Query<(&mut VertexRenderRange, &mut RecordVertexRenderRange)>,
) {
    cmds.drain().drain(..).for_each(|OpsMeshForceHemiLighting(entity, light, isadd, count)| {
        // log::warn!("Range: {:?}", val);
        if let Ok(mut item) = items.get_mut(entity) {
            // *record = RecordIndiceRenderRange(IndiceRenderRange(val.clone()));
            match item.0.binary_search(&light) {
                Ok(idx)  => { if isadd == false { item.0.remove(idx); } },
                Err(idx) => { if isadd == true  { item.0.insert(idx, light); } },
            }
        } else {
            if count < 2 {
                cmds.push(OpsMeshForceHemiLighting(entity, light, isadd, count + 1));
            }
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
        let mut entitycmd = if let Some(cmd) = commands.get_entity(entity) {
            cmd
        } else {
            return false;
        };

        ActionTransformNode::init(&mut entitycmd, scene);
        ActionAnime::as_anime_group_target(&mut entitycmd);
        ActionMesh::as_mesh(&mut entitycmd, empty.id());
        ActionMesh::as_instance_source(&mut entitycmd);

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
            .insert(InstanceBoneoffset::default())
            .insert(RecordInstanceBoneoffset::default())
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
            .insert(InstanceWorldMatrixDirty(false))
            .insert(InstanceColorDirty(false))
            .insert(InstanceTillOffDirty(false))
            .insert(InstanceBoneOffsetDirty(false))
            .insert(InstanceVec4ADirty(false))
            .insert(InstanceVec4BDirty(false))
            .insert(InstanceVec4CDirty(false))
            .insert(InstanceVec4DDirty(false))
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
        commands.insert(InstanceRGB(1., 1., 1.));
        commands.insert(InstanceAlpha(1.));
        commands.insert(InstanceColor(Vector4::new(1., 1., 1., 1.)));
        commands.insert(InstanceTillOff(Vector4::new(1., 1., 0., 0.)));
        commands.insert(InstanceBoneoffset::default());
        commands.insert(RecordInstanceBoneoffset::default());
        
        commands.insert(InstanceVec4A::default());
        commands.insert(InstanceVec4B::default());
        commands.insert(InstanceVec4C::default());
        commands.insert(InstanceVec4D::default());

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
