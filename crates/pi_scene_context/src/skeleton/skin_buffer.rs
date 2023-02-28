use std::sync::Arc;


use crate::bytes_write_to_memory;

// pub struct SkinDataBuffer {
//     pub bind: Arc<ShaderBindModelAboutSkin>,
// }

pub struct TempSkinBufferData(pub Vec<f32>);
// impl Uniform for TempSkinBufferData {
//     fn write_into(&self, index: u32, buffer: &mut [u8]) {
//         bytes_write_to_memory(bytemuck::cast_slice(self.0.as_slice()), index as usize + 0, buffer);
//     }
// }