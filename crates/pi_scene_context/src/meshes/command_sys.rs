
use std::{ops::Mul, f32::consts::E};

use pi_engine_shell::prelude::*;
use pi_scene_math::{Vector4, Matrix, Number, coordiante_system::CoordinateSytem3, vector::{TToolMatrix, TToolVector3, TToolRotation}, Rotation3, Vector3};

use crate::{
    geometry::{
        prelude::*,
        command_sys::*, instance::instance_boneoffset::{InstanceBoneOffsetDirty, InstanceBoneoffset, RecordInstanceBoneoffset}
    },
    pass::*,
    renderers::{
        prelude::*,
    },
    state::{MeshStates, DirtyMeshStates},
    layer_mask::prelude::*,
    scene::command_sys::ActionScene,
    transforms::{command_sys::ActionTransformNode, prelude::*},
    skeleton::prelude::*,
    materials::prelude::*,
    prelude::{RenderAlignment, ModelVelocity, ScalingMode, IndiceRenderRange, RecordIndiceRenderRange},
};

use super::{
    command::*,
    model::{RenderWorldMatrix, RenderWorldMatrixInv, RenderMatrixDirty, BindModel},
    abstract_mesh::AbstructMesh,
    Mesh,
    lighting::{MeshCastShadow, MeshReceiveShadow},
};


pub fn sys_act_mesh_create(
    mut cmds: ResMut<ActionListMeshCreate>,
    mut tree: ResMut<ActionListTransformNodeParent>,
    mut commands: Commands,
    mut allocator: ResMut<ResBindBufferAllocator>,
    device: Res<PiRenderDevice>,
    empty: Res<SingleEmptyEntity>,
) {
    cmds.drain().drain(..).for_each(|OpsMeshCreation(scene, entity, name, count)| {
        let mut entitycmd = if let Some(cmd) = commands.get_entity(entity) {
            cmd
        } else {
            return;
        };
        
        ActionScene::add_to_scene(&mut entitycmd, &mut tree, scene);
        ActionTransformNode::init_for_tree(&mut entitycmd);
        ActionTransformNode::as_transform_node(&mut entitycmd, name);
        ActionAnime::as_anime_group_target(&mut entitycmd);
        ActionMesh::as_mesh(&mut entitycmd);
        ActionMesh::as_instance_source(&mut entitycmd);

        if let Some(bind) = BindModel::new(&device, &mut allocator) {
            log::info!("BindModel New");
            entitycmd.insert(bind);
        }

        entitycmd.insert(MeshStates::default());
        entitycmd.insert(DirtyMeshStates);
        
        create_passobj::<Pass01,PassID01>(entity, &mut commands, &empty);
        create_passobj::<Pass02,PassID02>(entity, &mut commands, &empty);
        create_passobj::<Pass03,PassID03>(entity, &mut commands, &empty);
        create_passobj::<Pass04,PassID04>(entity, &mut commands, &empty);
        create_passobj::<Pass05,PassID05>(entity, &mut commands, &empty);
        create_passobj::<Pass06,PassID06>(entity, &mut commands, &empty);
        create_passobj::<Pass07,PassID07>(entity, &mut commands, &empty);
        create_passobj::<Pass08,PassID08>(entity, &mut commands, &empty);
    });
}

pub fn sys_act_instanced_mesh_create(
    mut cmds: ResMut<ActionListInstanceMeshCreate>,
    mut tree: ResMut<ActionListTransformNodeParent>,
    mut commands: Commands,
    mut meshes: Query<(&SceneID, &mut InstanceSourceRefs, &mut DirtyInstanceSourceRefs)>,
) {
    cmds.drain().drain(..).for_each(|OpsInstanceMeshCreation(source, instance, name, count)| {
        if let Ok((id_scene, mut instancelist, mut flag)) = meshes.get_mut(source) {
            
            let mut entitycmd = if let Some(cmd) = commands.get_entity(source) {
                cmd
            } else {
                return;
            };

            entitycmd
                .insert(InstanceColorDirty(true))
                .insert(InstanceTillOffDirty(true))
                .insert(InstanceWorldMatrixDirty(true))
                ;

            let mut ins_cmds = if let Some(cmd) = commands.get_entity(instance) {
                cmd
            } else {
                return;
            };
            // 
            ActionScene::add_to_scene(&mut ins_cmds, &mut tree, id_scene.0);
            ActionTransformNode::init_for_tree(&mut ins_cmds);
            ActionTransformNode::as_transform_node(&mut ins_cmds, name);
            ActionAnime::as_anime_group_target(&mut ins_cmds);
            ActionInstanceMesh::as_instance(&mut ins_cmds, source);

            instancelist.insert(instance);
            *flag = DirtyInstanceSourceRefs;
        } else {
            if count < 2 {
                cmds.push(OpsInstanceMeshCreation(source, instance, name, count + 1))
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
    mut instances: Query<(&InstanceSourceID, &mut InstanceColor)>,
) {
    cmds.drain().drain(..).for_each(|OpsInstanceColorAlpha(instance, r, g, b, a, count)| {
        if entities.contains(instance) {
            if let Ok((source, mut instance_data)) = instances.get_mut(instance) {
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
    mut instances: Query<(&InstanceSourceID, &mut InstanceRGB)>,
) {
    cmds.drain().drain(..).for_each(|OpsInstanceColor(instance, r, g, b, count)| {
        if entities.contains(instance) {
            if let Ok((source, mut instance_data)) = instances.get_mut(instance) {
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
    mut instances: Query<(&InstanceSourceID, &mut InstanceAlpha)>,
) {
    cmds.drain().drain(..).for_each(|OpsInstanceAlpha(instance, val, count)| {
        if entities.contains(instance) {
            if let Ok((source, mut instance_data)) = instances.get_mut(instance) {
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
    mut instances: Query<(&InstanceSourceID, &mut InstanceTillOff)>,
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



pub struct ActionMesh;
impl ActionMesh {
    pub(crate) fn as_mesh(
        commands: &mut EntityCommands,
    ) {
        let mut unclipdepth = false;

        #[cfg(not(target_arch = "wasm32"))]
        {
            unclipdepth = true;
        }

        commands
            .insert(AbstructMesh)
            .insert(Mesh)
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
            .insert(Opaque)
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
            ;
    }
    pub fn create(
        app: &mut App,
        scene: Entity,
        name: String,
    ) -> Entity {
        let mut queue = CommandQueue::default();
        let mut commands = Commands::new(&mut queue, &app.world);

        let entity = commands.spawn_empty().id();
        queue.apply(&mut app.world);

        let mut cmds = app.world.get_resource_mut::<ActionListMeshCreate>().unwrap();
        cmds.push(OpsMeshCreation(scene, entity, name, 0));

        entity
    }

    pub fn use_geometry(
        app: &mut App,
        id_mesh: Entity,
        vertex_desc: Vec<VertexBufferDesc>,
        indices_desc: Option<IndicesBufferDesc>,
    ) {
        let mut queue = CommandQueue::default();
        let mut commands = Commands::new(&mut queue, &app.world);

        let id_geo = commands.spawn_empty().id();

        let mut cmds = app.world.get_resource_mut::<ActionListGeometryCreate>().unwrap();
        ActionGeometry::create(&mut cmds, id_geo, id_mesh, vertex_desc, indices_desc);
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
            .insert(InstanceWorldMatrixDirty(false))
            .insert(InstanceColorDirty(false))
            .insert(InstanceTillOffDirty(false))
            .insert(InstanceBoneOffsetDirty(false))
            ;
    }
}
pub struct ActionInstanceMesh;
impl ActionInstanceMesh {
    pub fn create(
        app: &mut App,
        source: Entity,
        name: String,
    ) -> Entity {
        let mut queue = CommandQueue::default();
        let mut commands = Commands::new(&mut queue, &app.world);

        let entity = commands.spawn_empty().id();

        let mut cmds = app.world.get_resource_mut::<ActionListInstanceMeshCreate>().unwrap();
        cmds.push(OpsInstanceMeshCreation(source, entity, name, 0));

        entity
    }
    pub(crate) fn as_instance(
        commands: &mut EntityCommands,
        source: Entity,
    ) {
        commands.insert(AbstructMesh);
        commands.insert(InstanceSourceID(source));
        commands.insert(InstanceRGB(1., 1., 1.));
        commands.insert(InstanceAlpha(1.));
        commands.insert(InstanceColor(Vector4::new(1., 1., 1., 1.)));
        commands.insert(InstanceTillOff(Vector4::new(1., 1., 0., 0.)));
        commands.insert(InstanceBoneoffset::default());
        commands.insert(RecordInstanceBoneoffset::default());

        commands.insert(RenderMatrixDirty(true));
        commands.insert(RenderWorldMatrix(Matrix::identity()));
        commands.insert(RenderWorldMatrixInv(Matrix::identity()));
        commands.insert(ModelVelocity::default());
        commands.insert(ScalingMode::default());
    }
}

fn create_passobj<T: TPass + Component, T2: TPassID + Component>(
    model: Entity,
    commands: &mut Commands,
    mat: &SingleEmptyEntity,
) -> ObjectID {
    let id = commands.spawn_empty().id();

    commands.entity(model).insert(T2::new(id));

    commands.entity(id).insert(T::new())
        .insert(PassSource(model))
        .insert(MaterialID(mat.id()))
        ;

    id
}


    // pub fn sys_instance_color_modify(
    //     instances: Query<&InstanceSourceID, Changed<InstanceColor>>,
    //     mut commands: Commands,
    // ) {
    //     instances.iter().for_each(|source| {
    //         if let Some(mut cmd) = commands.get_entity(source.0) {
    //             cmd.insert(InstanceColorDirty(true));
    //         } else {
    //             return;
    //         };
    //     });
    // }
    // pub fn sys_instance_tilloff_modify(
    //     instances: Query<&InstanceSourceID, Changed<InstanceTillOff>>,
    //     mut commands: Commands,
    // ) {
    //     instances.iter().for_each(|source| {
    //         if let Some(mut cmd) = commands.get_entity(source.0) {
    //             cmd.insert(InstanceTillOffDirty(true));
    //         } else {
    //             return;
    //         };
    //     });
    // }

    
    pub fn sys_calc_render_matrix(
        mut meshes: Query<
            (ObjectID, &AbstructMesh, &LocalScaling, &LocalEulerAngles, &WorldMatrix, &WorldMatrixInv, &ScalingMode, &RenderAlignment, &ModelVelocity, &mut GlobalTransform),
            (Without<InstanceSourceID>, Or<(Changed<WorldMatrix>, Changed<WorldMatrixInv>, Changed<ScalingMode>, Changed<RenderAlignment>, Changed<ModelVelocity>)>)
        >,
        mut matrixs: Query<(&mut RenderWorldMatrix, &mut RenderWorldMatrixInv)>,
    ) {
        let time = pi_time::Instant::now();

        meshes.iter_mut().for_each(|(
            obj, _,
            localscaling, localeuler, worldmatrix, worldmatrix_inv, scalingmode, renderalignment, velocity, mut transform
        )| {
            if let Ok((mut wm, mut wmi)) = matrixs.get_mut(obj) {

                // log::warn!("calc_render_matrix:");
                // render_wm.0.clone_from(&worldmatrix.0);
                // render_wminv.0.clone_from(&worldmatrix_inv.0);
                let pos = transform.position().clone();
                let mut scl = Vector3::new(1., 1., 1.);
                match scalingmode.0 {
                    crate::prelude::EScalingMode::Hierarchy => {
                        if renderalignment.0 == ERenderAlignment::Local {
                            wm.0.clone_from(&worldmatrix.0);
                            wmi.0.clone_from(&worldmatrix_inv.0);
                            return;
                        }
                        scl.clone_from(transform.scaling());
                    },
                    crate::prelude::EScalingMode::Local => {
                        scl.clone_from(&localscaling.0);
                    },
                    crate::prelude::EScalingMode::Shape => {
                        // 1, 1, 1
                    },
                }

                let mut m = Matrix::identity();
                let rotation = renderalignment.0.calc_rotation(transform.rotation(), velocity);
                CoordinateSytem3::matrix4_compose_rotation(&scl, &rotation, &pos, &mut m);
                if let Some(local) = renderalignment.0.calc_local(velocity) {
                    m = m * local;
                }

                wm.0.clone_from(&m);
                m.try_inverse_mut();
                wmi.0.clone_from(&m);
            }

        });
        
        let time1 = pi_time::Instant::now();
        // log::debug!("SysRenderMatrixUpdate: {:?}", time1 - time);
    }
    
    pub fn sys_calc_render_matrix_instance(
        meshes: Query<&RenderAlignment>,
        mut instances: Query<
            (ObjectID, &AbstructMesh, &LocalScaling, &LocalEulerAngles, &WorldMatrix, &WorldMatrixInv, &ScalingMode, &ModelVelocity, &mut GlobalTransform, &InstanceSourceID),
            Or<(Changed<WorldMatrix>, Changed<WorldMatrixInv>, Changed<ModelVelocity>, Changed<ScalingMode>)>
        >,
        mut matrixs: Query<(&mut RenderWorldMatrix, &mut RenderWorldMatrixInv)>,
        mut inssources: Query<&mut InstanceWorldMatrixDirty>,
    ) {
        let time = pi_time::Instant::now();

        instances.iter_mut().for_each(|(
            obj, _,
            localscaling, localeuler, worldmatrix, worldmatrix_inv, scalingmode, velocity, mut transform, id_source
        )| {
            if let (Ok((mut wm, mut wmi)), Ok(renderalignment)) = (matrixs.get_mut(obj), meshes.get(id_source.0)) {
                
                if let Ok(mut dirty) = inssources.get_mut(id_source.0) {
                    *dirty = InstanceWorldMatrixDirty(true);
                }

                // let mut flag = true;

                // log::warn!("calc_render_matrix:");
                // render_wm.0.clone_from(&worldmatrix.0);
                // render_wminv.0.clone_from(&worldmatrix_inv.0);
                let pos = transform.position().clone();
                let mut scl = Vector3::new(1., 1., 1.);
                match scalingmode.0 {
                    crate::prelude::EScalingMode::Hierarchy => {
                        if renderalignment.0 == ERenderAlignment::Local {
                            wm.0.clone_from(&worldmatrix.0);
                            wmi.0.clone_from(&worldmatrix_inv.0);
                            // log::warn!("Normal Alignment");
                            return;
                        }
                        scl.clone_from(transform.scaling());
                        // flag = false;
                    },
                    crate::prelude::EScalingMode::Local => {
                        scl.clone_from(&localscaling.0);
                    },
                    crate::prelude::EScalingMode::Shape => {
                        // 1, 1, 1
                    },
                }

                let mut m = Matrix::identity();
                let rotation = renderalignment.0.calc_rotation(transform.rotation(), velocity);
                CoordinateSytem3::matrix4_compose_rotation(&scl, &rotation, &pos, &mut m);
                if let Some(local) = renderalignment.0.calc_local(velocity) {
                    m = m * local;
                }

                wm.0.clone_from(&m);
                m.try_inverse_mut();
                wmi.0.clone_from(&m);
            }

        });
        
        let time1 = pi_time::Instant::now();
        log::debug!("SysInstanceRenderMatrixUpdate: {:?}", time1 - time);
    }

    pub fn sys_render_matrix_for_uniform(
        mut meshes: Query<(&RenderWorldMatrix, &RenderWorldMatrixInv, &BindModel), Changed<RenderWorldMatrix>>,
    ) {
        meshes.iter_mut().for_each(|(worldmatrix, worldmatrix_inv, bind_model)| {
            // log::debug!("SysModelUniformUpdate:");

            bind_model.0.data().write_data(ShaderBindModelAboutMatrix::OFFSET_WORLD_MATRIX as usize, bytemuck::cast_slice(worldmatrix.0.as_slice()));
            bind_model.0.data().write_data(ShaderBindModelAboutMatrix::OFFSET_WORLD_MATRIX_INV as usize, bytemuck::cast_slice(worldmatrix_inv.0.as_slice()));
        });
    }

    pub fn sys_velocity_for_uniform(
        mut meshes: Query<(&ModelVelocity, &BindModel), Changed<ModelVelocity>>,
    ) {
        meshes.iter_mut().for_each(|(velocity, bind_model)| {
            let len = (velocity.x * velocity.x + velocity.y * velocity.y + velocity.z * velocity.z).sqrt();
            bind_model.0.data().write_data(ShaderBindModelAboutMatrix::OFFSET_VELOCITY as usize, bytemuck::cast_slice(&[velocity.x, velocity.y, velocity.z, len]));
        });
    }

    pub fn sys_skinoffset_for_uniform(
        mut meshes: Query<(&InstanceBoneoffset, &BindModel), Changed<InstanceBoneoffset>>,
    ) {
        meshes.iter_mut().for_each(|(skinoffset, bind_model)| {
            // log::debug!("SysModelUniformUpdate:");
            bind_model.0.data().write_data(ShaderBindModelAboutMatrix::OFFSET_U32_A as usize, bytemuck::cast_slice(&[skinoffset.0]));
        });
    }