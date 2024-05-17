use std::sync::Arc;
use pi_scene_shell::prelude::*;

use crate::{
    scene::{prelude::*, environment::{brdf::{BRDFTexture, BRDFSampler}, environment_texture::{EnvTexture, EnvIrradiance, EnvSampler}}},
    viewer::prelude::*,
    pass::*,
};

use super::base::*;

pub fn sys_set0_modify(
    mut items: Query<(&PassModelID, &PassSceneID, &PassViewerID, &PassEffectReady, &mut PassBindGroupScene), Or<(Changed<PassSceneID>, Changed<PassViewerID>, Changed<PassEffectReady>)>>,
    viewers: Query<Option<&BindViewer>>,
    scenes: Query<(&BindSceneEffect, &SceneLightingInfos, &BRDFTexture, &BRDFSampler, &MainCameraOpaqueTarget, &MainCameraDepthTarget, &SceneShadowRenderTarget, Option<&SceneShadowInfos>, &EnvTexture, &EnvIrradiance, &EnvSampler)>,
    device: Res<PiRenderDevice>,
    asset_mgr_bindgroup_layout: Res<ShareAssetMgr<BindGroupLayout>>,
    asset_mgr_bindgroup: Res<ShareAssetMgr<BindGroup>>,
    targets: Res<CustomRenderTargets>,
    mut errors: ResMut<ErrorRecord>,
) {
    let time1 = pi_time::Instant::now();

    items.iter_mut().for_each(|(idmodel, idscene, idviewer, meta, mut set0)| {
        // log::error!("Set0 Modify 1, {:?}", (scenes.get(idscene.0).is_ok(), viewers.get(idviewer.0).is_ok()));
        if let (
            Ok((
                bind_base_effect,
                scene_lighting, brdftexture, brdfsampler,
                opaque_target, depth_target,
                shadowtarget, scene_shadow,
                env_texture, env_irradiance, env_sampler
            )),
            Ok(bind_viewer)
        ) = (scenes.get(idscene.0), viewers.get(idviewer.0)) {
            // log::error!("Set0 Modify 2");
            if let Some((_, meta)) = &meta.0 {
                // log::error!("Set0 Modify 3");
                let bind_base_effect = if BindDefines::need_scene_effect(meta.binddefines) {
                    Some(bind_base_effect.0.clone())
                } else { None };

                let bind_viewer = match (BindDefines::need_viewer(meta.binddefines), bind_viewer) {
                    (true, Some(bindviewer)) => {
                        // log::error!("Set0 Modify 4");
                        Some(bindviewer.0.clone())
                    },
                    (false, _) => {
                        // log::error!("Set0 Modify 44");
                        None
                    },
                    _ => {
                        errors.record(idmodel.0, ErrorRecord::ERROR_PASS_BIND_VIEWER_NONE);
                        *set0 = PassBindGroupScene(None);
                        // log::error!("Set0 Modify 444");
                        return;
                    }
                };
                
                let bind_lighting = Some(scene_lighting.0.clone());
                let bind_shadow = match (BindDefines::need_shadowmap(meta.binddefines), &shadowtarget.0, scene_shadow) {
                    (true, Some(shadowtarget), Some(scene_shadow)) => {
                        if let Some(shadowtarget) = targets.get(shadowtarget.clone()) {
                            Some( scene_shadow.binds(&shadowtarget))
                        } else { 
                            errors.record(idmodel.0, ErrorRecord::ERROR_PASS_BIND_SHADOW_NONE); 
                            *set0 = PassBindGroupScene(None); return;
                        }
                    },
                    (false, _, _) => None,
                    _ => {
                        errors.record(idmodel.0, ErrorRecord::ERROR_PASS_BIND_SHADOW_NONE); 
                        *set0 = PassBindGroupScene(None); return;
                    },
                };
                let brdf = match (BindDefines::need_brdf(meta.binddefines), &brdftexture.0, &brdfsampler.0) {
                    (true, Some(v0), Some(v1)) => { Some((v0.clone(), v1.clone())) },
                    (false, _, _) => None,
                    _ => {
                        errors.record(idmodel.0, ErrorRecord::ERROR_PASS_BIND_BRDF_NONE);
                        *set0 = PassBindGroupScene(None); return;
                    },
                };
                let bind_camera_opaque = match (BindDefines::need_screen_opaque(meta.binddefines), opaque_target.binds()) {
                    (true, Some(v0)) => { Some(v0) },
                    (false, _) => None,
                    _ => {
                        errors.record(idmodel.0, ErrorRecord::ERROR_PASS_BIND_CAMERA_OPAQUE_NONE);
                        *set0 = PassBindGroupScene(None); return;
                    },
                };
                let bind_camera_depth = match (BindDefines::need_screen_depth(meta.binddefines), depth_target.binds()) {
                    (true, Some(v0)) => { Some(v0) },
                    (false, _) => None,
                    _ => {
                        errors.record(idmodel.0, ErrorRecord::ERROR_PASS_BIND_CAMERA_DEPTH_NONE);
                        *set0 = PassBindGroupScene(None); return;
                    },
                };
                
                let bind_env = match (BindDefines::need_env(meta.binddefines), &env_irradiance.0, &env_texture.0, &env_sampler.0, ) {
                    (true, Some(v0), Some(v1), Some(v2)) => { Some((v0.clone(), v1.clone(), v2.clone())) },
                    (false, _, _, _) => None,
                    _ => {
                        errors.record(idmodel.0, ErrorRecord::ERROR_PASS_BIND_ENV_NONE);
                        *set0 = PassBindGroupScene(None); return;
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

                    // log::error!("create_bind_group 0: Ok, {:?}", (set0.0.is_some()));
                    // *set0 = PassBindGroupScene(Some(data.clone()));
                    if let Some(old) = &set0.0 {
                        if old.key() != data.key() { *set0 = PassBindGroupScene(Some(data.clone())); }
                    } else { *set0 = PassBindGroupScene(Some(data.clone())); }
                } else {
                    errors.record(idmodel.0, ErrorRecord::ERROR_PASS_SET0_FAIL);
                    // log::error!("create_bind_group 0: Error");
                    *set0 = PassBindGroupScene(None);
                }
            } else {
                // log::error!("create_bind_group 0: 2 Error");
            }
        } else {
            errors.record(idmodel.0, ErrorRecord::ERROR_PASS_BIND_SCENE_NONE);
        }
    });

    // log::trace!("SysSet0ModifyFromScene: {:?}", pi_time::Instant::now() - time1);
}