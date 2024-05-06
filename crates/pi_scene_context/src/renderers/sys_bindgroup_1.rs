use std::sync::Arc;
use pi_scene_shell::prelude::*;

use crate::{
    skeleton::prelude::*,
    meshes::prelude::*,
    pass::*,
};

use super::base::*;

pub fn sys_set1_modify(
    mut items: Query<(&PassModelID, &PassBindEffectValue, &PassEffectReady, &mut PassBindGroupModel), (Changed<PassModelID>, Changed<PassEffectReady>, Changed<PassBindEffectValue>)>,
    models: Query<
        (
            Option<&BindModel>, &BindSkinValue, Option<&SkeletonID>, &ModelLightingIndexs
        ),
    >,
    device: Res<PiRenderDevice>,
    asset_mgr_bindgroup_layout: Res<ShareAssetMgr<BindGroupLayout>>,
    asset_mgr_bindgroup: Res<ShareAssetMgr<BindGroup>>,
) {
    let time1 = pi_time::Instant::now();

    items.iter_mut().for_each(|(idmodel, bind_effect_value, effect_ready, mut set0)| {
        match effect_ready.val() {
            Some((_key_meta, meta)) => {
                let mut bind_skin = None;
                let mut bind_matrix = None;
                let mut bind_lingingsidx = None;

                let bind_effect_value = match (BindDefines::need_effect_value(meta.binddefines), &bind_effect_value.0) {
                    (true, Some(bind)) => {
                        Some(bind.clone())
                    },
                    (false, _) => { None },
                    _ => { *set0 = PassBindGroupModel(None); return; }
                };
                if let Ok( ( bind_model, bind_skl, id_skl, lightingidxs) ) = models.get(idmodel.0) {
                    match (BindDefines::need_model(meta.binddefines), bind_model) {
                        (true, Some(bind)) => {
                            bind_matrix = Some(bind.0.clone());
                            match (&bind_skl.0, id_skl) {
                                (Some(bind), Some(_)) => { bind_skin = Some(bind.clone()); },
                                (None, None) => { },
                                _ => {
                                    *set0 = PassBindGroupModel(None);
                                    return;
                                    // log::warn!("Skinnnnnnn");
                                }
                            }; 
                        },
                        (false, _) => { },
                        _ => {
                            *set0 = PassBindGroupModel(None);
                            return;
                        }
                    };
                    match (BindDefines::need_lighting(meta.binddefines), &lightingidxs.bind) {
                        (true, Some(lighting)) => {
                            bind_lingingsidx = Some(lighting.clone());
                        },
                        (false, _) => { },
                        _ => { *set0 = PassBindGroupModel(None); return; }
                    };

                    let key = KeyBindGroupModel::new(bind_matrix, bind_skin.clone(), bind_effect_value, bind_lingingsidx);

                    let key_bind_group = key.key_bind_group();
                    // log::warn!("Set0Loaded : ");
                    if let Some(bind_group) = create_bind_group(&key_bind_group, &device, &asset_mgr_bindgroup_layout, &asset_mgr_bindgroup) {
                        let data = BindGroupModel::new(BindGroupUsage::new(key_bind_group, bind_group), key);
                        let data = Arc::new(data);
                        // log::error!("create_bind_group 0: Ok");
                        // *set0 = PassBindGroupModel(Some(data.clone()));
                        if let Some(old) = &set0.0 {
                            if old.key() != data.key() { *set0 = PassBindGroupModel(Some(data.clone())); }
                        } else { *set0 = PassBindGroupModel(Some(data.clone())); }
                    } else {
                        // log::error!("create_bind_group 0: Error");
                        *set0 = PassBindGroupModel(None);
                    }
                } else {
                    // log::error!("create_bind_group 0: Error");
                    *set0 = PassBindGroupModel(None);
                }
            },
            None => {
                // log::error!("create_bind_group 0: Error");
                *set0 = PassBindGroupModel(None);
            }
        }
    });

    // log::trace!("SysSet0ModifyFromScene: {:?}", pi_time::Instant::now() - time1);
}