
use pi_render::rhi::{device::RenderDevice, bind_group::BindGroup, texture::Sampler, asset::TextureRes};
use render_shader::unifrom_code::{ MaterialTextureBindDesc};

use crate::materials::bind_group::{RenderBindGroup, RenderBindGroupKey};

pub trait TForTextureBindGroup {
    fn bind_group(
        &self,
        device: &RenderDevice,
        group: &mut RenderBindGroup,
        textures: &[&TextureRes],
        samplers: &[&Sampler],
    );
}

impl TForTextureBindGroup for MaterialTextureBindDesc {
    fn bind_group(
        &self,
        device: &RenderDevice,
        group: &mut RenderBindGroup,
        textures: &[&TextureRes],
        samplers: &[&Sampler],
    ) {
        let mut entries = vec![];

        for i in 0..self.list.len() {
            entries.push(
                wgpu::BindGroupEntry {
                    binding: i as u32 * 2 + 0,
                    resource: wgpu::BindingResource::TextureView(&textures.get(i).unwrap().texture_view),
                }
            );
            
            entries.push(
                wgpu::BindGroupEntry {
                    binding: i as u32 * 2 + 1,
                    resource: wgpu::BindingResource::Sampler(samplers.get(i).unwrap()),
                }
            );
        }

        group.bind_group = Some(
            BindGroup::from(
                device.create_bind_group(
                    &wgpu::BindGroupDescriptor {
                        label: Some(self.label().as_str()),
                        layout: &group.layout,
                        entries: entries.as_slice(),
                    }
                )
            )
        );
    }
}

pub struct MaterialTextureBindGroupID(pub RenderBindGroupKey);