
use pi_engine_shell::prelude::*;
use pi_scene_math::{Vector4, Matrix, Number};

use crate::{
    geometry::{
        instance::{
            instance_color::{InstanceColor, InstanceColorDirty},
            instance_tilloff::{InstanceTillOff, InstanceTillOffDirty},
            instance_world_matrix::InstanceWorldMatrixDirty,
            *, self
        },
        command::*
    },
    pass::*,
    renderers::{
        pass::*,
        opaque::*,
        render_blend::*,
        render_sort::*,
        render_primitive::*,
        render_depth_and_stencil::*
    },
    state::{MeshStates, DirtyMeshStates},
    layer_mask::*, scene::command::ActionScene, flags::{SceneID}, transforms::command::ActionTransformNode, animation::command::ActionAnime, skeleton::skeleton::BindSkinValue, materials::material::MaterialID, prelude::ActionListTransformNodeParent
};

use super::{
    model::{RenderWorldMatrix, RenderWorldMatrixInv, RenderMatrixDirty, BindModel},
    abstract_mesh::AbstructMesh,
    Mesh,
    lighting::{MeshCastShadow, MeshReceiveShadow}
};

pub struct OpsMeshCreation(pub Entity, pub Entity, pub String);
impl OpsMeshCreation {
    pub fn ops(scene: Entity, entity: Entity, name: String) -> Self {
        Self(scene, entity, name)
    }
}
pub type ActionListMeshCreate = ActionList<OpsMeshCreation>;
pub fn sys_act_mesh_create(
    mut cmds: ResMut<ActionListMeshCreate>,
    mut tree: ResMut<ActionListTransformNodeParent>,
    mut commands: Commands,
    mut allocator: ResMut<ResBindBufferAllocator>,
    device: Res<PiRenderDevice>,
    empty: Res<SingleEmptyEntity>,
) {
    cmds.drain().drain(..).for_each(|OpsMeshCreation(scene, entity, name)| {
        let mut entitycmd = commands.entity(entity);
        
        ActionScene::add_to_scene(&mut entitycmd, &mut tree, scene);
        ActionTransformNode::init_for_tree(&mut entitycmd);
        ActionTransformNode::as_transform_node(&mut entitycmd, name);
        ActionAnime::as_anime_group_target(&mut entitycmd);
        ActionMesh::as_mesh(&mut entitycmd);

        entitycmd.insert(InstanceSourceRefs::default());
        entitycmd.insert(DirtyInstanceSourceRefs::default());
        entitycmd.insert(InstanceWorldMatrixDirty(true));
        entitycmd.insert(InstanceColorDirty(true));
        entitycmd.insert(InstanceTillOffDirty(true));

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

pub struct OpsInstanceMeshCreation(Entity, Entity, String);
impl OpsInstanceMeshCreation {
    pub fn ops(source: Entity, instance: Entity, name: String) -> Self {
        Self(source, instance, name)
    }
}
pub type ActionListInstanceMeshCreate = ActionList<OpsInstanceMeshCreation>;
pub fn sys_act_instanced_mesh_create(
    mut cmds: ResMut<ActionListInstanceMeshCreate>,
    mut tree: ResMut<ActionListTransformNodeParent>,
    mut commands: Commands,
    mut meshes: Query<(&SceneID, &mut InstanceSourceRefs, &mut DirtyInstanceSourceRefs)>,
) {
    cmds.drain().drain(..).for_each(|OpsInstanceMeshCreation(source, instance, name)| {
        if let Ok((id_scene, mut instancelist, mut flag)) = meshes.get_mut(source) {
            commands.entity(source)
                .insert(InstanceColorDirty(true))
                .insert(InstanceTillOffDirty(true))
                .insert(InstanceWorldMatrixDirty(true))
                ;

            let mut ins_cmds = commands.entity(instance);
            // 
            ActionScene::add_to_scene(&mut ins_cmds, &mut tree, id_scene.0);
            ActionTransformNode::init_for_tree(&mut ins_cmds);
            ActionTransformNode::as_transform_node(&mut ins_cmds, name);
            ActionAnime::as_anime_group_target(&mut ins_cmds);
            ActionInstanceMesh::as_instance(&mut ins_cmds, source);

            instancelist.insert(instance);
            *flag = DirtyInstanceSourceRefs;
        } else {
            cmds.push(OpsInstanceMeshCreation::ops(source, instance, name))
        }
    });
}

#[derive(Debug)]
pub enum OpsMeshShadow {
    CastShadow(Entity, bool),
    ReceiveShadow(Entity, bool),
}
pub type ActionListMeshShadow = ActionList<OpsMeshShadow>;
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

pub struct OpsInstanceColor(Entity, Vector4);
impl OpsInstanceColor {
    pub fn ops(instance: Entity, r: Number, g: Number, b: Number, a: Number) -> Self {
        Self(instance, Vector4::new(r, g, b, a))
    }
}
pub type ActionListInstanceColor = ActionList<OpsInstanceColor>;
pub fn sys_act_instance_color(
    mut cmds: ResMut<ActionListInstanceColor>,
    entities: Query<Entity>,
    mut instances: Query<(&InstanceSourceID, &mut InstanceColor)>,
    mut source_colors: Query<&mut InstanceColorDirty>,
) {
    cmds.drain().drain(..).for_each(|OpsInstanceColor(instance, val)| {
        if entities.contains(instance) {
            if let Ok((source, mut instance_data)) = instances.get_mut(instance) {
                *instance_data = InstanceColor(val);
                if let Ok(mut flag) = source_colors.get_mut(source.0) {
                    *flag = InstanceColorDirty(true);
                }
            } else {
                cmds.push(OpsInstanceColor(instance, val));
            }
        }
    });
}


pub struct OpsInstanceTillOff(Entity, Vector4);
impl OpsInstanceTillOff {
    pub fn ops(instance: Entity, uscale: Number, vscale: Number, uoffset: Number, voffset: Number) -> Self {
        Self(instance, Vector4::new(uscale, vscale, uoffset, voffset))
    }
}
pub type ActionListInstanceTillOff = ActionList<OpsInstanceTillOff>;
pub fn sys_act_instance_tilloff(
    mut cmds: ResMut<ActionListInstanceTillOff>,
    entities: Query<Entity>,
    mut instances: Query<(&InstanceSourceID, &mut InstanceTillOff)>,
    mut source_colors: Query<&mut InstanceTillOffDirty>,
) {
    cmds.drain().drain(..).for_each(|OpsInstanceTillOff(instance, val)| {
        if entities.contains(instance) {
            if let Ok((source, mut instance_data)) = instances.get_mut(instance) {
                *instance_data = InstanceTillOff(val);
                if let Ok(mut flag) = source_colors.get_mut(source.0) {
                    *flag = InstanceTillOffDirty(true);
                }
            } else {
                cmds.push(OpsInstanceTillOff(instance, val));
            }
        }
    });
}

pub struct BundleMesh(
    AbstructMesh,
    Mesh,
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
    Opaque,
    TransparentSortParam,
    ECullMode,
    FrontFace,
    PolygonMode,
    ModelDepthStencil,
    ModelBlend,
    BindSkinValue,
);

pub struct ActionMesh;
impl ActionMesh {
    pub(crate) fn as_mesh(
        commands: &mut EntityCommands,
    ) {
        commands
            .insert(AbstructMesh(true))
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
            .insert(ECullMode::Back)
            .insert(FrontFace::Ccw)
            .insert(PolygonMode::Fill)
            .insert(ModelDepthStencil::default())
            .insert(ModelBlend::default())
            .insert(BindSkinValue(None))
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
        cmds.push(OpsMeshCreation(scene, entity, name));

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
}

pub struct BundleInstanceMesh(
    AbstructMesh,
    InstanceSourceID,
    InstanceColor,
    InstanceTillOff,
    RenderMatrixDirty,
    RenderWorldMatrix,
    RenderWorldMatrixInv,
);

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
        cmds.push(OpsInstanceMeshCreation(source, entity, name));

        entity
    }
    pub fn color(
        app: &mut App,
        instance: Entity,
        color: Vector4,
    ) {
        let mut cmds = app.world.get_resource_mut::<ActionListInstanceColor>().unwrap();
        cmds.push(OpsInstanceColor(instance, color));
    }
    pub(crate) fn as_instance(
        commands: &mut EntityCommands,
        source: Entity,
    ) {
        commands.insert(AbstructMesh(true));
        commands.insert(InstanceSourceID(source));
        commands.insert(InstanceColor(Vector4::new(1., 1., 1., 1.)));
        commands.insert(InstanceTillOff(Vector4::new(1., 1., 0., 0.)));

        commands.insert(RenderMatrixDirty(true));
        commands.insert(RenderWorldMatrix(Matrix::identity()));
        commands.insert(RenderWorldMatrixInv(Matrix::identity()));
    }
}

pub struct BundlePass(
    PassSource,
    PassBindEffectValue,
    PassBindEffectTextures,
    PassBindGroupScene,
    PassBindGroupTextureSamplers,
    PassBindGroups,
    PassReady,
    PassShader,
    PassPipeline,
    PassDraw,
    MaterialID,
);

fn create_passobj<T: TPass + Component, T2: TPassID + Component>(
    model: Entity,
    commands: &mut Commands,
    empty: &SingleEmptyEntity,
) -> ObjectID {
    let id = commands.spawn_empty().id();

    commands.entity(model).insert(T2::new(id));

    commands.entity(id).insert(T::new())
        .insert(PassSource(model))
        .insert(PassBindEffectValue(None))
        .insert(PassBindEffectTextures(None))
        .insert(PassBindGroupScene(None))
        .insert(PassBindGroupModel(None))
        .insert(PassBindGroupTextureSamplers(None))
        .insert(PassBindGroups(None))
        .insert(PassReady(None))
        .insert(PassShader(None))
        .insert(PassPipeline(None))
        .insert(PassDraw(None))
        .insert(MaterialID(empty.id()))
        ;

    id
}



    // fn listen(
    //     e: Event,
    //     meshes: Query<GameObject, (&InstanceList, &PassID01, &PassID02, &PassID03, &PassID04, &PassID05, &PassID06, &PassID07, &PassID08, &GeometryID)>,
    //     mut delete: EntityDelete<GameObject>,
    // ) {
    //     if let Some((instances, pass01, pass02, pass03, pass04, pass05, pass06, pass07, pass08, id_geo)) = meshes.get_by_entity(e.id) {
    //         instances.list.iter().for_each(|id| {
    //             delete.despawn(id.clone());
    //         });
    //         delete.despawn(pass01.id());
    //         delete.despawn(pass02.id());
    //         delete.despawn(pass03.id());
    //         delete.despawn(pass04.id());
    //         delete.despawn(pass05.id());
    //         delete.despawn(pass06.id());
    //         delete.despawn(pass07.id());
    //         delete.despawn(pass08.id());
    //         delete.despawn(id_geo.0.clone());
    //     }
    // }

    // #[system]
    pub fn sys_instance_color_modify(
        instances: Query<&InstanceSourceID, Changed<InstanceColor>>,
        mut commands: Commands,
    ) {
        instances.iter().for_each(|source| {
            commands.entity(source.0).insert(InstanceColorDirty(true));
        });
    }
    pub fn sys_instance_tilloff_modify(
        instances: Query<&InstanceSourceID, Changed<InstanceTillOff>>,
        mut commands: Commands,
    ) {
        instances.iter().for_each(|source| {
            commands.entity(source.0).insert(InstanceTillOffDirty(true));
        });
    }
// }