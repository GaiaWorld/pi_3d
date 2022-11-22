use pi_render::rhi::{
    dyn_uniform_buffer::{Bind, BindOffset, Uniform},
    internal::bytemuck,
};

use pi_scene_context::{
    bytes_write_to_memory, materials::material::MaterialID, resources::RenderDynUniformBuffer,
    shaders::FragmentUniformBind,
};

pub struct CloudMaterialPropertype {
    pub bind_offset: BindOffset,
    pub sky_color: (f32, f32, f32, f32),
    pub cloud_color: (f32, f32, f32, f32),
    pub amplitude: f32,
    pub num_octaves: f32,

    pub width: f32,
    pub height: f32,
}
impl CloudMaterialPropertype {
    pub const EMISSIVE: usize = 12;
    pub const EMISSIVE_OFFSET: usize = 0 * 4;

    pub fn new(dynbuffer: &mut RenderDynUniformBuffer) -> Self {
        Self {
            bind_offset: dynbuffer.alloc_binding::<Self>(),
            sky_color: (0.15, 0.68, 1.0, 1.0),
            cloud_color: (1.0, 1., 1., 1.0),
            amplitude: 1.0,
            num_octaves: 4.0,
            width: 800.0,
            height: 600.0,
        }
    }
}
impl FragmentUniformBind for CloudMaterialPropertype {
    const ID: u32 = 1;
    const SIZE: usize = Self::EMISSIVE_OFFSET + Self::EMISSIVE * 4;
}
impl Bind for CloudMaterialPropertype {
    fn index() -> pi_render::rhi::dyn_uniform_buffer::BindIndex {
        pi_render::rhi::dyn_uniform_buffer::BindIndex::new(Self::ID as usize)
    }
    fn min_size() -> usize {
        Self::SIZE
    }
}
impl Uniform for CloudMaterialPropertype {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        let v = vec![
            self.sky_color.0,
            self.sky_color.1,
            self.sky_color.2,
            self.sky_color.3,
            self.cloud_color.0,
            self.cloud_color.1,
            self.cloud_color.2,
            self.cloud_color.3,
            self.amplitude,
            self.num_octaves,
            self.width,
            self.height,
        ];
        bytes_write_to_memory(bytemuck::cast_slice(&v), index as usize + 0, buffer);
    }
}

#[derive(Debug, Default)]
pub struct SingleCloudMaterialBindDynInfoSet {
    pub list: Vec<MaterialID>,
}
impl SingleCloudMaterialBindDynInfoSet {
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
