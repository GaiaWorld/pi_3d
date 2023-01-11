use pi_ecs::prelude::{Setup};
use pi_engine_shell::run_stage::ERunStageChap;

use self::{
    material::{SingleMaterialIDCommandList, SysMaterialIDCommand, SingleValueUniformCommands, SysEffectValueUniformComand},
    shader_effect::{PluginShaderEffect},
    uniforms::PluginMaterialUniforms
};

pub mod material;
pub mod material_meta;
pub mod uniforms;
pub mod value;
pub mod shader_effect;

pub type MBKK = usize;

pub struct PluginMaterial;
impl crate::Plugin for PluginMaterial {
    fn init(
        &mut self,
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        PluginShaderEffect.init(engine, stages);

        let world = engine.world_mut();
        world.insert_resource(SingleMaterialIDCommandList::default());
        world.insert_resource(SingleValueUniformCommands::default());
        
        SysMaterialIDCommand::setup(world, stages.query_stage::<SysMaterialIDCommand>(ERunStageChap::Command));
        SysEffectValueUniformComand::setup(world, stages.query_stage::<SysEffectValueUniformComand>(ERunStageChap::Command));
        PluginMaterialUniforms.init(engine, stages);

        Ok(())
    }
}