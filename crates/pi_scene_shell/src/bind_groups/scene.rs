use std::hash::Hash;

use pi_render::{
    renderer::{
        bind_group::*,
        bind::*,
        shader::{TShaderSetBlock, TShaderBindCode}
    },
    asset::TAssetKeyU64
};
use crate::{binds::*, forward_rendering::*, prelude::{EqAsResource, HashAsResource}, shader::{ShaderVarUniform, TBindGroupHashForShader}};

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct KeyShaderSetScene {
    pub lighting_enable: bool,
    pub shadow_enable: bool,
    // pub base_effect: bool,
    // pub brdf: bool,
    // pub env: bool,
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct KeyBindGroupScene {
    pub bind_viewer: Option<ShaderBindViewer>,
    pub bind_passindex: ShaderBindPassIndex,
    pub bind_base_effect: Option<ShaderBindSceneAboutEffect>,
    // pub bind_brdf: Option<(BindUseBRDFTexture, BindUseBRDFSampler)>,
    pub lighting: Option<ShaderBindSceneLightInfos>,
    pub shadowmap: Option<(ShaderBindShadowData, ShaderBindShadowTexture, ShaderBindShadowSampler)>,
    pub bind_brdf: Option<(ShaderBindBRDFTexture, ShaderBindBRDFSampler)>,
    pub camera_opaque: Option<(ShaderBindMainCameraOpaqueTexture, ShaderBindMainCameraOpaqueSampler)>,
    pub camera_depth: Option<(ShaderBindMainCameraDepthTexture, ShaderBindMainCameraDepthSampler)>,
    pub env: Option<(BindEnvIrradiance, ShaderBindEnvTexture, ShaderBindEnvSampler)>,
    pub key_set: KeyShaderSetScene,
    bind_count: u32,
    key_bindgroup: KeyBindGroup,
}
impl KeyBindGroupScene {
    pub fn new(
        bind_viewer: Option<ShaderBindViewer>,
        bind_passindex: ShaderBindPassIndex,
        bind_base_effect: Option<ShaderBindSceneAboutEffect>,
        lighting: Option<ShaderBindSceneLightInfos>,
        shadowmap: Option<(ShaderBindShadowData, ShaderBindShadowTexture, ShaderBindShadowSampler)>,
        bind_brdf: Option<(ShaderBindBRDFTexture, ShaderBindBRDFSampler)>,
        camera_opaque: Option<(ShaderBindMainCameraOpaqueTexture, ShaderBindMainCameraOpaqueSampler)>,
        camera_depth: Option<(ShaderBindMainCameraDepthTexture, ShaderBindMainCameraDepthSampler)>,
        env: Option<(BindEnvIrradiance, ShaderBindEnvTexture, ShaderBindEnvSampler)>,
    ) -> Self {
        let mut lighting_enable: bool = false;
        let mut shadow_enable: bool = false;

        // let key_set = KeyShaderSetScene::default();
        let mut key_binds = Vec::with_capacity(4);

        let mut binding = 0;

        if let Some(bind) = &bind_viewer {
            if let Some(key) = bind.key_bind() {
                key_binds.push(key);
                binding += 1;
            }
        }

        // if let Some(bind) = &bind_passindex {
        //     if let Some(key) = bind.key_bind() {
        //         key_binds.push(key);
        //         binding += 1;
        //     }
        // }
        if let Some(key) = bind_passindex.key_bind() {
            key_binds.push(key);
            binding += 1;
        }

        if let Some(bind) = &bind_base_effect {
            if let Some(key) = bind.key_bind() {
                key_binds.push(key);
                binding += 1;
            }
        };
        
        if let Some(bind) = &lighting {
            if let Some(key) = bind.key_bind() {
                key_binds.push(key);
                binding += 1;
                lighting_enable = true;
            }
        }

        if let Some((v0, v1, v2)) = &shadowmap {
            if let (Some(key0), Some(key1), Some(key2)) = (v0.key_bind(), v1.key_bind(), v2.key_bind()) {
                key_binds.push(key0); key_binds.push(key1); key_binds.push(key2);
                binding += 3;
                shadow_enable = true;
            }
        }
        
        if let Some((v0, v1)) = &bind_brdf {
            if let (Some(key0), Some(key1)) = (v0.key_bind(), v1.key_bind()) {
                key_binds.push(key0); key_binds.push(key1);
                binding += 2;
            }
        }
        
        if let Some((v0, v1)) = &camera_opaque {
            if let (Some(key0), Some(key1)) = (v0.key_bind(), v1.key_bind()) {
                key_binds.push(key0); key_binds.push(key1);
                binding += 2;
            }
        }

        if let Some((v0, v1)) = &camera_depth {
            if let (Some(key0), Some(key1)) = (v0.key_bind(), v1.key_bind()) {
                key_binds.push(key0); key_binds.push(key1); 
                binding += 2;
            }
        }
        
        if let Some((v0, v1, v2)) = &env {
            if let (Some(key0), Some(key1), Some(key2)) = (v0.key_bind(), v1.key_bind(), v2.key_bind()) {
                key_binds.push(key0); key_binds.push(key1); key_binds.push(key2);
                binding += 3;
            }
        }

        let result = Self {
            lighting,
            bind_passindex,
            shadowmap,
            bind_brdf,
            camera_opaque,
            camera_depth,
            env,
            bind_viewer,
            bind_base_effect,
            // bind_brdf,
            key_set: KeyShaderSetScene { lighting_enable, shadow_enable },
            bind_count: binding,
            key_bindgroup: KeyBindGroup::new(key_binds),
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

        {
            result += self.bind_passindex.vs_define_code(set, bind).as_str();
            bind += 1;
        }

        if let Some(item) = &self.bind_base_effect {
            result += item.vs_define_code(set, bind).as_str();
            bind += 1;
        }

        if let Some(v0) = &self.lighting {
            result += v0.vs_define_code(set, bind).as_str(); // bind += 1;
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

        {
            result += self.bind_passindex.fs_define_code(set, bind).as_str();
            bind += 1;
        }

        if let Some(item) = &self.bind_base_effect {
            result += item.fs_define_code(set, bind).as_str();
            bind += 1;
        }
        
        if let Some(v0) = &self.lighting {
            result += v0.fs_define_code(set, bind).as_str(); bind += 1;
        }
        if let Some((v0, v1, v2)) = &self.shadowmap {
            result += v0.fs_define_code(set, bind).as_str(); bind += 1;
            result += v1.fs_define_code(set, bind).as_str(); bind += 1;
            result += v2.fs_define_code(set, bind).as_str(); bind += 1;
        }
        if let Some((bind1, bind2)) = &self.bind_brdf {
            result += bind1.fs_define_code(set, bind).as_str(); bind += 1;
            result += bind2.fs_define_code(set, bind).as_str(); bind += 1;
            result += "
vec4 GetEnvironmentBRDFTexture(vec2 uv) {
    return texture(sampler2D(";
    result += ShaderVarUniform::BRDF_TEXUTRE;
    result += ", sampler";
    result += ShaderVarUniform::BRDF_TEXUTRE;
    result += "), uv);
}
";
        }

        if let Some((v0, v1)) = &self.camera_opaque {
            result += v0.fs_define_code(set, bind).as_str(); bind += 1;
            result += v1.fs_define_code(set, bind).as_str(); bind += 1;
        }

        if let Some((v0, v1)) = &self.camera_depth {
            result += v0.fs_define_code(set, bind).as_str(); bind += 1;
            result += v1.fs_define_code(set, bind).as_str(); bind += 1;
        }
        if let Some((bind1, bind2, bind3)) = &self.env {
            result += bind1.fs_define_code(set, bind).as_str(); bind += 1;
            result += bind2.fs_define_code(set, bind).as_str(); bind += 1;
            result += bind3.fs_define_code(set, bind).as_str(); // bind += 1;
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
impl TBindGroupHashForShader for BindGroupScene {
    fn hash_for_shader<H: std::hash::Hasher>(&self, state: &mut H) {
        // todo!()
    }
}