use std::sync::Arc;

use pi_render::{
    renderer::{
        bind_group::*,
        bind::*,
        shader::{TShaderSetBlock, TShaderBindCode}
    },
    asset::TAssetKeyU64
};
use crate::binds::*;

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct KeyShaderSetScene {
    pub base_effect: bool,
    pub brdf: bool,
    pub env: bool,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct KeyBindGroupScene {
    pub bind_base: Option<BindUseViewer>,
    pub bind_base_effect: Option<BindUseSceneAboutEffect>,
    // pub bind_brdf: Option<(BindUseBRDFTexture, BindUseBRDFSampler)>,
    pub key_set: KeyShaderSetScene,
    bind_count: u32,
    key_binds: Arc<IDBinds>,
}
impl KeyBindGroupScene {
    pub fn new(
        bind_viewer: Option<Arc<ShaderBindViewer>>,
        bind_base_effect: Option<Arc<ShaderBindSceneAboutEffect>>,
        // brdf: Option<(Arc<ShaderBindBRDFTexture>, Arc<ShaderBindBRDFSampler>)>,
        recorder: &mut BindsRecorder,
    ) -> Self {
        let mut key_set = KeyShaderSetScene::default();

        let mut binding = 0;
        let mut bind_base = None;

        if let Some(bind) = bind_viewer {
            bind_base = Some(BindUseViewer { data: bind, bind: binding as u32 });
            binding += 1;
        }

        let bind_base_effect = if let Some(bind_base_effect) = bind_base_effect {
            key_set.base_effect = true;
            let result = Some(BindUseSceneAboutEffect { data: bind_base_effect, bind: binding as u32 });
            binding += 1;
            result
        } else { None };
        
        // let bind_brdf = if let Some((v0, v1)) = brdf {
        //     key_set.brdf = true;
        //     let result = Some((
        //         BindUseBRDFTexture::new(binding + 0, v0),
        //         BindUseBRDFSampler::new(binding + 1, v1),
        //     ));
        //     binding += 2;
        //     result
        // } else { None };

        let mut result = Self {
            bind_base,
            bind_base_effect,
            // bind_brdf,
            key_set,
            bind_count: binding,
            key_binds: Arc::new(IDBinds::Binds00(vec![]))
        };

        result.key_binds = result._binds(recorder);

        result
    }
    fn _binds(&self, recorder: &mut BindsRecorder) -> Arc<IDBinds> {
        if let Some(mut binds) = EBinds::new(self.bind_count) {
            if let Some(bind) = &self.bind_base {
                binds.set( bind.bind as usize, bind.key_bind() );
            }

            if let Some(bind) = &self.bind_base_effect {
                binds.set( bind.bind as usize, bind.key_bind() );
            }
            // if let Some((v0, v1)) = &self.bind_brdf {
            //     binds.set( v0.bind as usize, v0.key_bind() );
            //     binds.set( v1.bind as usize, v1.key_bind() );
            // }
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
impl TAssetKeyU64 for KeyBindGroupScene {}

#[derive(Debug, Clone)]
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
    fn vs_define_code(&self, set: u32) -> String {
        let mut result = String::from("");

        if let Some(bind) = &self.key.bind_base {
            result += bind.vs_define_code(set).as_str();
        }

        if let Some(bind) = &self.key.bind_base_effect {
            result += bind.vs_define_code(set).as_str();
        }
        // if let Some((bind1, bind2)) = &self.key.bind_brdf {
        //     result += bind1.vs_define_code(set).as_str();
        //     result += bind2.vs_define_code(set).as_str();
        // }

        result
    }

    fn fs_define_code(&self, set: u32) -> String {
        let mut result = String::from("");


        if let Some(bind) = &self.key.bind_base {
            result += bind.fs_define_code(set).as_str();
        }

        if let Some(bind) = &self.key.bind_base_effect {
            result += bind.fs_define_code(set).as_str();
        }
//         if let Some((bind1, bind2)) = &self.key.bind_brdf {
//             result += bind1.fs_define_code(set).as_str();
//             result += bind2.fs_define_code(set).as_str();
//             result += "
// vec4 GetEnvironmentBRDFTexture(vec2 uv) {
//     return texture(sampler2D(";
//     result += ShaderVarUniform::BRDF_TEXUTRE;
//     result += ", sampler";
//     result += ShaderVarUniform::BRDF_TEXUTRE;
//     result += "), uv);
// }
// ";
//         }

        result
    }

    // fn vs_running_code(&self) -> String {
    //     String::from("")
    // }

    // fn fs_running_code(&self) -> String {
    //     String::from("")
    // }
}