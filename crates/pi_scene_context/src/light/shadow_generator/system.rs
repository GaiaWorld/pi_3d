
use pi_engine_shell::prelude::*;

use crate::{
    materials::prelude::*,
    pass::*,
    viewer::prelude::*,
};

use super::base::{ShadowEnable, ShadowMinZ, ShadowMaxZ, ShadowBias, ShadowNormalBias, ShadowDepthScale, KEY_SHADOW_DEPTH_BIAS, KEY_SHADOW_DEPTH_SCALE, KEY_SHADOW_NORMAL_BIAS, KEY_SHADOW_MINZ, KEY_SHADOW_MAXZ};

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
            Changed<BindEffectReset>,
        >,
    ) {
        shadows.iter().for_each(|(id_mat, _enable, minz, maxz, bias, normal_bias, depth_scale)| {
            if let Ok((mut bind, mut flag)) = materails.get_mut(id_mat.0.clone()) {
                bind.vec4(0, &[bias.0, normal_bias.0, depth_scale.0, 0.]);
                bind.vec2(0, &[minz.0, minz.0 + maxz.0]);
                *flag = BindEffectValueDirty;
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
    ) {
        shadows.iter().for_each(|(id_mat, enable, minz, maxz, bias, normal_bias, depth_scale)| {
            if enable.0 {
                if let Ok((mut bind, mut flag)) = materails.get_mut(id_mat.0.clone()) {
                    if let Some(slot) = bind.slot(&Atom::from(KEY_SHADOW_DEPTH_BIAS)) {
                        bind.float(slot, bias.0);
                    }
                    if let Some(slot) = bind.slot(&Atom::from(KEY_SHADOW_DEPTH_SCALE)) {
                        bind.float(slot, depth_scale.0);
                    }
                    if let Some(slot) = bind.slot(&Atom::from(KEY_SHADOW_NORMAL_BIAS)) {
                        bind.float(slot, normal_bias.0);
                    }
                    if let Some(slot) = bind.slot(&Atom::from(KEY_SHADOW_MINZ)) {
                        bind.float(slot, minz.0);
                    }
                    if let Some(slot) = bind.slot(&Atom::from(KEY_SHADOW_MAXZ)) {
                        bind.float(slot, maxz.0 + minz.0);
                    }
                    // log::warn!("Shadow: {:?}", (bias.0, depth_scale.0, normal_bias.0, maxz.0, minz.0));
                    *flag = BindEffectValueDirty;
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
        mut matcmds: ResMut<ActionListMaterialUse>,
        mut materials: Query<(&EPassTag, &mut MaterialRefs, &mut DirtyMaterialRefs)>,
        meshes: Query<&P>,
    ) {
        shadows.iter().for_each(|(id_mat, enable, modelist)| {
            if let Ok((pass, mut materialrefs, mut flag)) = materials.get_mut(id_mat.0) {
                if P::TAG == pass.as_pass() && enable.0 {
                    modelist.0.iter().for_each(|id_model| {
                        if let Ok(pass) = meshes.get(*id_model) {
                            
                            // if let Some(mut cmd) = commands.get_entity(pass.id()) {
                            //     cmd.insert(id_mat.clone());

                            //     matcmds.push(OpsMaterialUse::Use((), id_mat.0));

                            //     materialrefs.insert(pass.id());
                            //     *flag = DirtyMaterialRefs(true);
                            // }

                            matcmds.push(OpsMaterialUse::Use(*id_model, id_mat.0));

                            materialrefs.insert(pass.id());
                            *flag = DirtyMaterialRefs(true);
                        }
                    });
                }
            }
        });
    }
// }

