use std::sync::Arc;

use pi_render::rhi::{device::RenderDevice, texture::Sampler, dyn_uniform_buffer::Uniform};
use render_resource::{data_texture2d::DataTexture2D, sampler::{SamplerPool, SamplerDesc, EAnisotropyClamp}};
use render_shader::shader_bind::ShaderBindModelAboutSkin;

use crate::bytes_write_to_memory;


pub struct SkinTexture {
    pub tex: Arc<DataTexture2D>,
    pub sampler: Sampler,
    row: u32,
}

impl SkinTexture {
    pub fn new(device: &RenderDevice, samplerpool: &mut SamplerPool, bone_count: u32, frames: u32) -> Self {
        let tex = DataTexture2D::new_rgba_f32(device, (bone_count + 1) * 4, frames);

        let desc = SamplerDesc {
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            compare: None,
            anisotropy_clamp: EAnisotropyClamp::None,
            border_color: None,
        };
        samplerpool.create(&desc, &device);
        let sampler = samplerpool.get(SamplerPool::cacl_key(&desc)).unwrap();

        Self {
            tex: Arc::new(tex),
            sampler,
            row: frames,
        }
    }

    pub fn row(&self) -> u32 {
        self.row
    }
}

impl Uniform for SkinTexture {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        let size = self.tex.size();
        let data = vec![size.width as f32, size.height as f32, 1.0 / size.width as f32, 1.0 / size.height as f32];
        bytes_write_to_memory(bytemuck::cast_slice(data.as_slice()), index as usize + ShaderBindModelAboutSkin::OFFSET_BONE_TEX_SIZE as usize, buffer);
    }
}