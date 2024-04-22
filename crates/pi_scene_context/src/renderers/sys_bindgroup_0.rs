use std::sync::Arc;
use pi_scene_shell::prelude::*;

use crate::{
    scene::environment::BindSceneEffect,
    viewer::prelude::*,
    pass::*,
};

use super::base::*;

pub fn sys_set0_modify(
    mut items: Query<(&PassModelID, &PassSceneID, &PassViewerID, &PassEffectReady, &mut PassBindGroupScene), Or<(Changed<PassSceneID>, Changed<PassViewerID>, Changed<PassEffectReady>)>>,
    viewers: Query<Option<&BindViewer>>,
    scenes: Query<&BindSceneEffect>,
    device: Res<PiRenderDevice>,
    asset_mgr_bindgroup_layout: Res<ShareAssetMgr<BindGroupLayout>>,
    asset_mgr_bindgroup: Res<ShareAssetMgr<BindGroup>>,
    mut errors: ResMut<ErrorRecord>,
) {
    let time1 = pi_time::Instant::now();

    items.iter_mut().for_each(|(idmodel, idscene, idviewer, meta, mut set0)| {
        // log::warn!("Set0 Modify 1, {:?}", (scenes.get(idscene.0).is_ok(), viewers.get(idviewer.0).is_ok()));
        if let (Ok(bind_base_effect), Ok(bind_viewer)) = (scenes.get(idscene.0), viewers.get(idviewer.0)) {
            // log::warn!("Set0 Modify 2");
            if let Some((_, meta)) = &meta.0 {
                // log::warn!("Set0 Modify 3");
                let bind_base_effect = if BindDefines::need_scene_effect(meta.binddefines) {
                    Some(bind_base_effect.0.clone())
                } else { None };

                let bind_viewer = match (BindDefines::need_viewer(meta.binddefines), bind_viewer) {
                    (true, Some(bindviewer)) => {
                        Some(bindviewer.0.clone())
                    },
                    (false, _) => {
                        None
                    },
                    _ => {
                        errors.record(idmodel.0, ErrorRecord::ERROR_PASS_BIND_VIEWER_NONE);
                        *set0 = PassBindGroupScene(None);
                        return;
                    }
                };

                let key = KeyBindGroupScene::new(bind_viewer, bind_base_effect);
                let key_bind_group = key.key_bind_group();
                // log::warn!("Set0Loaded : {:?}", key_bind_group);
                if let Some(bind_group) = create_bind_group(&key_bind_group, &device, &asset_mgr_bindgroup_layout, &asset_mgr_bindgroup) {
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