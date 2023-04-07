
use pi_engine_shell::prelude::*;


use self::{environment::*};

pub mod coordinate_system;
pub mod command;
pub mod interface;
pub mod environment;

pub struct PluginScene;
impl Plugin for PluginScene {
    // fn init(
    //     &mut self,
    //     engine: &mut crate::engine::Engine,
    //     stages: &mut crate::run_stage::RunStage,
    // ) -> Result<(), crate::plugin::ErrorPlugin> {
    //     let world = engine.world_mut();

    //     world.insert_resource(SingleSceneCommandList::default());

    //     SysSceneCreateCommand::setup(world, stages.query_stage::<SysSceneCreateCommand>(ERunStageChap::Initial));

    //     SysSceneTimeUpdate::setup(world, stages.query_stage::<SysSceneTimeUpdate>(ERunStageChap::Command));
    //     SysSceneFogUpdate::setup(world, stages.query_stage::<SysSceneFogUpdate>(ERunStageChap::Command));
    //     SysSceneAmbientUpdate::setup(world, stages.query_stage::<SysSceneAmbientUpdate>(ERunStageChap::Command));

    //     Ok(())
    // }

    fn build(&self, app: &mut App) {
        todo!()
    }
    
}
