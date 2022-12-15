use pi_ecs::{prelude::{ResMut, Query}, query::{With, Changed, Or}};
use pi_ecs_macros::setup;
use pi_engine_shell::object::GameObject;
use pi_render::rhi::dyn_uniform_buffer::{BindOffset, Bind, Uniform};
use pi_scene_math::Matrix;

use crate::{shaders::FragmentUniformBind, resources::RenderDynUniformBuffer, bytes_write_to_memory, transforms::transform_node::{WorldMatrix, WorldMatrixInv}};

pub struct BuildinModelTemp<'a>(pub &'a Matrix, pub &'a Matrix);
impl<'a> Uniform for BuildinModelTemp<'a> {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        bytes_write_to_memory(bytemuck::cast_slice(self.0.as_slice()), index as usize + BuildinModelBind::OBJECT_TO_WORLD_OFFSIZE, buffer);
        bytes_write_to_memory(bytemuck::cast_slice(self.1.as_slice()), index as usize + BuildinModelBind::WORLD_TO_OBJECT_OFFSIZE, buffer);
    }
}

/// Model Uniform Bind
pub struct BuildinModelBind {
    pub bind_offset: BindOffset,
    pub matrix: Matrix,
    pub matrix_inv: Matrix,
    pub is_dirty: bool,
}
impl BuildinModelBind {
    pub const OBJECT_TO_WORLD: usize = 16;
    pub const WORLD_TO_OBJECT: usize = 16;

    pub const OBJECT_TO_WORLD_OFFSIZE: usize = 0 * 4;
    pub const WORLD_TO_OBJECT_OFFSIZE: usize = Self::OBJECT_TO_WORLD_OFFSIZE + Self::WORLD_TO_OBJECT * 4;

    pub fn new(
        dynbuffer: &mut RenderDynUniformBuffer,
    ) -> Self {
        Self {
            bind_offset: dynbuffer.alloc_binding::<Self>(),
            matrix: Matrix::identity(),
            matrix_inv: Matrix::identity(),
            is_dirty: true,
        }
    }

    pub fn update(&mut self, world: &Matrix, world_inv: &Matrix) {
        self.matrix.clone_from(world);
        self.matrix_inv.clone_from(world_inv);
        self.is_dirty = true;
    }
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

pub struct SysModelUniformUpdate;
#[setup]
impl SysModelUniformUpdate {
    #[system]
    pub fn tick(
        mut meshes: Query<GameObject, (&BuildinModelBind, &WorldMatrix, &WorldMatrixInv), Or<(Changed<WorldMatrix>, Changed<WorldMatrixInv>)>>,
        mut dynbuffer: ResMut<RenderDynUniformBuffer>,
    ) {
        meshes.iter_mut().for_each(|(model, worldmatrix, worldmatrix_inv)| {
            // println!("SysModelUniformUpdate:");

            let temp = BuildinModelTemp(&worldmatrix.0, &worldmatrix_inv.0);
            dynbuffer.as_mut().set_uniform::<BuildinModelTemp>(&model.bind_offset, &temp);
        });
    }
}