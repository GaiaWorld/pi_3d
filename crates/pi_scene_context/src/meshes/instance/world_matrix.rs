use std::{ops::Range, sync::Arc};

use pi_ecs::{prelude::{Query, ResMut, Res}, query::{Or, Changed}};
use pi_ecs_macros::setup;
use pi_engine_shell::object::{ObjectID, GameObject};
use pi_render::rhi::{device::RenderDevice, RenderQueue};
use pi_slotmap::DefaultKey;
use render_data_container::{VertexBuffer, EVertexDataFormat, VertexBufferPool, KeyVertexBuffer};
use render_geometry::vertex_data::{VertexBufferDesc, VertexAttribute, EVertexDataKind};

use crate::{geometry::{vertex_buffer_useinfo}, transforms::transform_node::WorldMatrix};

use super::{instanced_buffer::TInstancedBuffer, instanced_mesh::InstanceList};

pub struct InstancedWorldMatrixDirty;

pub struct InstancedBufferWorldMatrix {
    pub slot: vertex_buffer_useinfo::EVertexBufferSlot,
    key: render_data_container::KeyVertexBuffer,
}
impl InstancedBufferWorldMatrix {
    pub fn new(index: usize, id: String, pool: &mut VertexBufferPool) -> Self {
        let buffer: VertexBuffer = VertexBuffer::new(true, EVertexDataFormat::F32, false);
        let key = render_data_container::KeyVertexBuffer::from(id + "WorldMat");
        pool.map.insert(key.clone(), buffer);

        Self {
            slot: vertex_buffer_useinfo::EVertexBufferSlot::from_u8_unsafe(index as u8),
            key
        }
    }
}
impl TInstancedBuffer for InstancedBufferWorldMatrix {
    fn key(&self) -> render_data_container::KeyVertexBuffer {
        self.key.clone()
    }

    fn slot(&self) -> crate::geometry::vertex_buffer_useinfo::EVertexBufferSlot {
        self.slot
    }
}

pub struct SysInstancedWorldMatrixUpdate;
#[setup]
impl SysInstancedWorldMatrixUpdate {
    #[system]
    pub fn tick(
        instances: Query<GameObject, &WorldMatrix>,
        mut sources: Query<GameObject, (&InstanceList, &mut InstancedBufferWorldMatrix), Or<(Changed<InstanceList>, Changed<InstancedWorldMatrixDirty>)>>,
        mut vbpool: ResMut<VertexBufferPool>,
        device: Res<RenderDevice>,
        queue: Res<RenderQueue>,
    ) {
        sources.iter_mut().for_each(|(inslist, buffer)| {
            let mut list = vec![];
            inslist.list.iter().for_each(|insid| {
                if let Some(instance) = instances.get(insid.clone()) {
                    list.push(instance);
                }
            });

            buffer.update::<WorldMatrix>(list.as_slice(), &mut vbpool, &device, &queue);
        });
    }
}