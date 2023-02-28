use pi_ecs::{prelude::{Setup}};
use pi_engine_shell::run_stage::ERunStageChap;

use crate::{plugin::Plugin, };

use self::{command::{SysSceneCreateCommand, SingleSceneCommandList}, environment::{fog::SysSceneFogUpdate, ambient_light::SysSceneAmbientUpdate, scene_time::SysSceneTimeUpdate, }};

pub mod coordinate_system;
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

        SysSceneCreateCommand::setup(world, stages.query_stage::<SysSceneCreateCommand>(ERunStageChap::Initial));

        SysSceneTimeUpdate::setup(world, stages.query_stage::<SysSceneTimeUpdate>(ERunStageChap::Command));
        SysSceneFogUpdate::setup(world, stages.query_stage::<SysSceneFogUpdate>(ERunStageChap::Command));
        SysSceneAmbientUpdate::setup(world, stages.query_stage::<SysSceneAmbientUpdate>(ERunStageChap::Command));

        Ok(())
    }
}
