use pi_render::rhi::dyn_uniform_buffer::{BindOffset, Bind, Uniform};

use crate::{resources::RenderDynUniformBuffer, shaders::FragmentUniformBind, bytes_write_to_memory, materials::material::MaterialID};

use super::texture::DefaultTexture;

pub struct SkyboxMaterialPropertype {
    pub bind_offset: BindOffset,
    pub emissive_color: (f32, f32, f32),
    pub emissive_intensity: f32,
    pub texture: (Vec<u8>, u32, u32),
}
impl SkyboxMaterialPropertype {
    pub const EMISSIVE: usize = 4;
    pub const EMISSIVE_OFFSET: usize = 0 * 4;

    pub fn new(dynbuffer: &mut RenderDynUniformBuffer) -> Self {
        Self {
            bind_offset: dynbuffer.alloc_binding::<Self>(),
            emissive_color: (1., 1., 0.),
            emissive_intensity: 0.5,
            texture: pi_hal::image::from_memory(include_bytes!("./assets/bottom.jpg")).unwrap(),
        }
    }
}
impl FragmentUniformBind for SkyboxMaterialPropertype {
    const ID: u32 = 1;
    const SIZE: usize = Self::EMISSIVE_OFFSET + Self::EMISSIVE * 4;
}
impl Bind for SkyboxMaterialPropertype {
    fn index() -> pi_render::rhi::dyn_uniform_buffer::BindIndex {
        pi_render::rhi::dyn_uniform_buffer::BindIndex::new(Self::ID as usize)
    }
    fn min_size() -> usize {
        Self::SIZE
    }
}
impl Uniform for SkyboxMaterialPropertype {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        let v = vec![
            self.emissive_color.0, self.emissive_color.1, self.emissive_color.2, self.emissive_intensity
        ];
        bytes_write_to_memory(bytemuck::cast_slice(&v), index as usize + 0, buffer);
    }
}

#[derive(Debug, Default)]
pub struct SingleSkyboxMaterialBindDynInfoSet {
    pub list: Vec<MaterialID>,
}
impl SingleSkyboxMaterialBindDynInfoSet {
    pub fn remove(
        &mut self,
        id: MaterialID,
    ) {
        match self.list.binary_search(&id) {
            Ok(index) => {
                self.list.swap_remove(index);
            },
            Err(_) => {},
        }
    }
    pub fn add(
        &mut self,
        id: MaterialID,
    ) {
        match self.list.binary_search(&id) {
            Ok(index) => {
                self.list.swap_remove(index);
            },
            Err(index) => {
                self.list.insert(index, id);
            },
        }
    }
    pub fn list(&mut self) -> &mut Vec<MaterialID> {
        &mut self.list
    }
}
