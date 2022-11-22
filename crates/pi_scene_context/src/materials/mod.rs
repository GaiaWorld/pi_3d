use pi_ecs::prelude::{Setup};

use self::{material::{SingleMaterialIDCommandList, SysMaterialIDCommand}, uniform_buffer::{SysDynUnifromBufferUpdate, SingleDynUnifromBufferReBindFlag}, bind_group::RenderBindGroupPool};

pub mod material;
pub mod bind_group;
pub mod command;
pub mod uniform_buffer;

pub type MBKK = usize;

pub struct PluginMaterialID;
impl crate::Plugin for PluginMaterialID {
    fn init(
        &mut self,
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        SysMaterialIDCommand::setup(world, stages.command_stage());
        SysDynUnifromBufferUpdate::setup(world, stages.between_uniform_update_and_filter_culling());
        // SysDynUnifromBufferReBindFlag::setup(world, stages.between_uniform_update_and_filter_culling());

        world.insert_resource(SingleMaterialIDCommandList::default());
        world.insert_resource(SingleDynUnifromBufferReBindFlag::default());
        world.insert_resource(RenderBindGroupPool::default());

        Ok(())
    }
}