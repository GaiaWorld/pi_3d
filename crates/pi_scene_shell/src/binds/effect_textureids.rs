use pi_render::{renderer::{bind_buffer::{BindBufferAllocator, BindBufferRange}, texture::EKeyTexture}, rhi::sampler::SamplerDesc};
use pi_scene_math::Number;

use crate::{assets::texture::{TextureKeyList, TEXTURE_SLOT_COUNT}, shader::ShaderEffectMeta};

use super::{AtlasConfigs, BindAtlas, KeyAtlasFrame, TextureAtlas, TextureAtlasIdx};


#[derive(Clone, Hash, PartialEq, Eq)]
pub struct ShaderBindEffectTextureIDs {
    pub(crate) data: BindBufferRange,
    pub(crate) idxs: [TextureAtlasIdx; TEXTURE_SLOT_COUNT],
}
impl ShaderBindEffectTextureIDs {
    pub fn new(meta: &ShaderEffectMeta, keys: &TextureKeyList, config: &AtlasConfigs, pool: &mut TextureAtlas, alloc: &mut BindBufferAllocator) -> Option<Self> {
        let mut data = vec![];
        let mut idxs = [TextureAtlasIdx::zero(), TextureAtlasIdx::zero(), TextureAtlasIdx::zero(), TextureAtlasIdx::zero(), TextureAtlasIdx::zero(), TextureAtlasIdx::zero(), TextureAtlasIdx::zero(), TextureAtlasIdx::zero()];
        for idx in 0..TEXTURE_SLOT_COUNT {
            let key = &keys.0[idx];
            match &key.url {
                EKeyTexture::Tex(_k) => {
                    let id = TextureAtlasIdx::zero();
                    data.push(id.val());
                },
                EKeyTexture::Image(k) => {
                    let rect = if let Some(rect) = config.map.get(&k.url().url) {
                        rect.clone()
                    } else { [1., 1., 0., 0.] };
                    let key = KeyAtlasFrame {
                        key: key.url.clone(),
                        wrapu: key.sample.address_mode_u,
                        wrapv: key.sample.address_mode_v,
                        wrapw: key.sample.address_mode_w, 
                    };
                    let id = pool.set(&key, rect[0], rect[1], rect[2], rect[3]);
                    data.push(id.val());
                    idxs[idx] = id;
                },
                EKeyTexture::SRT(_k) => {
                    let id = TextureAtlasIdx::zero();
                    data.push(id.val());
                },
            };
        }

        if let Some(buffer) = alloc.allocate((TEXTURE_SLOT_COUNT * 4) as  wgpu::DynamicOffset) {
            buffer.write_data(0, bytemuck::cast_slice(&data));
            Some(Self { data: pool.data.clone(), idxs })
        } else {
            None
        }
        
    }
}