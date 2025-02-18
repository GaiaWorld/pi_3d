// use pi_render::renderer::bind::{KeyBindBuffer, KeyBindLayoutBuffer, TKeyBind};

// use crate::{prelude::*, run_stage::EngineCustomPlugins};


// #[derive(Clone, Hash, PartialEq, Eq)]
// pub struct BindEffectClipPlane {
//     pub data: BindBufferRange,
//     pub maxcount: u32,
// }
// // 每个 plane 数据为 4 个浮点数, nx ny nz d
// // 记 四个plane
// // 额外增加 vec4 表达是否使用对应 plane
// impl BindEffectClipPlane {
//     pub const ITEM_SIZE: usize = 4 * 4;
//     pub const TOTAL_SIZE: usize = 4 * 4 * 4 + 4 * 4;
//     pub fn new(maxcount: u32, bindbuffer: &mut BindBufferAllocator, _engineopt: &EngineCustomPlugins) -> Option<Self> {
//         if let Some(buffer) = bindbuffer.allocate(maxcount * Self::TOTAL_SIZE as u32) {
//             let tilloff = [1f32, 0f32, 0f32, 0f32];
//             let val = bytemuck::cast_slice(&tilloff);
//             let vcount = maxcount * 4;
//             for i in 0..vcount {
//                 buffer.0.write_data(i as usize * Self::ITEM_SIZE, val);
//             }
//             let address = [-1f32, -1f32, -1f32, -1f32];
//             let val = bytemuck::cast_slice(&address);
//             for i in 0..maxcount {
//                 buffer.0.write_data(vcount as usize * Self::ITEM_SIZE + i as usize * Self::ITEM_SIZE, val);
//             }

//             Some(
//                 Self {
//                     data: buffer,
//                     maxcount
//                 }
//             )
//         } else {
//             None
//         }
//     }
//     pub fn update(&self, matidx: usize, idx: usize, active: bool, a: f32, b: f32, c: f32, d: f32) {
//         self.data.write_data((idx * self.maxcount as usize + matidx) * Self::ITEM_SIZE, bytemuck::cast_slice(&[a, b, c, d]));
//         if active {
//             self.data.write_data(4 * self.maxcount as usize * Self::ITEM_SIZE + matidx * Self::ITEM_SIZE + idx, bytemuck::cast_slice(&[0f32]));
//         } else {
//             self.data.write_data(4 * self.maxcount as usize * Self::ITEM_SIZE + matidx * Self::ITEM_SIZE + idx, bytemuck::cast_slice(&[99999999f32]));
//         }
//     }
//     fn define_code(&self, set: u32, bind: u32) -> String {
//         let mut result = String::from("");
//         result += ShaderSetBind::code_set_bind_head(set, bind).as_str();
//         result += "ClipPlaneInfoArr {";
//         result += crate::prelude::S_BREAK;
//         result += ShaderSetBind::code_uniform_array(&crate::prelude::S_VEC4, ShaderVarUniform::CLIP_PLANE0, self.maxcount as u32).as_str();
//         result += crate::prelude::S_BREAK;
//         result += ShaderSetBind::code_uniform_array(&crate::prelude::S_VEC4, ShaderVarUniform::CLIP_PLANE1, self.maxcount as u32).as_str();
//         result += crate::prelude::S_BREAK;
//         result += ShaderSetBind::code_uniform_array(&crate::prelude::S_VEC4, ShaderVarUniform::CLIP_PLANE2, self.maxcount as u32).as_str();
//         result += crate::prelude::S_BREAK;
//         result += ShaderSetBind::code_uniform_array(&crate::prelude::S_VEC4, ShaderVarUniform::CLIP_PLANE3, self.maxcount as u32).as_str();
//         result += crate::prelude::S_BREAK;
//         result += ShaderSetBind::code_uniform_array(&crate::prelude::S_VEC4, ShaderVarUniform::ICLIP_PLANE, self.maxcount as u32).as_str();
//         result += "};\n";

//         result += "vec4 clipPlane(vec4 worldPos, int vMatIdx) {"; result += crate::prelude::S_BREAK;
//         result += "vec4 result = vec4(dot(PI_CLIP_PLANE0[vMatIdx], worldPos), dot(PI_CLIP_PLANE1[vMatIdx], worldPos), dot(PI_CLIP_PLANE2[vMatIdx], worldPos), dot(PI_CLIP_PLANE3[vMatIdx], worldPos));";
//         result += "result += I_CLIP_PLANE[vMatIdx];return result;";
//         result += "}"; result += crate::prelude::S_BREAK;
//         result
//     }
// }

// impl BindEffectClipPlane {
//     pub fn vs_define_code(&self, set: u32, bind: u32) -> String {
//         self.define_code(set, bind)
//     }
//     pub fn fs_define_code(&self, set: u32, bind: u32) -> String {
//         self.define_code(set, bind)
//     }
//     pub fn vs_running_code(&self) -> String {
//         let mut result = String::from("");
//         result += "vClipDistance = vec4(1., 1., 1., 1.);"; result += crate::prelude::S_BREAK;
//         result
//     }
//     pub fn fs_running_code(&self) -> String {
//         let mut result = String::from("");
//         result += "if (vClipDistance.x < 0. || vClipDistance.y < 0. || vClipDistance.z < 0. && vClipDistance.w < 0.) { discard; }"; result += crate::prelude::S_BREAK;
//         result
//     }
// }
// impl TKeyBind for BindEffectClipPlane {
//     fn key_bind(&self) -> Option<pi_render::renderer::bind::EKeyBind> {
//         Some(
//             pi_render::renderer::bind::EKeyBind::Buffer(
//                 KeyBindBuffer {
//                     data: self.data.clone(),
//                     layout: KeyBindLayoutBuffer {
//                         visibility: EShaderStage::VERTEXFRAGMENT,
//                         min_binding_size: self.data.size() as u32,
//                     }
//                 }
//             )
//         )
//     }
// }
// impl TBindDefine for BindEffectClipPlane {
//     fn bind_include(&self) -> u32 {
//         BindDefines::EFFECT_CLIPPLANE
//     }
// }
