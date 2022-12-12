use std::time::Instant;

use pi_render::rhi::{
    dyn_uniform_buffer::{Bind, BindOffset, Uniform},
    internal::bytemuck,
};

use pi_scene_context::{
    bytes_write_to_memory, materials::material::MaterialID, resources::RenderDynUniformBuffer,
    shaders::FragmentUniformBind,
};

pub struct WaterMaterialPropertype {
    pub bind_offset: BindOffset,
    pub sea_base: (f32, f32, f32, f32),
    pub sea_water_color: (f32, f32, f32, f32),
    pub num_octaves: Instant,
    pub width: f32,
    pub height: f32,
    phantom_data: f32,
}
impl WaterMaterialPropertype {
    pub const EMISSIVE: usize = 12;
    pub const EMISSIVE_OFFSET: usize = 0 * 4;

    pub fn new(dynbuffer: &mut RenderDynUniformBuffer) -> Self {
        Self {
            bind_offset: dynbuffer.alloc_binding::<Self>(),
            sea_base: (0.0, 0.09, 0.18, 1.0),
            sea_water_color: (0.48, 0.54, 0.36, 1.0),
            num_octaves: Instant::now(),
            width: 800.0,
            height: 600.0,
            phantom_data: 1.0,
        }
    }
}
impl FragmentUniformBind for WaterMaterialPropertype {
    const ID: u32 = 1;
    const SIZE: usize = Self::EMISSIVE_OFFSET + Self::EMISSIVE * 4;
}
impl Bind for WaterMaterialPropertype {
    fn index() -> pi_render::rhi::dyn_uniform_buffer::BindIndex {
        pi_render::rhi::dyn_uniform_buffer::BindIndex::new(Self::ID as usize)
    }
    fn min_size() -> usize {
        Self::SIZE
    }
}
impl Uniform for WaterMaterialPropertype {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        let v = vec![
            self.sea_base.0,
            self.sea_base.1,
            self.sea_base.2,
            self.sea_base.3,
            self.sea_water_color.0,
            self.sea_water_color.1,
            self.sea_water_color.2,
            self.sea_water_color.3,
            self.width,
            self.height,
            (self.num_octaves.elapsed().as_millis() as f32 / 200.0),
            self.phantom_data,
        ];
        bytes_write_to_memory(bytemuck::cast_slice(&v), index as usize + 0, buffer);
    }
}

#[derive(Debug, Default)]
pub struct SingleWaterMaterialBindDynInfoSet {
    pub list: Vec<MaterialID>,
}
impl SingleWaterMaterialBindDynInfoSet {
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
