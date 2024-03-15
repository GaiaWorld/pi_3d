use bevy_ecs::{component::Component, system::Resource};
use pi_scene_shell::prelude::*;

use super::vertex_buffer_useinfo::*;

#[derive(Component)]
pub struct GeometryDesc {
    pub list: Vec<VertexBufferDesc>,
}
impl GeometryDesc {
    pub fn slot_count(&self) -> usize {
        self.list.len()
    }
    pub fn get_desc(&self, slot: usize) -> VertexBufferDesc {
        self.list.get(slot).unwrap().clone()
    }
}

#[derive(Resource)]
pub struct GeometryVBLoader {
    pub loader_01: VertexBufferLoader<ObjectID, AssetResVBSlot01>,
    pub loader_02: VertexBufferLoader<ObjectID, AssetResVBSlot02>,
    pub loader_03: VertexBufferLoader<ObjectID, AssetResVBSlot03>,
    pub loader_04: VertexBufferLoader<ObjectID, AssetResVBSlot04>,
    pub loader_05: VertexBufferLoader<ObjectID, AssetResVBSlot05>,
    pub loader_06: VertexBufferLoader<ObjectID, AssetResVBSlot06>,
    pub loader_07: VertexBufferLoader<ObjectID, AssetResVBSlot07>,
    pub loader_08: VertexBufferLoader<ObjectID, AssetResVBSlot08>,
    pub loader_09: VertexBufferLoader<ObjectID, AssetResVBSlot09>,
    pub loader_10: VertexBufferLoader<ObjectID, AssetResVBSlot10>,
    pub loader_11: VertexBufferLoader<ObjectID, AssetResVBSlot11>,
    pub loader_12: VertexBufferLoader<ObjectID, AssetResVBSlot12>,
    pub loader_13: VertexBufferLoader<ObjectID, AssetResVBSlot13>,
    pub loader_14: VertexBufferLoader<ObjectID, AssetResVBSlot14>,
    pub loader_15: VertexBufferLoader<ObjectID, AssetResVBSlot15>,
    pub loader_16: VertexBufferLoader<ObjectID, AssetResVBSlot16>,
    pub loader_indices: VertexBufferLoader<ObjectID, AssetResBufferIndices>,
}
impl Default for GeometryVBLoader {
    fn default() -> Self {
        Self {
            loader_01: VertexBufferLoader::<ObjectID, AssetResVBSlot01>::default(),
            loader_02: VertexBufferLoader::<ObjectID, AssetResVBSlot02>::default(),
            loader_03: VertexBufferLoader::<ObjectID, AssetResVBSlot03>::default(),
            loader_04: VertexBufferLoader::<ObjectID, AssetResVBSlot04>::default(),
            loader_05: VertexBufferLoader::<ObjectID, AssetResVBSlot05>::default(),
            loader_06: VertexBufferLoader::<ObjectID, AssetResVBSlot06>::default(),
            loader_07: VertexBufferLoader::<ObjectID, AssetResVBSlot07>::default(),
            loader_08: VertexBufferLoader::<ObjectID, AssetResVBSlot08>::default(),
            loader_09: VertexBufferLoader::<ObjectID, AssetResVBSlot09>::default(),
            loader_10: VertexBufferLoader::<ObjectID, AssetResVBSlot10>::default(),
            loader_11: VertexBufferLoader::<ObjectID, AssetResVBSlot11>::default(),
            loader_12: VertexBufferLoader::<ObjectID, AssetResVBSlot12>::default(),
            loader_13: VertexBufferLoader::<ObjectID, AssetResVBSlot13>::default(),
            loader_14: VertexBufferLoader::<ObjectID, AssetResVBSlot14>::default(),
            loader_15: VertexBufferLoader::<ObjectID, AssetResVBSlot15>::default(),
            loader_16: VertexBufferLoader::<ObjectID, AssetResVBSlot16>::default(),
            loader_indices: VertexBufferLoader::<ObjectID, AssetResBufferIndices>::default(),
        }
    }
}

