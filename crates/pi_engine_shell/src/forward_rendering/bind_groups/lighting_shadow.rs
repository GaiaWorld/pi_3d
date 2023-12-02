
use std::sync::Arc;

use pi_render::{
    renderer::{
        shader::{TShaderSetBlock, TShaderBindCode},
        bind_group::*, bind::TKeyBind, buildin_var::ShaderVarUniform,
    },
    asset::TAssetKeyU64
};

use crate::forward_rendering::binds::*;

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct KeyShaderSetExtend {
    pub lighting_enable: bool,
    pub shadow_enable: bool,
}

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct KeyBindGroupSetExtend {
    pub lighting: Option<BindUseSceneLightInfos>,
    pub shadowmap: Option<(BindUseShadowData, BindUseShadowTexture, BindUseShadowSampler)>,
    pub bind_brdf: Option<(BindUseBRDFTexture, BindUseBRDFSampler)>,
    pub camera_opaque: Option<(BindUseMainCameraOpaqueTexture, BindUseMainCameraOpaqueSampler)>,
    pub camera_depth: Option<(BindUseMainCameraDepthTexture, BindUseMainCameraDepthSampler)>,
    pub env: Option<(BindUseEnvIrradiance, BindUseEnvTexture, BindUseEnvSampler)>,
    pub isactived: bool,
    bind_count: u32,
    key_binds: Arc<IDBinds>,
    pub key: KeyShaderSetExtend,
}
impl KeyBindGroupSetExtend {
    pub fn new(
        bind_lighting: Option<Arc<ShaderBindSceneLightInfos>>,
        bind_shadow: Option<(Arc<ShaderBindShadowData>, Arc<ShaderBindShadowTexture>, Arc<ShaderBindShadowSampler>)>,
        brdf: Option<(Arc<ShaderBindBRDFTexture>, Arc<ShaderBindBRDFSampler>)>,
        bind_camera_opaque: Option<(Arc<ShaderBindMainCameraOpaqueTexture>, Arc<ShaderBindMainCameraOpaqueSampler>)>,
        bind_camera_depth: Option<(Arc<ShaderBindMainCameraDepthTexture>, Arc<ShaderBindMainCameraDepthSampler>)>,
        bind_env: Option<(Arc<BindEnvIrradiance>, Arc<ShaderBindEnvTexture>, Arc<ShaderBindEnvSampler>)>,
        recorder: &mut BindsRecorder,
    ) -> Self {
        
        let mut lighting_enable: bool = false;
        let mut shadow_enable: bool = false;
        let mut lighting: Option<BindUseSceneLightInfos> = None;
        let mut shadowmap: Option<(BindUseShadowData, BindUseShadowTexture, BindUseShadowSampler)> = None;
        let mut camera_opaque: Option<(BindUseMainCameraOpaqueTexture, BindUseMainCameraOpaqueSampler)> = None;
        let mut camera_depth: Option<(BindUseMainCameraDepthTexture, BindUseMainCameraDepthSampler)> = None;
        let mut env = None;
        let mut binding = 0;

        if let Some(v1) = bind_lighting {
            lighting = Some(BindUseSceneLightInfos { data: v1, bind: binding as u32 });
            lighting_enable = true;
            binding += 1;
        }

        if let Some((v0, v1, v2)) = bind_shadow {
            shadowmap = Some((
                BindUseShadowData    { data: v0, bind: (binding + 0) as u32 },
                BindUseShadowTexture { data: v1, bind: (binding + 1) as u32 },
                BindUseShadowSampler { data: v2, bind: (binding + 2) as u32 },
            ));
            shadow_enable = true;
            binding += 3;
        }
        
        let bind_brdf = if let Some((v0, v1)) = brdf {
            let result = Some((
                BindUseBRDFTexture::new(binding + 0, v0),
                BindUseBRDFSampler::new(binding + 1, v1),
            ));
            binding += 2;
            result
        } else { None };
        
        if let Some((v1, v2)) = bind_camera_opaque {
            camera_opaque = Some((
                BindUseMainCameraOpaqueTexture { data: v1, bind: binding as u32 },
                BindUseMainCameraOpaqueSampler { data: v2, bind: (binding + 1) as u32 },
            ));
            binding += 2;
        }

        if let Some((v1, v2)) = bind_camera_depth {
            camera_depth = Some((
                BindUseMainCameraDepthTexture { data: v1, bind: binding as u32 },
                BindUseMainCameraDepthSampler { data: v2, bind: (binding + 1) as u32 },
            ));
            binding += 2;
        }
        
        if let Some((v1, v2, v3)) = bind_env {
            env = Some((
                BindUseEnvIrradiance { data: v1, bind: (binding + 0) as u32 },
                BindUseEnvTexture { data: v2, bind: (binding + 1) as u32 },
                BindUseEnvSampler { data: v3, bind: (binding + 2) as u32 },
            ));
            binding += 3;
        }

        let mut result = Self {
            lighting,
            shadowmap,
            bind_brdf,
            camera_opaque,
            camera_depth,
            env,
            isactived: binding > 0,
            bind_count: binding,
            key_binds: Arc::new(IDBinds::Binds00(vec![])),
            key: KeyShaderSetExtend { lighting_enable, shadow_enable }
        };
        result.key_binds = result._binds(recorder);

        result
    }
    fn _binds(&self, recorder: &mut BindsRecorder) -> Arc<IDBinds> {
        // log::warn!("Model Binds {:?} {:?}", self.key_binds, self.bind_count);
        if let Some(mut binds) = EBinds::new(self.bind_count) {
            if let Some(bind1) = &self.lighting {
                binds.set(bind1.bind as usize, bind1.key_bind());
            }
            if let Some((bind0, bind1, bind2)) = &self.shadowmap {
                binds.set(bind0.bind as usize, bind0.key_bind());
                binds.set(bind1.bind as usize, bind1.key_bind());
                binds.set(bind2.bind as usize, bind2.key_bind());
            }
            if let Some((v0, v1)) = &self.bind_brdf {
                binds.set( v0.bind as usize, v0.key_bind() );
                binds.set( v1.bind as usize, v1.key_bind() );
            }
            if let Some((bind1, bind2)) = &self.camera_opaque {
                binds.set(bind1.bind as usize, bind1.key_bind());
                binds.set(bind2.bind as usize, bind2.key_bind());
            }
            if let Some((bind1, bind2)) = &self.camera_depth {
                binds.set(bind1.bind as usize, bind1.key_bind());
                binds.set(bind2.bind as usize, bind2.key_bind());
            }
            if let Some((bind1, bind2, bind3)) = &self.env {
                binds.set(bind1.bind as usize, bind1.key_bind());
                binds.set(bind2.bind as usize, bind2.key_bind());
                binds.set(bind3.bind as usize, bind3.key_bind());
            }

            binds.record(recorder)
        } else {
            Arc::new(IDBinds::Binds00(vec![]))
        }
    }
    pub fn key_bind_group(&self) -> KeyBindGroup {
        KeyBindGroup(self.key_binds.binds())
    }
    pub fn key_bind_group_layout(&self) -> KeyBindGroupLayout {
        KeyBindGroup(self.key_binds.binds())
    }
    pub fn binds(&self) -> Arc<IDBinds> {
        self.key_binds.clone()
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

        if let Some(v0) = &self.key.lighting {
            result += v0.vs_define_code(set).as_str();
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

        if let Some(v0) = &self.key.lighting {
            result += v0.fs_define_code(set).as_str();
        }
        if let Some((v0, v1, v2)) = &self.key.shadowmap {
            result += v0.fs_define_code(set).as_str();
            result += v1.fs_define_code(set).as_str();
            result += v2.vs_define_code(set).as_str();
        }
        if let Some((bind1, bind2)) = &self.key.bind_brdf {
            result += bind1.fs_define_code(set).as_str();
            result += bind2.fs_define_code(set).as_str();
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
            result += v0.fs_define_code(set).as_str();
            result += v1.fs_define_code(set).as_str();
        }

        if let Some((v0, v1)) = &self.key.camera_depth {
            result += v0.fs_define_code(set).as_str();
            result += v1.fs_define_code(set).as_str();
        }
        if let Some((bind1, bind2, bind3)) = &self.key.env {
            result += bind1.fs_define_code(set).as_str();
            result += bind2.fs_define_code(set).as_str();
            result += bind3.fs_define_code(set).as_str();
        }

        result
    }
}