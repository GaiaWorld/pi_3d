

use std::{hash::Hash, sync::Arc};

use pi_assets::asset::Handle;

use pi_render::{
    renderer::{
        bind_group::*,
        shader::TShaderSetBlock,
        bind::KeyBindLayoutBindingType,
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
    key_binds: Option<Arc<IDBinds>>,
    count: usize,
}
impl KeyBindGroupTextureSamplers {
    pub fn new(
        effect_texture_samplers: EffectTextureSamplers,
        meta: Handle<ShaderEffectMeta>,
        recorder: &mut BindsRecorder,
    ) -> Self {
        let mut count = effect_texture_samplers.textures.len();
        let idbinds = if let Some(mut binds) = EBinds::new(count as u32 * 2) {
            let mut error = false;
            let mut binding = 0;
            for idx in 0..count {
                if error == false {
                    let tex = &effect_texture_samplers.textures[idx];
                    if let Some(layout) = texture_key_bind(BindDataTexture2D(tex.clone()), idx, &meta, binding as KeyBindLayoutBindingType) {
                        binds.set(binding, Some(layout)); binding += 1;
                    } else { error = true; }
                }
            }
            
            for idx in 0..count {
                if error == false {
                    let val = &effect_texture_samplers.samplers[idx];
                    if let Some(layout) = sampler_key_bind(val.clone(), idx, &meta, binding as KeyBindLayoutBindingType) {
                        binds.set(binding, Some(layout)); binding += 1;
                    } else { error = true; }
                }
            }

            if error == false {
                Some(binds.record(recorder))
            } else {
                count = 0;
                None
            }
        } else {
            None
        };

        Self { meta, key_binds: idbinds, count }
    }
    pub fn key_bind_group(&self) -> Option<KeyBindGroup> {
        if let Some(binds) = &self.key_binds {
            Some(
                KeyBindGroup(binds.binds())
            )
        } else {
            None
        }
    }
    pub fn key_bind_group_layout(&self) -> Option<KeyBindGroupLayout> {
        if let Some(binds) = &self.key_binds {
            Some(
                KeyBindGroup(binds.binds())
            )
        } else {
            None
        }
    }
    pub fn binds(&self) -> Option<Arc<IDBinds>> {
        self.key_binds.clone()
    }
}
impl Hash for KeyBindGroupTextureSamplers {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key_binds.hash(state);
        self.meta.key().hash(state);
    }
}
impl PartialEq for KeyBindGroupTextureSamplers {
    fn eq(&self, other: &Self) -> bool {
        self.key_binds == other.key_binds && self.meta.key() == other.meta.key()
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
