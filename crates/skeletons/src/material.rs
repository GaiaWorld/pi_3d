use pi_render::rhi::{
    dyn_uniform_buffer::{Bind, BindOffset, Uniform},
    internal::bytemuck,
};

use pi_scene_math::Matrix;

use pi_scene_context::{
    bytes_write_to_memory, materials::material::MaterialID, resources::RenderDynUniformBuffer,
    shaders::FragmentUniformBind,
};

pub struct SkeletonsPropertype {
    pub bind_offset: BindOffset,
    pub bones: Vec<Matrix>,
}
impl SkeletonsPropertype {
    pub const EMISSIVE: usize = 4;
    pub const EMISSIVE_OFFSET: usize = 0 * 4;

    pub fn new(dynbuffer: &mut RenderDynUniformBuffer) -> Self {
        Self {
            bind_offset: dynbuffer.alloc_binding::<Self>(),
            // bone_texture_width: 3.,
            bones: vec![
                Matrix::new(
                    1., 0., 0., 0., 0., 1., 0., 0., 0., 0., -1., 0., 0., 0., 0., 1.,
                ),
                Matrix::new(
                    1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 1., 0., 1.,
                ),
            ],
        }
    }
}
impl FragmentUniformBind for SkeletonsPropertype {
    const ID: u32 = 1;
    const SIZE: usize = Self::EMISSIVE_OFFSET + Self::EMISSIVE * 4;
}

impl Bind for SkeletonsPropertype {
    fn index() -> pi_render::rhi::dyn_uniform_buffer::BindIndex {
        pi_render::rhi::dyn_uniform_buffer::BindIndex::new(Self::ID as usize)
    }
    fn min_size() -> usize {
        Self::SIZE
    }
}
impl Uniform for SkeletonsPropertype {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        let mut v = vec![];
        for bone in &self.bones {
            v.append(&mut bone.as_slice().to_vec())
        }
        bytes_write_to_memory(bytemuck::cast_slice(&v), index as usize + 0, buffer);
    }
}

#[derive(Debug, Default)]
pub struct SingleSkeletonsBindDynInfoSet {
    pub list: Vec<MaterialID>,
}
impl SingleSkeletonsBindDynInfoSet {
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
