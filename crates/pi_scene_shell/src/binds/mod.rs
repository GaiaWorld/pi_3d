
mod effect_value;
mod effect_sampler2d;
mod effect_texture2d;
mod scene;
mod model;
mod effect_textureids;
mod effect_texture_info;
mod effect_texture_tilloff;
mod clipplane;

use pi_render::renderer::bind_buffer::BindBufferRange;
use pi_render::renderer::shader_stage::EShaderStage;
use pi_render::renderer::bind::KeyBindBuffer;
use pi_render::renderer::bind::KeyBindLayoutBuffer;

pub use effect_value::*;
pub use effect_sampler2d::*;
pub use effect_texture2d::*;
pub use scene::*;
pub use model::*;
pub use clipplane::*;
pub use effect_textureids::*;
pub use effect_texture_info::*;
pub use effect_texture_tilloff::*;

pub fn keybind_for_buffer(data: BindBufferRange, visibility: EShaderStage, min_binding_size: u32) -> Option<pi_render::renderer::bind::EKeyBind> {
    Some(
        pi_render::renderer::bind::EKeyBind::Buffer(
            KeyBindBuffer {
                data,
                layout: KeyBindLayoutBuffer {
                    visibility,
                    min_binding_size,
                }
            }
        )
    )
}