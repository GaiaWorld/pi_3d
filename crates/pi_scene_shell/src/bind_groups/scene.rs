use std::{hash::Hash, sync::Arc};

use pi_render::{
    renderer::{
        bind_group::*,
        bind::*,
        shader::{TShaderSetBlock, TShaderBindCode}
    },
    asset::TAssetKeyU64
};
use crate::{binds::*, prelude::{EqAsResource, HashAsResource}};

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct KeyShaderSetScene {
    // pub base_effect: bool,
    // pub brdf: bool,
    // pub env: bool,
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct KeyBindGroupScene {
    pub bind_viewer: Option<Arc<ShaderBindViewer>>,
    pub bind_base_effect: Option<Arc<ShaderBindSceneAboutEffect>>,
    // pub bind_brdf: Option<(BindUseBRDFTexture, BindUseBRDFSampler)>,
    pub key_set: KeyShaderSetScene,
    bind_count: u32,
    key_bindgroup: KeyBindGroup,
}
impl KeyBindGroupScene {
    pub fn new(
        bind_viewer: Option<Arc<ShaderBindViewer>>,
        bind_base_effect: Option<Arc<ShaderBindSceneAboutEffect>>,
    ) -> Self {
        let key_set = KeyShaderSetScene::default();
        let mut key_bindgroup = KeyBindGroup::default();

        let mut binding = 0;
    
        if let Some(bind) = &bind_viewer {
            if let Some(key) = bind.key_bind() {
                key_bindgroup.0.push(key);
                binding += 1;
            }
        }

        if let Some(bind) = &bind_base_effect {
            if let Some(key) = bind.key_bind() {
                key_bindgroup.0.push(key);
                binding += 1;
            }
        };

        let result = Self {
            bind_viewer,
            bind_base_effect,
            // bind_brdf,
            key_set,
            bind_count: binding,
            key_bindgroup,
        };

        result
    }
    pub fn key_bind_group(&self) -> KeyBindGroup {
        self.key_bindgroup.clone()
    }
    pub fn key_bind_group_layout(&self) -> KeyBindGroupLayout {
        self.key_bindgroup.key_bind_group_layout()
    }
}
impl TShaderSetBlock for KeyBindGroupScene {
    fn vs_define_code(&self, set: u32) -> String {
        let mut result = String::from("");
        let mut bind = 0;

        if let Some(item) = &self.bind_viewer {
            result += item.vs_define_code(set, bind).as_str();
            bind += 1;
        }

        if let Some(item) = &self.bind_base_effect {
            result += item.vs_define_code(set, bind).as_str();
            // bind += 1;
        }

        result
    }

    fn fs_define_code(&self, set: u32) -> String {
        let mut result = String::from("");
        let mut bind = 0;

        if let Some(item) = &self.bind_viewer {
            result += item.fs_define_code(set, bind).as_str();
            bind += 1;
        }

        if let Some(item) = &self.bind_base_effect {
            result += item.fs_define_code(set, bind).as_str();
            // bind += 1;
        }

        result
    }
}
impl TAssetKeyU64 for KeyBindGroupScene {}

#[derive(Clone)]
pub struct BindGroupScene {
    pub(crate) bind_group: BindGroupUsage,
    pub(crate) key: KeyBindGroupScene,
}
impl BindGroupScene {
    pub fn new(
        bind_group: BindGroupUsage,
        key: KeyBindGroupScene,
    ) -> Self {
        Self { bind_group, key }
    }
    pub fn key(&self) -> &KeyBindGroupScene { &self.key }
    pub fn bind_group(&self) -> &BindGroupUsage { &self.bind_group }
}
impl TShaderSetBlock for BindGroupScene {
    fn fs_define_code(&self, set: u32) -> String {
        self.key.fs_define_code(set)
    }

    fn vs_define_code(&self, set: u32) -> String {
        self.key.vs_define_code(set)
    }
}
impl EqAsResource for BindGroupScene {
    fn eq_resource(&self, other: &Self) -> bool {
        self.bind_group.key() == other.bind_group.key() && self.key == other.key
    }
}
impl HashAsResource for BindGroupScene {
    fn hash_resource<H: std::hash::Hasher>(&self, state: &mut H) {
        self.bind_group.key().asset_u64().hash(state);
    }
}
