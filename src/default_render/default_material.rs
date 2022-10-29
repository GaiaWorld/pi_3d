use pi_ecs::world::World;
use pi_hash::XHashMap;
use pi_render::rhi::{dyn_uniform_buffer::{Bind, BindOffset, DynUniformBuffer, Uniform}, device::RenderDevice, bind_group::BindGroup, pipeline::RenderPipeline, bind_group_layout::BindGroupLayout};
use pi_scene_math::{Color4, Matrix};
use pi_slotmap::DefaultKey;
use render_geometry::geometry::VertexAttributeMeta;
use render_material::material::{Material, UnifromData};
use render_pipeline_key::{pipeline_key::{gen_pipeline_key, PipelineKeyCalcolator}, fragment_state::gen_fragment_state_key};
use render_resource::{bind_group::AsMaterialBindGroup, bind_group_layout::AsUniformBindingBufferDynamic};

use crate::{shaders::{BuildinShaderDefined, FragmentUniformBind, }, materials::{MBKK, material::MaterialID}, bytes_write_to_memory, };

use super::shader::DefaultShader;


pub struct DefaultMaterialPropertype {
    pub bind_offset: BindOffset,
    pub emissive_color: (f32, f32, f32),
    pub emissive_intensity: f32,
}
impl DefaultMaterialPropertype {
    pub const EMISSIVE: usize = 4;
    pub const EMISSIVE_OFFSET: usize = 0 * 4;

    pub fn new(dynbuffer: &mut DynUniformBuffer) -> Self {
        Self {
            bind_offset: dynbuffer.alloc_binding::<Self>(),
            emissive_color: (1., 1., 1.),
            emissive_intensity: 1.,
        }
    }
}
impl FragmentUniformBind for DefaultMaterialPropertype {
    const ID: u32 = 1;
    const SIZE: usize = Self::EMISSIVE_OFFSET + Self::EMISSIVE * 4;
}
impl Bind for DefaultMaterialPropertype {
    fn index() -> pi_render::rhi::dyn_uniform_buffer::BindIndex {
        pi_render::rhi::dyn_uniform_buffer::BindIndex::new(Self::ID as usize)
    }
    fn min_size() -> usize {
        Self::SIZE
    }
}
impl Uniform for DefaultMaterialPropertype {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        let v = vec![
            self.emissive_color.0, self.emissive_color.1, self.emissive_color.2, self.emissive_intensity
        ];
        bytes_write_to_memory(bytemuck::cast_slice(&v), index as usize + 0, buffer);
    }
}

#[derive(Debug, Default)]
pub struct SingleDefaultMaterialBindDynInfoSet {
    pub list: Vec<MaterialID>,
}
impl SingleDefaultMaterialBindDynInfoSet {
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
