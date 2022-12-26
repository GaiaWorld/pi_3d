use std::ops::Range;

use derive_deref::{Deref, DerefMut};
use pi_assets::asset::Handle;
use render_data_container::{KeyVertexBuffer, VertexBuffer, VertexBufferUse};
use render_geometry::vertex_data::{TVertexBufferDesc, VertexBufferDesc};

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
        self.desc().range.clone()
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
pub struct AssetKeyVBSlot1(pub KeyVertexBuffer);
impl AsKeyVertexBuffer for AssetKeyVBSlot1 {
    fn create(desc: &VertexBufferDesc) -> Self {
        Self(desc.bufferkey.clone())
    }
}
pub struct AssetDescVBSlot1 {
    pub desc: VertexBufferDesc,
}
impl From<VertexBufferDesc> for AssetDescVBSlot1 {
    fn from(value: VertexBufferDesc) -> Self {
        Self { desc: value }
    }
}
impl TAsVertexBufferKey for AssetDescVBSlot1 {
    fn key(&self) -> &KeyVertexBuffer {
        &self.desc.bufferkey
    }
}
impl TVertexBufferUseInfo for AssetDescVBSlot1 {
    const ASK_SLOT_COUNT: u8 = 1;

    fn desc(&self) -> &VertexBufferDesc {
        &self.desc
    }

    fn slot() -> u32 {
        0
    }
}

#[derive(Debug, Deref, DerefMut)]
pub struct AssetResVBSlot1(pub VertexBufferUse);
impl From<Handle<VertexBuffer>> for AssetResVBSlot1 {
    fn from(value: Handle<VertexBuffer>) -> Self {
        Self(VertexBufferUse::Handle(value))
    }
}
impl TAssetResVertexBuffer for AssetResVBSlot1 {
    fn buffer(&self) -> VertexBufferUse {
        self.0.clone()
    }
}

// ============== 2
#[derive(Debug, Deref, DerefMut, Clone, Hash)]
pub struct AssetKeyVBSlot2(pub KeyVertexBuffer);
impl AsKeyVertexBuffer for AssetKeyVBSlot2 {
    fn create(desc: &VertexBufferDesc) -> Self {
        Self(desc.bufferkey.clone())
    }
}
pub struct AssetDescVBSlot2 {
    pub desc: VertexBufferDesc,
}
impl From<VertexBufferDesc> for AssetDescVBSlot2 {
    fn from(value: VertexBufferDesc) -> Self {
        Self { desc: value }
    }
}
impl TAsVertexBufferKey for AssetDescVBSlot2 {
    fn key(&self) -> &KeyVertexBuffer {
        &self.desc.bufferkey
    }
}
impl TVertexBufferUseInfo for AssetDescVBSlot2 {
    const ASK_SLOT_COUNT: u8 = 2;

    fn desc(&self) -> &VertexBufferDesc {
        &self.desc
    }

    fn slot() -> u32 {
        1
    }
}

#[derive(Debug, Deref, DerefMut)]
pub struct AssetResVBSlot2(pub VertexBufferUse);
impl From<Handle<VertexBuffer>> for AssetResVBSlot2 {
    fn from(value: Handle<VertexBuffer>) -> Self {
        Self(VertexBufferUse::Handle(value))
    }
}
impl TAssetResVertexBuffer for AssetResVBSlot2 {
    fn buffer(&self) -> VertexBufferUse {
        self.0.clone()
    }
}

// ============== 3
#[derive(Debug, Deref, DerefMut, Clone, Hash)]
pub struct AssetKeyVBSlot3(pub KeyVertexBuffer);
impl AsKeyVertexBuffer for AssetKeyVBSlot3 {
    fn create(desc: &VertexBufferDesc) -> Self {
        Self(desc.bufferkey.clone())
    }
}
pub struct AssetDescVBSlot3 {
    pub desc: VertexBufferDesc,
}
impl From<VertexBufferDesc> for AssetDescVBSlot3 {
    fn from(value: VertexBufferDesc) -> Self {
        Self { desc: value }
    }
}
impl TAsVertexBufferKey for AssetDescVBSlot3 {
    fn key(&self) -> &KeyVertexBuffer {
        &self.desc.bufferkey
    }
}
impl TVertexBufferUseInfo for AssetDescVBSlot3 {
    const ASK_SLOT_COUNT: u8 = 3;

    fn desc(&self) -> &VertexBufferDesc {
        &self.desc
    }

    fn slot() -> u32 {
        2
    }
}

#[derive(Debug, Deref, DerefMut)]
pub struct AssetResVBSlot3(pub VertexBufferUse);
impl From<Handle<VertexBuffer>> for AssetResVBSlot3 {
    fn from(value: Handle<VertexBuffer>) -> Self {
        Self(VertexBufferUse::Handle(value))
    }
}
impl TAssetResVertexBuffer for AssetResVBSlot3 {
    fn buffer(&self) -> VertexBufferUse {
        self.0.clone()
    }
}

// ============== 4
#[derive(Debug, Deref, DerefMut, Clone, Hash)]
pub struct AssetKeyVBSlot4(pub KeyVertexBuffer);
impl AsKeyVertexBuffer for AssetKeyVBSlot4 {
    fn create(desc: &VertexBufferDesc) -> Self {
        Self(desc.bufferkey.clone())
    }
}
pub struct AssetDescVBSlot4 {
    pub desc: VertexBufferDesc,
}
impl From<VertexBufferDesc> for AssetDescVBSlot4 {
    fn from(value: VertexBufferDesc) -> Self {
        Self { desc: value }
    }
}
impl TAsVertexBufferKey for AssetDescVBSlot4 {
    fn key(&self) -> &KeyVertexBuffer {
        &self.desc.bufferkey
    }
}
impl TVertexBufferUseInfo for AssetDescVBSlot4 {
    const ASK_SLOT_COUNT: u8 = 4;

    fn desc(&self) -> &VertexBufferDesc {
        &self.desc
    }

    fn slot() -> u32 {
        3
    }
}

#[derive(Debug, Deref, DerefMut)]
pub struct AssetResVBSlot4(pub VertexBufferUse);
impl From<Handle<VertexBuffer>> for AssetResVBSlot4 {
    fn from(value: Handle<VertexBuffer>) -> Self {
        Self(VertexBufferUse::Handle(value))
    }
}
impl TAssetResVertexBuffer for AssetResVBSlot4 {
    fn buffer(&self) -> VertexBufferUse {
        self.0.clone()
    }
}

// ============== 5
#[derive(Debug, Deref, DerefMut, Clone, Hash)]
pub struct AssetKeyVBSlot5(pub KeyVertexBuffer);
impl AsKeyVertexBuffer for AssetKeyVBSlot5 {
    fn create(desc: &VertexBufferDesc) -> Self {
        Self(desc.bufferkey.clone())
    }
}
pub struct AssetDescVBSlot5 {
    pub desc: VertexBufferDesc,
}
impl From<VertexBufferDesc> for AssetDescVBSlot5 {
    fn from(value: VertexBufferDesc) -> Self {
        Self { desc: value }
    }
}
impl TAsVertexBufferKey for AssetDescVBSlot5 {
    fn key(&self) -> &KeyVertexBuffer {
        &self.desc.bufferkey
    }
}
impl TVertexBufferUseInfo for AssetDescVBSlot5 {
    const ASK_SLOT_COUNT: u8 = 5;

    fn desc(&self) -> &VertexBufferDesc {
        &self.desc
    }

    fn slot() -> u32 {
        4
    }
}

#[derive(Debug, Deref, DerefMut)]
pub struct AssetResVBSlot5(pub VertexBufferUse);
impl From<Handle<VertexBuffer>> for AssetResVBSlot5 {
    fn from(value: Handle<VertexBuffer>) -> Self {
        Self(VertexBufferUse::Handle(value))
    }
}
impl TAssetResVertexBuffer for AssetResVBSlot5 {
    fn buffer(&self) -> VertexBufferUse {
        self.0.clone()
    }
}

// ============== 6
#[derive(Debug, Deref, DerefMut, Clone, Hash)]
pub struct AssetKeyVBSlot6(pub KeyVertexBuffer);
impl AsKeyVertexBuffer for AssetKeyVBSlot6 {
    fn create(desc: &VertexBufferDesc) -> Self {
        Self(desc.bufferkey.clone())
    }
}
pub struct AssetDescVBSlot6 {
    pub desc: VertexBufferDesc,
}
impl From<VertexBufferDesc> for AssetDescVBSlot6 {
    fn from(value: VertexBufferDesc) -> Self {
        Self { desc: value }
    }
}
impl TAsVertexBufferKey for AssetDescVBSlot6 {
    fn key(&self) -> &KeyVertexBuffer {
        &self.desc.bufferkey
    }
}
impl TVertexBufferUseInfo for AssetDescVBSlot6 {
    const ASK_SLOT_COUNT: u8 = 6;

    fn desc(&self) -> &VertexBufferDesc {
        &self.desc
    }

    fn slot() -> u32 {
        5
    }
}

#[derive(Debug, Deref, DerefMut)]
pub struct AssetResVBSlot6(pub VertexBufferUse);
impl From<Handle<VertexBuffer>> for AssetResVBSlot6 {
    fn from(value: Handle<VertexBuffer>) -> Self {
        Self(VertexBufferUse::Handle(value))
    }
}
impl TAssetResVertexBuffer for AssetResVBSlot6 {
    fn buffer(&self) -> VertexBufferUse {
        self.0.clone()
    }
}

// ============== 7
#[derive(Debug, Deref, DerefMut, Clone, Hash)]
pub struct AssetKeyVBSlot7(pub KeyVertexBuffer);
impl AsKeyVertexBuffer for AssetKeyVBSlot7 {
    fn create(desc: &VertexBufferDesc) -> Self {
        Self(desc.bufferkey.clone())
    }
}

pub struct AssetDescVBSlot7 {
    pub desc: VertexBufferDesc,
}
impl From<VertexBufferDesc> for AssetDescVBSlot7 {
    fn from(value: VertexBufferDesc) -> Self {
        Self { desc: value }
    }
}
impl TAsVertexBufferKey for AssetDescVBSlot7 {
    fn key(&self) -> &KeyVertexBuffer {
        &self.desc.bufferkey
    }
}
impl TVertexBufferUseInfo for AssetDescVBSlot7 {
    const ASK_SLOT_COUNT: u8 = 7;

    fn desc(&self) -> &VertexBufferDesc {
        &self.desc
    }

    fn slot() -> u32 {
        6
    }
}

#[derive(Debug, Deref, DerefMut)]
pub struct AssetResVBSlot7(pub VertexBufferUse);
impl From<Handle<VertexBuffer>> for AssetResVBSlot7 {
    fn from(value: Handle<VertexBuffer>) -> Self {
        Self(VertexBufferUse::Handle(value))
    }
}
impl TAssetResVertexBuffer for AssetResVBSlot7 {
    fn buffer(&self) -> VertexBufferUse {
        self.0.clone()
    }
}

// ============== 8
#[derive(Debug, Deref, DerefMut, Clone, Hash)]
pub struct AssetKeyVBSlot8(pub KeyVertexBuffer);
impl AsKeyVertexBuffer for AssetKeyVBSlot8 {
    fn create(desc: &VertexBufferDesc) -> Self {
        Self(desc.bufferkey.clone())
    }
}
pub struct AssetDescVBSlot8 {
    pub desc: VertexBufferDesc,
}
impl From<VertexBufferDesc> for AssetDescVBSlot8 {
    fn from(value: VertexBufferDesc) -> Self {
        Self { desc: value }
    }
}
impl TAsVertexBufferKey for AssetDescVBSlot8 {
    fn key(&self) -> &KeyVertexBuffer {
        &self.desc.bufferkey
    }
}
impl TVertexBufferUseInfo for AssetDescVBSlot8 {
    const ASK_SLOT_COUNT: u8 = 8;

    fn desc(&self) -> &VertexBufferDesc {
        &self.desc
    }

    fn slot() -> u32 {
        7
    }
}

#[derive(Debug, Deref, DerefMut)]
pub struct AssetResVBSlot8(pub VertexBufferUse);
impl From<Handle<VertexBuffer>> for AssetResVBSlot8 {
    fn from(value: Handle<VertexBuffer>) -> Self {
        Self(VertexBufferUse::Handle(value))
    }
}
impl TAssetResVertexBuffer for AssetResVBSlot8 {
    fn buffer(&self) -> VertexBufferUse {
        self.0.clone()
    }
}

// ============== 9
#[derive(Debug, Deref, DerefMut, Clone, Hash)]
pub struct AssetKeyVBSlot9(pub KeyVertexBuffer);
impl AsKeyVertexBuffer for AssetKeyVBSlot9 {
    fn create(desc: &VertexBufferDesc) -> Self {
        Self(desc.bufferkey.clone())
    }
}
pub struct AssetDescVBSlot9 {
    pub desc: VertexBufferDesc,
}
impl From<VertexBufferDesc> for AssetDescVBSlot9 {
    fn from(value: VertexBufferDesc) -> Self {
        Self { desc: value }
    }
}
impl TAsVertexBufferKey for AssetDescVBSlot9 {
    fn key(&self) -> &KeyVertexBuffer {
        &self.desc.bufferkey
    }
}
impl TVertexBufferUseInfo for AssetDescVBSlot9 {
    const ASK_SLOT_COUNT: u8 = 9;

    fn desc(&self) -> &VertexBufferDesc {
        &self.desc
    }

    fn slot() -> u32 {
        8
    }
}

#[derive(Debug, Deref, DerefMut)]
pub struct AssetResVBSlot9(pub VertexBufferUse);
impl From<Handle<VertexBuffer>> for AssetResVBSlot9 {
    fn from(value: Handle<VertexBuffer>) -> Self {
        Self(VertexBufferUse::Handle(value))
    }
}
impl TAssetResVertexBuffer for AssetResVBSlot9 {
    fn buffer(&self) -> VertexBufferUse {
        self.0.clone()
    }
}

// ============== 10
#[derive(Debug, Deref, DerefMut, Clone, Hash)]
pub struct AssetKeyVBSlot10(pub KeyVertexBuffer);
impl AsKeyVertexBuffer for AssetKeyVBSlot10 {
    fn create(desc: &VertexBufferDesc) -> Self {
        Self(desc.bufferkey.clone())
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
        &self.desc.bufferkey
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
        Self(desc.bufferkey.clone())
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
        &self.desc.bufferkey
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
        Self(desc.bufferkey.clone())
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
        &self.desc.bufferkey
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
        Self(desc.bufferkey.clone())
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
        &self.desc.bufferkey
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
impl TAssetResVertexBuffer for AssetResVBSlot13 {
    fn buffer(&self) -> VertexBufferUse {
        self.0.clone()
    }
}
