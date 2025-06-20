use std::hash::Hash;

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
    // pub matidx: ShaderBindModelMatIdx,
    pub matrix: Option<ShaderBindModelAbout>,
    pub skin: Option<ShaderBindModelAboutSkinValue>,
    // pub matrixinv: Option<ShaderBindModelMatrixInv>,
    // pub morphinfluence: Option<ShaderBindModelMorphinfluence>,
    // pub skinoffset: Option<ShaderBindModelSkinOffset>,
    // pub velocity: Option<ShaderBindModelVelocity>,
    pub lightingidxs: Option<BindModelLightIndexs>,
    pub key: KeyShaderSetModel,
    bind_count: u32,
    key_bindgroup: KeyBindGroup,
}
impl KeyBindGroupModel {
    pub fn new(
        // matidx: ShaderBindModelMatIdx,
        matrix: Option<ShaderBindModelAbout>,
        // matrixinv: Option<ShaderBindModelMatrixInv>,
        // morphinfluence: Option<ShaderBindModelMorphinfluence>,
        // skinoffset: Option<ShaderBindModelSkinOffset>,
        // velocity: Option<ShaderBindModelVelocity>,
        skin: Option<ShaderBindModelAboutSkinValue>,
        lightingidxs: Option<BindModelLightIndexs>,
    ) -> Self {
        let mut key = KeyShaderSetModel::default();
        let mut key_binds = Vec::with_capacity(4);

        let mut binding = 0;

        // {
        //     if let Some(key) = matidx.key_bind() {
        //         key_binds.push(key);
        //         binding += 1;
        //     }
        // }

        if let Some(bind) = &matrix {
            if let Some(key) = bind.key_bind() {
                key_binds.push(key);
                binding += 1;
            }
        }

        // if let Some(bind) = &matrixinv {
        //     if let Some(key) = bind.key_bind() {
        //         key_binds.push(key);
        //         binding += 1;
        //     }
        // }
        
        // if let Some(bind) = &morphinfluence {
        //     if let Some(key) = bind.key_bind() {
        //         key_binds.push(key);
        //         binding += 1;
        //     }
        // }
        
        // if let Some(bind) = &skinoffset {
        //     if let Some(key) = bind.key_bind() {
        //         key_binds.push(key);
        //         binding += 1;
        //     }
        // }
        
        // if let Some(bind) = &velocity {
        //     if let Some(key) = bind.key_bind() {
        //         key_binds.push(key);
        //         binding += 1;
        //     }
        // }
        
        if let Some(bind) = &skin {
            key.skin = bind.skin;
            if let Some(key) = bind.key_bind() {
                key_binds.push(key);
                binding += 1;
            }
        }

        if let Some(bind) = &lightingidxs {
            if let Some(key) = bind.key_bind() {
                key_binds.push(key);
                binding += 1;
            }
        }

        let result = Self {
            // matidx,
            matrix,
            // matrixinv,
            // skinoffset,
            // morphinfluence,
            // velocity,
            skin,
            lightingidxs,
            key,
            bind_count: binding,
            key_bindgroup: KeyBindGroup::new(key_binds)
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

        // {
        //     result += self.matidx.vs_define_code(set, bind).as_str();
        //     bind += 1;
        // }
        if let Some(item) = &self.matrix {
            result += item.vs_define_code(set, bind).as_str();
            bind += 1;
        }
        // if let Some(item) = &self.matrixinv {
        //     result += item.vs_define_code(set, bind).as_str();
        //     bind += 1;
        // }
        // if let Some(item) = &self.morphinfluence {
        //     result += item.vs_define_code(set, bind).as_str();
        //     bind += 1;
        // }
        // if let Some(item) = &self.skinoffset {
        //     result += item.vs_define_code(set, bind).as_str();
        //     bind += 1;
        // }
        // if let Some(item) = &self.velocity {
        //     result += item.vs_define_code(set, bind).as_str();
        //     bind += 1;
        // }

        if let Some(item) = &self.skin {
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

        // {
        //     result += self.matidx.fs_define_code(set, bind).as_str();
        //     bind += 1;
        // }

        if let Some(item) = &self.matrix {
            result += item.fs_define_code(set, bind).as_str();
            bind += 1;
        }
        // if let Some(item) = &self.matrixinv {
        //     result += item.fs_define_code(set, bind).as_str();
        //     bind += 1;
        // }
        // if let Some(item) = &self.morphinfluence {
        //     result += item.fs_define_code(set, bind).as_str();
        //     bind += 1;
        // }
        // if let Some(item) = &self.skinoffset {
        //     result += item.fs_define_code(set, bind).as_str();
        //     bind += 1;
        // }
        // if let Some(item) = &self.velocity {
        //     result += item.fs_define_code(set, bind).as_str();
        //     bind += 1;
        // }
        if let Some(item) = &self.skin {
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
    pub fn vs_running_model_snippet(&self, _meta: &ShaderEffectMeta) -> String {
        let mut result = String::from("");

        if self.key.matrix.is_some() {
            result += "
    mat4 PI_ObjectToWorld = U_PI_ObjectToWorld;
    uvec4 PI_SkinBoneOffset = U_PI_SkinBoneOffset;
    vec4 PI_ObjectVelocity = U_PI_ObjectVelocity;
";
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
impl TBindGroupHashForShader for BindGroupModel {
    fn hash_for_shader<H: std::hash::Hasher>(&self, _state: &mut H) {
        // todo!()
    }
}