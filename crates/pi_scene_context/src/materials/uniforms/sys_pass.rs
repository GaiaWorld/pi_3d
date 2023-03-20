

use std::marker::PhantomData;

use pi_ecs::{prelude::{Query, Commands, Component}, query::{Changed, Or}};
use pi_ecs_macros::setup;
use pi_engine_shell::{object::{GameObject}, run_stage::TSystemStageInfo};
use pi_render::render_3d::bind_groups::texture_sampler::EffectTextureSamplers;
use crate::{
    materials::{
        material::{MaterialUsedList}, shader_effect::{AssetKeyShaderEffect, AssetResShaderEffectMeta},
    },
    pass::*
};


use super::{
    uniform::{BindEffectValues}, 
    sys_uniform::SysMaterialMetaChange,
    sys_texture::{SysTextureResReady1, SysTextureResReady2},
};

pub struct SysEffectValueToModelByMaterialModify<T: TPassID + Component>(PhantomData<T>);
impl<T: TPassID + Component> TSystemStageInfo for SysEffectValueToModelByMaterialModify<T> {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            // SysMaterailCreateCommands::key(), SysMaterialIDCommand::key(),
            // SysMaterialMetaChange::<T>::key(), 
        ]
    }
}
#[setup]
impl<T: TPassID + Component> SysEffectValueToModelByMaterialModify<T> {
    #[system]
    fn sys(
        materials: Query<
            GameObject,
            (&AssetKeyShaderEffect, &AssetResShaderEffectMeta, &BindEffectValues, &MaterialUsedList, &EPassTag),
            Or<(Changed<EPassTag>, Changed<MaterialUsedList>, Changed<BindEffectValues>)>
        >,
        mut models: Query<GameObject, (&T, &mut PassDirtyBindEffectValue)>,
        mut dirty_cmd: Commands<GameObject, FlagPassDirtyBindEffectValue>,
        mut pass01_cmd: Commands<GameObject, PassBindEffectValue>,
        mut passready_cmd: Commands<GameObject, PassReady>,
    ) {
        materials.iter().for_each(|(effect_key, effect, bind, list, pass)| {
            list.0.iter().for_each(|(id_obj, _)| {
                if let Some((passid, mut dirty)) = models.get_mut(id_obj.clone()) {
                    let pass = pass.as_pass();
                    if dirty.0 & pass == 0 {
                        dirty.0 += pass;
                    }
                    dirty_cmd.insert(id_obj.clone(), FlagPassDirtyBindEffectValue);

                    let data = if effect.textures.len() == 0 {
                        Some((effect_key.0.clone(), effect.0.clone()))
                    } else {
                        None
                    };
                    // list_model.0.iter().for_each(|(id_obj, _)| {
                    //     if let Some(passid) = models.get(id_obj.clone()) {
                    //         ready01_cmd.insert(passid.id(), PassReady(data.clone()));
                    //     }
                    // });

                    if pass & T::TAG == T::TAG {
                        passready_cmd.insert(passid.id(), PassReady(data));
                        pass01_cmd.insert(passid.id(), PassBindEffectValue(Some(bind.bind.clone())));
                    }
                }
            });
        });
    }
}

pub struct SysEffectTexturesToModelByMaterialModify<T: TPassID + Component>(PhantomData<T>);
impl<T: TPassID + Component> TSystemStageInfo for SysEffectTexturesToModelByMaterialModify<T> {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            // SysMaterailCreateCommands::key(), SysMaterialIDCommand::key(),
            SysTextureResReady1::key(), SysTextureResReady2::key(), 
            // SysTextureResReady3::key(), SysTextureResReady1::key(), 
        ]
    }
}
#[setup]
impl<T: TPassID + Component> SysEffectTexturesToModelByMaterialModify<T> {
    #[system]
    pub fn sys(
        materials: Query<
            GameObject,
            (&EffectTextureSamplers, &MaterialUsedList, &EPassTag, &AssetKeyShaderEffect, &AssetResShaderEffectMeta),
            Or<(Changed<EPassTag>, Changed<MaterialUsedList>, Changed<EffectTextureSamplers>, )>
        >,
        mut models: Query<GameObject, (&T, &mut PassDirtyBindEffectTextures)>,
        mut dirty_cmd: Commands<GameObject, FlagPassDirtyBindEffectTextures>,
        mut pass01_cmd: Commands<GameObject, PassBindEffectTextures>,
        mut ready01_cmd: Commands<GameObject, PassReady>,
    ) {
        materials.iter().for_each(|(bind, list, pass, effect_key, meta)| {
            list.0.iter().for_each(|(id_obj, _)| {
                log::info!("set_model_effect_texture_samplers >>");
                if let Some((passid, mut dirty)) = models.get_mut(id_obj.clone()) {
                    let pass = pass.as_pass();
                    if dirty.0 & pass == 0 {
                        dirty.0 += pass;
                    }
                    dirty_cmd.insert(id_obj.clone(), FlagPassDirtyBindEffectTextures);
                    
                    
                    if pass & T::TAG == T::TAG {
                        let data = Some((effect_key.0.clone(), meta.0.clone()));
                        pass01_cmd.insert(passid.id(), PassBindEffectTextures(Some(bind.clone()))); 
                        ready01_cmd.insert(passid.id(), PassReady(data));
                    }
                }
            });
        });
    }
}