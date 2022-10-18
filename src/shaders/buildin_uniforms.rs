use pi_render::rhi::dyn_uniform_buffer::{BindOffset, DynUniformBuffer, Bind};

use super::FragmentUniformBind;


pub struct BuildinCameraBind {
    pub bind_offset: BindOffset,
}
impl BuildinCameraBind {

    pub const PI_MATRIX_V: usize = 16;
    pub const PI_MATRIX_P: usize = 16;
    pub const PI_CAMERA_POSITION: usize = 4;
    pub const PI_ORTHCAMERA_DIRECT: usize = 4;

    pub const PI_MATRIX_V_OFFSIZE: usize = 0 * 4;
    pub const PI_MATRIX_P_OFFSIZE: usize = Self::PI_MATRIX_V_OFFSIZE + Self::PI_MATRIX_V * 4;
    pub const PI_CAMERA_POSITION_OFFSIZE: usize = Self::PI_MATRIX_P_OFFSIZE + Self::PI_MATRIX_P * 4;
    pub const PI_ORTHCAMERA_DIRECT_OFFSIZE: usize = Self::PI_CAMERA_POSITION_OFFSIZE + Self::PI_CAMERA_POSITION * 4;

}
impl FragmentUniformBind for BuildinCameraBind {
    const ID: u32 = 0;
    const SIZE: usize = Self::PI_ORTHCAMERA_DIRECT_OFFSIZE + Self::PI_ORTHCAMERA_DIRECT * 4;
}
impl Bind for BuildinCameraBind {
    fn index() -> pi_render::rhi::dyn_uniform_buffer::BindIndex {
        pi_render::rhi::dyn_uniform_buffer::BindIndex::new(Self::ID as usize)
    }
    fn min_size() -> usize {
        Self::SIZE
    }
}

pub struct BuildinTimeBind {
    pub bind_offset: BindOffset,
}
impl BuildinTimeBind {
    pub const TIME: usize = 4;
    pub const DELTA_TIME: usize = 4;

    pub const TIME_OFFSIZE: usize = 0 * 4;
    pub const DELTA_TIME_OFFSIZE: usize = Self::TIME_OFFSIZE + Self::TIME * 4;

}
impl FragmentUniformBind for BuildinTimeBind {
    const ID: u32 = 1;
    const SIZE: usize = Self::DELTA_TIME_OFFSIZE + Self::DELTA_TIME * 4;
}
impl Bind for BuildinTimeBind {
    fn index() -> pi_render::rhi::dyn_uniform_buffer::BindIndex {
        pi_render::rhi::dyn_uniform_buffer::BindIndex::new(Self::ID as usize)
    }
    fn min_size() -> usize {
        Self::SIZE
    }
}

pub struct BuildinFogBind {
    pub bind_offset: BindOffset,
}
impl BuildinFogBind {
    pub const FOG_PARAM: usize = 4;
    pub const FOG_COLOR: usize = 4;

    pub const FOG_PARAM_OFFSIZE: usize = 0 * 4;
    pub const FOG_COLOR_OFFSIZE: usize = Self::FOG_PARAM_OFFSIZE + Self::FOG_PARAM_OFFSIZE * 4;
}
impl FragmentUniformBind for BuildinFogBind {
    const ID: u32 = 2;
    const SIZE: usize = Self::FOG_COLOR_OFFSIZE + Self::FOG_COLOR * 4;
}
impl Bind for BuildinFogBind {
    fn index() -> pi_render::rhi::dyn_uniform_buffer::BindIndex {
        pi_render::rhi::dyn_uniform_buffer::BindIndex::new(Self::ID as usize)
    }
    fn min_size() -> usize {
        Self::SIZE
    }
}

pub struct BuildinAmbientLightBind {
    pub bind_offset: BindOffset,
}
impl BuildinAmbientLightBind {
    pub const AMBIENT_LIGHT: usize = 4;
    pub const AMBIENT_LIGHT_OFFSIZE: usize = 0 * 4;
}
impl FragmentUniformBind for BuildinAmbientLightBind {
    const ID: u32 = 3;
    const SIZE: usize = Self::AMBIENT_LIGHT_OFFSIZE + Self::AMBIENT_LIGHT * 4;
}
impl Bind for BuildinAmbientLightBind {
    fn index() -> pi_render::rhi::dyn_uniform_buffer::BindIndex {
        pi_render::rhi::dyn_uniform_buffer::BindIndex::new(Self::ID as usize)
    }
    fn min_size() -> usize {
        Self::SIZE
    }
}

/// Model Uniform Bind
pub struct BuildinModelBind {
    pub bind_offset: BindOffset,
}
impl BuildinModelBind {
    pub const OBJECT_TO_WORLD: usize = 16;
    pub const WORLD_TO_OBJECT: usize = 16;

    pub const OBJECT_TO_WORLD_OFFSIZE: usize = 0 * 4;
    pub const WORLD_TO_OBJECT_OFFSIZE: usize = Self::OBJECT_TO_WORLD_OFFSIZE + Self::WORLD_TO_OBJECT * 4;
}
impl FragmentUniformBind for BuildinModelBind {
    const ID: u32 = 0;
    const SIZE: usize = Self::WORLD_TO_OBJECT_OFFSIZE + Self::WORLD_TO_OBJECT * 4;
}
impl Bind for BuildinModelBind {
    fn index() -> pi_render::rhi::dyn_uniform_buffer::BindIndex {
        pi_render::rhi::dyn_uniform_buffer::BindIndex::new(Self::ID as usize)
    }
    fn min_size() -> usize {
        Self::SIZE
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