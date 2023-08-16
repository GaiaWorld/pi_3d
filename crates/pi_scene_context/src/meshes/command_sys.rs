
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
    prelude::{RenderAlignment, ModelVelocity, ScalingMode, IndiceRenderRange, RecordIndiceRenderRange}, object::ActionEntity,
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
        ActionMesh::init(&mut commands, entity, scene, &mut tree, &mut allocator, &device, &empty);
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
            if let Some(mut cmd) = commands.get_entity(source) {
                cmd
                    .insert(InstanceColorDirty(true))
                    .insert(InstanceTillOffDirty(true))
                    .insert(InstanceWorldMatrixDirty(true))
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
            ActionInstanceMesh::init(&mut ins_cmds, &mut tree, source, id_scene.0, name);
            ActionAnime::as_anime_group_target(&mut ins_cmds);

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
    mut instances: Query<(&InstanceMesh, &mut InstanceColor)>,
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
    mut instances: Query<(&InstanceMesh, &mut InstanceRGB)>,
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
    mut instances: Query<(&InstanceMesh, &mut InstanceAlpha)>,
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
    pub fn init(
        commands: &mut Commands,
        entity: Entity,
        scene: Entity,
        tree: &mut ActionListTransformNodeParent,
        allocator: &mut ResBindBufferAllocator,
        device: &PiRenderDevice,
        empty: &SingleEmptyEntity,
    ) {
        let mut entitycmd = if let Some(cmd) = commands.get_entity(entity) {
            cmd
        } else {
            return;
        };

        ActionTransformNode::init(&mut entitycmd, tree, scene, String::from(""));
        ActionAnime::as_anime_group_target(&mut entitycmd);
        ActionMesh::as_mesh(&mut entitycmd);
        ActionMesh::as_instance_source(&mut entitycmd);

        if let Some(bind) = BindModel::new(&device, allocator) {
            log::info!("BindModel New");
            entitycmd.insert(bind);
        }

        entitycmd.insert(MeshStates::default());
        entitycmd.insert(DirtyMeshStates);

        create_passobj::<Pass01,PassID01>(entity, commands, &empty);
        create_passobj::<Pass02,PassID02>(entity, commands, &empty);
        create_passobj::<Pass03,PassID03>(entity, commands, &empty);
        create_passobj::<Pass04,PassID04>(entity, commands, &empty);
        create_passobj::<Pass05,PassID05>(entity, commands, &empty);
        create_passobj::<Pass06,PassID06>(entity, commands, &empty);
        create_passobj::<Pass07,PassID07>(entity, commands, &empty);
        create_passobj::<Pass08,PassID08>(entity, commands, &empty);
    }
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
            ;
    }
}
pub struct ActionInstanceMesh;
impl ActionInstanceMesh {
    pub fn init(
        commands: &mut EntityCommands,
        tree: &mut ActionListTransformNodeParent,
        source: Entity,
        scene: Entity,
        name: String,
    ) {
        ActionTransformNode::init(commands, tree, scene, name);
        ActionInstanceMesh::as_instance(commands, source);
    }
    pub(crate) fn as_instance(
        commands: &mut EntityCommands,
        source: Entity,
    ) {
        commands.insert(AbstructMesh);
        commands.insert(InstanceMesh(source));
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

    let mut entitycmd = commands.entity(id);
    ActionEntity::init(&mut entitycmd);
    entitycmd.insert(T::new())
        .insert(ModelPass(model))
        .insert(MaterialID(mat.id()))
        ;

    id
}
