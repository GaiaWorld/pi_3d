

use std::hash::Hash;

use pi_assets::asset::Handle;

use pi_render::{
    renderer::{
        bind_group::*,
        shader::TShaderSetBlock,
        texture::{ETextureViewUsage, BindDataTexture2D},
        sampler::BindDataSampler,
    },
    asset::TAssetKeyU64
};
use crate::{binds::*, shader::* };


pub type KeyShaderSetTextureSamplers = u64;


#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
pub struct EffectTextureSamplers {
    pub textures: Vec<ETextureViewUsage>,
    pub samplers: Vec<BindDataSampler>,
}

#[derive(Debug, Clone)]
pub struct KeyBindGroupTextureSamplers {
    pub meta: Handle<ShaderEffectMeta>,
    key_bindgroup: KeyBindGroup,
    count: usize,
}
impl KeyBindGroupTextureSamplers {
    pub fn new(
        effect_texture_samplers: EffectTextureSamplers,
        meta: Handle<ShaderEffectMeta>,
    ) -> Option<Self> {
        let mut key_bindgroup = KeyBindGroup::default();
        let count = effect_texture_samplers.textures.len();
        
        let mut error = false;
        for idx in 0..count {
            if error == false {
                let tex = &effect_texture_samplers.textures[idx];
                if let Some(key) = texture_key_bind(BindDataTexture2D(tex.clone()), idx, &meta) {
                    key_bindgroup.0.push(key);
                } else { error = true; }
            }
        }
        
        for idx in 0..count {
            if error == false {
                let val = &effect_texture_samplers.samplers[idx];
                if let Some(key) = sampler_key_bind(val.clone(), idx, &meta) {
                    key_bindgroup.0.push(key);
                } else { error = true; }
            }
        }

        if error == false && count > 0 {
            Some(Self { meta, key_bindgroup, count })
        } else {
            None
        }
    }
    pub fn key_bind_group(&self) -> KeyBindGroup {
        self.key_bindgroup.clone()
    }
    pub fn key_bind_group_layout(&self) -> KeyBindGroupLayout {
        self.key_bindgroup.key_bind_group_layout()
    }
}
impl Hash for KeyBindGroupTextureSamplers {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key_bindgroup.hash(state);
        self.meta.key().hash(state);
    }
}
impl PartialEq for KeyBindGroupTextureSamplers {
    fn eq(&self, other: &Self) -> bool {
        self.key_bindgroup == other.key_bindgroup && self.meta.key() == other.meta.key()
    }
}
impl Eq for KeyBindGroupTextureSamplers {
    fn assert_receiver_is_total_eq(&self) {}
}
impl TAssetKeyU64 for KeyBindGroupTextureSamplers {}

#[derive(Debug, Clone)]
pub struct BindGroupTextureSamplers {
    pub(crate) bind_group: BindGroupUsage,
    pub(crate) key: KeyBindGroupTextureSamplers,
}

impl BindGroupTextureSamplers {
    pub fn new(
        key: KeyBindGroupTextureSamplers,
        bind_group: BindGroupUsage,
    ) -> Self {
        Self { bind_group, key }
    }
    pub fn key(&self) -> &KeyBindGroupTextureSamplers { &self.key }
    pub fn bind_group(&self) -> &BindGroupUsage { &self.bind_group }
}

impl TShaderSetBlock for BindGroupTextureSamplers {
    fn vs_define_code(&self, set: u32) -> String {
        let mut binding = 0;

        let mut result = String::from("");

        for idx in 0..self.key.count {
            result += vs_define_texture(&self.key.meta, idx, set, binding).as_str(); binding += 1;
        }
        for idx in 0..self.key.count {
            result += vs_define_sampler(&self.key.meta, idx, set, binding).as_str(); binding += 1;
        }

        result
    }

    fn fs_define_code(&self, set: u32) -> String {
        let mut binding = 0;

        let mut result = String::from("");

        for idx in 0..self.key.count {
            result += fs_define_texture(&self.key.meta, idx, set, binding).as_str(); binding += 1;
        }
        for idx in 0..self.key.count {
            result += fs_define_sampler(&self.key.meta, idx, set, binding).as_str(); binding += 1;
        }

        result
    }

    // fn vs_running_code(&self) -> String {
    //     String::from("")
    // }

    // fn fs_running_code(&self) -> String {
    //     String::from("")
    // }
}
