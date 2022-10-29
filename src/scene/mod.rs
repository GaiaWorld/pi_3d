use pi_ecs::{sys::system, prelude::{ResMut, Query, Setup}, query::Write};
use pi_ecs_macros::setup;
use pi_render::rhi::dyn_uniform_buffer::DynUniformBuffer;

use crate::{object::{ObjectID, GameObject}, flags::SceneID, plugin::Plugin};

use self::{coordinate_system::SceneCoordinateSytem, scene_time::SceneTime};

pub mod scene_time;
pub mod coordinate_system;

#[derive(Debug)]
pub enum SceneCommand {
    Create(ObjectID),
    AddObject(ObjectID, SceneID),
}

#[derive(Debug, Default)]
pub struct SingleSceneCommandList {
    pub list: Vec<SceneCommand>,
}
pub struct SysSceneCommand;
#[setup]
impl SysSceneCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingleSceneCommandList>,
        mut scenes: Query<GameObject, (Write<SceneCoordinateSytem>, Write<SceneTime>)>,
        mut objects: Query<GameObject, Write<SceneID>>,
        mut dynbuffer: ResMut<DynUniformBuffer>,
    ) {
        cmds.list.drain(..).for_each(|cmd| {
            match cmd {
                SceneCommand::Create(entity) => {
                    match scenes.get_mut(entity) {
                        Some(mut scene) => {
                            scene.0.insert_no_notify(SceneCoordinateSytem::default());
                            scene.1.insert_no_notify(SceneTime::new(&mut dynbuffer));
                        },
                        None => todo!(),
                    }
                },
                SceneCommand::AddObject(entity, scene) => {
                    match objects.get_mut(entity) {
                        Some(mut object) => {
                            object.insert_no_notify(scene);
                        },
                        None => todo!(),
                    }
                },
            }
        });

    }
}

pub struct PluginScene;
impl Plugin for PluginScene {
    fn init(
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        SysSceneCommand::setup(world, stages.command_stage());

        Ok(())
    }
}

pub trait InterfaceScene {
    fn create_scene(
        &mut self,
    ) -> ObjectID;

    fn add_to_scene(
        &mut self,
        object: ObjectID,
        scene: ObjectID,
    ) -> &mut Self;
}

impl InterfaceScene for crate::engine::Engine {
    fn create_scene(
        &mut self,
    ) -> ObjectID {
        let entity = self.new_object();
        let world = self.world_mut();

        let commands = world.get_resource_mut::<SingleSceneCommandList>().unwrap();
        commands.list.push(SceneCommand::Create(entity));

        entity
    }

    fn add_to_scene(
        &mut self,
        object: ObjectID,
        scene: ObjectID,
    ) -> &mut Self {
        let world = self.world_mut();
        
        let commands = world.get_resource_mut::<SingleSceneCommandList>().unwrap();
        commands.list.push(SceneCommand::AddObject(object, SceneID(scene)));

        self
    }
}