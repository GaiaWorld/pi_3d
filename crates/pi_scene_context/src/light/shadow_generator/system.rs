
use pi_engine_shell::prelude::*;

use crate::{
    materials::{
        prelude::*,
        command_sys::*
    },
    pass::*,
    viewer::prelude::*,
    pass::*
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
            (&mut BindEffect, &mut BindEffectValueDirty),
            Changed<BindEffect>,
        >,
        mut commads: Commands,
    ) {
        shadows.iter().for_each(|(id_mat, enable, minz, maxz, bias, normal_bias, depth_scale)| {
            if let Ok((mut bind, mut flag)) = materails.get_mut(id_mat.0.clone()) {
                if let Some(bind) = &mut bind.0 {
                    bind.vec4(0, &[**bias, **normal_bias, **depth_scale, 0.]);
                    bind.vec2(0, &[**minz, **minz + **maxz]);
                    *flag = BindEffectValueDirty(true);
                }
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
            (&mut BindEffect, &mut BindEffectValueDirty)
        >,
        mut commands: Commands,
    ) {
        shadows.iter().for_each(|(id_mat, enable, minz, maxz, bias, normal_bias, depth_scale)| {
            if enable.0 {
                if let Ok((mut bind, mut flag)) = materails.get_mut(id_mat.0.clone()) {
                    if let Some(bind) = &mut bind.0 {
                        bind.vec4(0, &[**bias, **normal_bias, **depth_scale, 0.]);
                        bind.vec2(0, &[**minz, **minz + **maxz]);
                        *flag = BindEffectValueDirty(true);
                    }
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
    pub fn sys_shadow_generator_apply_while_shadow_modify<P: TPassID + Component>(
        shadows: Query<
            (&MaterialID, &ShadowEnable, &ModelList),
            Or<(
                Changed<MaterialID>, Changed<ShadowEnable>, Changed<FlagModelList>, 
            )>
        >,
        mut commands: Commands,
        mut materials: Query<(&EPassTag, &mut MaterialRefs, &mut DirtyMaterialRefs)>,
        meshes: Query<&P>,
    ) {
        shadows.iter().for_each(|(id_mat, enable, modelist)| {
            if let Ok((pass, mut materialrefs, mut flag)) = materials.get_mut(id_mat.0) {
                if P::TAG == pass.as_pass() && enable.0 {
                    modelist.0.iter().for_each(|(id_model, _)| {
                        if let Ok(pass) = meshes.get(*id_model) {
                            commands.entity(pass.id()).insert(id_mat.clone());
                            materialrefs.insert(pass.id());
                            *flag = DirtyMaterialRefs(true);
                        }
                    });
                }
            }
        });
    }
// }

