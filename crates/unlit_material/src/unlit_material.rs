use material_textures::Texture2DScaleOffset;
use pi_engine_shell::object::ObjectID;
use pi_render::rhi::{dyn_uniform_buffer::{Bind, BindOffset, Uniform}, internal::bytemuck};

use pi_scene_context::{shaders::{FragmentUniformBind, FragmentUniformBindTexture, FragmentUniformBindTextureSampler, }, materials::{material::MaterialID}, bytes_write_to_memory, resources::RenderDynUniformBuffer};




pub struct UnlitMaterialPropertype {
    pub bind_offset: BindOffset,
    pub base_color: (f32, f32, f32),
    pub opacity: f32,
    pub main_texture_mat: Texture2DScaleOffset,
    pub dirty: bool,
}
impl UnlitMaterialPropertype {
    pub const BASE_INFO: usize = 4;
    pub const BASE_INFO_OFFSET: usize = 0 * 4;
    pub const MAIN_TEXTURE_MAT: usize = 4;
    pub const MAIN_TEXTURE_MAT_OFFSET: usize = Self::BASE_INFO_OFFSET +  Self::BASE_INFO * 4;

    pub fn new(dynbuffer: &mut RenderDynUniformBuffer) -> Self {
        Self {
            bind_offset: dynbuffer.alloc_binding::<Self>(),
            base_color: (1., 1., 1.),
            opacity: 1.,
            main_texture_mat: Texture2DScaleOffset::default(),
            dirty: false,
        }
    }
}
impl FragmentUniformBind for UnlitMaterialPropertype {
    const ID: u32 = 1;
    const SIZE: usize = Self::MAIN_TEXTURE_MAT_OFFSET + Self::MAIN_TEXTURE_MAT * 4;
}
impl Bind for UnlitMaterialPropertype {
    fn index() -> pi_render::rhi::dyn_uniform_buffer::BindIndex {
        pi_render::rhi::dyn_uniform_buffer::BindIndex::new(Self::ID as usize)
    }
    fn min_size() -> usize {
        Self::SIZE
    }
}
impl Uniform for UnlitMaterialPropertype {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        let v = vec![
            self.base_color.0, self.base_color.1, self.base_color.2, self.opacity,
            self.main_texture_mat.u_tiling, self.main_texture_mat.v_tiling, self.main_texture_mat.u_offset, self.main_texture_mat.v_offset, 
        ];
        bytes_write_to_memory(bytemuck::cast_slice(&v), index as usize + 0, buffer);
    }
}

#[derive(Debug, Default)]
pub struct SingleUnlitMaterialBindDynInfoSet {
    pub list: Vec<MaterialID>,
}
impl SingleUnlitMaterialBindDynInfoSet {
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
