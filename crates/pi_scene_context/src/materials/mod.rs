use pi_ecs::prelude::{Setup};

use self::{material::{SingleMaterialIDCommandList, SysMaterialIDCommand, SingleValueUniformCommands, SysValueUniformComand}, uniform_buffer::{SysDynUnifromBufferUpdate, SingleDynUnifromBufferReBindFlag}, bind_group::RenderBindGroupPool, material_meta::{PluginMaterialMeta}, uniforms::PluginMaterialUniforms};

pub mod material;
pub mod bind_group;
pub mod uniform_buffer;
pub mod material_meta;
pub mod uniforms;
pub mod value;

pub type MBKK = usize;

pub struct PluginMaterial;
impl crate::Plugin for PluginMaterial {
    fn init(
        &mut self,
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        PluginMaterialMeta.init(engine, stages);
        PluginMaterialUniforms.init(engine, stages);

        let world = engine.world_mut();
        world.insert_resource(SingleMaterialIDCommandList::default());
        world.insert_resource(SingleDynUnifromBufferReBindFlag::default());
        world.insert_resource(RenderBindGroupPool::default());
        world.insert_resource(SingleValueUniformCommands::default());
        
        SysValueUniformComand::setup(world, stages.command_stage());
        SysMaterialIDCommand::setup(world, stages.command_stage());
        SysDynUnifromBufferUpdate::setup(world, stages.between_uniform_update_and_filter_culling());

        Ok(())
    }
}