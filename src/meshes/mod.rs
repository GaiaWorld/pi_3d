use pi_ecs::{prelude::{ResMut, Query, Setup}, query::Write};
use pi_ecs_macros::setup;
use pi_render::rhi::dyn_uniform_buffer::{BindOffset, DynUniformBuffer};

use crate::{object::{ObjectID, GameObject}, flags::{RenderSortParam, RenderBlend, PrimitiveState, RenderDepthAndStencil, SceneID}, transforms::{InterfaceTransformNode, transform_node_command::{TransformNodeCommand, SingleTransformNodeCommandList}, transform_node::GlobalTransform}, scene::{SingleSceneCommandList, SceneCommand, InterfaceScene}, plugin::Plugin, layer_mask::LayerMask};

use self::model::BuildinModelBind;

pub mod cube;
pub mod plane;
pub mod model;

pub struct Mesh {
    materials: Vec<ObjectID>,
}
impl Default for Mesh {
    fn default() -> Self {
        Self {
            materials: vec![],
        }
    }
}

pub struct MeshID(pub ObjectID);
#[derive(Debug)]
pub enum MeshCommand {
    Create(ObjectID),
    Destroy(ObjectID),
}

#[derive(Debug, Default)]
pub struct SingleMeshCommandList {
    pub list: Vec<MeshCommand>,
}

pub struct SysMeshCommand;
#[setup]
impl SysMeshCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingleMeshCommandList>,
        mut meshes: Query<GameObject, (Write<LayerMask>, Write<RenderSortParam>, Write<RenderBlend>, Write<PrimitiveState>, Write<RenderDepthAndStencil>, Write<BuildinModelBind>)>,
        mut dynbuffer: ResMut<DynUniformBuffer>,
    ) {
        cmds.list.drain(..).for_each(|cmd| {
            match cmd {
                MeshCommand::Create(entity) => {
                    match meshes.get_mut(entity) {
                        Some(mut item) => {
                            item.0.insert_no_notify(LayerMask::default());
                            item.1.insert_no_notify(RenderSortParam::opaque());
                            item.2.insert_no_notify(RenderBlend::default());
                            item.3.insert_no_notify(PrimitiveState::default());
                            item.4.insert_no_notify(RenderDepthAndStencil::default());
                            item.5.insert_no_notify(BuildinModelBind::new(&mut dynbuffer));
                        },
                        None => {
                            
                        },
                    }
                },
                MeshCommand::Destroy(_) => todo!(),
            }
        });
    }
}

pub struct PluginMesh;
impl Plugin for PluginMesh {
    fn init(
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        SysMeshCommand::setup(world, stages.command_stage());

        world.insert_resource(SingleMeshCommandList::default());

        Ok(())
    }
}

pub trait InterfaceMesh {
    fn create_mesh(
        &mut self,
        scene: ObjectID,
    ) -> ObjectID;

    fn as_mesh(
        &mut self,
        object: ObjectID,
    ) -> &mut Self;
}
impl InterfaceMesh for crate::engine::Engine {
    fn create_mesh(
        &mut self,
        scene: ObjectID,
    ) -> ObjectID {

        let entity = self.new_object();
        let world = self.world_mut();

        self.add_to_scene(entity, scene);
        self.as_transform_node(entity);
        self.transform_parent(entity, scene);

        self.as_mesh(entity);

        entity
    }

    fn as_mesh(
        &mut self,
        object: ObjectID,
    ) -> &mut Self {
        let world = self.world_mut();

        let commands = world.get_resource_mut::<SingleMeshCommandList>().unwrap();
        commands.list.push(MeshCommand::Create(object));

        self
    }
}