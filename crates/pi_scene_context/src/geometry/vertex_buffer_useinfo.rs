use std::ops::Range;

use pi_engine_shell::prelude::*;
use pi_render::renderer::{vertex_buffer::{EVertexBufferRange, KeyVertexBuffer}, vertex_buffer_desc::VertexBufferDesc, vertices::EVerticesBufferUsage};

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

#[derive(Component)]
pub struct GeometryID(pub ObjectID);
impl TEntityRef for GeometryID {
    fn id(&self) -> Entity {
        self.0
    }
}

#[derive(Debug, Clone, Default, Component)]
pub struct DirtyGeometryRef;

pub type GeometryRefs = EntityRefInfo<DirtyGeometryRef, GeometryID>;

#[derive(Component)]
pub struct MeshID(pub ObjectID);

// ============== 1
#[derive(Debug, Deref, DerefMut, Clone, Hash, Component)]
pub struct AssetKeyVBSlot01(pub KeyVertexBuffer);
impl AsKeyVertexBuffer for AssetKeyVBSlot01 {
    fn create(desc: &VertexBufferDesc) -> Self { Self(desc.bufferkey().clone()) }
}
#[derive(Component)]
pub struct AssetDescVBSlot01(pub(crate) VertexBufferDesc);
impl From<VertexBufferDesc> for AssetDescVBSlot01 {
    fn from(value: VertexBufferDesc) -> Self { Self(value) }
}
impl TVertexBufferUseInfo for AssetDescVBSlot01 {
    const ASK_SLOT_COUNT: u8 = 1;
    fn desc(&self) -> &VertexBufferDesc { &self.0 }
}

#[derive(Debug, Deref, DerefMut, Component)]
pub struct AssetResVBSlot01(pub EVerticesBufferUsage);
impl From<EVerticesBufferUsage> for AssetResVBSlot01 {
    fn from(value: EVerticesBufferUsage) -> Self { Self(value) }
}
impl TAssetResVertexBuffer for AssetResVBSlot01 {
    fn buffer(&self) -> EVerticesBufferUsage { self.0.clone() }
}

// ============== 2
#[derive(Debug, Deref, DerefMut, Clone, Hash, Component)]
pub struct AssetKeyVBSlot02(pub KeyVertexBuffer);
impl AsKeyVertexBuffer for AssetKeyVBSlot02 {
    fn create(desc: &VertexBufferDesc) -> Self { Self(desc.bufferkey().clone()) }
}
#[derive(Component)]
pub struct AssetDescVBSlot02(pub(crate) VertexBufferDesc);
impl From<VertexBufferDesc> for AssetDescVBSlot02 {
    fn from(value: VertexBufferDesc) -> Self { Self(value) }
}
impl TVertexBufferUseInfo for AssetDescVBSlot02 {
    const ASK_SLOT_COUNT: u8 = 2;
    fn desc(&self) -> &VertexBufferDesc { &self.0 }
}

#[derive(Debug, Deref, DerefMut, Component)]
pub struct AssetResVBSlot02(pub EVerticesBufferUsage);
impl From<EVerticesBufferUsage> for AssetResVBSlot02 {
    fn from(value: EVerticesBufferUsage) -> Self { Self(value) }
}
impl TAssetResVertexBuffer for AssetResVBSlot02 {
    fn buffer(&self) -> EVerticesBufferUsage { self.0.clone() }
}

// ============== 3
#[derive(Debug, Deref, DerefMut, Clone, Hash, Component)]
pub struct AssetKeyVBSlot03(pub KeyVertexBuffer);
impl AsKeyVertexBuffer for AssetKeyVBSlot03 {
    fn create(desc: &VertexBufferDesc) -> Self { Self(desc.bufferkey().clone()) }
}
#[derive(Component)]
pub struct AssetDescVBSlot03(pub(crate) VertexBufferDesc);
impl From<VertexBufferDesc> for AssetDescVBSlot03 {
    fn from(value: VertexBufferDesc) -> Self { Self(value) }
}
impl TVertexBufferUseInfo for AssetDescVBSlot03 {
    const ASK_SLOT_COUNT: u8 = 3;
    fn desc(&self) -> &VertexBufferDesc { &self.0 }
}

#[derive(Debug, Deref, DerefMut, Component)]
pub struct AssetResVBSlot03(pub EVerticesBufferUsage);
impl From<EVerticesBufferUsage> for AssetResVBSlot03 {
    fn from(value: EVerticesBufferUsage) -> Self { Self(value) }
}
impl TAssetResVertexBuffer for AssetResVBSlot03 {
    fn buffer(&self) -> EVerticesBufferUsage { self.0.clone() }
}

// ============== 4
#[derive(Debug, Deref, DerefMut, Clone, Hash, Component)]
pub struct AssetKeyVBSlot04(pub KeyVertexBuffer);
impl AsKeyVertexBuffer for AssetKeyVBSlot04 {
    fn create(desc: &VertexBufferDesc) -> Self { Self(desc.bufferkey().clone()) }
}
#[derive(Component)]
pub struct AssetDescVBSlot04(pub(crate) VertexBufferDesc);
impl From<VertexBufferDesc> for AssetDescVBSlot04 {
    fn from(value: VertexBufferDesc) -> Self { Self(value) }
}
impl TVertexBufferUseInfo for AssetDescVBSlot04 {
    const ASK_SLOT_COUNT: u8 = 4;
    fn desc(&self) -> &VertexBufferDesc { &self.0 }
}

#[derive(Debug, Deref, DerefMut, Component)]
pub struct AssetResVBSlot04(pub EVerticesBufferUsage);
impl From<EVerticesBufferUsage> for AssetResVBSlot04 {
    fn from(value: EVerticesBufferUsage) -> Self { Self(value) }
}
impl TAssetResVertexBuffer for AssetResVBSlot04 {
    fn buffer(&self) -> EVerticesBufferUsage { self.0.clone() }
}

// ============== 5
#[derive(Debug, Deref, DerefMut, Clone, Hash, Component)]
pub struct AssetKeyVBSlot05(pub KeyVertexBuffer);
impl AsKeyVertexBuffer for AssetKeyVBSlot05 {
    fn create(desc: &VertexBufferDesc) -> Self { Self(desc.bufferkey().clone()) }
}
#[derive(Component)]
pub struct AssetDescVBSlot05(pub(crate) VertexBufferDesc);
impl From<VertexBufferDesc> for AssetDescVBSlot05 {
    fn from(value: VertexBufferDesc) -> Self { Self(value) }
}
impl TVertexBufferUseInfo for AssetDescVBSlot05 {
    const ASK_SLOT_COUNT: u8 = 5;
    fn desc(&self) -> &VertexBufferDesc { &self.0 }
}

#[derive(Debug, Deref, DerefMut, Component)]
pub struct AssetResVBSlot05(pub EVerticesBufferUsage);
impl From<EVerticesBufferUsage> for AssetResVBSlot05 {
    fn from(value: EVerticesBufferUsage) -> Self { Self(value) }
}
impl TAssetResVertexBuffer for AssetResVBSlot05 {
    fn buffer(&self) -> EVerticesBufferUsage { self.0.clone() }
}

// ============== 6
#[derive(Debug, Deref, DerefMut, Clone, Hash, Component)]
pub struct AssetKeyVBSlot06(pub KeyVertexBuffer);
impl AsKeyVertexBuffer for AssetKeyVBSlot06 {
    fn create(desc: &VertexBufferDesc) -> Self { Self(desc.bufferkey().clone()) }
}
#[derive(Component)]
pub struct AssetDescVBSlot06(pub(crate) VertexBufferDesc);
impl From<VertexBufferDesc> for AssetDescVBSlot06 {
    fn from(value: VertexBufferDesc) -> Self { Self(value) }
}
impl TVertexBufferUseInfo for AssetDescVBSlot06 {
    const ASK_SLOT_COUNT: u8 = 6;
    fn desc(&self) -> &VertexBufferDesc { &self.0 }
}

#[derive(Debug, Deref, DerefMut, Component)]
pub struct AssetResVBSlot06(pub EVerticesBufferUsage);
impl From<EVerticesBufferUsage> for AssetResVBSlot06 {
    fn from(value: EVerticesBufferUsage) -> Self { Self(value) }
}
impl TAssetResVertexBuffer for AssetResVBSlot06 {
    fn buffer(&self) -> EVerticesBufferUsage { self.0.clone() }
}

// ============== 7
#[derive(Debug, Deref, DerefMut, Clone, Hash, Component)]
pub struct AssetKeyVBSlot07(pub KeyVertexBuffer);
impl AsKeyVertexBuffer for AssetKeyVBSlot07 {
    fn create(desc: &VertexBufferDesc) -> Self { Self(desc.bufferkey().clone()) }
}
#[derive(Component)]

pub struct AssetDescVBSlot07(pub(crate) VertexBufferDesc);
impl From<VertexBufferDesc> for AssetDescVBSlot07 {
    fn from(value: VertexBufferDesc) -> Self { Self(value) }
}
impl TVertexBufferUseInfo for AssetDescVBSlot07 {
    const ASK_SLOT_COUNT: u8 = 7;
    fn desc(&self) -> &VertexBufferDesc { &self.0 }
}

#[derive(Debug, Deref, DerefMut, Component)]
pub struct AssetResVBSlot07(pub EVerticesBufferUsage);
impl From<EVerticesBufferUsage> for AssetResVBSlot07 {
    fn from(value: EVerticesBufferUsage) -> Self { Self(value) }
}
impl TAssetResVertexBuffer for AssetResVBSlot07 {
    fn buffer(&self) -> EVerticesBufferUsage { self.0.clone() }
}

// ============== 8
#[derive(Debug, Deref, DerefMut, Clone, Hash, Component)]
pub struct AssetKeyVBSlot08(pub KeyVertexBuffer);
impl AsKeyVertexBuffer for AssetKeyVBSlot08 {
    fn create(desc: &VertexBufferDesc) -> Self { Self(desc.bufferkey().clone()) }
}
#[derive(Component)]
pub struct AssetDescVBSlot08(pub(crate) VertexBufferDesc);
impl From<VertexBufferDesc> for AssetDescVBSlot08 {
    fn from(value: VertexBufferDesc) -> Self { Self(value) }
}
impl TVertexBufferUseInfo for AssetDescVBSlot08 {
    const ASK_SLOT_COUNT: u8 = 8;
    fn desc(&self) -> &VertexBufferDesc { &self.0 }
}

#[derive(Debug, Deref, DerefMut, Component)]
pub struct AssetResVBSlot08(pub EVerticesBufferUsage);
impl From<EVerticesBufferUsage> for AssetResVBSlot08 {
    fn from(value: EVerticesBufferUsage) -> Self { Self(value) }
}
impl TAssetResVertexBuffer for AssetResVBSlot08 {
    fn buffer(&self) -> EVerticesBufferUsage { self.0.clone() }
}

// ============== 9
#[derive(Debug, Deref, DerefMut, Clone, Hash, Component)]
pub struct AssetKeyVBSlot09(pub KeyVertexBuffer);
impl AsKeyVertexBuffer for AssetKeyVBSlot09 {
    fn create(desc: &VertexBufferDesc) -> Self { Self(desc.bufferkey().clone()) }
}
#[derive(Component)]
pub struct AssetDescVBSlot09(pub(crate) VertexBufferDesc);
impl From<VertexBufferDesc> for AssetDescVBSlot09 {
    fn from(value: VertexBufferDesc) -> Self { Self(value) }
}
impl TVertexBufferUseInfo for AssetDescVBSlot09 {
    const ASK_SLOT_COUNT: u8 = 9;
    fn desc(&self) -> &VertexBufferDesc { &self.0 }
}

#[derive(Debug, Deref, DerefMut, Component)]
pub struct AssetResVBSlot09(pub EVerticesBufferUsage);
impl From<EVerticesBufferUsage> for AssetResVBSlot09 {
    fn from(value: EVerticesBufferUsage) -> Self { Self(value) }
}
impl TAssetResVertexBuffer for AssetResVBSlot09 {
    fn buffer(&self) -> EVerticesBufferUsage { self.0.clone() }
}

// ============== 10
#[derive(Debug, Deref, DerefMut, Clone, Hash, Component)]
pub struct AssetKeyVBSlot10(pub KeyVertexBuffer);
impl AsKeyVertexBuffer for AssetKeyVBSlot10 {
    fn create(desc: &VertexBufferDesc) -> Self { Self(desc.bufferkey().clone()) }
}
#[derive(Component)]
pub struct AssetDescVBSlot10(pub(crate) VertexBufferDesc);
impl From<VertexBufferDesc> for AssetDescVBSlot10 {
    fn from(value: VertexBufferDesc) -> Self { Self(value) }
}
impl TVertexBufferUseInfo for AssetDescVBSlot10{
    const ASK_SLOT_COUNT: u8 = 10;
    fn desc(&self) -> &VertexBufferDesc { &self.0 }
}

#[derive(Debug, Deref, DerefMut, Component)]
pub struct AssetResVBSlot10(pub EVerticesBufferUsage);
impl From<EVerticesBufferUsage> for AssetResVBSlot10 {
    fn from(value: EVerticesBufferUsage) -> Self { Self(value) }
}
impl TAssetResVertexBuffer for AssetResVBSlot10 {
    fn buffer(&self) -> EVerticesBufferUsage { self.0.clone() }
}

// ============== 11
#[derive(Debug, Deref, DerefMut, Clone, Hash, Component)]
pub struct AssetKeyVBSlot11(pub KeyVertexBuffer);
impl AsKeyVertexBuffer for AssetKeyVBSlot11 {
    fn create(desc: &VertexBufferDesc) -> Self { Self(desc.bufferkey().clone()) }
}
#[derive(Component)]
pub struct AssetDescVBSlot11(pub(crate) VertexBufferDesc);
impl From<VertexBufferDesc> for AssetDescVBSlot11 {
    fn from(value: VertexBufferDesc) -> Self { Self(value) }
}
impl TVertexBufferUseInfo for AssetDescVBSlot11 {
    const ASK_SLOT_COUNT: u8 = 11;
    fn desc(&self) -> &VertexBufferDesc { &self.0 }
}

#[derive(Debug, Deref, DerefMut, Component)]
pub struct AssetResVBSlot11(pub EVerticesBufferUsage);
impl From<EVerticesBufferUsage> for AssetResVBSlot11 {
    fn from(value: EVerticesBufferUsage) -> Self { Self(value) }
}
impl TAssetResVertexBuffer for AssetResVBSlot11 {
    fn buffer(&self) -> EVerticesBufferUsage { self.0.clone() }
}

// ============== 12
#[derive(Debug, Deref, DerefMut, Clone, Hash, Component)]
pub struct AssetKeyVBSlot12(pub KeyVertexBuffer);
impl AsKeyVertexBuffer for AssetKeyVBSlot12 {
    fn create(desc: &VertexBufferDesc) -> Self { Self(desc.bufferkey().clone()) }
}
#[derive(Component)]
pub struct AssetDescVBSlot12(pub(crate) VertexBufferDesc);
impl From<VertexBufferDesc> for AssetDescVBSlot12 {
    fn from(value: VertexBufferDesc) -> Self { Self(value) }
}
impl TVertexBufferUseInfo for AssetDescVBSlot12 {
    const ASK_SLOT_COUNT: u8 = 12;
    fn desc(&self) -> &VertexBufferDesc { &self.0 }
}

#[derive(Debug, Deref, DerefMut, Component)]
pub struct AssetResVBSlot12(pub EVerticesBufferUsage);
impl From<EVerticesBufferUsage> for AssetResVBSlot12 {
    fn from(value: EVerticesBufferUsage) -> Self { Self(value) }
}
impl TAssetResVertexBuffer for AssetResVBSlot12 {
    fn buffer(&self) -> EVerticesBufferUsage { self.0.clone() }
}


// ============== 13
#[derive(Debug, Deref, DerefMut, Clone, Hash, Component)]
pub struct AssetKeyVBSlot13(pub KeyVertexBuffer);
impl AsKeyVertexBuffer for AssetKeyVBSlot13 {
    fn create(desc: &VertexBufferDesc) -> Self { Self(desc.bufferkey().clone()) }
}
#[derive(Component)]
pub struct AssetDescVBSlot13(pub(crate) VertexBufferDesc);
impl From<VertexBufferDesc> for AssetDescVBSlot13 {
    fn from(value: VertexBufferDesc) -> Self { Self(value) }
}
impl TVertexBufferUseInfo for AssetDescVBSlot13 {
    const ASK_SLOT_COUNT: u8 = 13;
    fn desc(&self) -> &VertexBufferDesc { &self.0 }
}

#[derive(Debug, Deref, DerefMut, Component)]
pub struct AssetResVBSlot13(pub EVerticesBufferUsage);
impl From<EVerticesBufferUsage> for AssetResVBSlot13 {
    fn from(value: EVerticesBufferUsage) -> Self { Self(value) }
}
impl TAssetResVertexBuffer for AssetResVBSlot13 {
    fn buffer(&self) -> EVerticesBufferUsage { self.0.clone() }
}


// ============== 14
#[derive(Debug, Deref, DerefMut, Clone, Hash, Component)]
pub struct AssetKeyVBSlot14(pub KeyVertexBuffer);
impl AsKeyVertexBuffer for AssetKeyVBSlot14 {
    fn create(desc: &VertexBufferDesc) -> Self { Self(desc.bufferkey().clone()) }
}
#[derive(Component)]
pub struct AssetDescVBSlot14(pub(crate) VertexBufferDesc);
impl From<VertexBufferDesc> for AssetDescVBSlot14 {
    fn from(value: VertexBufferDesc) -> Self { Self(value) }
}
impl TVertexBufferUseInfo for AssetDescVBSlot14 {
    const ASK_SLOT_COUNT: u8 = 14;
    fn desc(&self) -> &VertexBufferDesc { &self.0 }
}

#[derive(Debug, Deref, DerefMut, Component)]
pub struct AssetResVBSlot14(pub EVerticesBufferUsage);
impl From<EVerticesBufferUsage> for AssetResVBSlot14 {
    fn from(value: EVerticesBufferUsage) -> Self { Self(value) }
}
impl TAssetResVertexBuffer for AssetResVBSlot14 {
    fn buffer(&self) -> EVerticesBufferUsage { self.0.clone() }
}



// ============== 15
#[derive(Debug, Deref, DerefMut, Clone, Hash, Component)]
pub struct AssetKeyVBSlot15(pub KeyVertexBuffer);
impl AsKeyVertexBuffer for AssetKeyVBSlot15 {
    fn create(desc: &VertexBufferDesc) -> Self { Self(desc.bufferkey().clone()) }
}
#[derive(Component)]
pub struct AssetDescVBSlot15(pub(crate) VertexBufferDesc);
impl From<VertexBufferDesc> for AssetDescVBSlot15 {
    fn from(value: VertexBufferDesc) -> Self { Self(value) }
}
impl TVertexBufferUseInfo for AssetDescVBSlot15 {
    const ASK_SLOT_COUNT: u8 = 15;
    fn desc(&self) -> &VertexBufferDesc { &self.0 }
}

#[derive(Debug, Deref, DerefMut, Component)]
pub struct AssetResVBSlot15(pub EVerticesBufferUsage);
impl From<EVerticesBufferUsage> for AssetResVBSlot15 {
    fn from(value: EVerticesBufferUsage) -> Self { Self(value) }
}
impl TAssetResVertexBuffer for AssetResVBSlot15 {
    fn buffer(&self) -> EVerticesBufferUsage { self.0.clone() }
}



// ============== 16
#[derive(Debug, Deref, DerefMut, Clone, Hash, Component)]
pub struct AssetKeyVBSlot16(pub KeyVertexBuffer);
impl AsKeyVertexBuffer for AssetKeyVBSlot16 {
    fn create(desc: &VertexBufferDesc) -> Self { Self(desc.bufferkey().clone()) }
}
#[derive(Component)]
pub struct AssetDescVBSlot16(pub(crate) VertexBufferDesc);
impl From<VertexBufferDesc> for AssetDescVBSlot16 {
    fn from(value: VertexBufferDesc) -> Self { Self(value) }
}
impl TVertexBufferUseInfo for AssetDescVBSlot16 {
    const ASK_SLOT_COUNT: u8 = 16;
    fn desc(&self) -> &VertexBufferDesc { &self.0 }
}

#[derive(Debug, Deref, DerefMut, Component)]
pub struct AssetResVBSlot16(pub EVerticesBufferUsage);
impl From<EVerticesBufferUsage> for AssetResVBSlot16 {
    fn from(value: EVerticesBufferUsage) -> Self { Self(value) }
}
impl TAssetResVertexBuffer for AssetResVBSlot16 {
    fn buffer(&self) -> EVerticesBufferUsage { self.0.clone() }
}

