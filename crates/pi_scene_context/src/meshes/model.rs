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
pub struct Mesh;

#[derive(Component)]
pub struct MeshID(pub ObjectID);
impl TEntityRef for MeshID {
    fn id(&self) -> Entity {
        self.0
    }
}

#[derive(Debug, Clone, Default, Component)]
pub struct DirtyMeshRef;

pub type MeshRefs = EntityRefInfo<DirtyMeshRef, MeshID>;


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

