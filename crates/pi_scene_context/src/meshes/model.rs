use std::{sync::Arc};

use pi_engine_shell::prelude::*;
use pi_scene_math::Matrix;
use pi_share::Share;

use crate::{
    transforms::{transform_node::{WorldMatrix, WorldMatrixInv}},
    geometry::instance::{InstanceSourceID, instance_world_matrix::InstanceWorldMatrixDirty, InstanceSourceRefs},
};

use super::{abstract_mesh::AbstructMesh};

#[derive(Component)]
pub struct BindModel(pub Arc<ShaderBindModelAboutMatrix>);
impl BindModel {
    pub fn new(
        device: &PiRenderDevice,
        allocator: &mut BindBufferAllocator,
    ) -> Option<Self> {

        if let Some(bind) = ShaderBindModelAboutMatrix::new(allocator) {
            Some(Self(Arc::new(bind)))
        } else {
            None
        }
    }
}

#[derive(Component)]
pub struct RenderMatrixDirty(pub bool);
impl TInstanceFlag for RenderMatrixDirty {
    fn dirty(&self) -> bool {
        self.0
    }

    fn reset(&mut self) {
        self.0 = false
    }
}

#[derive(Debug, Clone, Component)]
pub struct RenderWorldMatrix(pub Matrix);
impl RenderWorldMatrix {
    pub fn new(m: Matrix) -> Self {
        Self(m)
    }
}
impl TInstanceData for RenderWorldMatrix {
    fn vertex_kind(&self) -> EVertexDataKind {
        EVertexDataKind::InsWorldRow1
    }

    fn collect(list: &Vec<&Self>) -> Vec<u8> {
        let mut result = vec![];

        list.iter().for_each(|v| {
            bytemuck::cast_slice(v.0.as_slice()).iter().for_each(|v| {
                result.push(*v);
            })
        });

        result
    }

    // fn size() -> usize {
    //     16
    // }

    // fn bytes_size() -> usize {
    //     16 * 4
    // }

    // fn local_offset(&self) -> usize {
    //     0
    // }
}

#[derive(Debug, Clone, Component)]
pub struct RenderWorldMatrixInv(pub Matrix);
impl RenderWorldMatrixInv {
    pub fn new(m: Matrix) -> Self {
        Self(m)
    }
}

    pub fn sys_calc_render_matrix(
        mut meshes: Query<
            (ObjectID, &AbstructMesh, &WorldMatrix, &WorldMatrixInv, Option<&InstanceSourceID>),
            Or<(Changed<WorldMatrix>, Changed<WorldMatrixInv>)>
        >,
        mut commands: Commands,
    ) {
        let time = pi_time::Instant::now();

        meshes.iter_mut().for_each(|(
            obj, _,
            worldmatrix, worldmatrix_inv, id_source
        )| {
            // log::warn!("calc_render_matrix:");
            // render_wm.0.clone_from(&worldmatrix.0);
            // render_wminv.0.clone_from(&worldmatrix_inv.0);
            commands.entity(obj)
                .insert(RenderWorldMatrix(worldmatrix.0.clone()))
                .insert(RenderWorldMatrixInv(worldmatrix_inv.0.clone()))
                .insert(RenderMatrixDirty(true));

            if let Some(id_source) = id_source {
                commands.entity(id_source.0).insert(InstanceWorldMatrixDirty(true));
                // if let Some(mut flag) = source_mesh.get_mut(id_source.0) {
                //     flag.0 = true;
                // }
            }
        });
        
        let time1 = pi_time::Instant::now();
        log::debug!("SysRenderMatrixUpdate: {:?}", time1 - time);
    }

    pub fn sys_render_matrix_for_uniform(
        mut meshes: Query<(&RenderWorldMatrix, &RenderWorldMatrixInv, &mut RenderMatrixDirty, &BindModel), Changed<RenderMatrixDirty>>,
    ) {
        meshes.iter_mut().for_each(|(worldmatrix, worldmatrix_inv, mut flag, bind_model)| {
            // log::debug!("SysModelUniformUpdate:");

            bind_model.0.data().write_data(ShaderBindModelAboutMatrix::OFFSET_WORLD_MATRIX as usize, bytemuck::cast_slice(worldmatrix.0.as_slice()));
            bind_model.0.data().write_data(ShaderBindModelAboutMatrix::OFFSET_WORLD_MATRIX_INV as usize, bytemuck::cast_slice(worldmatrix_inv.0.as_slice()));
            flag.0 = false;
        });
    }
