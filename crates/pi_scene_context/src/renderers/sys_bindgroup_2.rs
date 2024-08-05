use std::sync::Arc;
use pi_scene_shell::prelude::*;

use crate::pass::*;

use super::base::*;

pub fn sys_set2_modify(
    // mut items: Query<(&PassEffectReady, &PassBindEffectTextures, &mut PassBindGroupTextureSamplers), Or<(Changed<PassBindEffectTextures>, Changed<PassEffectReady>)>>,
    device: Res<PiRenderDevice>,
    asset_mgr_bindgroup_layout: Res<ShareAssetMgr<BindGroupLayout>>,
    asset_mgr_bindgroup: Res<ShareAssetMgr<BindGroup>>,
) {
    // let time1 = pi_time::Instant::now();

    // items.iter_mut().for_each(|(meta1, effect_texture_samplers, mut set2)| {
    //     // log::error!("sys_set2_modify: {:?}", (effect_texture_samplers.val().is_some(), meta1.0.is_some()));
    //     _set2_modify(
    //         meta1,
    //         effect_texture_samplers.val().as_ref(),
    //         &mut set2,
    //         &device,
    //         &asset_mgr_bindgroup_layout,
    //         &asset_mgr_bindgroup,
    //     );
    // });

    // log::trace!("SysSet0ModifyFromScene: {:?}", pi_time::Instant::now() - time1);
}
