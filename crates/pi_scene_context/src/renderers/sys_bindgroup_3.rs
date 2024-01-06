use std::sync::Arc;
use pi_engine_shell::prelude::*;
use crate::{
    pass::*,
    scene::{prelude::*, environment::{brdf::{BRDFTexture, BRDFSampler}, environment_texture::{EnvTexture, EnvIrradiance, EnvSampler}}}
};

use super::base::create_bind_group;

pub fn sys_set3_modify(
    mut passes: Query<(&PassModelID, &PassSceneForSet3, &PassEffectReady, &mut PassBindGroupLightingShadow), Or<(Changed<PassSceneForSet3>, Changed<PassEffectReady>)>>,
    scenes: Query<(&SceneLightingInfos, &BRDFTexture, &BRDFSampler, &MainCameraOpaqueTarget, &MainCameraDepthTarget, &SceneShadowRenderTarget, Option<&SceneShadowInfos>, &EnvTexture, &EnvIrradiance, &EnvSampler)>,
    device: Res<PiRenderDevice>,
    targets: Res<CustomRenderTargets>,
    asset_mgr_bindgroup_layout: Res<ShareAssetMgr<BindGroupLayout>>,
    asset_mgr_bindgroup: Res<ShareAssetMgr<BindGroup>>,
    mut errors: ResMut<ErrorRecord>,
) {
    passes.iter_mut().for_each(|(idmodel, idscene, meta, mut set)| {
        if let Some((_metakey, meta)) = &meta.0 {
            if let Ok((
                scene_lighting, brdftexture, brdfsampler,
                opaque_target, depth_target,
                shadowtarget, scene_shadow,
                env_texture, env_irradiance, env_sampler
            )) = scenes.get(idscene.0) {
                let bind_lighting = Some(scene_lighting.0.clone());
                let bind_shadow = match (BindDefines::need_shadowmap(meta.binddefines), &shadowtarget.0, scene_shadow) {
                    (true, Some(shadowtarget), Some(scene_shadow)) => {
                        if let Some(shadowtarget) = targets.get(shadowtarget.clone()) {
                            Some( scene_shadow.binds(&shadowtarget))
                        } else { 
                            errors.record(idmodel.0, ErrorRecord::ERROR_PASS_BIND_SHADOW_NONE); 
                            *set = PassBindGroupLightingShadow(None); return;
                        }
                    },
                    (false, _, _) => None,
                    _ => {
                        errors.record(idmodel.0, ErrorRecord::ERROR_PASS_BIND_SHADOW_NONE); 
                        *set = PassBindGroupLightingShadow(None); return;
                    },
                };
                let brdf = match (BindDefines::need_brdf(meta.binddefines), &brdftexture.0, &brdfsampler.0) {
                    (true, Some(v0), Some(v1)) => { Some((v0.clone(), v1.clone())) },
                    (false, _, _) => None,
                    _ => {
                        errors.record(idmodel.0, ErrorRecord::ERROR_PASS_BIND_BRDF_NONE);
                        *set = PassBindGroupLightingShadow(None); return;
                    },
                };
                let bind_camera_opaque = match (BindDefines::need_screen_opaque(meta.binddefines), opaque_target.binds()) {
                    (true, Some(v0)) => { Some(v0) },
                    (false, _) => None,
                    _ => {
                        errors.record(idmodel.0, ErrorRecord::ERROR_PASS_BIND_CAMERA_OPAQUE_NONE);
                        *set = PassBindGroupLightingShadow(None); return;
                    },
                };
                let bind_camera_depth = match (BindDefines::need_screen_depth(meta.binddefines), depth_target.binds()) {
                    (true, Some(v0)) => { Some(v0) },
                    (false, _) => None,
                    _ => {
                        errors.record(idmodel.0, ErrorRecord::ERROR_PASS_BIND_CAMERA_DEPTH_NONE);
                        *set = PassBindGroupLightingShadow(None); return;
                    },
                };
                
                let bind_env = match (BindDefines::need_env(meta.binddefines), &env_irradiance.0, &env_texture.0, &env_sampler.0, ) {
                    (true, Some(v0), Some(v1), Some(v2)) => { Some((v0.clone(), v1.clone(), v2.clone())) },
                    (false, _, _, _) => None,
                    _ => {
                        errors.record(idmodel.0, ErrorRecord::ERROR_PASS_BIND_ENV_NONE);
                        *set = PassBindGroupLightingShadow(None); return;
                    },
                };

                // log::warn!("Set3: {:?}", (brdf.is_none(), _metakey));
                let key = KeyBindGroupSetExtend::new(
                    bind_lighting,
                    bind_shadow,
                    brdf,
                    bind_camera_opaque,
                    bind_camera_depth,
                    bind_env,
                );
    
                let key_bind_group = key.key_bind_group();
                if let Some(bind_group) = create_bind_group(&key_bind_group, &device, &asset_mgr_bindgroup_layout, &asset_mgr_bindgroup) {
                    let data = BindGroupSetExtend::new(BindGroupUsage::new(key_bind_group, bind_group), key);
                    let data = Arc::new(data);
                    // log::error!("create_bind_group 2: Ok");
                    // *set = PassBindGroupLightingShadow(Some(data.clone()));
                    if let Some(old) = &set.0 {
                        if old.key() != data.key() { *set = PassBindGroupLightingShadow(Some(data.clone())); }
                    } else { *set = PassBindGroupLightingShadow(Some(data.clone())); }
                } else {
                    errors.record(idmodel.0, ErrorRecord::ERROR_PASS_SET3_FAIL);
                    *set = PassBindGroupLightingShadow(None);
                };
            }
        }
    });
}


// fn bind_lighting(scene_lighting: &SceneLightingInfos, modellighting: &ModelLightingIndexs) -> Option<(Arc<ShaderBindSceneLightInfos>, Arc<BindModelLightIndexs>)> {
//     Some((scene_lighting.0.clone(), modellighting.bind.as_ref().unwrap().clone()))
// }