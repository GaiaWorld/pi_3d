use std::ops::Range;

use derive_deref::{Deref, DerefMut};
use pi_assets::asset::Handle;
use render_data_container::{KeyVertexBuffer, VertexBuffer, VertexBufferUse};
use render_geometry::vertex_data::{VertexBufferDesc};

pub trait AsKeyVertexBuffer {
    fn create(desc: &VertexBufferDesc) -> Self;
}

#[derive(Debug, Clone, Copy)]
pub enum EVertexBufferSlot {
    Slot01,
    Slot02,
    Slot03,
    Slot04,
    Slot05,
    Slot06,
    Slot07,
    Slot08,
    Slot09,
    Slot10,
    Slot11,
    Slot12,
    Slot13,
    Slot14,
    Slot15,
    Slot16,
    Slot17,
    Slot18,
}
impl EVertexBufferSlot {
    pub fn from_u8_unsafe(index: u8) -> Self {
        if index == 0 {
            Self::Slot01
        }
        else if index == 1 {
            Self::Slot02
        }
        else if index == 2 {
            Self::Slot03
        }
        else if index == 3 {
            Self::Slot04
        }
        else if index == 4 {
            Self::Slot05
        }
        else if index == 5 {
            Self::Slot06
        }
        else if index == 6 {
            Self::Slot07
        }
        else if index == 7 {
            Self::Slot08
        }
        else if index == 8 {
            Self::Slot09
        }
        else if index == 9 {
            Self::Slot10
        }
        else if index == 10 {
            Self::Slot11
        }
        else if index == 11 {
            Self::Slot12
        }
        else if index == 12 {
            Self::Slot13
        }
        else if index == 13 {
            Self::Slot14
        }
        else if index == 14 {
            Self::Slot15
        }
        else if index == 15 {
            Self::Slot16
        }
        else if index == 16 {
            Self::Slot17
        }
        else {
            Self::Slot18
        }
    }
}

pub trait TVertexBufferUseInfo: TAsVertexBufferKey + From<VertexBufferDesc> {
    const ASK_SLOT_COUNT: u8;
    fn desc(&self) -> &VertexBufferDesc;
    fn range(&self) -> Option<Range<wgpu::BufferAddress>> {
        self.desc().range().clone()
    }
    fn slot() -> u32;
}
pub trait TAsVertexBufferKey {
    fn key(&self) -> &KeyVertexBuffer;
}
impl std::ops::Deref for dyn TAsVertexBufferKey {
    type Target = KeyVertexBuffer;
    fn deref(&self) -> &Self::Target {
        self.key()
    }
}

pub trait TAssetResVertexBuffer {
    fn buffer(&self) -> VertexBufferUse;
}

// ============== 1
#[derive(Debug, Deref, DerefMut, Clone, Hash)]
pub struct AssetKeyVBSlot01(pub KeyVertexBuffer);
impl AsKeyVertexBuffer for AssetKeyVBSlot01 {
    fn create(desc: &VertexBufferDesc) -> Self {
        Self(desc.bufferkey().clone())
    }
}
pub struct AssetDescVBSlot01 {
    pub desc: VertexBufferDesc,
}
impl From<VertexBufferDesc> for AssetDescVBSlot01 {
    fn from(value: VertexBufferDesc) -> Self {
        Self { desc: value }
    }
}
impl TAsVertexBufferKey for AssetDescVBSlot01 {
    fn key(&self) -> &KeyVertexBuffer {
        self.desc.bufferkey()
    }
}
impl TVertexBufferUseInfo for AssetDescVBSlot01 {
    const ASK_SLOT_COUNT: u8 = 1;

    fn desc(&self) -> &VertexBufferDesc {
        &self.desc
    }

    fn slot() -> u32 {
        0
    }
}

#[derive(Debug, Deref, DerefMut)]
pub struct AssetResVBSlot01(pub VertexBufferUse);
impl From<Handle<VertexBuffer>> for AssetResVBSlot01 {
    fn from(value: Handle<VertexBuffer>) -> Self {
        Self(VertexBufferUse::Handle(value))
    }
}
impl From<KeyVertexBuffer> for AssetResVBSlot01 {
    fn from(value: KeyVertexBuffer) -> Self {
        Self(VertexBufferUse::Arc(value))
    }
}
impl TAssetResVertexBuffer for AssetResVBSlot01 {
    fn buffer(&self) -> VertexBufferUse {
        self.0.clone()
    }
}

// ============== 2
#[derive(Debug, Deref, DerefMut, Clone, Hash)]
pub struct AssetKeyVBSlot02(pub KeyVertexBuffer);
impl AsKeyVertexBuffer for AssetKeyVBSlot02 {
    fn create(desc: &VertexBufferDesc) -> Self {
        Self(desc.bufferkey().clone())
    }
}
pub struct AssetDescVBSlot02 {
    pub desc: VertexBufferDesc,
}
impl From<VertexBufferDesc> for AssetDescVBSlot02 {
    fn from(value: VertexBufferDesc) -> Self {
        Self { desc: value }
    }
}
impl TAsVertexBufferKey for AssetDescVBSlot02 {
    fn key(&self) -> &KeyVertexBuffer {
        self.desc.bufferkey()
    }
}
impl TVertexBufferUseInfo for AssetDescVBSlot02 {
    const ASK_SLOT_COUNT: u8 = 2;

    fn desc(&self) -> &VertexBufferDesc {
        &self.desc
    }

    fn slot() -> u32 {
        1
    }
}

#[derive(Debug, Deref, DerefMut)]
pub struct AssetResVBSlot02(pub VertexBufferUse);
impl From<Handle<VertexBuffer>> for AssetResVBSlot02 {
    fn from(value: Handle<VertexBuffer>) -> Self {
        Self(VertexBufferUse::Handle(value))
    }
}
impl From<KeyVertexBuffer> for AssetResVBSlot02 {
    fn from(value: KeyVertexBuffer) -> Self {
        Self(VertexBufferUse::Arc(value))
    }
}
impl TAssetResVertexBuffer for AssetResVBSlot02 {
    fn buffer(&self) -> VertexBufferUse {
        self.0.clone()
    }
}

// ============== 3
#[derive(Debug, Deref, DerefMut, Clone, Hash)]
pub struct AssetKeyVBSlot03(pub KeyVertexBuffer);
impl AsKeyVertexBuffer for AssetKeyVBSlot03 {
    fn create(desc: &VertexBufferDesc) -> Self {
        Self(desc.bufferkey().clone())
    }
}
pub struct AssetDescVBSlot03 {
    pub desc: VertexBufferDesc,
}
impl From<VertexBufferDesc> for AssetDescVBSlot03 {
    fn from(value: VertexBufferDesc) -> Self {
        Self { desc: value }
    }
}
impl TAsVertexBufferKey for AssetDescVBSlot03 {
    fn key(&self) -> &KeyVertexBuffer {
        self.desc.bufferkey()
    }
}
impl TVertexBufferUseInfo for AssetDescVBSlot03 {
    const ASK_SLOT_COUNT: u8 = 3;

    fn desc(&self) -> &VertexBufferDesc {
        &self.desc
    }

    fn slot() -> u32 {
        2
    }
}

#[derive(Debug, Deref, DerefMut)]
pub struct AssetResVBSlot03(pub VertexBufferUse);
impl From<Handle<VertexBuffer>> for AssetResVBSlot03 {
    fn from(value: Handle<VertexBuffer>) -> Self {
        Self(VertexBufferUse::Handle(value))
    }
}
impl From<KeyVertexBuffer> for AssetResVBSlot03 {
    fn from(value: KeyVertexBuffer) -> Self {
        Self(VertexBufferUse::Arc(value))
    }
}
impl TAssetResVertexBuffer for AssetResVBSlot03 {
    fn buffer(&self) -> VertexBufferUse {
        self.0.clone()
    }
}

// ============== 4
#[derive(Debug, Deref, DerefMut, Clone, Hash)]
pub struct AssetKeyVBSlot04(pub KeyVertexBuffer);
impl AsKeyVertexBuffer for AssetKeyVBSlot04 {
    fn create(desc: &VertexBufferDesc) -> Self {
        Self(desc.bufferkey().clone())
    }
}
pub struct AssetDescVBSlot04 {
    pub desc: VertexBufferDesc,
}
impl From<VertexBufferDesc> for AssetDescVBSlot04 {
    fn from(value: VertexBufferDesc) -> Self {
        Self { desc: value }
    }
}
impl TAsVertexBufferKey for AssetDescVBSlot04 {
    fn key(&self) -> &KeyVertexBuffer {
        self.desc.bufferkey()
    }
}
impl TVertexBufferUseInfo for AssetDescVBSlot04 {
    const ASK_SLOT_COUNT: u8 = 4;

    fn desc(&self) -> &VertexBufferDesc {
        &self.desc
    }

    fn slot() -> u32 {
        3
    }
}

#[derive(Debug, Deref, DerefMut)]
pub struct AssetResVBSlot04(pub VertexBufferUse);
impl From<Handle<VertexBuffer>> for AssetResVBSlot04 {
    fn from(value: Handle<VertexBuffer>) -> Self {
        Self(VertexBufferUse::Handle(value))
    }
}
impl From<KeyVertexBuffer> for AssetResVBSlot04 {
    fn from(value: KeyVertexBuffer) -> Self {
        Self(VertexBufferUse::Arc(value))
    }
}
impl TAssetResVertexBuffer for AssetResVBSlot04 {
    fn buffer(&self) -> VertexBufferUse {
        self.0.clone()
    }
}

// ============== 5
#[derive(Debug, Deref, DerefMut, Clone, Hash)]
pub struct AssetKeyVBSlot05(pub KeyVertexBuffer);
impl AsKeyVertexBuffer for AssetKeyVBSlot05 {
    fn create(desc: &VertexBufferDesc) -> Self {
        Self(desc.bufferkey().clone())
    }
}
pub struct AssetDescVBSlot05 {
    pub desc: VertexBufferDesc,
}
impl From<VertexBufferDesc> for AssetDescVBSlot05 {
    fn from(value: VertexBufferDesc) -> Self {
        Self { desc: value }
    }
}
impl TAsVertexBufferKey for AssetDescVBSlot05 {
    fn key(&self) -> &KeyVertexBuffer {
        self.desc.bufferkey()
    }
}
impl TVertexBufferUseInfo for AssetDescVBSlot05 {
    const ASK_SLOT_COUNT: u8 = 5;

    fn desc(&self) -> &VertexBufferDesc {
        &self.desc
    }

    fn slot() -> u32 {
        4
    }
}

#[derive(Debug, Deref, DerefMut)]
pub struct AssetResVBSlot05(pub VertexBufferUse);
impl From<Handle<VertexBuffer>> for AssetResVBSlot05 {
    fn from(value: Handle<VertexBuffer>) -> Self {
        Self(VertexBufferUse::Handle(value))
    }
}
impl From<KeyVertexBuffer> for AssetResVBSlot05 {
    fn from(value: KeyVertexBuffer) -> Self {
        Self(VertexBufferUse::Arc(value))
    }
}
impl TAssetResVertexBuffer for AssetResVBSlot05 {
    fn buffer(&self) -> VertexBufferUse {
        self.0.clone()
    }
}

// ============== 6
#[derive(Debug, Deref, DerefMut, Clone, Hash)]
pub struct AssetKeyVBSlot06(pub KeyVertexBuffer);
impl AsKeyVertexBuffer for AssetKeyVBSlot06 {
    fn create(desc: &VertexBufferDesc) -> Self {
        Self(desc.bufferkey().clone())
    }
}
pub struct AssetDescVBSlot06 {
    pub desc: VertexBufferDesc,
}
impl From<VertexBufferDesc> for AssetDescVBSlot06 {
    fn from(value: VertexBufferDesc) -> Self {
        Self { desc: value }
    }
}
impl TAsVertexBufferKey for AssetDescVBSlot06 {
    fn key(&self) -> &KeyVertexBuffer {
        self.desc.bufferkey()
    }
}
impl TVertexBufferUseInfo for AssetDescVBSlot06 {
    const ASK_SLOT_COUNT: u8 = 6;

    fn desc(&self) -> &VertexBufferDesc {
        &self.desc
    }

    fn slot() -> u32 {
        5
    }
}

#[derive(Debug, Deref, DerefMut)]
pub struct AssetResVBSlot06(pub VertexBufferUse);
impl From<Handle<VertexBuffer>> for AssetResVBSlot06 {
    fn from(value: Handle<VertexBuffer>) -> Self {
        Self(VertexBufferUse::Handle(value))
    }
}
impl From<KeyVertexBuffer> for AssetResVBSlot06 {
    fn from(value: KeyVertexBuffer) -> Self {
        Self(VertexBufferUse::Arc(value))
    }
}
impl TAssetResVertexBuffer for AssetResVBSlot06 {
    fn buffer(&self) -> VertexBufferUse {
        self.0.clone()
    }
}

// ============== 7
#[derive(Debug, Deref, DerefMut, Clone, Hash)]
pub struct AssetKeyVBSlot07(pub KeyVertexBuffer);
impl AsKeyVertexBuffer for AssetKeyVBSlot07 {
    fn create(desc: &VertexBufferDesc) -> Self {
        Self(desc.bufferkey().clone())
    }
}

pub struct AssetDescVBSlot07 {
    pub desc: VertexBufferDesc,
}
impl From<VertexBufferDesc> for AssetDescVBSlot07 {
    fn from(value: VertexBufferDesc) -> Self {
        Self { desc: value }
    }
}
impl TAsVertexBufferKey for AssetDescVBSlot07 {
    fn key(&self) -> &KeyVertexBuffer {
        self.desc.bufferkey()
    }
}
impl TVertexBufferUseInfo for AssetDescVBSlot07 {
    const ASK_SLOT_COUNT: u8 = 7;

    fn desc(&self) -> &VertexBufferDesc {
        &self.desc
    }

    fn slot() -> u32 {
        6
    }
}

#[derive(Debug, Deref, DerefMut)]
pub struct AssetResVBSlot07(pub VertexBufferUse);
impl From<Handle<VertexBuffer>> for AssetResVBSlot07 {
    fn from(value: Handle<VertexBuffer>) -> Self {
        Self(VertexBufferUse::Handle(value))
    }
}
impl From<KeyVertexBuffer> for AssetResVBSlot07 {
    fn from(value: KeyVertexBuffer) -> Self {
        Self(VertexBufferUse::Arc(value))
    }
}
impl TAssetResVertexBuffer for AssetResVBSlot07 {
    fn buffer(&self) -> VertexBufferUse {
        self.0.clone()
    }
}

// ============== 8
#[derive(Debug, Deref, DerefMut, Clone, Hash)]
pub struct AssetKeyVBSlot08(pub KeyVertexBuffer);
impl AsKeyVertexBuffer for AssetKeyVBSlot08 {
    fn create(desc: &VertexBufferDesc) -> Self {
        Self(desc.bufferkey().clone())
    }
}
pub struct AssetDescVBSlot08 {
    pub desc: VertexBufferDesc,
}
impl From<VertexBufferDesc> for AssetDescVBSlot08 {
    fn from(value: VertexBufferDesc) -> Self {
        Self { desc: value }
    }
}
impl TAsVertexBufferKey for AssetDescVBSlot08 {
    fn key(&self) -> &KeyVertexBuffer {
        self.desc.bufferkey()
    }
}
impl TVertexBufferUseInfo for AssetDescVBSlot08 {
    const ASK_SLOT_COUNT: u8 = 8;

    fn desc(&self) -> &VertexBufferDesc {
        &self.desc
    }

    fn slot() -> u32 {
        7
    }
}

#[derive(Debug, Deref, DerefMut)]
pub struct AssetResVBSlot08(pub VertexBufferUse);
impl From<Handle<VertexBuffer>> for AssetResVBSlot08 {
    fn from(value: Handle<VertexBuffer>) -> Self {
        Self(VertexBufferUse::Handle(value))
    }
}
impl From<KeyVertexBuffer> for AssetResVBSlot08 {
    fn from(value: KeyVertexBuffer) -> Self {
        Self(VertexBufferUse::Arc(value))
    }
}
impl TAssetResVertexBuffer for AssetResVBSlot08 {
    fn buffer(&self) -> VertexBufferUse {
        self.0.clone()
    }
}

// ============== 9
#[derive(Debug, Deref, DerefMut, Clone, Hash)]
pub struct AssetKeyVBSlot09(pub KeyVertexBuffer);
impl AsKeyVertexBuffer for AssetKeyVBSlot09 {
    fn create(desc: &VertexBufferDesc) -> Self {
        Self(desc.bufferkey().clone())
    }
}
pub struct AssetDescVBSlot09 {
    pub desc: VertexBufferDesc,
}
impl From<VertexBufferDesc> for AssetDescVBSlot09 {
    fn from(value: VertexBufferDesc) -> Self {
        Self { desc: value }
    }
}
impl TAsVertexBufferKey for AssetDescVBSlot09 {
    fn key(&self) -> &KeyVertexBuffer {
        self.desc.bufferkey()
    }
}
impl TVertexBufferUseInfo for AssetDescVBSlot09 {
    const ASK_SLOT_COUNT: u8 = 9;

    fn desc(&self) -> &VertexBufferDesc {
        &self.desc
    }

    fn slot() -> u32 {
        8
    }
}

#[derive(Debug, Deref, DerefMut)]
pub struct AssetResVBSlot09(pub VertexBufferUse);
impl From<Handle<VertexBuffer>> for AssetResVBSlot09 {
    fn from(value: Handle<VertexBuffer>) -> Self {
        Self(VertexBufferUse::Handle(value))
    }
}
impl From<KeyVertexBuffer> for AssetResVBSlot09 {
    fn from(value: KeyVertexBuffer) -> Self {
        Self(VertexBufferUse::Arc(value))
    }
}
impl TAssetResVertexBuffer for AssetResVBSlot09 {
    fn buffer(&self) -> VertexBufferUse {
        self.0.clone()
    }
}

// ============== 10
#[derive(Debug, Deref, DerefMut, Clone, Hash)]
pub struct AssetKeyVBSlot10(pub KeyVertexBuffer);
impl AsKeyVertexBuffer for AssetKeyVBSlot10 {
    fn create(desc: &VertexBufferDesc) -> Self {
        Self(desc.bufferkey().clone())
    }
}
pub struct AssetDescVBSlot10 {
    pub desc: VertexBufferDesc,
}
impl From<VertexBufferDesc> for AssetDescVBSlot10 {
    fn from(value: VertexBufferDesc) -> Self {
        Self { desc: value }
    }
}
impl TAsVertexBufferKey for AssetDescVBSlot10 {
    fn key(&self) -> &KeyVertexBuffer {
        self.desc.bufferkey()
    }
}
impl TVertexBufferUseInfo for AssetDescVBSlot10{
    const ASK_SLOT_COUNT: u8 = 10;

    fn desc(&self) -> &VertexBufferDesc {
        &self.desc
    }

    fn slot() -> u32 {
        9
    }
}

#[derive(Debug, Deref, DerefMut)]
pub struct AssetResVBSlot10(pub VertexBufferUse);
impl From<Handle<VertexBuffer>> for AssetResVBSlot10 {
    fn from(value: Handle<VertexBuffer>) -> Self {
        Self(VertexBufferUse::Handle(value))
    }
}
impl From<KeyVertexBuffer> for AssetResVBSlot10 {
    fn from(value: KeyVertexBuffer) -> Self {
        Self(VertexBufferUse::Arc(value))
    }
}
impl TAssetResVertexBuffer for AssetResVBSlot10 {
    fn buffer(&self) -> VertexBufferUse {
        self.0.clone()
    }
}

// ============== 11
#[derive(Debug, Deref, DerefMut, Clone, Hash)]
pub struct AssetKeyVBSlot11(pub KeyVertexBuffer);
impl AsKeyVertexBuffer for AssetKeyVBSlot11 {
    fn create(desc: &VertexBufferDesc) -> Self {
        Self(desc.bufferkey().clone())
    }
}
pub struct AssetDescVBSlot11 {
    pub desc: VertexBufferDesc,
}
impl From<VertexBufferDesc> for AssetDescVBSlot11 {
    fn from(value: VertexBufferDesc) -> Self {
        Self { desc: value }
    }
}
impl TAsVertexBufferKey for AssetDescVBSlot11 {
    fn key(&self) -> &KeyVertexBuffer {
        self.desc.bufferkey()
    }
}
impl TVertexBufferUseInfo for AssetDescVBSlot11 {
    const ASK_SLOT_COUNT: u8 = 11;

    fn desc(&self) -> &VertexBufferDesc {
        &self.desc
    }

    fn slot() -> u32 {
        10
    }
}

#[derive(Debug, Deref, DerefMut)]
pub struct AssetResVBSlot11(pub VertexBufferUse);
impl From<Handle<VertexBuffer>> for AssetResVBSlot11 {
    fn from(value: Handle<VertexBuffer>) -> Self {
        Self(VertexBufferUse::Handle(value))
    }
}
impl From<KeyVertexBuffer> for AssetResVBSlot11 {
    fn from(value: KeyVertexBuffer) -> Self {
        Self(VertexBufferUse::Arc(value))
    }
}
impl TAssetResVertexBuffer for AssetResVBSlot11 {
    fn buffer(&self) -> VertexBufferUse {
        self.0.clone()
    }
}

// ============== 12
#[derive(Debug, Deref, DerefMut, Clone, Hash)]
pub struct AssetKeyVBSlot12(pub KeyVertexBuffer);
impl AsKeyVertexBuffer for AssetKeyVBSlot12 {
    fn create(desc: &VertexBufferDesc) -> Self {
        Self(desc.bufferkey().clone())
    }
}
pub struct AssetDescVBSlot12 {
    pub desc: VertexBufferDesc,
}
impl From<VertexBufferDesc> for AssetDescVBSlot12 {
    fn from(value: VertexBufferDesc) -> Self {
        Self { desc: value }
    }
}
impl TAsVertexBufferKey for AssetDescVBSlot12 {
    fn key(&self) -> &KeyVertexBuffer {
        self.desc.bufferkey()
    }
}
impl TVertexBufferUseInfo for AssetDescVBSlot12 {
    const ASK_SLOT_COUNT: u8 = 12;

    fn desc(&self) -> &VertexBufferDesc {
        &self.desc
    }

    fn slot() -> u32 {
        11
    }
}

#[derive(Debug, Deref, DerefMut)]
pub struct AssetResVBSlot12(pub VertexBufferUse);
impl From<Handle<VertexBuffer>> for AssetResVBSlot12 {
    fn from(value: Handle<VertexBuffer>) -> Self {
        Self(VertexBufferUse::Handle(value))
    }
}
impl From<KeyVertexBuffer> for AssetResVBSlot12 {
    fn from(value: KeyVertexBuffer) -> Self {
        Self(VertexBufferUse::Arc(value))
    }
}
impl TAssetResVertexBuffer for AssetResVBSlot12 {
    fn buffer(&self) -> VertexBufferUse {
        self.0.clone()
    }
}


// ============== 13
#[derive(Debug, Deref, DerefMut, Clone, Hash)]
pub struct AssetKeyVBSlot13(pub KeyVertexBuffer);
impl AsKeyVertexBuffer for AssetKeyVBSlot13 {
    fn create(desc: &VertexBufferDesc) -> Self {
        Self(desc.bufferkey().clone())
    }
}
pub struct AssetDescVBSlot13 {
    pub desc: VertexBufferDesc,
}
impl From<VertexBufferDesc> for AssetDescVBSlot13 {
    fn from(value: VertexBufferDesc) -> Self {
        Self { desc: value }
    }
}
impl TAsVertexBufferKey for AssetDescVBSlot13 {
    fn key(&self) -> &KeyVertexBuffer {
        self.desc.bufferkey()
    }
}
impl TVertexBufferUseInfo for AssetDescVBSlot13 {
    const ASK_SLOT_COUNT: u8 = 13;

    fn desc(&self) -> &VertexBufferDesc {
        &self.desc
    }

    fn slot() -> u32 {
        12
    }
}

#[derive(Debug, Deref, DerefMut)]
pub struct AssetResVBSlot13(pub VertexBufferUse);
impl From<Handle<VertexBuffer>> for AssetResVBSlot13 {
    fn from(value: Handle<VertexBuffer>) -> Self {
        Self(VertexBufferUse::Handle(value))
    }
}
impl From<KeyVertexBuffer> for AssetResVBSlot13 {
    fn from(value: KeyVertexBuffer) -> Self {
        Self(VertexBufferUse::Arc(value))
    }
}
impl TAssetResVertexBuffer for AssetResVBSlot13 {
    fn buffer(&self) -> VertexBufferUse {
        self.0.clone()
    }
}
