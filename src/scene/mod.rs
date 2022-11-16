use pi_ecs::{sys::system, prelude::{ResMut, Query, Setup}, query::Write};
use pi_ecs_macros::setup;

use crate::{object::{ObjectID, GameObject}, flags::SceneID, plugin::Plugin, resources::RenderDynUniformBuffer};

use self::{coordinate_system::SceneCoordinateSytem, scene_time::{SceneTime}, scene_sys::SysDirtySceneTick, command::SysSceneCommand};

pub mod scene_time;
pub mod coordinate_system;
pub mod scene_sys;
pub mod command;
pub mod interface;

pub struct PluginScene;
impl Plugin for PluginScene {
    fn init(
        &mut self,
        world: &mut pi_ecs::world::World,
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {

        SysDirtySceneTick::setup(world, stages.dirty_state_stage());
        SysSceneCommand::setup(world, stages.command_stage());

        Ok(())
    }
}
