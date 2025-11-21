use std::sync::Arc;
use pi_scene_shell::prelude::*;
use pi_slotmap::Key;

use crate::{
    skeleton::prelude::*,
    meshes::prelude::*,
};

use super::base::*;

pub fn _set1_modify(
    idmodel: Entity,
    _key_meta: &Atom,
    meta: &Handle<ShaderEffectMeta>,
    models: &Query< ( &BindModel, &BindSkinValue, &SkeletonID, &ModelLightingIndexs, &ModelBindDefines  ), >,
    device: &PiRenderDevice,
    asset_mgr_bindgroup_layout: &ShareAssetMgr<BindGroupLayout>,
    asset_mgr_bindgroup: &ShareAssetMgr<BindGroup>,
    errors: &mut ErrorRecord
) -> Option<Arc<BindGroupModel>> {
    let mut result = None;
    let mut bind_skin = None;
    let mut bind_lingingsidx = None;

    if let Ok( ( bind_model, bind_skl, id_skl, lightingidxs, modelbinddefines) ) = models.get(idmodel) {
            let binddefines = meta.binddefines | modelbinddefines.0;
            match (&bind_skl.0, id_skl.0) {
                (Some(bind), Some(_)) => { bind_skin = Some(bind.clone()); },
                (None, None) => { },
                _ => {
                    errors.record(idmodel.index(), ErrorRecord::ERROR_PASS_BIND_SKIN_NONE); 
                    return result;
                }
            }; 
            match (BindDefines::need_lighting(binddefines), &lightingidxs.bind) {
                (true, Some(lighting)) => {
                    bind_lingingsidx = Some(lighting.clone());
                },
                (false, _) => { },
                _ => { 
                    errors.record(idmodel.index(), ErrorRecord::ERROR_PASS_BIND_LIGHTING_NONE); 
                    return result;
                }
            };
    
            let key = KeyBindGroupModel::new(bind_model.matrix.clone(), bind_skin.clone(), bind_lingingsidx);
    
            let key_bind_group = key.key_bind_group();
            if let Some(bind_group) = create_bind_group(&key_bind_group, &device, &asset_mgr_bindgroup_layout, &asset_mgr_bindgroup) {
                let data = BindGroupModel::new(BindGroupUsage::new(key_bind_group, bind_group), key);
                let data = Arc::new(data);
                result = Some(data.clone());
            } else {
                errors.record(idmodel.index(), ErrorRecord::ERROR_PASS_SET1_FAIL); 
                log::warn!("create_bind_group 0: Error");
            }
    } else {
        log::warn!("create_bind_group 0: Error");
    }

    return result;
}