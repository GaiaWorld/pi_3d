

use pi_engine_shell::prelude::*;
use crate::{
    materials::{
        material::{MaterialRefs, DirtyMaterialRefs}, shader_effect::*,
    },
    pass::*
};


use super::{
    uniform::*, 
    // sys_uniform::SysMaterialMetaChange,
    // sys_texture::{SysTextureResReady1, SysTextureResReady2},
};

// pub struct SysEffectValueToModelByMaterialModify<T: TPassID + Component>(PhantomData<T>);
// impl<T: TPassID + Component> TSystemStageInfo for SysEffectValueToModelByMaterialModify<T> {
//     fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
//         vec![
//             // SysMaterailCreateCommands::key(), SysMaterialIDCommand::key(),
//             // SysMaterialMetaChange::<T>::key(), 
//         ]
//     }
// }
// #[setup]
// impl<T: TPassID + Component> SysEffectValueToModelByMaterialModify<T> {
//     #[system]
    pub fn sys_effect_bind_to_model_while_mat_modify(
        materials: Query<
            (&AssetKeyShaderEffect, &AssetResShaderEffectMeta, &BindEffect, &MaterialRefs, &EPassTag),
            Or<(Changed<EPassTag>, Changed<DirtyMaterialRefs>, Changed<BindEffect>)>
        >,
        mut passes: Query<(&mut PassEffectReady, &mut PassBindEffectValue)>,
    ) {
        // log::info!("MaterialBind : ");
        materials.iter().for_each(|(effect_key, effect, bind, list, _pass)| {
            
            // log::info!("MaterialBind : 1");
            list.iter().for_each(|target| {
                // log::info!("MaterialBind : 2");
                if let Ok((mut passready, mut passbind)) = passes.get_mut(target.clone()) {

                    let data = if effect.textures.len() == 0 {
                        Some((effect_key.0.clone(), effect.0.clone()))
                    } else {
                        None
                    };
                    
                    *passready = PassEffectReady(data);
                    *passbind = PassBindEffectValue(Some(bind.bind.clone()));
                }
            });
        });
    }
// }

// pub struct SysEffectTexturesToModelByMaterialModify<T: TPassID + Component>(PhantomData<T>);
// impl<T: TPassID + Component> TSystemStageInfo for SysEffectTexturesToModelByMaterialModify<T> {
//     fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
//         vec![
//             // SysMaterailCreateCommands::key(), SysMaterialIDCommand::key(),
//             SysTextureResReady1::key(), SysTextureResReady2::key(), 
//             // SysTextureResReady3::key(), SysTextureResReady1::key(), 
//         ]
//     }
// }
// #[setup]
// impl<T: TPassID + Component> SysEffectTexturesToModelByMaterialModify<T> {
//     #[system]
    pub fn sys_effect_tex_to_model_while_mat_modify(
        materials: Query<
            (&EffectTextureSamplersComp, &MaterialRefs, &EPassTag, &AssetKeyShaderEffect, &AssetResShaderEffectMeta),
            Or<(Changed<EPassTag>, Changed<DirtyMaterialRefs>, Changed<EffectTextureSamplersComp>, )>
        >,
        mut passes: Query<(&mut PassEffectReady, &mut PassBindEffectTextures)>,
    ) {
        materials.iter().for_each(|(bind, list, _pass, effect_key, meta)| {
            list.iter().for_each(|target| {

                if let Ok((mut passready, mut passbind)) = passes.get_mut(target.clone()) {
                    // let pass = pass.as_pass();
                    // if dirty.0 & pass == 0 {
                    //     dirty.0 += pass;
                    // }
                    // commands.entity(id_obj.clone()).insert(FlagPassDirtyBindEffectTextures);

                    // if pass & T::TAG == T::TAG {
                    //     let data = Some((effect_key.0.clone(), meta.0.clone()));
                    //     commands.entity(passid.id()).insert(PassBindEffectTextures(Some(bind.0.clone()))).insert(PassReady(data));
                    // }
                    let data = Some((effect_key.0.clone(), meta.0.clone()));
                    *passready = PassEffectReady(data);
                    *passbind = PassBindEffectTextures(Some(bind.0.clone()));
                }
            });
        });
    }
// }