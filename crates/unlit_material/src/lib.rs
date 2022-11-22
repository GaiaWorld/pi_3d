use bind_group::{SingleUnlitBindGroupList, SysUnlitMaterialTextureBindGroupUpdate, SysUnlitMaterialBindGroupUpdate};
use command::{SysUnlitMaterialCommand, SingleUnlitMaterialCommandList};
use define::SysUnlitMaterialDefinesUpdate;
use pi_ecs::prelude::Setup;
use pi_engine_shell::plugin::Plugin;
use unlit_material_sys::{UnlitModelUniformUpdate, UnlitMaterialUniformUpdate, SysUnlitMaterialPipelineKey, UnlitMaterialFilter};

pub mod unlit_material;
pub mod unlit_material_sys;
pub mod shader;
pub mod command;
pub mod pipeline;
pub mod bind_group;
pub mod interface;
pub mod define;
pub mod assets;

pub struct PluginUnlitMaterial;
impl Plugin for PluginUnlitMaterial {
    fn init(
        &mut self,
        engine: &mut pi_engine_shell::engine_shell::EnginShell,
        stages: &mut pi_engine_shell::run_stage::RunStage,
    ) -> Result<(), pi_engine_shell::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        SysUnlitMaterialDefinesUpdate::setup(world, stages.command_stage());
        SysUnlitMaterialBindGroupUpdate::setup(world, stages.command_stage());
        SysUnlitMaterialTextureBindGroupUpdate::setup(world, stages.command_stage());
        SysUnlitMaterialCommand::setup(world, stages.command_stage());
        UnlitModelUniformUpdate::setup(world, stages.uniform_update());
        UnlitMaterialUniformUpdate::setup(world, stages.uniform_update());
        SysUnlitMaterialPipelineKey::setup(world, stages.uniform_update());
        UnlitMaterialFilter::setup(world, stages.filter_culling());

        world.insert_resource(SingleUnlitBindGroupList::default());
        world.insert_resource(SingleUnlitMaterialCommandList::default());
        
        Ok(())
    }
}