use derive_deref::{DerefMut, Deref};
use pi_assets::asset::Handle;
use pi_render::rhi::{asset::TextureRes, texture::Sampler, device::RenderDevice};
use pi_share::ThreadSync;
use render_resource::{ImageAssetKey, sampler::{SamplerDesc, SamplerPool}};

use crate::value::{UniformBind, SlotActiveRequire};

pub trait ValueTextureKey: ThreadSync + 'static {
    fn new(path: ImageAssetKey) -> Self;
    fn key(&self) -> &ImageAssetKey;
}

pub trait UniformTexture {
    fn texture(&self) -> &TextureRes;
}

pub trait UniformSampler: ThreadSync + 'static {
    fn new(desc: &SamplerDesc, device: &RenderDevice, pool: &mut SamplerPool) -> Self;
    fn sampler(&self) -> &Sampler;
}

// ==== ==== ==== ==== 1
#[derive(Debug, Deref, DerefMut, Clone, Default, Hash)]
pub struct TextureSlot1(pub ImageAssetKey);
impl UniformBind for TextureSlot1 {
    fn bind(&self) -> u32 {
        0
    }
}
impl ValueTextureKey for TextureSlot1 {
    fn new(path: ImageAssetKey) -> Self {
        Self(path)
    }
    fn key(&self) -> &ImageAssetKey {
        &self.0
    }
}
impl SlotActiveRequire for TextureSlot1 {
    const ASK_SLOT_COUNT: u8 = 1;
}

#[derive(Deref, DerefMut)]
pub struct TextureResSlot1(pub Handle<TextureRes>);
impl From<Handle<TextureRes>> for TextureResSlot1 {
    fn from(h: Handle<TextureRes>) -> Self { Self(h) }
}
impl UniformTexture for TextureResSlot1 {
    fn texture(&self) -> &TextureRes {
        &self.0
    }
}

pub struct SamplerSlot1 {
    pub sampler: Sampler,
}
impl UniformBind for SamplerSlot1 {
    fn bind(&self) -> u32 {
        0
    }
}
impl UniformSampler for SamplerSlot1 {
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
pub struct TextureSlot2(pub ImageAssetKey);
impl UniformBind for TextureSlot2 {
    fn bind(&self) -> u32 {
        1
    }
}
impl ValueTextureKey for TextureSlot2 {
    fn new(path: ImageAssetKey) -> Self {
        Self(path)
    }
    fn key(&self) -> &ImageAssetKey {
        &self.0
    }
}
impl SlotActiveRequire for TextureSlot2 {
    const ASK_SLOT_COUNT: u8 = 2;
}

#[derive(Deref, DerefMut)]
pub struct TextureResSlot2(pub Handle<TextureRes>);
impl From<Handle<TextureRes>> for TextureResSlot2 {
    fn from(h: Handle<TextureRes>) -> Self { Self(h) }
}
impl UniformTexture for TextureResSlot2 {
    fn texture(&self) -> &TextureRes {
        &self.0
    }
}

pub struct SamplerSlot2 {
    pub sampler: Sampler,
}
impl UniformBind for SamplerSlot2 {
    fn bind(&self) -> u32 {
        0
    }
}
impl UniformSampler for SamplerSlot2 {
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
pub struct TextureSlot3(pub ImageAssetKey);
impl UniformBind for TextureSlot3 {
    fn bind(&self) -> u32 {
        2
    }
}
impl ValueTextureKey for TextureSlot3 {
    fn new(path: ImageAssetKey) -> Self {
        Self(path)
    }
    fn key(&self) -> &ImageAssetKey {
        &self.0
    }
}
impl SlotActiveRequire for TextureSlot3 {
    const ASK_SLOT_COUNT: u8 = 3;
}

#[derive(Deref, DerefMut)]
pub struct TextureResSlot3(pub Handle<TextureRes>);
impl From<Handle<TextureRes>> for TextureResSlot3 {
    fn from(h: Handle<TextureRes>) -> Self { Self(h) }
}
impl UniformTexture for TextureResSlot3 {
    fn texture(&self) -> &TextureRes {
        &self.0
    }
}

pub struct SamplerSlot3 {
    pub sampler: Sampler,
}
impl UniformBind for SamplerSlot3 {
    fn bind(&self) -> u32 {
        0
    }
}
impl UniformSampler for SamplerSlot3 {
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
pub struct TextureSlot4(pub ImageAssetKey);
impl UniformBind for TextureSlot4 {
    fn bind(&self) -> u32 {
        3
    }
}
impl ValueTextureKey for TextureSlot4 {
    fn new(path: ImageAssetKey) -> Self {
        Self(path)
    }
    fn key(&self) -> &ImageAssetKey {
        &self.0
    }
}
impl SlotActiveRequire for TextureSlot4 {
    const ASK_SLOT_COUNT: u8 = 4;
}


#[derive(Deref, DerefMut)]
pub struct TextureResSlot4(pub Handle<TextureRes>);
impl From<Handle<TextureRes>> for TextureResSlot4 {
    fn from(h: Handle<TextureRes>) -> Self { Self(h) }
}
impl UniformTexture for TextureResSlot4 {
    fn texture(&self) -> &TextureRes {
        &self.0
    }
}

pub struct SamplerSlot4 {
    pub sampler: Sampler,
}
impl UniformBind for SamplerSlot4 {
    fn bind(&self) -> u32 {
        0
    }
}
impl UniformSampler for SamplerSlot4 {
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


