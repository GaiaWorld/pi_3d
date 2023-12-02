use std::sync::Arc;

use pi_render::{
    renderer::{
        bind_group::*,
        shader::{TShaderSetBlock, TShaderBindCode},
        bind::*
    },
    asset::TAssetKeyU64
};
use crate::{binds::*, shader::*, prelude::{BindModelLightIndexs, BindUseModelLightIndexs} };

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct KeyShaderSetModel {
    pub skin: ESkinCode,
    pub camera_opaque: Option<u64>,
    pub camera_depth: Option<u64>,
    pub shadow: Option<u64>,
    pub lighting: Option<u64>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct KeyBindGroupModel {
    pub matrix: Option<BindUseModelMatrix>,
    pub skin: Option<BindUseSkinValue>,
    pub effect_value: Option<BindUseEffectValue>,
    pub lightingidxs: Option<BindUseModelLightIndexs>,
    // pub lighting: Option<(BindUseSceneLightInfos, BindUseModelLightIndexs)>,
    // pub shadowmap: Option<(BindUseShadowTexture, BindUseShadowSampler)>,
    // pub camera_opaque: Option<(BindUseCameraOpaqueTexture, BindUseCameraOpaqueSampler)>,
    // pub camera_depth: Option<(BindUseCameraDepthTexture, BindUseCameraDepthSampler)>,
    pub key: KeyShaderSetModel,
    bind_count: u32,
    key_binds: Arc<IDBinds>,
}
impl KeyBindGroupModel {
    pub fn new(
        bind_matrix: Option<Arc<ShaderBindModelAboutMatrix>>,
        bind_skin: Option<Arc<ShaderBindModelAboutSkinValue>>,
        bind_effect_value: Option<Arc<ShaderBindEffectValue>>,
        bind_lingingsidx: Option<Arc<BindModelLightIndexs>>,
        // bind_lighting: Option<(Arc<ShaderBindSceneLightInfos>, Arc<BindModelLightIndexs>)>,
        // bind_shadow: Option<(Arc<ShaderBindShadowTexture>, Arc<ShaderBindShadowSampler>)>,
        // bind_camera_opaque: Option<(Arc<ShaderBindCameraOpaqueTexture>, Arc<ShaderBindCameraOpaqueSampler>)>,
        // bind_camera_depth: Option<(Arc<ShaderBindCameraDepthTexture>, Arc<ShaderBindCameraDepthSampler>)>,
        recorder: &mut BindsRecorder
    ) -> Self {
        let mut key = KeyShaderSetModel::default();
        
        let mut matrix: Option<BindUseModelMatrix> = None;
        let mut skin: Option<BindUseSkinValue> = None;
        let mut effect_value: Option<BindUseEffectValue> = None;
        let mut lightingidxs: Option<BindUseModelLightIndexs> = None;
        // let mut lighting: Option<(BindUseSceneLightInfos, BindUseModelLightIndexs)> = None;
        // let mut shadowmap: Option<(BindUseShadowTexture, BindUseShadowSampler)> = None;
        // let mut camera_opaque: Option<(BindUseCameraOpaqueTexture, BindUseCameraOpaqueSampler)> = None;
        // let mut camera_depth: Option<(BindUseCameraDepthTexture, BindUseCameraDepthSampler)> = None;

        let mut binding = 0;

        if let Some(bind) = bind_matrix {
            matrix = Some(BindUseModelMatrix { data: bind, bind: binding as u32 });
            binding += 1;
        }

        if let Some(bind) = bind_skin {
            key.skin = bind.skin;
            skin = Some(BindUseSkinValue { data: bind, bind: binding as u32 });
            binding += 1;
        }

        if let Some(bind) = bind_effect_value {
            effect_value = Some(BindUseEffectValue { data: bind, bind: binding as u32 });
            binding += 1;
        }

        if let Some(bind) = bind_lingingsidx {
            lightingidxs = Some(BindUseModelLightIndexs { data: bind, bind: binding as u32 });
            binding += 1;
        }

        // if let Some((v1, v2)) = bind_lighting {
        //     lighting = Some((
        //         BindUseSceneLightInfos { data: v1, bind: binding as u32 },
        //         BindUseModelLightIndexs { data: v2, bind: (binding + 1) as u32 },
        //     ));
        //     binding += 2;
        // }

        // if let Some((v1, v2)) = bind_shadow {
        //     shadowmap = Some((
        //         BindUseShadowTexture { data: v1, bind: binding as u32 },
        //         BindUseShadowSampler { data: v2, bind: (binding + 1) as u32 },
        //     ));
        //     binding += 2;
        // }

        // if let Some((v1, v2)) = bind_camera_opaque {
        //     camera_opaque = Some((
        //         BindUseCameraOpaqueTexture { data: v1, bind: binding as u32 },
        //         BindUseCameraOpaqueSampler { data: v2, bind: (binding + 1) as u32 },
        //     ));
        //     binding += 2;
        // }

        // if let Some((v1, v2)) = bind_camera_depth {
        //     camera_depth = Some((
        //         BindUseCameraDepthTexture { data: v1, bind: binding as u32 },
        //         BindUseCameraDepthSampler { data: v2, bind: (binding + 1) as u32 },
        //     ));
        //     binding += 2;
        // }

        let mut result = Self {
            matrix,
            skin,
            effect_value,
            lightingidxs,
            // lighting,
            // shadowmap,
            // camera_opaque,
            // camera_depth,
            key,
            bind_count: binding,
            key_binds: Arc::new(IDBinds::Binds00(vec![]))
        };
        result.key_binds = result._binds(recorder);

        result
    }
    
    fn _binds(&self, recorder: &mut BindsRecorder) -> Arc<IDBinds> {
        // log::warn!("Model Binds {:?} {:?}", self.key_binds, self.bind_count);
        if let Some(mut binds) = EBinds::new(self.bind_count) {

            if let Some(bind) = &self.matrix {
                binds.set(bind.bind as usize, bind.key_bind());
            }

            if let Some(bind) = &self.skin {
                binds.set(bind.bind as usize, bind.key_bind());
            }
            
            if let Some(bind) = &self.effect_value {
                binds.set(bind.bind as usize, bind.key_bind());
            }

            if let Some(bind) = &self.lightingidxs {
                binds.set(bind.bind as usize, bind.key_bind());
            }
            
            // if let Some((bind1, bind2)) = &self.lighting {
            //     binds.set(bind1.bind as usize, bind1.key_bind());
            //     binds.set(bind2.bind as usize, bind2.key_bind());
            // }
            // if let Some((bind1, bind2)) = &self.shadowmap {
            //     binds.set(bind1.bind as usize, bind1.key_bind());
            //     binds.set(bind2.bind as usize, bind2.key_bind());
            // }
            // if let Some((bind1, bind2)) = &self.camera_opaque {
            //     binds.set(bind1.bind as usize, bind1.key_bind());
            //     binds.set(bind2.bind as usize, bind2.key_bind());
            // }
            // if let Some((bind1, bind2)) = &self.camera_depth {
            //     binds.set(bind1.bind as usize, bind1.key_bind());
            //     binds.set(bind2.bind as usize, bind2.key_bind());
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
impl TAssetKeyU64 for KeyBindGroupModel {}

#[derive(Debug, Clone)]
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
    pub fn vs_running_model_snippet(&self) -> String {
        let mut result = String::from("");
        if self.key.matrix.is_some() {
            result += "
    mat4 PI_ObjectToWorld = U_PI_ObjectToWorld;
    vec4 PI_ObjectVelocity = U_PI_ObjectVelocity;
    uint PI_SkinBoneOffset0 = U_PI_SkinBoneOffset0;
    uint PI_SkinBoneOffset1 = U_PI_SkinBoneOffset1;
";
        }
        result
    }
}
impl TShaderSetBlock for BindGroupModel {
    fn vs_define_code(&self, set: u32) -> String {

        let mut result = String::from("");

        if let Some(bind) = &self.key.matrix {
            result += bind.vs_define_code(set).as_str();
        }

        if let Some(bind) = &self.key.skin {
            result += bind.vs_define_code(set).as_str();
        }

        if let Some(bind) = &self.key.effect_value {
            result += bind.vs_define_code(set).as_str();
        }

        if let Some(bind) = &self.key.lightingidxs {
            result += bind.vs_define_code(set).as_str();
        }

        result
    }

    fn fs_define_code(&self, set: u32) -> String {

        let mut result = String::from("");

        if let Some(bind) = &self.key.matrix {
            result += bind.fs_define_code(set).as_str();
        }
        if let Some(bind) = &self.key.skin {
            result += bind.fs_define_code(set).as_str();
        }
        if let Some(bind) = &self.key.effect_value {
            result += bind.fs_define_code(set).as_str();
        }
        if let Some(bind) = &self.key.lightingidxs {
            result += bind.fs_define_code(set).as_str();
        }

        result
    }

    // fn vs_running_code(&self) -> String {
    //     let mut result = String::from("");

    //     if let Some(skin) = &self.key.skin {
    //         result += skin.vs_running_code(set).as_str();
    //     }

    //     result
    // }

    // fn fs_running_code(&self) -> String {
    //     String::from("")
    // }
}