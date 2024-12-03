

use std::hash::Hash;

use pi_assets::asset::Handle;

use pi_render::{
    asset::TAssetKeyU64, renderer::{
        bind::TKeyBind,
        bind_group::*, sampler::BindDataSampler, shader::TShaderSetBlock, 
        shader_stage::EShaderStage,
        texture::{BindDataTexture2D, ETextureViewUsage}
    }
};
use crate::{binds::*, prelude::{EngineCustomPlugins, EqAsResource, HashAsResource}, shader::* };


pub type KeyShaderSetTextureSamplers = u64;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct EffectTextureSampler(pub ETextureViewUsage, pub BindDataSampler, pub EShaderStage, pub wgpu::TextureSampleType, pub wgpu::TextureViewDimension, pub wgpu::SamplerBindingType);
impl EffectTextureSampler {
    pub fn tilloff(&self) -> [f32;4] {
        match &self.0 {
            ETextureViewUsage::Tex(arc) => [1., 1., 0., 0.],
            ETextureViewUsage::TexWithId(arc) => [1., 1., 0., 0.],
            ETextureViewUsage::Image(arc) => [1., 1., 0., 0.],
            ETextureViewUsage::ImageFrame(arc) => {
                arc.texture().tilloff()
            },
            ETextureViewUsage::SRT(arc) => [1., 1., 0., 0.],
        }
    }
    pub fn coord(&self) -> u32 {
        match &self.0 {
            ETextureViewUsage::Tex(arc) => 0,
            ETextureViewUsage::TexWithId(arc) => 0,
            ETextureViewUsage::Image(arc) => 0,
            ETextureViewUsage::ImageFrame(arc) => {
                arc.texture().coord()
            },
            ETextureViewUsage::SRT(arc) => 0,
        }
    }
}

#[derive(Default, Clone, Hash, PartialEq, Eq)]
pub struct EffectTextureSamplers {
    pub textures: Vec<EffectTextureSampler>,
}

#[derive(Clone)]
pub struct KeyBindGroupTextureSamplers(KeyBindGroup, EffectTextureSamplers);
impl KeyBindGroupTextureSamplers {
    pub fn new(
        effect_texture_samplers: EffectTextureSamplers,
        // runtimedesc: &Vec<RuntimeUniformTexture2DDesc>,
    ) -> Option<Self> {
        let mut key_binds = Vec::with_capacity(8);
        let count = effect_texture_samplers.textures.len();

        for idx in 0..count {
            let EffectTextureSampler(tex, sampler, visibility, texture_sample_type, view_dimension, binding_type) = &effect_texture_samplers.textures[idx];

            let key_samp = sampler_key_bind(sampler.clone(), *visibility, *binding_type);
            key_binds.push(key_samp);

            let key_tex = texture_key_bind(BindDataTexture2D(tex.clone()), *visibility, *texture_sample_type, *view_dimension);
            key_binds.push(key_tex);
        }

        if count > 0 {
            let key_bindgroup = KeyBindGroup::new(key_binds);
            Some(Self(key_bindgroup, effect_texture_samplers))
        } else {
            None
        }
    }
    pub fn key_bind_group(&self) -> KeyBindGroup {
        self.0.clone()
    }
    pub fn key_bind_group_layout(&self) -> KeyBindGroupLayout {
        self.0.key_bind_group_layout()
    }
}
impl Hash for KeyBindGroupTextureSamplers {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl PartialEq for KeyBindGroupTextureSamplers {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Eq for KeyBindGroupTextureSamplers {
    fn assert_receiver_is_total_eq(&self) {}
}
impl TAssetKeyU64 for KeyBindGroupTextureSamplers {}

#[derive(Clone)]
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

impl BindGroupTextureSamplers {
    pub fn vs_define_code(&self, set: u32, meta: &ShaderEffectMeta, engineopt: &EngineCustomPlugins) -> String {
        let mut binding = 0;

        let mut result = String::from("");

        let count = self.key.1.textures.len();
        for idx in 0..count {
            let item = &self.key.1.textures[idx];
            let key = &meta.textures[idx];
            let slotname = key.slotname.as_str();
            if  item.2.mode() & wgpu::ShaderStages::VERTEX == wgpu::ShaderStages::VERTEX {
                result += sampler_bind_code(slotname, item.5, set, binding).as_str(); binding += 1;
                result += texture_bind_code_mat(&engineopt, &item.3, item.4, slotname, slotname, set, binding).as_str(); binding += 1;
            }
        }

        result
    }

    pub fn fs_define_code(&self, set: u32, meta: &ShaderEffectMeta, engineopt: &EngineCustomPlugins) -> String {
        let mut binding = 0;

        let mut result = String::from("");

        let count = self.key.1.textures.len();
        for idx in 0..count {
            let item = &self.key.1.textures[idx];
            let key = &meta.textures[idx];
            let slotname = key.slotname.as_str();
            if  item.2.mode() & wgpu::ShaderStages::FRAGMENT == wgpu::ShaderStages::FRAGMENT {
                result += sampler_bind_code(slotname, item.5, set, binding).as_str(); binding += 1;
                result += texture_bind_code_mat(&engineopt, &item.3, item.4, slotname, slotname, set, binding).as_str(); binding += 1;
            }
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

impl EqAsResource for BindGroupTextureSamplers {
    fn eq_resource(&self, other: &Self) -> bool {
        self.bind_group.key() == other.bind_group.key() && self.key == other.key
    }
}
impl HashAsResource for BindGroupTextureSamplers {
    fn hash_resource<H: std::hash::Hasher>(&self, state: &mut H) {
        self.bind_group.key().asset_u64().hash(state);
    }
}