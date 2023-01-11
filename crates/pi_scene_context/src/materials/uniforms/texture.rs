use derive_deref::{DerefMut, Deref};
use pi_assets::asset::Handle;
use pi_render::rhi::{asset::TextureRes, texture::Sampler, device::RenderDevice};
use pi_share::ThreadSync;
use render_resource::{ImageAssetKey, sampler::{SamplerDesc, SamplerPool}};

use crate::materials::value::{UniformBind, SlotActiveRequire};

#[derive(Debug, Clone, Copy)]
pub enum ETextureSlot {
    Slot0,
    Slot1,
    Slot2,
    Slot3,
}


pub trait ValueTextureKey: ThreadSync + 'static {
    fn new(path: ImageAssetKey) -> Self;
    fn key(&self) -> &ImageAssetKey;
}

pub trait UniformTexture {
    fn texture(&self) -> &TextureRes;
}

pub trait UniformSampler: ThreadSync + 'static {
    const ASK_SLOT_COUNT: u8;
    fn new(desc: &SamplerDesc, device: &RenderDevice, pool: &mut SamplerPool) -> Self;
    fn sampler(&self) -> &Sampler;
}

// ==== ==== ==== ==== 1
#[derive(Debug, Deref, DerefMut, Clone, Default, Hash)]
pub struct TextureSlot01(pub ImageAssetKey);
impl UniformBind for TextureSlot01 {
    fn bind(&self) -> u32 {
        0
    }
}
impl ValueTextureKey for TextureSlot01 {
    fn new(path: ImageAssetKey) -> Self {
        Self(path)
    }
    fn key(&self) -> &ImageAssetKey {
        &self.0
    }
}
impl SlotActiveRequire for TextureSlot01 {
    const ASK_SLOT_COUNT: u8 = 1;
}

#[derive(Deref, DerefMut)]
pub struct TextureResSlot01(pub Handle<TextureRes>);
impl From<Handle<TextureRes>> for TextureResSlot01 {
    fn from(h: Handle<TextureRes>) -> Self { Self(h) }
}
impl UniformTexture for TextureResSlot01 {
    fn texture(&self) -> &TextureRes {
        &self.0
    }
}

pub struct SamplerSlot01 {
    pub sampler: Sampler,
}
impl UniformBind for SamplerSlot01 {
    fn bind(&self) -> u32 {
        0
    }
}
impl UniformSampler for SamplerSlot01 {
    const ASK_SLOT_COUNT: u8 = 1;
    fn new(desc: &SamplerDesc, device: &RenderDevice, pool: &mut SamplerPool) -> Self {
        pool.create(desc, device);
        let sampler = pool.get(SamplerPool::cacl_key(desc)).unwrap();
        Self {
            sampler
        }
    }
    fn sampler(&self) -> &Sampler {
        &self.sampler
    }
}

// ==== ==== ==== ==== 2
#[derive(Debug, Deref, DerefMut, Clone, Default, Hash)]
pub struct TextureSlot02(pub ImageAssetKey);
impl UniformBind for TextureSlot02 {
    fn bind(&self) -> u32 {
        1
    }
}
impl ValueTextureKey for TextureSlot02 {
    fn new(path: ImageAssetKey) -> Self {
        Self(path)
    }
    fn key(&self) -> &ImageAssetKey {
        &self.0
    }
}
impl SlotActiveRequire for TextureSlot02 {
    const ASK_SLOT_COUNT: u8 = 2;
}

#[derive(Deref, DerefMut)]
pub struct TextureResSlot02(pub Handle<TextureRes>);
impl From<Handle<TextureRes>> for TextureResSlot02 {
    fn from(h: Handle<TextureRes>) -> Self { Self(h) }
}
impl UniformTexture for TextureResSlot02 {
    fn texture(&self) -> &TextureRes {
        &self.0
    }
}

pub struct SamplerSlot02 {
    pub sampler: Sampler,
}
impl UniformBind for SamplerSlot02 {
    fn bind(&self) -> u32 {
        0
    }
}
impl UniformSampler for SamplerSlot02 {
    const ASK_SLOT_COUNT: u8 = 2;
    fn new(desc: &SamplerDesc, device: &RenderDevice, pool: &mut SamplerPool) -> Self {
        pool.create(desc, device);
        let sampler = pool.get(SamplerPool::cacl_key(desc)).unwrap();
        Self {
            sampler
        }
    }
    fn sampler(&self) -> &Sampler {
        &self.sampler
    }
}

// ==== ==== ==== ==== 3
#[derive(Debug, Deref, DerefMut, Clone, Default, Hash)]
pub struct TextureSlot03(pub ImageAssetKey);
impl UniformBind for TextureSlot03 {
    fn bind(&self) -> u32 {
        2
    }
}
impl ValueTextureKey for TextureSlot03 {
    fn new(path: ImageAssetKey) -> Self {
        Self(path)
    }
    fn key(&self) -> &ImageAssetKey {
        &self.0
    }
}
impl SlotActiveRequire for TextureSlot03 {
    const ASK_SLOT_COUNT: u8 = 3;
}

#[derive(Deref, DerefMut)]
pub struct TextureResSlot03(pub Handle<TextureRes>);
impl From<Handle<TextureRes>> for TextureResSlot03 {
    fn from(h: Handle<TextureRes>) -> Self { Self(h) }
}
impl UniformTexture for TextureResSlot03 {
    fn texture(&self) -> &TextureRes {
        &self.0
    }
}

pub struct SamplerSlot03 {
    pub sampler: Sampler,
}
impl UniformBind for SamplerSlot03 {
    fn bind(&self) -> u32 {
        0
    }
}
impl UniformSampler for SamplerSlot03 {
    const ASK_SLOT_COUNT: u8 = 3;
    fn new(desc: &SamplerDesc, device: &RenderDevice, pool: &mut SamplerPool) -> Self {
        pool.create(desc, device);
        let sampler = pool.get(SamplerPool::cacl_key(desc)).unwrap();
        Self {
            sampler
        }
    }
    fn sampler(&self) -> &Sampler {
        &self.sampler
    }
}

// ==== ==== ==== ==== 4

#[derive(Debug, Deref, DerefMut, Clone, Default, Hash)]
pub struct TextureSlot04(pub ImageAssetKey);
impl UniformBind for TextureSlot04 {
    fn bind(&self) -> u32 {
        3
    }
}
impl ValueTextureKey for TextureSlot04 {
    fn new(path: ImageAssetKey) -> Self {
        Self(path)
    }
    fn key(&self) -> &ImageAssetKey {
        &self.0
    }
}
impl SlotActiveRequire for TextureSlot04 {
    const ASK_SLOT_COUNT: u8 = 4;
}


#[derive(Deref, DerefMut)]
pub struct TextureResSlot04(pub Handle<TextureRes>);
impl From<Handle<TextureRes>> for TextureResSlot04 {
    fn from(h: Handle<TextureRes>) -> Self { Self(h) }
}
impl UniformTexture for TextureResSlot04 {
    fn texture(&self) -> &TextureRes {
        &self.0
    }
}

pub struct SamplerSlot04 {
    pub sampler: Sampler,
}
impl UniformBind for SamplerSlot04 {
    fn bind(&self) -> u32 {
        0
    }
}
impl UniformSampler for SamplerSlot04 {
    const ASK_SLOT_COUNT: u8 = 4;
    fn new(desc: &SamplerDesc, device: &RenderDevice, pool: &mut SamplerPool) -> Self {
        pool.create(desc, device);
        let sampler = pool.get(SamplerPool::cacl_key(desc)).unwrap();
        Self {
            sampler
        }
    }
    fn sampler(&self) -> &Sampler {
        &self.sampler
    }
}


