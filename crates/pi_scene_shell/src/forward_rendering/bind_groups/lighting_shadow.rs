
use std::{hash::Hash, sync::Arc};

use pi_render::{
    renderer::{
        shader::{TShaderSetBlock, TShaderBindCode},
        bind_group::*, bind::TKeyBind, buildin_var::ShaderVarUniform,
    },
    asset::TAssetKeyU64
};

use crate::{forward_rendering::binds::*, prelude::{EqAsResource, HashAsResource}};

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct KeyShaderSetExtend {
    pub lighting_enable: bool,
    pub shadow_enable: bool,
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct KeyBindGroupSetExtend {
    pub lighting: Option<Arc<ShaderBindSceneLightInfos>>,
    pub shadowmap: Option<(Arc<ShaderBindShadowData>, Arc<ShaderBindShadowTexture>, Arc<ShaderBindShadowSampler>)>,
    pub bind_brdf: Option<(Arc<ShaderBindBRDFTexture>, Arc<ShaderBindBRDFSampler>)>,
    pub camera_opaque: Option<(Arc<ShaderBindMainCameraOpaqueTexture>, Arc<ShaderBindMainCameraOpaqueSampler>)>,
    pub camera_depth: Option<(Arc<ShaderBindMainCameraDepthTexture>, Arc<ShaderBindMainCameraDepthSampler>)>,
    pub env: Option<(Arc<BindEnvIrradiance>, Arc<ShaderBindEnvTexture>, Arc<ShaderBindEnvSampler>)>,
    pub isactived: bool,
    bind_count: u32,
    pub key_bindgroup: KeyBindGroup,
    pub key: KeyShaderSetExtend,
}
impl KeyBindGroupSetExtend {
    pub fn new(
        lighting: Option<Arc<ShaderBindSceneLightInfos>>,
        shadowmap: Option<(Arc<ShaderBindShadowData>, Arc<ShaderBindShadowTexture>, Arc<ShaderBindShadowSampler>)>,
        bind_brdf: Option<(Arc<ShaderBindBRDFTexture>, Arc<ShaderBindBRDFSampler>)>,
        camera_opaque: Option<(Arc<ShaderBindMainCameraOpaqueTexture>, Arc<ShaderBindMainCameraOpaqueSampler>)>,
        camera_depth: Option<(Arc<ShaderBindMainCameraDepthTexture>, Arc<ShaderBindMainCameraDepthSampler>)>,
        env: Option<(Arc<BindEnvIrradiance>, Arc<ShaderBindEnvTexture>, Arc<ShaderBindEnvSampler>)>,
    ) -> Self {
        
        let mut lighting_enable: bool = false;
        let mut shadow_enable: bool = false;
        let mut binding = 0;
        let mut key_bindgroup = KeyBindGroup::default();

        if let Some(bind) = &lighting {
            if let Some(key) = bind.key_bind() {
                key_bindgroup.0.push(key);
                binding += 1;
                lighting_enable = true;
            }
        }

        if let Some((v0, v1, v2)) = &shadowmap {
            if let (Some(key0), Some(key1), Some(key2)) = (v0.key_bind(), v1.key_bind(), v2.key_bind()) {
                key_bindgroup.0.push(key0); key_bindgroup.0.push(key1); key_bindgroup.0.push(key2);
                binding += 3;
                shadow_enable = true;
            }
        }
        
        if let Some((v0, v1)) = &bind_brdf {
            if let (Some(key0), Some(key1)) = (v0.key_bind(), v1.key_bind()) {
                key_bindgroup.0.push(key0); key_bindgroup.0.push(key1);
                binding += 2;
            }
        }
        
        if let Some((v0, v1)) = &camera_opaque {
            if let (Some(key0), Some(key1)) = (v0.key_bind(), v1.key_bind()) {
                key_bindgroup.0.push(key0); key_bindgroup.0.push(key1);
                binding += 2;
            }
        }

        if let Some((v0, v1)) = &camera_depth {
            if let (Some(key0), Some(key1)) = (v0.key_bind(), v1.key_bind()) {
                key_bindgroup.0.push(key0); key_bindgroup.0.push(key1); 
                binding += 2;
            }
        }
        
        if let Some((v0, v1, v2)) = &env {
            if let (Some(key0), Some(key1), Some(key2)) = (v0.key_bind(), v1.key_bind(), v2.key_bind()) {
                key_bindgroup.0.push(key0); key_bindgroup.0.push(key1); key_bindgroup.0.push(key2);
                binding += 3;
            }
        }

        let result = Self {
            lighting,
            shadowmap,
            bind_brdf,
            camera_opaque,
            camera_depth,
            env,
            isactived: binding > 0,
            bind_count: binding,
            key_bindgroup,
            key: KeyShaderSetExtend { lighting_enable, shadow_enable }
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
impl TAssetKeyU64 for KeyBindGroupSetExtend {}


#[derive(Clone)]
pub struct BindGroupSetExtend {
    pub(crate) bind_group: BindGroupUsage,
    pub(crate) key: KeyBindGroupSetExtend,
}
impl BindGroupSetExtend {
    pub fn new(
        bind_group: BindGroupUsage,
        key: KeyBindGroupSetExtend,
    ) -> Self {
        Self { bind_group, key }
    }
    pub fn key(&self) -> &KeyBindGroupSetExtend { &self.key }
    pub fn bind_group(&self) -> &BindGroupUsage { &self.bind_group }
}
impl TShaderSetBlock for BindGroupSetExtend {
    fn vs_define_code(&self, set: u32) -> String {

        let mut result = String::from("");
        let bind = 0;

        if let Some(v0) = &self.key.lighting {
            result += v0.vs_define_code(set, bind).as_str(); // bind += 1;
        }
        // if let Some((v0, v1, v2)) = &self.key.shadowmap {
        //     result += v0.vs_define_code(set).as_str();
        //     result += v1.vs_define_code(set).as_str();
        //     result += v2.vs_define_code(set).as_str();
        // }
        // if let Some((bind1, bind2)) = &self.key.bind_brdf {
        //     result += bind1.vs_define_code(set).as_str();
        //     result += bind2.vs_define_code(set).as_str();
        // }

        result
    }

    fn fs_define_code(&self, set: u32) -> String {

        let mut result = String::from("");
        let mut bind = 0;

        if let Some(v0) = &self.key.lighting {
            result += v0.fs_define_code(set, bind).as_str(); bind += 1;
        }
        if let Some((v0, v1, v2)) = &self.key.shadowmap {
            result += v0.fs_define_code(set, bind).as_str(); bind += 1;
            result += v1.fs_define_code(set, bind).as_str(); bind += 1;
            result += v2.fs_define_code(set, bind).as_str(); bind += 1;
        }
        if let Some((bind1, bind2)) = &self.key.bind_brdf {
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

        if let Some((v0, v1)) = &self.key.camera_opaque {
            result += v0.fs_define_code(set, bind).as_str(); bind += 1;
            result += v1.fs_define_code(set, bind).as_str(); bind += 1;
        }

        if let Some((v0, v1)) = &self.key.camera_depth {
            result += v0.fs_define_code(set, bind).as_str(); bind += 1;
            result += v1.fs_define_code(set, bind).as_str(); bind += 1;
        }
        if let Some((bind1, bind2, bind3)) = &self.key.env {
            result += bind1.fs_define_code(set, bind).as_str(); bind += 1;
            result += bind2.fs_define_code(set, bind).as_str(); bind += 1;
            result += bind3.fs_define_code(set, bind).as_str(); // bind += 1;
        }

        result
    }
}
impl EqAsResource for BindGroupSetExtend {
    fn eq_resource(&self, other: &Self) -> bool {
        self.bind_group.key() == other.bind_group.key() && self.key == other.key
    }
}
impl HashAsResource for BindGroupSetExtend {
    fn hash_resource<H: std::hash::Hasher>(&self, state: &mut H) {
        self.bind_group.key().asset_u64().hash(state);
    }
}