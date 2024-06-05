use std::ops::Range;

use derive_deref::{Deref, DerefMut};
use pi_scene_shell::prelude::*;

pub const VB_SLOTS_COUNT: usize = 8;

pub trait AsKeyVertexBuffer {
    fn create(desc: &VertexBufferDesc) -> Self;
}
pub trait TVertexBufferUseInfo: From<VertexBufferDesc> {
    const ASK_SLOT_COUNT: u8;
    fn desc(&self) -> &VertexBufferDesc;
    fn range(&self) -> Option<Range<wgpu::BufferAddress>> {
        self.desc().range().clone()
    }
    fn key(&self) -> &KeyVertexBuffer {
        self.desc().bufferkey()
    }
    fn slot() -> u32 { Self::ASK_SLOT_COUNT as u32 - 1 }
}

pub trait TAssetResVertexBuffer {
    fn buffer(&self) -> EVerticesBufferUsage;
}

#[derive(Clone)]
pub struct EVerteicesMemory {
    pub data: Vec<u8>,
    pub itemcount: u32,
    pub slot: u32,
}

#[derive(Clone)]
pub enum EVerticesBufferTmp {
    Memory(EVerteicesMemory),
    Buffer(EVerticesBufferUsage),
}

#[derive(Component)]
pub struct GeometryID(pub Entity);
impl Default for GeometryID {
    fn default() -> Self {
        Self(Entity::from_bits(0))
    }
}
impl TEntityRef for GeometryID {
    fn id(&self) -> Entity {
        self.0
    }
}

#[derive(Clone, Component, Default)]
pub struct DirtyGeometryRef;

pub type GeometryRefs = EntityRefInfo<DirtyGeometryRef>;

#[derive(Component)]
pub struct MeshID(pub Entity);
impl Default for MeshID {
    fn default() -> Self {
        Self(Entity::from_bits(0))
    }
}

#[derive(Deref, DerefMut, Clone, Hash, Component, Default)]
pub struct LoadedKeyVBSlots(pub [Option<KeyVertexBuffer>;VB_SLOTS_COUNT]);
#[derive(Deref, DerefMut, Component, Default)]
pub struct AssetDescVBSlots(pub [Option<AssetDescVBSlot>;VB_SLOTS_COUNT]);
impl AssetDescVBSlots {
    pub fn key(&self, slot: usize) -> Option<KeyVertexBuffer> {
        match self.get(slot).unwrap() {
            Some(desc) => Some(desc.key()),
            None => None,
        }
    }
}
#[derive(Deref, DerefMut, Component, Default)]
pub struct AssetResVBSlots(pub [Option<AssetResVBSlot>;VB_SLOTS_COUNT]);

#[derive(Deref, DerefMut, Clone, Hash, Component, Default)]
pub struct AssetKeyVBSlot(pub KeyVertexBuffer);
impl AsKeyVertexBuffer for AssetKeyVBSlot {
    fn create(desc: &VertexBufferDesc) -> Self { Self(desc.bufferkey().clone()) }
}
#[derive(Component, Default)]
pub struct AssetDescVBSlot(pub(crate) VertexBufferDesc);
impl From<VertexBufferDesc> for AssetDescVBSlot {
    fn from(value: VertexBufferDesc) -> Self { Self(value) }
}
impl TVertexBufferUseInfo for AssetDescVBSlot {
    const ASK_SLOT_COUNT: u8 = 1;
    fn desc(&self) -> &VertexBufferDesc { &self.0 }
}
impl AssetDescVBSlot {
    pub fn key(&self) -> KeyVertexBuffer {
        self.0.bufferkey().clone()
    }
}

#[derive(Deref, DerefMut, Component)]
pub struct AssetResVBSlot(pub EVerticesBufferTmp);
impl From<EVerticesBufferUsage> for AssetResVBSlot {
    fn from(value: EVerticesBufferUsage) -> Self { Self(EVerticesBufferTmp::Buffer(value)) }
}
impl Default for AssetResVBSlot {
    fn default() -> Self {
        Self( EVerticesBufferTmp::Memory(EVerteicesMemory { data: vec![], itemcount: 0, slot: 0 }) )
    }
}
