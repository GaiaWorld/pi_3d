use pi_ecs::{prelude::{ResMut, Query, Res}, query::{With, Changed, Or, Write}};
use pi_ecs_macros::setup;
use pi_engine_shell::object::GameObject;
use pi_render::rhi::{dyn_uniform_buffer::{BindOffset, Bind, Uniform}, device::RenderDevice, RenderQueue};
use pi_scene_math::Matrix;
use render_data_container::{VertexBufferPool, VertexBuffer};
use render_geometry::vertex_data::EVertexDataKind;

use crate::{shaders::FragmentUniformBind, resources::RenderDynUniformBuffer, bytes_write_to_memory, transforms::transform_node::{WorldMatrix, WorldMatrixInv}, geometry::instance::types::{TInstancedData, InstancedValue}};

use super::{instance::{instanced_mesh::{InstanceSource, InstanceList}, world_matrix::{InstancedWorldMatrixDirty}}, abstract_mesh::AbstructMesh};

/// 对应 EModelAboutBind::ModelMatrix
/// 必须,固定使用,bind固定
pub struct BuildinModelBind {
    pub bind_offset: BindOffset,
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
        }
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

#[derive(Debug, Clone)]
pub struct RenderWorldMatrix(pub Matrix);
impl RenderWorldMatrix {
    pub fn new(m: Matrix) -> Self {
        Self(m)
    }
}
impl TInstancedData for RenderWorldMatrix {
    fn vertex_kind(&self) -> EVertexDataKind {
        EVertexDataKind::InsWorldRow1
    }

    fn value(&self) -> &InstancedValue {
        todo!()
    }

    fn size() -> usize {
        16
    }

    fn bytes_size() -> usize {
        16 * 4
    }

    fn local_offset(&self) -> usize {
        0
    }

    fn write_instance_buffer(&self, buffer: &mut VertexBuffer, offset: usize) {
        buffer.update_f32(self.0.as_slice(), offset);
    }
}

#[derive(Debug, Clone)]
pub struct RenderWorldMatrixInv(pub Matrix);
impl RenderWorldMatrixInv {
    pub fn new(m: Matrix) -> Self {
        Self(m)
    }
}


pub struct BuildinModelTemp<'a>(pub &'a Matrix, pub &'a Matrix);
impl<'a> Uniform for BuildinModelTemp<'a> {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        bytes_write_to_memory(bytemuck::cast_slice(self.0.as_slice()), index as usize + BuildinModelBind::OBJECT_TO_WORLD_OFFSIZE, buffer);
        bytes_write_to_memory(bytemuck::cast_slice(self.1.as_slice()), index as usize + BuildinModelBind::WORLD_TO_OBJECT_OFFSIZE, buffer);
    }
}


pub struct SysModelMatrixUpdate;
#[setup]
impl SysModelMatrixUpdate {
    #[system]
    pub fn tick(
        mut meshes: Query<GameObject, (&AbstructMesh, Write<RenderWorldMatrix>, Write<RenderWorldMatrixInv>, &WorldMatrix, &WorldMatrixInv), Or<(Changed<WorldMatrix>, Changed<WorldMatrixInv>)>>,
        mut dynbuffer: ResMut<RenderDynUniformBuffer>,
    ) {
        meshes.iter_mut().for_each(|(_, mut render_worldmatrix, mut render_worldmatrix_inv, worldmatrix, worldmatrix_inv)| {
            // println!("SysModelUniformUpdate:");
            render_worldmatrix.write(RenderWorldMatrix::new(worldmatrix.0.clone()));
            render_worldmatrix_inv.write(RenderWorldMatrixInv::new(worldmatrix_inv.0.clone()));
        });
    }
}

pub struct SysModelUniformUpdate;
#[setup]
impl SysModelUniformUpdate {
    #[system]
    pub fn tick(
        mut meshes: Query<GameObject, (&BuildinModelBind, &RenderWorldMatrix, &RenderWorldMatrixInv), Or<(Changed<RenderWorldMatrix>, Changed<RenderWorldMatrixInv>)>>,
        mut dynbuffer: ResMut<RenderDynUniformBuffer>,
    ) {
        meshes.iter_mut().for_each(|(model, worldmatrix, worldmatrix_inv)| {
            // println!("SysModelUniformUpdate:");

            let temp = BuildinModelTemp(&worldmatrix.0, &worldmatrix_inv.0);
            dynbuffer.as_mut().set_uniform::<BuildinModelTemp>(&model.bind_offset, &temp);
        });
    }
}

pub struct SysInstancedModelUpdate;
#[setup]
impl SysInstancedModelUpdate {
    #[system]
    pub fn tick(
        instances: Query<GameObject, &InstanceSource, Changed<RenderWorldMatrix>>,
        mut sources: Query<GameObject, Write<InstancedWorldMatrixDirty>>,
    ) {
        instances.iter().for_each(|source| {
            if let Some(mut source) = sources.get_mut(source.0.clone()) {
                source.write(InstancedWorldMatrixDirty);
            }
        });
    }
}
