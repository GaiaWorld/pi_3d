use std::sync::Arc;
use pi_scene_shell::prelude::*;

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
    passindex: usize, matidx: u32,
    errors: &mut ErrorRecord
) -> Option<Arc<BindGroupModel>> {
    let mut result = None;
    let mut bind_skin = None;
    let mut matrix = None;
    let mut matrixinv = None;
    let mut morphinfluence = None;
    let mut skinoffset = None;
    let mut velocity = None;
    let mut bind_lingingsidx = None;

    if let Ok( ( bind_model, bind_skl, id_skl, lightingidxs, modelbinddefines) ) = models.get(idmodel) {
        if let Some(bindmatidx) = bind_model.matidx.clone() {
            let binddefines = meta.binddefines | modelbinddefines.0;
            match (BindDefines::need_model(binddefines), &bind_model.matrix) {
                (true, Some(bind)) => { matrix = Some(bind.clone()); },
                (false, _) => { },
                _ => { errors.record(idmodel, ErrorRecord::ERROR_PASS_BIND_MODEL_NONE); return result; }
            };
            match (BindDefines::need_model_matrix_inv(binddefines), &bind_model.matrixinv) {
                (true, Some(bind)) => { matrixinv = Some(bind.clone()); },
                (false, _) => { },
                _ => { errors.record(idmodel, ErrorRecord::ERROR_PASS_BIND_MODEL_INV_NONE); return result; }
            };
            match (BindDefines::need_model_morphinfluence(binddefines), &bind_model.morphinfluence) {
                (true, Some(bind)) => { morphinfluence = Some(bind.clone()); },
                (false, _) => { },
                _ => { errors.record(idmodel, ErrorRecord::ERROR_PASS_BIND_MORPH_NONE); return result; }
            };
            match (BindDefines::need_model_skin_ins(binddefines), &bind_model.skinoff) {
                (true, Some(bind)) => { skinoffset = Some(bind.clone()); },
                (false, _) => { },
                _ => { errors.record(idmodel, ErrorRecord::ERROR_PASS_BIND_SKININS_NONE); return result; }
            };
            match (BindDefines::need_model_velocity(binddefines), &bind_model.velocity) {
                (true, Some(bind)) => { velocity = Some(bind.clone()); },
                (false, _) => { },
                _ => { errors.record(idmodel, ErrorRecord::ERROR_PASS_BIND_VELOCITY_NONE); return result; }
            };
            match (&bind_skl.0, id_skl.0) {
                (Some(bind), Some(_)) => { bind_skin = Some(bind.clone()); },
                (None, None) => { },
                _ => {
                    errors.record(idmodel, ErrorRecord::ERROR_PASS_BIND_SKIN_NONE); 
                    return result;
                }
            }; 
            match (BindDefines::need_lighting(binddefines), &lightingidxs.bind) {
                (true, Some(lighting)) => {
                    bind_lingingsidx = Some(lighting.clone());
                },
                (false, _) => { },
                _ => { 
                    errors.record(idmodel, ErrorRecord::ERROR_PASS_BIND_LIGHTING_NONE); 
                    return result;
                }
            };
    
            let key = KeyBindGroupModel::new(bindmatidx, matrix, matrixinv, morphinfluence, skinoffset, velocity, bind_skin.clone(), bind_lingingsidx);
    
            let key_bind_group = key.key_bind_group();
            // log::warn!("Set0Loaded : ");
            if let Some(bind_group) = create_bind_group(&key_bind_group, &device, &asset_mgr_bindgroup_layout, &asset_mgr_bindgroup) {
                let data = BindGroupModel::new(BindGroupUsage::new(key_bind_group, bind_group), key);
                let data = Arc::new(data);
                // log::error!("create_bind_group 0: Ok");
                // *set0 = PassBindGroupModel(Some(data.clone()));
                result = Some(data.clone());
            } else {
                errors.record(idmodel, ErrorRecord::ERROR_PASS_SET1_FAIL); 
                // log::error!("create_bind_group 0: Error");
            }
        }
    } else {
        // log::error!("create_bind_group 0: Error");
    }

    return result;
}