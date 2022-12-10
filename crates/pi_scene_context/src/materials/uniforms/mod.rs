use pi_ecs::prelude::Setup;
use pi_engine_shell::plugin::Plugin;
use pi_scene_math::Number;

use self::{sys_float::PluginFloatSlot, sys_uint::PluginUintSlot, sys_int::PluginIntSlot, sys_mat2::PluginMat2Slot, sys_mat4::PluginMat4Slot, sys_vec2::PluginVec2Slot, sys_vec4::PluginVec4Slot, sys_texture::PluginTextureSlot, uniform::{SysMaterialMetaChange, SysValueBindgroupUpdate}};

pub mod value_uniform;
pub mod texture_uniform;
pub mod sys_mat4;
pub mod sys_mat2;
pub mod sys_float;
pub mod sys_int;
pub mod sys_uint;
pub mod sys_vec2;
pub mod sys_vec4;
pub mod uniform;
pub mod float;
pub mod vec2;
pub mod vec4;
pub mod mat2;
pub mod mat4;
pub mod int;
pub mod uint;
pub mod boolean;
pub mod byte;
pub mod texture;
pub mod sys_texture;

pub(crate) fn update_data(
    data: &mut [Number],
    slot: usize,
    value: &[Number],
    num_count: usize,
) {
    if value.len() >= num_count {
        for i in 0..num_count {
            data[slot * num_count + i] = value[i];
        }
    }
}

pub struct PluginMaterialUniforms;
impl Plugin for PluginMaterialUniforms {
    fn init(
        &mut self,
        engine: &mut pi_engine_shell::engine_shell::EnginShell,
        stages: &mut pi_engine_shell::run_stage::RunStage,
    ) -> Result<(), pi_engine_shell::plugin::ErrorPlugin> {

        PluginFloatSlot.init(engine, stages);
        PluginIntSlot.init(engine, stages);
        PluginMat2Slot.init(engine, stages);
        PluginMat4Slot.init(engine, stages);
        PluginUintSlot.init(engine, stages);
        PluginVec2Slot.init(engine, stages);
        PluginVec4Slot.init(engine, stages);

        PluginTextureSlot.init(engine, stages);

        let world = engine.world_mut();
        SysMaterialMetaChange::setup(world, stages.command_stage());
        SysValueBindgroupUpdate::setup(world, stages.between_uniform_update_and_filter_culling());

        Ok(())
    }
}