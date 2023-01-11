use pi_ecs::{prelude::{Setup}};
use pi_engine_shell::run_stage::ERunStageChap;

use crate::{plugin::Plugin, };

use self::{scene_sys::SysDirtySceneTick, command::{SysSceneCommand, SingleSceneCommandList}};

pub mod scene_time;
pub mod coordinate_system;
pub mod scene_sys;
pub mod command;
pub mod interface;
pub mod environment;

pub struct PluginScene;
impl Plugin for PluginScene {
    fn init(
        &mut self,
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        world.insert_resource(SingleSceneCommandList::default());

        SysSceneCommand::setup(world, stages.query_stage::<SysSceneCommand>(ERunStageChap::Command));
        SysDirtySceneTick::setup(world, stages.query_stage::<SysDirtySceneTick>(ERunStageChap::Command));

        Ok(())
    }
}
