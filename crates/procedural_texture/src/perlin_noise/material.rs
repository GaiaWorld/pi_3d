use pi_render::rhi::{
    dyn_uniform_buffer::{Bind, BindOffset, Uniform},
    internal::bytemuck,
};

use pi_scene_context::{
    bytes_write_to_memory, materials::material::MaterialID, resources::RenderDynUniformBuffer,
    shaders::FragmentUniformBind,
};

pub struct PerlinNoiseMaterialPropertype {
    pub bind_offset: BindOffset,
    pub size: f32,
    pub width: f32,
    pub height: f32,
}
impl PerlinNoiseMaterialPropertype {
    pub const EMISSIVE: usize = 4;
    pub const EMISSIVE_OFFSET: usize = 0 * 4;

    pub fn new(dynbuffer: &mut RenderDynUniformBuffer) -> Self {
        Self {
            bind_offset: dynbuffer.alloc_binding::<Self>(),
            size: 10.0,
            width: 800.0,
            height: 600.0,
        }
    }
}
impl FragmentUniformBind for PerlinNoiseMaterialPropertype {
    const ID: u32 = 1;
    const SIZE: usize = Self::EMISSIVE_OFFSET + Self::EMISSIVE * 4;
}
impl Bind for PerlinNoiseMaterialPropertype {
    fn index() -> pi_render::rhi::dyn_uniform_buffer::BindIndex {
        pi_render::rhi::dyn_uniform_buffer::BindIndex::new(Self::ID as usize)
    }
    fn min_size() -> usize {
        Self::SIZE
    }
}
impl Uniform for PerlinNoiseMaterialPropertype {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        let v = vec![self.size, self.width, self.height, 0.0];
        bytes_write_to_memory(bytemuck::cast_slice(&v), index as usize + 0, buffer);
    }
}

#[derive(Debug, Default)]
pub struct SinglePerlinNoiseMaterialBindDynInfoSet {
    pub list: Vec<MaterialID>,
}
impl SinglePerlinNoiseMaterialBindDynInfoSet {
    pub fn remove(&mut self, id: MaterialID) {
        match self.list.binary_search(&id) {
            Ok(index) => {
                self.list.swap_remove(index);
            }
            Err(_) => {}
        }
    }
    pub fn add(&mut self, id: MaterialID) {
        match self.list.binary_search(&id) {
            Ok(index) => {
                self.list.swap_remove(index);
            }
            Err(index) => {
                self.list.insert(index, id);
            }
        }
    }
    pub fn list(&mut self) -> &mut Vec<MaterialID> {
        &mut self.list
    }
}
