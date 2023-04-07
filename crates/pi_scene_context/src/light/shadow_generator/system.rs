
use pi_engine_shell::prelude::*;

use crate::{
    materials::{
        material::{MaterialID, MaterialUsedList},
        uniforms::{uniform::{BindEffectValues, BindEffectValueDirty}},
        command::{EMaterialIDCommand, ActionMaterial}
    },
    pass::{Pass01, PassID01},
    viewer::{
        ModelList,
        FlagModelList,
    }
};

use super::base::{ShadowEnable, ShadowMinZ, ShadowMaxZ, ShadowBias, ShadowNormalBias, ShadowDepthScale};


// pub struct SysShadowParamUpdateWhileMatCreate;
// impl TSystemStageInfo for SysShadowParamUpdateWhileMatCreate {
//     fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
//         vec![
//             SysMaterialMetaChange::<PassID01>::key()
//         ]
//     }
// }
// #[setup]
// impl SysShadowParamUpdateWhileMatCreate {
//     #[system]
    pub fn sys_shadow_param_update_while_mat_create(
        shadows: Query<
            (&MaterialID, &ShadowEnable, &ShadowMinZ, &ShadowMaxZ, &ShadowBias, &ShadowNormalBias, &ShadowDepthScale),
        >,
        mut materails: Query<
            &mut BindEffectValues,
            Changed<BindEffectValues>,
        >,
        mut bind_dirty: Commands<BindEffectValueDirty>,
    ) {
        shadows.iter().for_each(|(id_mat, enable, minz, maxz, bias, normal_bias, depth_scale)| {
            if let Some(mut bind) = materails.get_mut(id_mat.0.clone()) {
                bind.vec4(0, &[**bias, **normal_bias, **depth_scale, 0.]);
                bind.vec2(0, &[**minz, **minz + **maxz]);
                bind_dirty.insert(id_mat.0.clone(), BindEffectValueDirty(true));
            }
        });
    }
// }

// pub struct SysShadowParamUpdate;
// impl TSystemStageInfo for SysShadowParamUpdate {
//     fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
//         vec![
//             SysMaterialMetaChange::<PassID01>::key()
//         ]
//     }
// }
// #[setup]
// impl SysShadowParamUpdate {
//     #[system]
    pub fn sys_shadow_param_update(
        shadows: Query<
            (&MaterialID, &ShadowEnable, &ShadowMinZ, &ShadowMaxZ, &ShadowBias, &ShadowNormalBias, &ShadowDepthScale),
            Or<(
                Changed<MaterialID>, Changed<ShadowEnable>, Changed<ShadowMinZ>, Changed<ShadowMaxZ>, Changed<ShadowBias>, Changed<ShadowNormalBias>, Changed<ShadowDepthScale>, 
            )>
        >,
        mut materails: Query<
            &mut BindEffectValues
        >,
        mut commands: Commands,
    ) {
        shadows.iter().for_each(|(id_mat, enable, minz, maxz, bias, normal_bias, depth_scale)| {
            if enable.0 {
                if let Some(mut bind) = materails.get_mut(id_mat.0.clone()) {
                    bind.vec4(0, &[**bias, **normal_bias, **depth_scale, 0.]);
                    bind.vec2(0, &[**minz, **minz + **maxz]);
                    commands.entity(id_mat.0).insert(BindEffectValueDirty(true));
                }
            }
        });
    }
// }

// pub struct SysShadowGeneratorAppyWhileShadowModify;
// impl TSystemStageInfo for SysShadowGeneratorAppyWhileShadowModify {
//     fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
//         vec![
//             SysModelListUpdateByModel::key(),
//             SysModelListUpdateByViewer::key(),
//         ]
//     }
// }
// #[setup]
// impl SysShadowGeneratorAppyWhileShadowModify {
//     #[system]
    pub fn sys_shadow_generator_apply_while_shadow_modify(
        shadows: Query<
            (&MaterialID, &ShadowEnable, &ModelList),
            Or<(
                Changed<MaterialID>, Changed<ShadowEnable>, Changed<FlagModelList>, 
            )>
        >,
        mut materail: Query<&mut MaterialUsedList>,
        mut commands: Commands,
    ) {
        shadows.iter().for_each(|(id_mat, enable, modelist)| {
            if enable.0 {
                modelist.0.iter().for_each(|(id_model, _)| {
                    if let Some(mut uselist) = materail.get_mut(id_mat.0) {
                        let mut entitycmd = commands.entity(id_mat.0);
                        ActionMaterial::use_material(&mut entitycmd, id_model.0, &mut uselist);
                    }
                });
            } else {
                
            }
        });
    }
// }

