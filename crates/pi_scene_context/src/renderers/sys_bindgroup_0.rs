use std::sync::Arc;
use pi_scene_shell::prelude::*;

use crate::{
    scene::{prelude::*, environment::{brdf::{BRDFTexture, BRDFSampler}, environment_texture::{EnvTexture, EnvIrradiance, EnvSampler}}},
    viewer::prelude::*,
};

use super::base::*;

pub fn _set0_modify(
    idmodel: Entity,
    idscene: Entity,
    idviewer: Entity,
    meta: &Handle<ShaderEffectMeta>,
    viewers: &Query<&BindViewer>,
    scenes: &Query<(&BindSceneEffect, &SceneLightingInfos, &BRDFTexture, &BRDFSampler, &MainCameraOpaqueTarget, &MainCameraDepthTarget, &SceneShadowRenderTarget, Option<&SceneShadowInfos>, &EnvTexture, &EnvIrradiance, &EnvSampler)>,
    device: &PiRenderDevice,
    asset_mgr_bindgroup_layout: &ShareAssetMgr<BindGroupLayout>,
    asset_mgr_bindgroup: &ShareAssetMgr<BindGroup>,
    targets: &CustomRenderTargets,
    errors: &mut ErrorRecord,
) -> Option<Arc<BindGroupScene>> {
    let mut result = None;
    if let (
        Ok((
            bind_base_effect,
            scene_lighting, brdftexture, brdfsampler,
            opaque_target, depth_target,
            shadowtarget, scene_shadow,
            env_texture, env_irradiance, env_sampler
        )),
        Ok(bind_viewer)
    ) = (scenes.get(idscene), viewers.get(idviewer)) {
        // log::error!("Set0 Modify 2 {:?}", (meta.0.is_some(), idpass));
        // log::error!("Set0 Modify 3");
        let bind_base_effect = if BindDefines::need_scene_effect(meta.binddefines) {
            Some(bind_base_effect.0.as_ref().unwrap().clone())
        } else { None };

        let bind_viewer = match (BindDefines::need_viewer(meta.binddefines), &bind_viewer.0) {
            (true, Some(bindviewer)) => {
                // log::error!("Set0 Modify 4");
                Some(bindviewer.clone())
            },
            (false, _) => {
                // log::error!("Set0 Modify 44 {:?}", meta.key());
                None
            },
            _ => {
                errors.record(idmodel, ErrorRecord::ERROR_PASS_BIND_VIEWER_NONE);
                return result;
            }
        };
        
        let bind_lighting = Some(scene_lighting.0.as_ref().unwrap().clone());
        let bind_shadow = match (BindDefines::need_shadowmap(meta.binddefines), &shadowtarget.0, scene_shadow) {
            (true, Some(shadowtarget), Some(scene_shadow)) => {
                if let Some(shadowtarget) = targets.get(shadowtarget.clone()) {
                    Some( scene_shadow.binds(&shadowtarget))
                } else { 
                    errors.record(idmodel, ErrorRecord::ERROR_PASS_BIND_SHADOW_NONE); 
                    return result;
                }
            },
            (false, _, _) => None,
            (true, _, _) => {
                return result;
            },
            _ => {
                errors.record(idmodel, ErrorRecord::ERROR_PASS_BIND_SHADOW_NONE); 
                return result;
            },
        };
        let brdf = match (BindDefines::need_brdf(meta.binddefines), &brdftexture.0, &brdfsampler.0) {
            (true, Some(v0), Some(v1)) => { Some((v0.clone(), v1.clone())) },
            (false, _, _) => None,
            _ => {
                errors.record(idmodel, ErrorRecord::ERROR_PASS_BIND_BRDF_NONE);
                return result;
            },
        };
        let bind_camera_opaque = match (BindDefines::need_screen_opaque(meta.binddefines), opaque_target.binds()) {
            (true, Some(v0)) => { Some(v0) },
            (false, _) => None,
            _ => {
                errors.record(idmodel, ErrorRecord::ERROR_PASS_BIND_CAMERA_OPAQUE_NONE);
                return result;
            },
        };
        let bind_camera_depth = match (BindDefines::need_screen_depth(meta.binddefines), depth_target.binds()) {
            (true, Some(v0)) => { Some(v0) },
            (false, _) => None,
            _ => {
                errors.record(idmodel, ErrorRecord::ERROR_PASS_BIND_CAMERA_DEPTH_NONE);
                return result;
            },
        };
        
        let bind_env = match (BindDefines::need_env(meta.binddefines), &env_irradiance.0, &env_texture.0, &env_sampler.0, ) {
            (true, Some(v0), Some(v1), Some(v2)) => { Some((v0.clone(), v1.clone(), v2.clone())) },
            (false, _, _, _) => None,
            _ => {
                errors.record(idmodel, ErrorRecord::ERROR_PASS_BIND_ENV_NONE);
                return result;
            },
        };

        let key = KeyBindGroupScene::new(
            bind_viewer, bind_base_effect,
            bind_lighting,
            bind_shadow,
            brdf,
            bind_camera_opaque,
            bind_camera_depth,
            bind_env,
        );
        let key_bind_group = key.key_bind_group();
        // log::warn!("Set0Loaded : {:?}", key_bind_group);
        if let Some(bind_group) = create_bind_group(&key_bind_group, &device, &asset_mgr_bindgroup_layout, &asset_mgr_bindgroup) {
            // log::error!("Set0 Modify 5");
            let data = BindGroupScene::new(BindGroupUsage::new(key_bind_group, bind_group), key);
            let data = Arc::new(data);

            result = Some(data.clone());
        } else {
            errors.record(idmodel, ErrorRecord::ERROR_PASS_SET0_FAIL);
            // log::error!("create_bind_group 0: Error");
            result = None;
        }
    } else {
        errors.record(idmodel, ErrorRecord::ERROR_PASS_BIND_SCENE_NONE);
    }

    return result;
}