use pi_ecs::{prelude::{Setup}};

use crate::{plugin::Plugin, };

use self::{scene_sys::SysDirtySceneTick, command::{SysSceneCommand, SingleSceneCommandList}};

pub mod scene_time;
pub mod coordinate_system;
pub mod scene_sys;
pub mod command;
pub mod interface;

pub struct PluginScene;
impl Plugin for PluginScene {
    fn init(
        &mut self,
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        world.insert_resource(SingleSceneCommandList::default());

        SysDirtySceneTick::setup(world, stages.dirty_state_stage());
        SysSceneCommand::setup(world, stages.command_stage());

        Ok(())
    }
}
