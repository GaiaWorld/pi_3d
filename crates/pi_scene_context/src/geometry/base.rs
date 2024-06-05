use pi_scene_shell::prelude::*;

use super::vertex_buffer_useinfo::*;

#[derive(Component, Default)]
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
impl HashAsResource for GeometryDesc {
    fn hash_resource<H: std::hash::Hasher>(&self, state: &mut H) {
        self.list.iter().for_each(|item| {
            item.hash_resource(state);
        });
    }
}

#[derive(Component, Default, Clone, Copy, PartialEq, Eq)]
pub struct GeometryResourceHash(pub u64);

#[derive(Component, Default, PartialEq, Eq)]
pub struct GeometryLayoutHash(pub u64);

#[derive(Resource)]
pub struct GeometryVBLoader {
    pub loader_01: VertexBufferLoader<(ObjectID, u8), AssetResVBSlot>,
    pub loader_indices: VertexBufferLoader<ObjectID, AssetResBufferIndices>,
}
impl Default for GeometryVBLoader {
    fn default() -> Self {
        Self {
            loader_01: VertexBufferLoader::<(ObjectID, u8), AssetResVBSlot>::default(),
            loader_indices: VertexBufferLoader::<ObjectID, AssetResBufferIndices>::default(),
        }
    }
}

