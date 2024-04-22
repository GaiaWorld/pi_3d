use std::{hash::Hash, sync::Arc};

use pi_render::{
    renderer::{
        bind_group::*,
        shader::{TShaderSetBlock, TShaderBindCode},
        bind::*
    },
    asset::TAssetKeyU64
};
use crate::{binds::*, prelude::{BindModelLightIndexs, EqAsResource, HashAsResource}, shader::* };

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct KeyShaderSetModel {
    pub skin: ESkinCode,
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct KeyBindGroupModel {
    pub matrix: Option<Arc<ShaderBindModelAboutMatrix>>,
    pub skin: Option<Arc<ShaderBindModelAboutSkinValue>>,
    pub effect_value: Option<Arc<ShaderBindEffectValue>>,
    pub lightingidxs: Option<Arc<BindModelLightIndexs>>,
    pub key: KeyShaderSetModel,
    bind_count: u32,
    key_bindgroup: KeyBindGroup,
}
impl KeyBindGroupModel {
    pub fn new(
        matrix: Option<Arc<ShaderBindModelAboutMatrix>>,
        skin: Option<Arc<ShaderBindModelAboutSkinValue>>,
        effect_value: Option<Arc<ShaderBindEffectValue>>,
        lightingidxs: Option<Arc<BindModelLightIndexs>>,
    ) -> Self {
        let mut key = KeyShaderSetModel::default();
        let mut key_bindgroup = KeyBindGroup::default();

        let mut binding = 0;

        if let Some(bind) = &matrix {
            if let Some(key) = bind.key_bind() {
                key_bindgroup.0.push(key);
                binding += 1;
            }
        }

        if let Some(bind) = &skin {
            key.skin = bind.skin;
            if let Some(key) = bind.key_bind() {
                key_bindgroup.0.push(key);
                binding += 1;
            }
        }

        if let Some(bind) = &effect_value {
            if let Some(key) = bind.key_bind() {
                key_bindgroup.0.push(key);
                binding += 1;
            }
        }

        if let Some(bind) = &lightingidxs {
            if let Some(key) = bind.key_bind() {
                key_bindgroup.0.push(key);
                binding += 1;
            }
        }

        let result = Self {
            matrix,
            skin,
            effect_value,
            lightingidxs,
            key,
            bind_count: binding,
            key_bindgroup
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
impl TShaderSetBlock for KeyBindGroupModel {
    fn vs_define_code(&self, set: u32) -> String {

        let mut result = String::from("");
        let mut bind = 0;
        if let Some(item) = &self.matrix {
            result += item.vs_define_code(set, bind).as_str();
            bind += 1;
        }

        if let Some(item) = &self.skin {
            result += item.vs_define_code(set, bind).as_str();
            bind += 1;
        }

        if let Some(item) = &self.effect_value {
            result += item.vs_define_code(set, bind).as_str();
            bind += 1;
        }

        if let Some(item) = &self.lightingidxs {
            result += item.vs_define_code(set, bind).as_str();
            // bind += 1;
        }

        result
    }

    fn fs_define_code(&self, set: u32) -> String {
        let mut result = String::from("");
        let mut bind = 0;

        if let Some(item) = &self.matrix {
            result += item.fs_define_code(set, bind).as_str();
            bind += 1;
        }
        if let Some(item) = &self.skin {
            result += item.fs_define_code(set, bind).as_str();
            bind += 1;
        }
        if let Some(item) = &self.effect_value {
            result += item.fs_define_code(set, bind).as_str();
            bind += 1;
        }
        if let Some(item) = &self.lightingidxs {
            result += item.fs_define_code(set, bind).as_str();
            // bind += 1;
        }

        result
    }
}
impl TAssetKeyU64 for KeyBindGroupModel {}

#[derive(Clone)]
pub struct BindGroupModel {
    pub(crate) bind_group: BindGroupUsage,
    pub(crate) key: KeyBindGroupModel,
}
impl BindGroupModel {
    pub fn new(
        bind_group: BindGroupUsage,
        key: KeyBindGroupModel,
    ) -> Self {
        Self { bind_group, key }
    }
    pub fn key(&self) -> &KeyBindGroupModel { &self.key }
    pub fn bind_group(&self) -> &BindGroupUsage { &self.bind_group }
    pub fn vs_running_model_snippet(&self, meta: &ShaderEffectMeta) -> String {
        let mut result = String::from("");
        if self.key.matrix.is_some() {
            result += "
    mat4 PI_ObjectToWorld = U_PI_ObjectToWorld;
    vec4 PI_ObjectVelocity = U_PI_ObjectVelocity;
    uint PI_SkinBoneOffset0 = U_PI_SkinBoneOffset0;
    uint PI_SkinBoneOffset1 = U_PI_SkinBoneOffset1;
";
        }
        if self.key.effect_value.is_some() {
            result += meta.uniforms.vs_running_code().as_str();
        }
        result
    }
}
impl TShaderSetBlock for BindGroupModel {
    fn fs_define_code(&self, set: u32) -> String {
        self.key.fs_define_code(set)
    }

    fn vs_define_code(&self, set: u32) -> String {
        self.key.vs_define_code(set)
    }
}

impl EqAsResource for BindGroupModel {
    fn eq_resource(&self, other: &Self) -> bool {
        self.bind_group.key() == other.bind_group.key() && self.key == other.key
    }
}
impl HashAsResource for BindGroupModel {
    fn hash_resource<H: std::hash::Hasher>(&self, state: &mut H) {
        self.bind_group.key().asset_u64().hash(state);
    }
}
