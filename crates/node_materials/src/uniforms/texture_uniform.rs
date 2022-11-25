use pi_assets::asset::Handle;
use pi_atom::Atom;
use pi_render::rhi::{device::RenderDevice, bind_group::BindGroup, texture::Sampler, asset::TextureRes};
use pi_scene_context::materials::bind_group::{RenderBindGroupPool, RenderBindGroup, RenderBindGroupKey};
use render_resource::{ImageAssetKey, sampler::SamplerDesc};

use super::{texture::UniformTexture, texture::UniformSampler};

pub struct UniformTextureStatistics {
    pub texture_count: u8,
    pub sampler_count: u8,
}

#[derive(Debug)]
pub struct TextureDesc {
    pub slotname: Atom,
    pub path: ImageAssetKey,
    pub sampler: SamplerDesc,
    pub sampler_binding_type: wgpu::SamplerBindingType,
    pub tex_sampler_type: wgpu::TextureSampleType,
    pub dimension: wgpu::TextureViewDimension,
    pub multisampled: bool,
}

#[derive(Debug)]
pub struct TextureBindDesc {
    pub texture_desc_list: Vec<TextureDesc>,
}
impl TextureBindDesc {
    pub const BIND_GROUP_SET: u32 = RenderBindGroupPool::TEXTURE_BIND_GROUP_SET;
    pub fn layout_entries(&self) -> Vec<wgpu::BindGroupLayoutEntry> {
        let mut result = vec![];
        
        let mut i = 0;
        self.texture_desc_list.iter().for_each(|item| {
            result.push(
                wgpu::BindGroupLayoutEntry {
                    binding: i * 2 + 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: item.tex_sampler_type,
                        view_dimension: item.dimension,
                        multisampled: item.multisampled
                    },
                    count: None,
                }
            );
            
            result.push(
                wgpu::BindGroupLayoutEntry {
                    binding: i * 2 + 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(item.sampler_binding_type),
                    count: None,
                }
            );

            i += 1;
        });
        
        result
    }

    pub fn label(&self) -> String {
        let mut result = String::from("");
        
        self.texture_desc_list.iter().for_each(|item| {
            result += "#";
            result += item.slotname.as_str();
        });

        result
    }

    pub fn bind_group(
        &self,
        device: &RenderDevice,
        group: &mut RenderBindGroup,
        textures: &[&TextureRes],
        samplers: &[&Sampler],
    ) {
        let mut entries = vec![];

        for i in 0..self.texture_desc_list.len() {
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

pub struct TextureBindGroup(pub RenderBindGroupKey);