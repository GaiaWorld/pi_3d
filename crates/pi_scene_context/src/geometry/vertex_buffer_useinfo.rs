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
pub struct EVerteicesInstance {
    pub data: Range<usize>,
    pub itemcount: u32,
    pub slot: u8,
}
impl EVerteicesInstance {
    pub fn reset(&mut self) {
        self.data.start = 0;
        self.data.end = 0;
        self.itemcount = 0;
        self.slot = 0;
    }
}
impl Default for EVerteicesInstance {
    fn default() -> Self {
        Self {
            data: Range { start: 0, end: 0 },
            itemcount: 0,
            slot: 0,
        }
    }
}
impl Drop for EVerteicesInstance {
    fn drop(&mut self) {
        // self.data.clear();
        // log::error!("EVerteicesMemory Drop");
    }
}

///
/// 顶点Buffer数据
/// * 每物体的实例数据,内存数据,保存在物体上
/// * 每顶点的数据,为Buffer的引用
#[derive(Clone)]
pub enum EVerticesBufferTmp {
    Instance(u32),
    Buffer(EVerticesBufferUsage),
}

/// 在 Mesh 实体上记录其使用的 Geometry
#[derive(Component, Default)]
pub struct GeometryID(pub ObjectID);
impl TEntityRef for GeometryID {
    fn id(&self) -> Entity {
        self.0
    }
}

/// 在 Geometry 实体上记录使用其的 Mesh
#[derive(Component, Default)]
pub struct MeshID(pub ObjectID);

/// 在 Geometry 实体上记录其已加载成功的 VertexBuffer资源的Key
/// Idx 对应VB的 Slot
#[derive(Deref, DerefMut, Clone, Hash, Component, Default)]
// pub struct LoadedKeyVBSlots(pub SmallVec<[Option<KeyVertexBuffer>;VB_SLOTS_COUNT]>);
pub struct LoadedKeyVBSlots(pub [Option<KeyVertexBuffer>;VB_SLOTS_COUNT]);

/// 在 Geometry 实体上记录其需要加载的 VertexBuffer资源的描述
#[derive(Deref, DerefMut, Component, Default)]
// pub struct AssetDescVBSlots(pub SmallVec<[Option<AssetDescVBSlot>;VB_SLOTS_COUNT]>);
pub struct AssetDescVBSlots(pub [Option<AssetDescVBSlot>;VB_SLOTS_COUNT]);
impl AssetDescVBSlots {
    pub fn key(&self, slot: usize) -> Option<KeyVertexBuffer> {
        match self.get(slot) {
            Some(Some(desc)) => Some(desc.key()),
            _ => None,
        }
    }
}

/// 在 Geometry 实体上记录其已加载的 VertexBuffer资源的引用信息
#[derive(Deref, DerefMut, Component, Default)]
// pub struct AssetResVBSlots(pub SmallVec<[Option<AssetResVBSlot>;VB_SLOTS_COUNT]>);
pub struct AssetResVBSlots(pub [Option<AssetResVBSlot>;VB_SLOTS_COUNT]);

/// VertexBuffer资源的描述
#[derive(Default, Debug)]
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

///  VertexBuffer资源的引用信息
#[derive(Deref, DerefMut)]
pub struct AssetResVBSlot(pub EVerticesBufferTmp);
impl From<EVerticesBufferUsage> for AssetResVBSlot {
    fn from(value: EVerticesBufferUsage) -> Self { Self(EVerticesBufferTmp::Buffer(value)) }
}
