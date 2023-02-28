

use std::{mem::replace, sync::Arc};

use pi_assets::mgr::AssetMgr;
use pi_ecs::{prelude::{ResMut, Query, Res, Commands}, query::{Changed, Or, With}};
use pi_ecs_macros::setup;
use pi_engine_shell::{object::{GameObject, ObjectID}, run_stage::TSystemStageInfo};
use pi_render::{rhi::{device::RenderDevice}, renderer::{bind_buffer::{BindBufferAllocator}, sampler::{SamplerRes, BindDataSampler}}, render_3d::{shader::uniform_texture::UniformTextureWithSamplerParam, binds::effect_sampler2d::{EffectBindSampler2D01, EffectBindSampler2D02, EffectBindSampler2D03, EffectBindSampler2D04}}};
use pi_share::Share;
use crate::{
    materials::{
        value::FromValueUniformStatistics,
        shader_effect::{AssetKeyShaderEffect},
        material::{MaterialUsedList},
        command::{SysMaterialIDCommand, SysMaterailCreateCommands, SysAssetShaderEffectLoad}
    },
    pass::{
        EPassTag,
        Pass01BindEffectValue, Pass02BindEffectValue, Pass03BindEffectValue, Pass04BindEffectValue, Pass05BindEffectValue, Pass06BindEffectValue, Pass07BindEffectValue, Pass08BindEffectValue,
        Pass01BindEffectTextures, Pass02BindEffectTextures, Pass03BindEffectTextures, Pass04BindEffectTextures, Pass05BindEffectTextures, Pass06BindEffectTextures, Pass07BindEffectTextures, Pass08BindEffectTextures,
        Pass01Ready, Pass02Ready, Pass03Ready, Pass04Ready, Pass05Ready, Pass06Ready, Pass07Ready, Pass08Ready
    }
};


use crate::materials::shader_effect::AssetResShaderEffectMeta;

use super::{
    uniform::{BindEffectValues, BindEffectValueDirty},
    texture::{UniformTextureWithSamplerParams, TextureSlot01, TextureSlot02, TextureSlot03, TextureSlot04},
};


#[derive(Debug)]
pub enum EUniformCommand {
    Mat4(ObjectID, usize, pi_scene_math::Matrix, bool),
    Mat2(ObjectID, usize, pi_scene_math::Matrix2, bool),
    Vec4(ObjectID, usize, pi_scene_math::Vector4, bool),
    Vec2(ObjectID, usize, pi_scene_math::Vector2, bool),
    Float(ObjectID, usize, pi_scene_math::Number, bool),
    Int(ObjectID, usize, i32, bool),
    Uint(ObjectID, usize, u32, bool),
    Texture(ObjectID, UniformTextureWithSamplerParam, bool),
}

#[derive(Debug, Default)]
pub struct SingleUniformCommands(pub Vec<EUniformCommand>);

pub struct SysUniformComand;
impl TSystemStageInfo for SysUniformComand {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysMaterialMetaChange::key()
        ]
    }
}
#[setup]
impl SysUniformComand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingleUniformCommands>,
        mut bindvalues:   Query<GameObject, &mut BindEffectValues,     With<BindEffectValues>>,
        mut textureparams:    Query<GameObject, &mut UniformTextureWithSamplerParams,         With<UniformTextureWithSamplerParams>>,
        mut flag_cmd: Commands<GameObject, BindEffectValueDirty>,
    ) {
        let mut list = replace(&mut cmds.0, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                EUniformCommand::Mat4  (entity, slot, value, ismust) => {
                    if let Some(mut bindvalues) = bindvalues.get_mut(entity) {
                        bindvalues.mat4(slot, value.as_slice());
                        flag_cmd.insert(entity, BindEffectValueDirty(true));
                    } else {
                        cmds.0.push(cmd);
                    }
                },
                EUniformCommand::Mat2  (entity, slot, value, ismust) => {
                    if let Some(mut bindvalues) = bindvalues.get_mut(entity) {
                        bindvalues.mat2(slot, value.as_slice());
                        flag_cmd.insert(entity, BindEffectValueDirty(true));
                    } else {
                        cmds.0.push(cmd);
                    }
                },
                EUniformCommand::Vec4  (entity, slot, value, ismust) => {
                    if let Some(mut bindvalues) = bindvalues.get_mut(entity) {
                        bindvalues.vec4(slot, value.as_slice());
                        flag_cmd.insert(entity, BindEffectValueDirty(true));
                    } else {
                        cmds.0.push(cmd);
                    }
                },
                EUniformCommand::Vec2  (entity, slot, value, ismust) => {
                    if let Some(mut bindvalues) = bindvalues.get_mut(entity) {
                        bindvalues.vec2(slot, value.as_slice());
                        flag_cmd.insert(entity, BindEffectValueDirty(true));
                    } else {
                        cmds.0.push(cmd);
                    }
                },
                EUniformCommand::Float (entity, slot, value, ismust) => {
                    if let Some(mut bindvalues) = bindvalues.get_mut(entity) {
                        bindvalues.float(slot, value);
                        flag_cmd.insert(entity, BindEffectValueDirty(true));
                    } else {
                        cmds.0.push(cmd);
                    }
                },
                EUniformCommand::Int   (entity, slot, value, ismust) => {
                    if let Some(mut bindvalues) = bindvalues.get_mut(entity) {
                        bindvalues.int(slot, value);
                        flag_cmd.insert(entity, BindEffectValueDirty(true));
                    } else {
                        cmds.0.push(cmd);
                    }
                },
                EUniformCommand::Uint  (entity, slot, value, ismust) => {
                    if let Some(mut bindvalues) = bindvalues.get_mut(entity) {
                        bindvalues.uint(slot, value);
                        flag_cmd.insert(entity, BindEffectValueDirty(true));
                    } else {
                        cmds.0.push(cmd);
                    }
                },
                EUniformCommand::Texture(entity, param, ismust) => {
                    if let Some(mut textureparams) = textureparams.get_mut(entity) {
                        textureparams.0.insert(param.slotname.clone(), Arc::new(param));
                    }
                },
            }
        });
    }
}

/// * Material 参数变化 影响 Model
///   * 没有纹理时 Model 的 Pass 即准备好
///   * 有纹理时 Model 的 Pass 需等待纹理加载好
pub struct SysMaterialMetaChange;
impl TSystemStageInfo for SysMaterialMetaChange {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            // SysAssetShaderEffectLoad::key(), 
        ]
    }
}
#[setup]
impl SysMaterialMetaChange {
    #[system]
    pub fn cmd(
        mut materials: Query<
            GameObject,
            (
                ObjectID, &AssetKeyShaderEffect, &AssetResShaderEffectMeta, &MaterialUsedList, &EPassTag
            ),
            Changed<AssetResShaderEffectMeta>
        >,
        mut effect_values_cmd: Commands<GameObject, BindEffectValues>,
        mut effect_values_flag_cmd: Commands<GameObject, BindEffectValueDirty>,
        mut ready01_cmd: Commands<GameObject, Pass01Ready>,
        mut ready02_cmd: Commands<GameObject, Pass02Ready>,
        mut ready03_cmd: Commands<GameObject, Pass03Ready>,
        mut ready04_cmd: Commands<GameObject, Pass04Ready>,
        mut ready05_cmd: Commands<GameObject, Pass05Ready>,
        mut ready06_cmd: Commands<GameObject, Pass06Ready>,
        mut ready07_cmd: Commands<GameObject, Pass07Ready>,
        mut ready08_cmd: Commands<GameObject, Pass08Ready>,
        mut allocator: ResMut<BindBufferAllocator>,
        device: Res<RenderDevice>,
    ) {
        // log::debug!("SysMaterialMetaChange: ");
        materials.iter_mut().for_each(|(
            matid,
            effect_key, effect, list_model, passtag
        )| {
            log::info!("SysMaterialMetaChange: ");
            if let Some(effect_val_bind) = BindEffectValues::new(&device, effect_key.0.clone(), effect.0.clone(), &mut allocator) {
                log::info!("SysMaterialMetaChange: 1");
                effect_values_cmd.insert(matid, effect_val_bind);
                effect_values_flag_cmd.insert(matid, BindEffectValueDirty(true));
            } else {
                effect_values_flag_cmd.insert(matid, BindEffectValueDirty(false));
            }
            
            let data = if effect.textures.len() == 0 {
                Some((effect_key.0.clone(), effect.0.clone()))
            } else {
                None
            };
            match passtag 
            {
                EPassTag::ShadowCast => list_model.0.iter().for_each(|(id_obj, _)| { ready01_cmd.insert(id_obj.clone(), Pass01Ready(data.clone())); }),
                EPassTag::Opaque => list_model.0.iter().for_each(|(id_obj, _)| { ready02_cmd.insert(id_obj.clone(), Pass02Ready(data.clone())); }),
                EPassTag::Sky => list_model.0.iter().for_each(|(id_obj, _)| { ready03_cmd.insert(id_obj.clone(), Pass03Ready(data.clone())); }),
                EPassTag::Water => list_model.0.iter().for_each(|(id_obj, _)| { ready04_cmd.insert(id_obj.clone(), Pass04Ready(data.clone())); }),
                EPassTag::Transparent => list_model.0.iter().for_each(|(id_obj, _)| { ready05_cmd.insert(id_obj.clone(), Pass05Ready(data.clone())); }),
                EPassTag::AlphaTest => list_model.0.iter().for_each(|(id_obj, _)| { ready06_cmd.insert(id_obj.clone(), Pass06Ready(data.clone())); }),
                EPassTag::OpaqueExtend => list_model.0.iter().for_each(|(id_obj, _)| { ready07_cmd.insert(id_obj.clone(), Pass07Ready(data.clone())); }),
                EPassTag::TransparentExtend => list_model.0.iter().for_each(|(id_obj, _)| { ready08_cmd.insert(id_obj.clone(), Pass08Ready(data.clone())); }),
            }
        });
    }
}

pub struct SysMaterialTexturesChange;
impl TSystemStageInfo for SysMaterialTexturesChange {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            // SysAssetShaderEffectLoad::key(),
            SysUniformComand::key()
        ]
    }
}
#[setup]
impl SysMaterialTexturesChange {
    #[system]
    pub fn cmd(
        mut materials: Query<
            GameObject,
            (
                ObjectID, &AssetResShaderEffectMeta, &UniformTextureWithSamplerParams
            ),
            Or<(Changed<AssetResShaderEffectMeta>, Changed<UniformTextureWithSamplerParams>)>
        >,
        mut tex01_cmd:  Commands<GameObject, TextureSlot01>,
        mut tex02_cmd:  Commands<GameObject, TextureSlot02>,
        mut tex03_cmd:  Commands<GameObject, TextureSlot03>,
        mut tex04_cmd:  Commands<GameObject, TextureSlot04>,
        mut samp01_cmd:  Commands<GameObject, EffectBindSampler2D01>,
        mut samp02_cmd:  Commands<GameObject, EffectBindSampler2D02>,
        mut samp03_cmd:  Commands<GameObject, EffectBindSampler2D03>,
        mut samp04_cmd:  Commands<GameObject, EffectBindSampler2D04>,
        device: Res<RenderDevice>,
        asset_samp: Res<Share<AssetMgr<SamplerRes>>>,
    ) {
        // log::debug!("SysMaterialMetaChange: ");
        materials.iter_mut().for_each(|(
            matid,
            effect, texparams
        )| {
            texparams.0.iter().for_each(|(slotname, value)| {
                match effect.textures.binary_search_by(|a| { a.slotname.cmp(slotname) }) {
                    Ok(index) => {
                        if index == 0 {
                            tex01_cmd.insert(matid, TextureSlot01(value.clone()));
                            if let Some(samp) = BindDataSampler::create(value.sample.clone(), &device, &asset_samp) {
                                samp01_cmd.insert(matid, EffectBindSampler2D01(samp))
                            }
                        } else if index == 1 {
                            tex02_cmd.insert(matid, TextureSlot02(value.clone()));
                            if let Some(samp) = BindDataSampler::create(value.sample.clone(), &device, &asset_samp) {
                                samp02_cmd.insert(matid, EffectBindSampler2D02(samp))
                            }
                        } else if index == 2 {
                            tex03_cmd.insert(matid, TextureSlot03(value.clone()));
                            if let Some(samp) = BindDataSampler::create(value.sample.clone(), &device, &asset_samp) {
                                samp03_cmd.insert(matid, EffectBindSampler2D03(samp))
                            }
                        } else if index == 3 {
                            tex04_cmd.insert(matid, TextureSlot04(value.clone()));
                            if let Some(samp) = BindDataSampler::create(value.sample.clone(), &device, &asset_samp) {
                                samp04_cmd.insert(matid, EffectBindSampler2D04(samp))
                            }
                        }
                    },
                    _ => {}
                }
            });
        });
    }
}

pub struct SysBindValueUpdate;
impl TSystemStageInfo for SysBindValueUpdate {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysUniformComand::key(),
        ]
    }
}
#[setup]
impl SysBindValueUpdate
{
    #[system]
    pub fn update(
        mut items: Query<
            GameObject, 
            (&mut BindEffectValues, &mut BindEffectValueDirty,),
            Changed<BindEffectValueDirty>,
        >,
    ) {
        items.iter_mut().for_each(|(mut bind, mut flag)| {
            bind.update();
            flag.0 = false;
        });
    }
}