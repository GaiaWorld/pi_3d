

use std::{sync::Arc};

use pi_engine_shell::prelude::*;

use crate::{
    materials::{
        shader_effect::{AssetKeyShaderEffect},
    },
    pass::*
};


use crate::materials::shader_effect::AssetResShaderEffectMeta;

use super::{
    uniform::*,
    texture::{UniformTextureWithSamplerParams, TextureSlot01, TextureSlot02, TextureSlot03, TextureSlot04},
};


#[derive(Debug, Clone)]
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

pub type ActionListUniform = ActionList<EUniformCommand>;
pub fn sys_act_uniform(
    mut cmds: ResMut<ActionList<EUniformCommand>>,
    mut commands:  Commands,
    mut bindvalues: Query<(&mut BindEffect, &mut BindEffectValueDirty)>,
    mut textureparams: Query<&mut UniformTextureWithSamplerParams>,
) {
    cmds.drain().drain(..).for_each(|cmd| {
        match cmd {
            EUniformCommand::Mat4  (entity, slot, value, _MainTex) => {
                if let Ok((mut bindvalues, mut flag)) = bindvalues.get_mut(entity) {
                    if let Some(bindvalues) = &mut bindvalues.0 {
                        bindvalues.mat4(slot, value.as_slice());
                        *flag = BindEffectValueDirty(true);
                    } else {
                        cmds.push(cmd);
                    } 
                    // bindvalues.mat4(slot, value.as_slice());
                    // commands.entity(entity).insert(BindEffectValueDirty(true));
                } else {
                    cmds.push(cmd);
                }
            },
            EUniformCommand::Mat2  (entity, slot, value, _) => {
                if let Ok((mut bindvalues, mut flag)) = bindvalues.get_mut(entity) {
                    if let Some(bindvalues) = &mut bindvalues.0 {
                        bindvalues.mat2(slot, value.as_slice());
                        *flag = BindEffectValueDirty(true);
                    } else {
                        cmds.push(cmd);
                    } 
                    // bindvalues.mat2(slot, value.as_slice());
                    // commands.entity(entity).insert(BindEffectValueDirty(true));
                } else {
                    cmds.push(cmd);
                }
            },
            EUniformCommand::Vec4  (entity, slot, value, _MainTex) => {
                if let Ok((mut bindvalues, mut flag)) = bindvalues.get_mut(entity) {
                    if let Some(bindvalues) = &mut bindvalues.0 {
                        bindvalues.vec4(slot, value.as_slice());
                        *flag = BindEffectValueDirty(true);
                    } else {
                        cmds.push(cmd);
                    } 
                } else {
                    cmds.push(cmd);
                }
            },
            EUniformCommand::Vec2  (entity, slot, value, _) => {
                if let Ok((mut bindvalues, mut flag)) = bindvalues.get_mut(entity) {
                    if let Some(bindvalues) = &mut bindvalues.0 {
                        bindvalues.vec2(slot, value.as_slice());
                        *flag = BindEffectValueDirty(true);
                    } else {
                        cmds.push(cmd);
                    } 
                } else {
                    cmds.push(cmd);
                }
            },
            EUniformCommand::Float (entity, slot, value, _) => {
                if let Ok((mut bindvalues, mut flag)) = bindvalues.get_mut(entity) {
                    if let Some(bindvalues) = &mut bindvalues.0 {
                        bindvalues.float(slot, value);
                        *flag = BindEffectValueDirty(true);
                    } else {
                        cmds.push(cmd);
                    } 
                } else {
                    cmds.push(cmd);
                }
            },
            EUniformCommand::Int   (entity, slot, value, _) => {
                if let Ok((mut bindvalues, mut flag)) = bindvalues.get_mut(entity) {
                    if let Some(bindvalues) = &mut bindvalues.0 {
                        bindvalues.int(slot, value);
                        *flag = BindEffectValueDirty(true);
                    } else {
                        cmds.push(cmd);
                    } 
                } else {
                    cmds.push(cmd);
                }
            },
            EUniformCommand::Uint  (entity, slot, value, _) => {
                if let Ok((mut bindvalues, mut flag)) = bindvalues.get_mut(entity) {
                    if let Some(bindvalues) = &mut bindvalues.0 {
                        bindvalues.uint(slot, value);
                        *flag = BindEffectValueDirty(true);
                    } else {
                        cmds.push(cmd);
                    } 
                } else {
                    cmds.push(cmd);
                }
            },
            EUniformCommand::Texture(entity, param, ismust) => {
                if let Ok(mut textureparams) = textureparams.get_mut(entity) {
                    log::warn!("EUniformCommand::Texture");
                    textureparams.0.insert(param.slotname.clone(), Arc::new(param));
                } else {
                    cmds.push(EUniformCommand::Texture(entity, param, ismust));
                }
            },
        }
    });
}

pub struct ActionMaterialUniform;
impl ActionMaterialUniform {
    pub fn modify(
        app: &mut App,
        cmd: EUniformCommand,
    ) {
        app.world.get_resource_mut::<ActionListUniform>().unwrap().push(cmd);
    }
}

/// * Material 参数变化 影响 Model
///   * 没有纹理时 Model 的 Pass 即准备好
///   * 有纹理时 Model 的 Pass 需等待纹理加载好
// pub struct SysMaterialMetaChange<T: TPassID + Component>(PhantomData<T>);
// impl<T: TPassID + Component> TSystemStageInfo for SysMaterialMetaChange<T> {
//     fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
//         vec![
//             // SysAssetShaderEffectLoad::key(), 
//         ]
//     }
// }
// #[setup]
// impl<T: TPassID + Component> SysMaterialMetaChange<T> {
//     #[system]
    pub fn sys_material_init<T: TPassID + Component>(
        mut materials: Query<
            (
                ObjectID, &AssetKeyShaderEffect, &AssetResShaderEffectMeta, &EPassTag, &mut BindEffect, &mut BindEffectValueDirty
            ),
            Changed<AssetResShaderEffectMeta>
        >,
        mut commands: Commands,
        mut allocator: ResMut<ResBindBufferAllocator>,
        device: Res<PiRenderDevice>,
    ) {
        log::debug!("SysMaterialMetaInit: ");
        materials.iter_mut().for_each(|(
            matid,
            effect_key, effect, passtag, mut bindeffect, mut flag
        )| {
            let pass = passtag.as_pass();
            if pass & T::TAG == T::TAG {
                if let Some(effect_val_bind) = BindEffectValues::new(&device, effect_key.0.clone(), effect.0.clone(), &mut allocator) {
                    log::warn!("SysMaterialMetaInit: 1");
                    // commands.entity(matid).insert(effect_val_bind).insert(BindEffectValueDirty(true));
                    *bindeffect = BindEffect(Some(effect_val_bind));
                    *flag = BindEffectValueDirty(true);
                } else {
                    // commands.entity(matid).insert(BindEffectValueDirty(false));
                    *flag = BindEffectValueDirty(false);
                }
                
                // let data = if effect.textures.len() == 0 {
                //     Some((effect_key.0.clone(), effect.0.clone()))
                // } else {
                //     None
                // };
                // list_model.0.iter().for_each(|(id_obj, _)| {
                //     if let Some(passid) = models.get(id_obj.clone()) {
                //         ready01_cmd.insert(passid.id(), PassReady(data.clone()));
                //     }
                // });
            }
        });
    }
// }

// pub struct SysMaterialTexturesChange;
// impl TSystemStageInfo for SysMaterialTexturesChange {
//     fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
//         vec![
//             // SysAssetShaderEffectLoad::key(),
//             // SysUniformComand::key()
//         ]
//     }
// }
// #[setup]
// impl SysMaterialTexturesChange {
//     #[system]
    pub fn sys_material_textures_modify(
        mut materials: Query<
            (
                ObjectID, &AssetResShaderEffectMeta, &UniformTextureWithSamplerParams
            ),
            Or<(Changed<AssetResShaderEffectMeta>, Changed<UniformTextureWithSamplerParams>)>
        >,
        mut commands:  Commands,
        device: Res<PiRenderDevice>,
        asset_samp: Res<ShareAssetMgr<SamplerRes>>,
    ) {
        // log::debug!("SysMaterialMetaChange: ");
        materials.iter_mut().for_each(|(
            matid,
            effect, texparams
        )| {
            let mut entitycmd = commands.entity(matid);
            texparams.0.iter().for_each(|(slotname, value)| {
                // log::warn!("TextureSlot01 {:?}", slotname);
                match effect.textures.binary_search_by(|a| { a.slotname.cmp(slotname) }) {
                    Ok(index) => {
                        if index == 0 {
                            // log::warn!("TextureSlot01");
                            entitycmd.insert(TextureSlot01(value.clone()));
                            if let Some(samp) = BindDataSampler::create(value.sample.clone(), &device, &asset_samp) {
                                entitycmd.insert(EffectBindSampler2D01Comp(EffectBindSampler2D01(samp)));
                            }
                        } else if index == 1 {
                            entitycmd.insert(TextureSlot02(value.clone()));
                            if let Some(samp) = BindDataSampler::create(value.sample.clone(), &device, &asset_samp) {
                                entitycmd.insert(EffectBindSampler2D02Comp(EffectBindSampler2D02(samp)));
                            }
                        } else if index == 2 {
                            entitycmd.insert(TextureSlot03(value.clone()));
                            if let Some(samp) = BindDataSampler::create(value.sample.clone(), &device, &asset_samp) {
                                entitycmd.insert(EffectBindSampler2D03Comp(EffectBindSampler2D03(samp)));
                            }
                        } else if index == 3 {
                            entitycmd.insert(TextureSlot04(value.clone()));
                            if let Some(samp) = BindDataSampler::create(value.sample.clone(), &device, &asset_samp) {
                                entitycmd.insert(EffectBindSampler2D04Comp(EffectBindSampler2D04(samp)));
                            }
                        }
                    },
                    _ => {}
                }
            });
        });
    }
// }

// pub struct SysBindValueUpdate;
// impl TSystemStageInfo for SysBindValueUpdate {
//     fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
//         vec![
//             // SysUniformComand::key(),
//         ]
//     }
// }
// #[setup]
// impl SysBindValueUpdate
// {
//     #[system]
    pub fn sys_material_uniform_apply(
        mut items: Query<
            (&mut BindEffect, &mut BindEffectValueDirty,),
            Changed<BindEffectValueDirty>,
        >,
    ) {
        items.iter_mut().for_each(|(mut bind, mut flag)| {
            if let Some(bind) = &mut bind.0 {
                bind.update();
                flag.0 = false;
            }
        });
    }
// }