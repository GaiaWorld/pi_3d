use std::sync::Arc;
use pi_scene_shell::prelude::*;

use crate::{
    skeleton::prelude::*,
    meshes::prelude::*,
};

use super::base::*;

pub fn _set1_modify(
    idmodel: Entity,
    bind_effect_value: &Option<Arc<ShaderBindEffectValue>>,
    _key_meta: &Atom,
    meta: &Handle<ShaderEffectMeta>,
    models: &Query< ( Option<&BindModel>, &BindSkinValue, &SkeletonID, &ModelLightingIndexs ), >,
    device: &PiRenderDevice,
    asset_mgr_bindgroup_layout: &ShareAssetMgr<BindGroupLayout>,
    asset_mgr_bindgroup: &ShareAssetMgr<BindGroup>,
) -> Option<Arc<BindGroupModel>> {
    let mut result = None;
    let mut bind_skin = None;
    let mut bind_matrix = None;
    let mut bind_lingingsidx = None;

    let bind_effect_value = match (BindDefines::need_effect_value(meta.binddefines), bind_effect_value) {
        (true, Some(bind)) => {
            Some(bind.clone())
        },
        (false, _) => { None },
        _ => { return result; }
    };
    if let Ok( ( bind_model, bind_skl, id_skl, lightingidxs) ) = models.get(idmodel) {
        match (BindDefines::need_model(meta.binddefines), bind_model) {
            (true, Some(bind)) => {
                bind_matrix = Some(bind.0.as_ref().unwrap().clone());
                match (&bind_skl.0, id_skl.0) {
                    (Some(bind), Some(_)) => { bind_skin = Some(bind.clone()); },
                    (None, None) => { },
                    _ => {
                        return result;
                        // log::warn!("Skinnnnnnn");
                    }
                }; 
            },
            (false, _) => { },
            _ => {
                return result;
            }
        };
        match (BindDefines::need_lighting(meta.binddefines), &lightingidxs.bind) {
            (true, Some(lighting)) => {
                bind_lingingsidx = Some(lighting.clone());
            },
            (false, _) => { },
            _ => { return result; }
        };

        let key = KeyBindGroupModel::new(bind_matrix, bind_skin.clone(), bind_effect_value, bind_lingingsidx);

        let key_bind_group = key.key_bind_group();
        // log::warn!("Set0Loaded : ");
        if let Some(bind_group) = create_bind_group(&key_bind_group, &device, &asset_mgr_bindgroup_layout, &asset_mgr_bindgroup) {
            let data = BindGroupModel::new(BindGroupUsage::new(key_bind_group, bind_group), key);
            let data = Arc::new(data);
            // log::error!("create_bind_group 0: Ok");
            // *set0 = PassBindGroupModel(Some(data.clone()));
            result = Some(data.clone());
        } else {
            // log::error!("create_bind_group 0: Error");
        }
    } else {
        // log::error!("create_bind_group 0: Error");
    }

    return result;
}