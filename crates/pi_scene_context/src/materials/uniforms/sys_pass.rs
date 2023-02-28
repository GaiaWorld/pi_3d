

use pi_ecs::{prelude::{Query, Commands}, query::{Changed, Or}};
use pi_ecs_macros::setup;
use pi_engine_shell::{object::{GameObject, ObjectID}, run_stage::TSystemStageInfo};
use pi_render::render_3d::bind_groups::texture_sampler::EffectTextureSamplers;
use crate::{
    materials::{
        value::FromValueUniformStatistics,
        material::{MaterialUsedList}, shader_effect::{AssetKeyShaderEffect, AssetResShaderEffectMeta},
    },
    pass::{
        EPassTag,
        Pass01BindEffectValue, Pass02BindEffectValue, Pass03BindEffectValue, Pass04BindEffectValue, Pass05BindEffectValue, Pass06BindEffectValue, Pass07BindEffectValue, Pass08BindEffectValue,
        Pass01BindEffectTextures, Pass02BindEffectTextures, Pass03BindEffectTextures, Pass04BindEffectTextures, Pass05BindEffectTextures, Pass06BindEffectTextures, Pass07BindEffectTextures, Pass08BindEffectTextures,
        Pass01Ready, Pass02Ready, Pass03Ready, Pass04Ready, Pass05Ready, Pass06Ready, Pass07Ready, Pass08Ready
    }
};


use super::{
    uniform::{BindEffectValues}, 
    sys_uniform::SysMaterialMetaChange,
    sys_texture::{SysTextureResReady1, SysTextureResReady2},
};

pub struct SysEffectValueToModelByMaterialModify;
impl TSystemStageInfo for SysEffectValueToModelByMaterialModify {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            // SysMaterailCreateCommands::key(), SysMaterialIDCommand::key(),
            SysMaterialMetaChange::key(), 
        ]
    }
}
#[setup]
impl SysEffectValueToModelByMaterialModify {
    #[system]
    fn sys(
        materials: Query<
            GameObject,
            (&BindEffectValues, &MaterialUsedList, &EPassTag),
            Or<(Changed<EPassTag>, Changed<MaterialUsedList>, Changed<BindEffectValues>)>
        >,
        mut pass01_cmd: Commands<GameObject, Pass01BindEffectValue>,
        mut pass02_cmd: Commands<GameObject, Pass02BindEffectValue>,
        mut pass03_cmd: Commands<GameObject, Pass03BindEffectValue>,
        mut pass04_cmd: Commands<GameObject, Pass04BindEffectValue>,
        mut pass05_cmd: Commands<GameObject, Pass05BindEffectValue>,
        mut pass06_cmd: Commands<GameObject, Pass06BindEffectValue>,
        mut pass07_cmd: Commands<GameObject, Pass07BindEffectValue>,
        mut pass08_cmd: Commands<GameObject, Pass08BindEffectValue>,
    ) {
        materials.iter().for_each(|(bind, list, pass)| {
            list.0.iter().for_each(|(id_obj, _)| {
                match pass {
                    EPassTag::ShadowCast => {
                        pass01_cmd.insert(id_obj.clone(), Pass01BindEffectValue(Some(bind.bind.clone())));
                    },
                    EPassTag::Opaque => {
                        pass02_cmd.insert(id_obj.clone(), Pass02BindEffectValue(Some(bind.bind.clone())))
                    },
                    EPassTag::Sky => {
                        pass03_cmd.insert(id_obj.clone(), Pass03BindEffectValue(Some(bind.bind.clone())))
                    },
                    EPassTag::Water => {
                        pass04_cmd.insert(id_obj.clone(), Pass04BindEffectValue(Some(bind.bind.clone())))
                    },
                    EPassTag::Transparent => {
                        pass05_cmd.insert(id_obj.clone(), Pass05BindEffectValue(Some(bind.bind.clone())))
                    },
                    EPassTag::AlphaTest => {
                        pass06_cmd.insert(id_obj.clone(), Pass06BindEffectValue(Some(bind.bind.clone())))
                    },
                    EPassTag::OpaqueExtend => {
                        pass07_cmd.insert(id_obj.clone(), Pass07BindEffectValue(Some(bind.bind.clone())))
                    },
                    EPassTag::TransparentExtend => {
                        pass08_cmd.insert(id_obj.clone(), Pass08BindEffectValue(Some(bind.bind.clone())))
                    },
                }
            });
        });
    }
}

fn set_model_effect_texture_samplers(
    id_obj: &ObjectID,
    effect_key: &AssetKeyShaderEffect,
    meta: &AssetResShaderEffectMeta,
    bind: &EffectTextureSamplers,
    pass: &EPassTag,
    binds_cmd: (
        &mut Commands<GameObject, Pass01BindEffectTextures>,
        &mut Commands<GameObject, Pass02BindEffectTextures>,
        &mut Commands<GameObject, Pass03BindEffectTextures>,
        &mut Commands<GameObject, Pass04BindEffectTextures>,
        &mut Commands<GameObject, Pass05BindEffectTextures>,
        &mut Commands<GameObject, Pass06BindEffectTextures>,
        &mut Commands<GameObject, Pass07BindEffectTextures>,
        &mut Commands<GameObject, Pass08BindEffectTextures>,
    ),
    readys_cmd: (
        &mut Commands<GameObject, Pass01Ready>,
        &mut Commands<GameObject, Pass02Ready>,
        &mut Commands<GameObject, Pass03Ready>,
        &mut Commands<GameObject, Pass04Ready>,
        &mut Commands<GameObject, Pass05Ready>,
        &mut Commands<GameObject, Pass06Ready>,
        &mut Commands<GameObject, Pass07Ready>,
        &mut Commands<GameObject, Pass08Ready>,
    )
) {
    let (
        pass01_cmd, pass02_cmd, pass03_cmd, pass04_cmd, 
        pass05_cmd, pass06_cmd, pass07_cmd, pass08_cmd
    ) = binds_cmd;
    let (
        ready01_cmd, ready02_cmd, ready03_cmd, ready04_cmd, 
        ready05_cmd, ready06_cmd, ready07_cmd, ready08_cmd
    ) = readys_cmd;
    
            
    let data = Some((effect_key.0.clone(), meta.0.clone()));
    match pass {
        EPassTag::ShadowCast => {
            pass01_cmd.insert(id_obj.clone(), Pass01BindEffectTextures(Some(bind.clone()))); 
            ready01_cmd.insert(id_obj.clone(), Pass01Ready(data));
        },
        EPassTag::Opaque => {
            pass02_cmd.insert(id_obj.clone(), Pass02BindEffectTextures(Some(bind.clone()))); 
            ready02_cmd.insert(id_obj.clone(), Pass02Ready(data));
        },
        EPassTag::Sky => {
            pass03_cmd.insert(id_obj.clone(), Pass03BindEffectTextures(Some(bind.clone()))); 
            ready03_cmd.insert(id_obj.clone(), Pass03Ready(data));
        },
        EPassTag::Water => {
            pass04_cmd.insert(id_obj.clone(), Pass04BindEffectTextures(Some(bind.clone()))); 
            ready04_cmd.insert(id_obj.clone(), Pass04Ready(data));
        },
        EPassTag::Transparent => {
            pass05_cmd.insert(id_obj.clone(), Pass05BindEffectTextures(Some(bind.clone()))); 
            ready05_cmd.insert(id_obj.clone(), Pass05Ready(data));
        },
        EPassTag::AlphaTest => {
            pass06_cmd.insert(id_obj.clone(), Pass06BindEffectTextures(Some(bind.clone()))); 
            ready06_cmd.insert(id_obj.clone(), Pass06Ready(data));
        },
        EPassTag::OpaqueExtend => {
            pass07_cmd.insert(id_obj.clone(), Pass07BindEffectTextures(Some(bind.clone()))); 
            ready07_cmd.insert(id_obj.clone(), Pass07Ready(data));
        },
        EPassTag::TransparentExtend => {
            pass08_cmd.insert(id_obj.clone(), Pass08BindEffectTextures(Some(bind.clone()))); 
            ready08_cmd.insert(id_obj.clone(), Pass08Ready(data));
        },
    }
}

pub struct SysEffectTexturesToModelByMaterialModify;
impl TSystemStageInfo for SysEffectTexturesToModelByMaterialModify {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            // SysMaterailCreateCommands::key(), SysMaterialIDCommand::key(),
            SysTextureResReady1::key(), SysTextureResReady2::key(), 
            // SysTextureResReady3::key(), SysTextureResReady1::key(), 
        ]
    }
}
#[setup]
impl SysEffectTexturesToModelByMaterialModify {
    #[system]
    pub fn sys(
        materials: Query<
            GameObject,
            (&EffectTextureSamplers, &MaterialUsedList, &EPassTag, &AssetKeyShaderEffect, &AssetResShaderEffectMeta),
            Or<(Changed<EPassTag>, Changed<MaterialUsedList>, Changed<EffectTextureSamplers>, )>
        >,
        mut pass01_cmd: Commands<GameObject, Pass01BindEffectTextures>,
        mut pass02_cmd: Commands<GameObject, Pass02BindEffectTextures>,
        mut pass03_cmd: Commands<GameObject, Pass03BindEffectTextures>,
        mut pass04_cmd: Commands<GameObject, Pass04BindEffectTextures>,
        mut pass05_cmd: Commands<GameObject, Pass05BindEffectTextures>,
        mut pass06_cmd: Commands<GameObject, Pass06BindEffectTextures>,
        mut pass07_cmd: Commands<GameObject, Pass07BindEffectTextures>,
        mut pass08_cmd: Commands<GameObject, Pass08BindEffectTextures>,
        mut ready01_cmd: Commands<GameObject, Pass01Ready>,
        mut ready02_cmd: Commands<GameObject, Pass02Ready>,
        mut ready03_cmd: Commands<GameObject, Pass03Ready>,
        mut ready04_cmd: Commands<GameObject, Pass04Ready>,
        mut ready05_cmd: Commands<GameObject, Pass05Ready>,
        mut ready06_cmd: Commands<GameObject, Pass06Ready>,
        mut ready07_cmd: Commands<GameObject, Pass07Ready>,
        mut ready08_cmd: Commands<GameObject, Pass08Ready>,
    ) {
        materials.iter().for_each(|(bind, list, pass, effect_key, meta)| {
            list.0.iter().for_each(|(id_obj, _)| {
                
                log::info!("set_model_effect_texture_samplers >>");
                set_model_effect_texture_samplers(
                    id_obj, 
                    effect_key, meta,
                    bind, 
                    pass,
                    (
                        &mut pass01_cmd, &mut pass02_cmd, &mut pass03_cmd, &mut pass04_cmd, 
                        &mut pass05_cmd, &mut pass06_cmd, &mut pass07_cmd, &mut pass08_cmd
                    ),
                    (
                        &mut ready01_cmd, &mut ready02_cmd, &mut ready03_cmd, &mut ready04_cmd, 
                        &mut ready05_cmd, &mut ready06_cmd, &mut ready07_cmd, &mut ready08_cmd
                    )
                );
            });
        });
    }
}