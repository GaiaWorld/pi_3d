use std::{time::Instant, sync::Arc};

use pi_assets::{asset::Handle, mgr::AssetMgr};
use pi_ecs::{prelude::{Query, Commands}, query::{Changed, Or}};
use pi_ecs_macros::setup;
use pi_engine_shell::{object::{GameObject, ObjectID}, run_stage::TSystemStageInfo};
use pi_render::{rhi::{device::RenderDevice, RenderQueue}, renderer::{bind_buffer::{BindBufferAllocator}, attributes::EVertexDataKind, vertex_buffer::{KeyVertexBuffer, VertexBufferAllocator, EVertexBufferRange}, instance::types::{TInstanceFlag, TInstancedData}}, render_3d::binds::model::base::ShaderBindModelAboutMatrix};
use pi_scene_math::Matrix;
use pi_share::Share;

use crate::{
    transforms::{transform_node::{WorldMatrix, WorldMatrixInv}, transform_node_sys::SysWorldMatrixCalc},
    geometry::instance::{InstanceSource, instance_world_matrix::InstancedWorldMatrixDirty},
};

use super::{abstract_mesh::AbstructMesh};

pub struct BindModel(pub Arc<ShaderBindModelAboutMatrix>);
impl BindModel {
    pub fn new(
        device: &RenderDevice,
        allocator: &mut BindBufferAllocator,
    ) -> Option<Self> {

        if let Some(bind) = ShaderBindModelAboutMatrix::new(allocator) {
            Some(Self(Arc::new(bind)))
        } else {
            None
        }
    }
}

pub struct RenderMatrixDirty(pub bool);
impl TInstanceFlag for RenderMatrixDirty {
    fn dirty(&self) -> bool {
        self.0
    }

    fn reset(&mut self) {
        self.0 = false
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

#[derive(Debug, Clone)]
pub struct RenderWorldMatrixInv(pub Matrix);
impl RenderWorldMatrixInv {
    pub fn new(m: Matrix) -> Self {
        Self(m)
    }
}

pub struct SysRenderMatrixUpdate;
impl TSystemStageInfo for SysRenderMatrixUpdate {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysWorldMatrixCalc::key()
        ]
    }
}
#[setup]
impl SysRenderMatrixUpdate {
    #[system]
    pub fn tick(
        mut source_mesh: Query<GameObject, &mut InstancedWorldMatrixDirty>,
        mut meshes: Query<
            GameObject,
            (ObjectID, &AbstructMesh, &WorldMatrix, &WorldMatrixInv, &mut RenderWorldMatrix, &mut RenderWorldMatrixInv, &RenderMatrixDirty, Option<&InstanceSource>),
            Or<(Changed<WorldMatrix>, Changed<WorldMatrixInv>)>
        >,
        mut wm_cmd: Commands<GameObject, RenderMatrixDirty>,
        mut ins_cmd: Commands<GameObject, InstancedWorldMatrixDirty>,
    ) {
        let time = Instant::now();

        meshes.iter_mut().for_each(|(
            obj, _,
            worldmatrix, worldmatrix_inv, mut render_wm, mut render_wminv,
            mut dirty, id_source
        )| {
            // log::debug!("SysModelUniformUpdate:");
            render_wm.0.clone_from(&worldmatrix.0);
            render_wminv.0.clone_from(&worldmatrix_inv.0);
            wm_cmd.insert(obj, RenderMatrixDirty(true));

            if let Some(id_source) = id_source {
                ins_cmd.insert(id_source.0, InstancedWorldMatrixDirty(true));
                // if let Some(mut flag) = source_mesh.get_mut(id_source.0) {
                //     flag.0 = true;
                // }
            }
        });
        
        let time1 = Instant::now();
        log::info!("SysRenderMatrixUpdate: {:?}", time1 - time);
    }
}

pub struct SysRenderMatrixUniformUpdate;
impl TSystemStageInfo for SysRenderMatrixUniformUpdate {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysRenderMatrixUpdate::key()
        ]
    }
}
#[setup]
impl SysRenderMatrixUniformUpdate {
    #[system]
    pub fn tick(
        mut meshes: Query<GameObject, (&RenderWorldMatrix, &RenderWorldMatrixInv, &mut RenderMatrixDirty, &BindModel), Changed<RenderMatrixDirty>>,
    ) {
        meshes.iter_mut().for_each(|(worldmatrix, worldmatrix_inv, mut flag, bind_model)| {
            // log::info!("SysModelUniformUpdate:");

            bind_model.0.data().write_data(ShaderBindModelAboutMatrix::OFFSET_WORLD_MATRIX as usize, bytemuck::cast_slice(worldmatrix.0.as_slice()));
            bind_model.0.data().write_data(ShaderBindModelAboutMatrix::OFFSET_WORLD_MATRIX_INV as usize, bytemuck::cast_slice(worldmatrix_inv.0.as_slice()));
            flag.0 = false;
        });
    }
}