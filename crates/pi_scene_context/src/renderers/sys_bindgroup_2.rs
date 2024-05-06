use std::sync::Arc;
use pi_scene_shell::prelude::*;

use crate::pass::*;

use super::base::*;

pub fn sys_set2_modify(
    mut items: Query<(&PassEffectReady, &PassBindEffectTextures, &mut PassBindGroupTextureSamplers), (Changed<PassBindEffectTextures>, Changed<PassEffectReady>)>,
    device: Res<PiRenderDevice>,
    asset_mgr_bindgroup_layout: Res<ShareAssetMgr<BindGroupLayout>>,
    asset_mgr_bindgroup: Res<ShareAssetMgr<BindGroup>>,
) {
    let time1 = pi_time::Instant::now();

    items.iter_mut().for_each(|(meta1, effect_texture_samplers, mut set2)| {
        // log::error!("sys_set2_modify: {:?}", (effect_texture_samplers.val().is_some(), meta1.0.is_some()));
        if let (Some(effect_texture_samplers), Some(mat)) = (effect_texture_samplers.val(), meta1.0.as_ref()) {
            let key = KeyBindGroupTextureSamplers::new(effect_texture_samplers.clone(), mat.1.clone());

            if let Some(key) = key {
                let key_bind_group = key.key_bind_group();
                if let Some(bind_group) = create_bind_group(&key_bind_group, &device, &asset_mgr_bindgroup_layout, &asset_mgr_bindgroup) {
                    let data = BindGroupTextureSamplers::new(key, BindGroupUsage::new(key_bind_group, bind_group));
                    let data = Arc::new(data);
                    if let Some(old) = &set2.0 {
                        if old.key() != data.key() {
                            *set2 = PassBindGroupTextureSamplers(Some(data.clone()));
                        }
                    } else {
                        *set2 = PassBindGroupTextureSamplers(Some(data.clone()));
                    }
                } else {
                    // log::error!("Set2: NN");
                    *set2 = PassBindGroupTextureSamplers(None);
                };
            } else {
                // log::error!("Set2: NNN");
                *set2 = PassBindGroupTextureSamplers(None);
            }
        } else {
            *set2 = PassBindGroupTextureSamplers(None);
        }
    });

    // log::trace!("SysSet0ModifyFromScene: {:?}", pi_time::Instant::now() - time1);
}
