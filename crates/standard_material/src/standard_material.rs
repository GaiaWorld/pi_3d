use pi_engine_shell::object::ObjectID;
use pi_render::rhi::{dyn_uniform_buffer::{Bind, BindOffset, Uniform}, internal::bytemuck};

use pi_scene_context::{shaders::{FragmentUniformBind, FragmentUniformBindTexture, FragmentUniformBindTextureSampler, }, materials::{material::MaterialID}, bytes_write_to_memory, resources::RenderDynUniformBuffer, texture::texture2d::{Texture2D, scale_offset::Texture2DScaleOffset}, };




pub struct StandardMaterialPropertype {
    pub bind_offset: BindOffset,
    pub emissive_color: (f32, f32, f32),
    pub emissive_intensity: f32,
    pub emissive_texture_mat: Texture2DScaleOffset,
    pub dirty: bool,
}
impl StandardMaterialPropertype {
    pub const EMISSIVE_INFO: usize = 4;
    pub const EMISSIVE_INFO_OFFSET: usize = 0 * 4;
    pub const EMISSIVE_TEXTURE_MAT: usize = 4;
    pub const EMISSIVE_TEXTURE_MAT_OFFSET: usize = Self::EMISSIVE_INFO_OFFSET +  Self::EMISSIVE_INFO * 4;

    pub fn new(dynbuffer: &mut RenderDynUniformBuffer) -> Self {
        Self {
            bind_offset: dynbuffer.alloc_binding::<Self>(),
            emissive_color: (1., 1., 1.),
            emissive_intensity: 1.,
            emissive_texture_mat: Texture2DScaleOffset::default(),
            dirty: false,
        }
    }
}
impl FragmentUniformBind for StandardMaterialPropertype {
    const ID: u32 = 1;
    const SIZE: usize = Self::EMISSIVE_TEXTURE_MAT_OFFSET + Self::EMISSIVE_TEXTURE_MAT * 4;
}
impl Bind for StandardMaterialPropertype {
    fn index() -> pi_render::rhi::dyn_uniform_buffer::BindIndex {
        pi_render::rhi::dyn_uniform_buffer::BindIndex::new(Self::ID as usize)
    }
    fn min_size() -> usize {
        Self::SIZE
    }
}
impl Uniform for StandardMaterialPropertype {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        let v = vec![
            self.emissive_color.0, self.emissive_color.1, self.emissive_color.2, self.emissive_intensity,
            self.emissive_texture_mat.u_tiling, self.emissive_texture_mat.v_tiling, self.emissive_texture_mat.u_offset, self.emissive_texture_mat.v_offset, 
        ];
        bytes_write_to_memory(bytemuck::cast_slice(&v), index as usize + 0, buffer);
    }
}

#[derive(Debug, Default)]
pub struct SingleStandardMaterialBindDynInfoSet {
    pub list: Vec<MaterialID>,
}
impl SingleStandardMaterialBindDynInfoSet {
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
