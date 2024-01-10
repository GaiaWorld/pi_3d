use pi_scene_shell::prelude::*;



pub trait TRenderShader {
    fn vs_module(&self) -> &wgpu::ShaderModule;
    fn vs_entry_point(&self) -> &'static str;
    fn fs_module(&self) -> &wgpu::ShaderModule;
    fn fs_entry_point(&self) -> &'static str;
    fn defines(&self) {}
}
impl TRenderShader for Shader3D {
    fn vs_module(&self) -> &wgpu::ShaderModule {
        &self.vs
    }

    fn vs_entry_point(&self) -> &'static str {
        self.vs_point
    }

    fn fs_module(&self) -> &wgpu::ShaderModule {
        &self.fs
    }

    fn fs_entry_point(&self) -> &'static str {
        self.fs_point
    }
}

pub trait FragmentUniformBind {
    const ID: u32;
    const SIZE: usize;

    const ENTRY: wgpu::BindGroupLayoutEntry = wgpu::BindGroupLayoutEntry {
        binding: Self::ID as u32,
        visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: true,
            min_binding_size: wgpu::BufferSize::new(Self::SIZE as wgpu::BufferAddress),
        },
        count: None,
    };

    // fn entry<'a>(
    //     bind_offset: &BindOffset,
    //     buff: &'a render_resource::uniform_buffer::RenderDynUniformBuffer,
    // ) -> wgpu::BindGroupEntry<'a> {
    //     bind_group_entry_buffer(Self::ID as u32, buff.buffer().unwrap(), **bind_offset, Self::SIZE as u32)
    // }

    // fn dyn_entry<'a>(
    //     buff: &'a render_resource::uniform_buffer::RenderDynUniformBuffer,
    // ) -> wgpu::BindGroupEntry<'a> {
    //     bind_group_entry_buffer(Self::ID as u32, buff.buffer().unwrap(), 0, Self::SIZE as u32)
    // }
}

pub trait FragmentUniformBindTexture {
    const TEXTURE_BIND: u8;
    const TEXTURE_SAMPLER_TYPE: wgpu::TextureSampleType;
    const DIM: wgpu::TextureViewDimension;
    const MULTI: bool;

    const ENTRY_TEXTURE: wgpu::BindGroupLayoutEntry = wgpu::BindGroupLayoutEntry {
        binding: Self::TEXTURE_BIND as u32,
        visibility: wgpu::ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Texture {
            sample_type: Self::TEXTURE_SAMPLER_TYPE,
            view_dimension: Self::DIM,
            multisampled: Self::MULTI
        },
        count: None,
    };

    fn entry_texture<'a>(
        view: &'a wgpu::TextureView,
    ) -> wgpu::BindGroupEntry<'a> {
        wgpu::BindGroupEntry {
            binding: Self::TEXTURE_BIND as u32,
            resource: wgpu::BindingResource::TextureView(view),
        }
    }
}

pub trait FragmentUniformBindTextureSampler {
    const SAMPLER_BIND: u8;
    const SAMPLER_TYPE: wgpu::SamplerBindingType;

    const ENTRY_SAMPLER: wgpu::BindGroupLayoutEntry = wgpu::BindGroupLayoutEntry {
        binding: Self::SAMPLER_BIND as u32,
        visibility: wgpu::ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Sampler(Self::SAMPLER_TYPE),
        count: None,
    };

    fn entry_sampler<'a>(
        value: &'a Sampler,
    ) -> wgpu::BindGroupEntry<'a> {
        wgpu::BindGroupEntry {
            binding: Self::SAMPLER_BIND as u32,
            resource: wgpu::BindingResource::Sampler(value),
        }
    }
}

pub fn bind_group_entry_buffer(
    id: u32,
    buffer: &wgpu::Buffer,
    offset: u32,
    size: u32,
) -> wgpu::BindGroupEntry {
    wgpu::BindGroupEntry {
        binding: id,
        resource: wgpu::BindingResource::Buffer(
            wgpu::BufferBinding {
                buffer,
                offset:  offset as wgpu::BufferAddress,
                size: wgpu::BufferSize::new(size as wgpu::BufferAddress),
            }
        ),
    }
}