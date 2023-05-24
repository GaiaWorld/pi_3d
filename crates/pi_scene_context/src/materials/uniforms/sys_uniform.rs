

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


// #[derive(Debug, Clone)]
// pub enum OpsUniformByName {
//     Mat4(ObjectID, Atom, pi_scene_math::Matrix, bool),
//     Mat2(ObjectID, Atom, pi_scene_math::Matrix2, bool),
//     Vec4(ObjectID, Atom, pi_scene_math::Vector4, bool),
//     Vec2(ObjectID, Atom, pi_scene_math::Vector2, bool),
//     Float(ObjectID, Atom, pi_scene_math::Number, bool),
//     Int(ObjectID, Atom, i32, bool),
//     Uint(ObjectID, Atom, u32, bool),
//     Texture(ObjectID, UniformTextureWithSamplerParam, bool),
// }
// pub type ActionListUniformByName = ActionList<OpsUniformByName>;
// pub fn sys_act_uniform_by_name(
//     mut cmds: ResMut<ActionListUniformByName>,
//     mut commands:  Commands,
//     mut bindvalues: Query<(&mut BindEffect, &mut BindEffectValueDirty)>,
//     mut textureparams: Query<&mut UniformTextureWithSamplerParams>,
// ) {
//     cmds.drain().drain(..).for_each(|cmd| {
//         match cmd {
//             OpsUniformByName::Mat4  (entity, slot, value, ismust) => {
//                 if let Ok((mut bindvalues, mut flag)) = bindvalues.get_mut(entity) {
//                     if let Some(bindvalues) = &mut bindvalues.0 {
//                         if let Some(slot) = bindvalues.keys.get(&slot) {
//                             bindvalues.mat4(*slot, value.as_slice());
//                             *flag = BindEffectValueDirty(true);
//                         }
//                     } else {
//                         cmds.push(OpsUniformByName::Mat4  (entity, slot, value, ismust));
//                     } 
//                     // bindvalues.mat4(slot, value.as_slice());
//                     // commands.entity(entity).insert(BindEffectValueDirty(true));
//                 } else {
//                     cmds.push(OpsUniformByName::Mat4  (entity, slot, value, ismust));
//                 }
//             },
//             OpsUniformByName::Mat2  (entity, slot, value, ismust) => {
//                 if let Ok((mut bindvalues, mut flag)) = bindvalues.get_mut(entity) {
//                     if let Some(bindvalues) = &mut bindvalues.0 {
//                         if let Some(slot) = bindvalues.keys.get(&slot) {
//                             bindvalues.mat2(*slot, value.as_slice());
//                             *flag = BindEffectValueDirty(true);
//                         }
//                     } else {
//                         cmds.push(OpsUniformByName::Mat2  (entity, slot, value, ismust));
//                     } 
//                     // bindvalues.mat2(slot, value.as_slice());
//                     // commands.entity(entity).insert(BindEffectValueDirty(true));
//                 } else {
//                     cmds.push(OpsUniformByName::Mat2  (entity, slot, value, ismust));
//                 }
//             },
//             OpsUniformByName::Vec4  (entity, slot, value, ismust) => {
//                 if let Ok((mut bindvalues, mut flag)) = bindvalues.get_mut(entity) {
//                     if let Some(bindvalues) = &mut bindvalues.0 {
//                         if let Some(slot) = bindvalues.keys.get(&slot) {
//                             bindvalues.vec4(*slot, value.as_slice());
//                             *flag = BindEffectValueDirty(true);
//                         }
//                     } else {
//                         cmds.push(OpsUniformByName::Vec4  (entity, slot, value, ismust));
//                     } 
//                 } else {
//                     cmds.push(OpsUniformByName::Vec4  (entity, slot, value, ismust));
//                 }
//             },
//             OpsUniformByName::Vec2  (entity, slot, value, ismust) => {
//                 if let Ok((mut bindvalues, mut flag)) = bindvalues.get_mut(entity) {
//                     if let Some(bindvalues) = &mut bindvalues.0 {
//                         if let Some(slot) = bindvalues.keys.get(&slot) {
//                             bindvalues.vec2(*slot, value.as_slice());
//                             *flag = BindEffectValueDirty(true);
//                         }
//                     } else {
//                         cmds.push(OpsUniformByName::Vec2  (entity, slot, value, ismust));
//                     } 
//                 } else {
//                     cmds.push(OpsUniformByName::Vec2  (entity, slot, value, ismust));
//                 }
//             },
//             OpsUniformByName::Float (entity, slot, value, ismust) => {
//                 if let Ok((mut bindvalues, mut flag)) = bindvalues.get_mut(entity) {
//                     if let Some(bindvalues) = &mut bindvalues.0 {
//                         if let Some(slot) = bindvalues.keys.get(&slot) {
//                             bindvalues.float(*slot, value);
//                             *flag = BindEffectValueDirty(true);
//                         }
//                     } else {
//                         cmds.push(OpsUniformByName::Float (entity, slot, value, ismust));
//                     } 
//                 } else {
//                     cmds.push(OpsUniformByName::Float (entity, slot, value, ismust));
//                 }
//             },
//             OpsUniformByName::Int   (entity, slot, value, ismust) => {
//                 if let Ok((mut bindvalues, mut flag)) = bindvalues.get_mut(entity) {
//                     if let Some(bindvalues) = &mut bindvalues.0 {
//                         if let Some(slot) = bindvalues.keys.get(&slot) {
//                             bindvalues.int(*slot, value);
//                             *flag = BindEffectValueDirty(true);
//                         }
//                     } else {
//                         cmds.push(OpsUniformByName::Int   (entity, slot, value, ismust));
//                     } 
//                 } else {
//                     cmds.push(OpsUniformByName::Int   (entity, slot, value, ismust));
//                 }
//             },
//             OpsUniformByName::Uint  (entity, slot, value, ismust) => {
//                 if let Ok((mut bindvalues, mut flag)) = bindvalues.get_mut(entity) {
//                     if let Some(bindvalues) = &mut bindvalues.0 {
//                         if let Some(slot) = bindvalues.keys.get(&slot) {
//                             bindvalues.uint(*slot, value);
//                             *flag = BindEffectValueDirty(true);
//                         }
//                     } else {
//                         cmds.push(OpsUniformByName::Uint  (entity, slot, value, ismust));
//                     } 
//                 } else {
//                     cmds.push(OpsUniformByName::Uint  (entity, slot, value, ismust));
//                 }
//             },
//             OpsUniformByName::Texture(entity, param, ismust) => {
//                 if let Ok(mut textureparams) = textureparams.get_mut(entity) {
//                     log::warn!("EUniformCommand::Texture");
//                     textureparams.0.insert(param.slotname.clone(), Arc::new(param));
//                 } else {
//                     cmds.push(OpsUniformByName::Texture(entity, param, ismust));
//                 }
//             },
//         }
//     });
// }


// #[derive(Debug, Clone)]
// pub enum EUniformCommand {
//     Mat4(ObjectID, usize, pi_scene_math::Matrix, bool),
//     Mat2(ObjectID, usize, pi_scene_math::Matrix2, bool),
//     Vec4(ObjectID, usize, pi_scene_math::Vector4, bool),
//     Vec2(ObjectID, usize, pi_scene_math::Vector2, bool),
//     Float(ObjectID, usize, pi_scene_math::Number, bool),
//     Int(ObjectID, usize, i32, bool),
//     Uint(ObjectID, usize, u32, bool),
//     Texture(ObjectID, UniformTextureWithSamplerParam, bool),
// }
// pub type ActionListUniform = ActionList<EUniformCommand>;
// pub fn sys_act_uniform(
//     mut cmds: ResMut<ActionList<EUniformCommand>>,
//     mut commands:  Commands,
//     mut bindvalues: Query<(&mut BindEffect, &mut BindEffectValueDirty)>,
//     mut textureparams: Query<&mut UniformTextureWithSamplerParams>,
// ) {
//     cmds.drain().drain(..).for_each(|cmd| {
//         match cmd {
//             EUniformCommand::Mat4  (entity, slot, value, _MainTex) => {
//                 if let Ok((mut bindvalues, mut flag)) = bindvalues.get_mut(entity) {
//                     if let Some(bindvalues) = &mut bindvalues.0 {
//                         bindvalues.mat4(slot, value.as_slice());
//                         *flag = BindEffectValueDirty(true);
//                     } else {
//                         cmds.push(cmd);
//                     } 
//                     // bindvalues.mat4(slot, value.as_slice());
//                     // commands.entity(entity).insert(BindEffectValueDirty(true));
//                 } else {
//                     cmds.push(cmd);
//                 }
//             },
//             EUniformCommand::Mat2  (entity, slot, value, _) => {
//                 if let Ok((mut bindvalues, mut flag)) = bindvalues.get_mut(entity) {
//                     if let Some(bindvalues) = &mut bindvalues.0 {
//                         bindvalues.mat2(slot, value.as_slice());
//                         *flag = BindEffectValueDirty(true);
//                     } else {
//                         cmds.push(cmd);
//                     } 
//                     // bindvalues.mat2(slot, value.as_slice());
//                     // commands.entity(entity).insert(BindEffectValueDirty(true));
//                 } else {
//                     cmds.push(cmd);
//                 }
//             },
//             EUniformCommand::Vec4  (entity, slot, value, _MainTex) => {
//                 if let Ok((mut bindvalues, mut flag)) = bindvalues.get_mut(entity) {
//                     if let Some(bindvalues) = &mut bindvalues.0 {
//                         bindvalues.vec4(slot, value.as_slice());
//                         *flag = BindEffectValueDirty(true);
//                     } else {
//                         cmds.push(cmd);
//                     } 
//                 } else {
//                     cmds.push(cmd);
//                 }
//             },
//             EUniformCommand::Vec2  (entity, slot, value, _) => {
//                 if let Ok((mut bindvalues, mut flag)) = bindvalues.get_mut(entity) {
//                     if let Some(bindvalues) = &mut bindvalues.0 {
//                         bindvalues.vec2(slot, value.as_slice());
//                         *flag = BindEffectValueDirty(true);
//                     } else {
//                         cmds.push(cmd);
//                     } 
//                 } else {
//                     cmds.push(cmd);
//                 }
//             },
//             EUniformCommand::Float (entity, slot, value, _) => {
//                 if let Ok((mut bindvalues, mut flag)) = bindvalues.get_mut(entity) {
//                     if let Some(bindvalues) = &mut bindvalues.0 {
//                         bindvalues.float(slot, value);
//                         *flag = BindEffectValueDirty(true);
//                     } else {
//                         cmds.push(cmd);
//                     } 
//                 } else {
//                     cmds.push(cmd);
//                 }
//             },
//             EUniformCommand::Int   (entity, slot, value, _) => {
//                 if let Ok((mut bindvalues, mut flag)) = bindvalues.get_mut(entity) {
//                     if let Some(bindvalues) = &mut bindvalues.0 {
//                         bindvalues.int(slot, value);
//                         *flag = BindEffectValueDirty(true);
//                     } else {
//                         cmds.push(cmd);
//                     } 
//                 } else {
//                     cmds.push(cmd);
//                 }
//             },
//             EUniformCommand::Uint  (entity, slot, value, _) => {
//                 if let Ok((mut bindvalues, mut flag)) = bindvalues.get_mut(entity) {
//                     if let Some(bindvalues) = &mut bindvalues.0 {
//                         bindvalues.uint(slot, value);
//                         *flag = BindEffectValueDirty(true);
//                     } else {
//                         cmds.push(cmd);
//                     } 
//                 } else {
//                     cmds.push(cmd);
//                 }
//             },
//             EUniformCommand::Texture(entity, param, ismust) => {
//                 if let Ok(mut textureparams) = textureparams.get_mut(entity) {
//                     log::warn!("EUniformCommand::Texture");
//                     textureparams.0.insert(param.slotname.clone(), Arc::new(param));
//                 } else {
//                     cmds.push(EUniformCommand::Texture(entity, param, ismust));
//                 }
//             },
//         }
//     });
// }

// pub struct ActionMaterialUniform;
// impl ActionMaterialUniform {
//     pub fn modify(
//         app: &mut App,
//         cmd: EUniformCommand,
//     ) {
//         app.world.get_resource_mut::<ActionListUniform>().unwrap().push(cmd);
//     }
// }

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
    pub fn sys_material_init(
        mut materials: Query<
            (
                ObjectID, &AssetKeyShaderEffect, &AssetResShaderEffectMeta, &EPassTag, &mut BindEffect, &mut BindEffectValueDirty
            ),
            Changed<AssetResShaderEffectMeta>
        >,
        mut allocator: ResMut<ResBindBufferAllocator>,
        device: Res<PiRenderDevice>,
    ) {
        log::debug!("SysMaterialMetaInit: ");
        materials.iter_mut().for_each(|(
            matid,
            effect_key, effect, passtag, mut bindeffect, mut flag
        )| {
            if let Some(effect_val_bind) = BindEffectValues::new(&device, effect_key.0.clone(), effect.0.clone(), &mut allocator) {
                log::warn!("SysMaterialMetaInit: 1");
                *bindeffect = BindEffect(Some(effect_val_bind));
                *flag = BindEffectValueDirty(true);
            } else {
                *flag = BindEffectValueDirty(true);
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
                ObjectID, &AssetResShaderEffectMeta, &mut UniformTextureWithSamplerParams
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
            effect, mut texparams
        )| {
            let mut entitycmd = commands.entity(matid);

            for index in 0..effect.textures.len() {
                let item = effect.textures.get(index).unwrap();
                let param = if let Some(param) = texparams.0.get(&item.slotname) {
                    param
                } else {
                    texparams.0.insert(
                        item.slotname.clone(), 
                        Arc::new(
                                UniformTextureWithSamplerParam {
                                slotname: item.slotname.clone(),
                                filter: true,
                                sample: KeySampler::default(),
                                url: EKeyTexture::Tex(Atom::from(DefaultTexture::path(item.initial, wgpu::TextureDimension::D2))),
                            }
                        )
                    );
                    texparams.0.get(&item.slotname).unwrap()
                };
                
                if index == 0 {
                    // log::warn!("TextureSlot01");
                    entitycmd.insert(TextureSlot01(param.clone()));
                    if let Some(samp) = BindDataSampler::create(param.sample.clone(), &device, &asset_samp) {
                        entitycmd.insert(EffectBindSampler2D01Comp(EffectBindSampler2D01(samp)));
                    }
                } else if index == 1 {
                    entitycmd.insert(TextureSlot02(param.clone()));
                    if let Some(samp) = BindDataSampler::create(param.sample.clone(), &device, &asset_samp) {
                        entitycmd.insert(EffectBindSampler2D02Comp(EffectBindSampler2D02(samp)));
                    }
                } else if index == 2 {
                    entitycmd.insert(TextureSlot03(param.clone()));
                    if let Some(samp) = BindDataSampler::create(param.sample.clone(), &device, &asset_samp) {
                        entitycmd.insert(EffectBindSampler2D03Comp(EffectBindSampler2D03(samp)));
                    }
                } else if index == 3 {
                    entitycmd.insert(TextureSlot04(param.clone()));
                    if let Some(samp) = BindDataSampler::create(param.sample.clone(), &device, &asset_samp) {
                        entitycmd.insert(EffectBindSampler2D04Comp(EffectBindSampler2D04(samp)));
                    }
                }
            }

            // texparams.0.iter().for_each(|(slotname, value)| {
            //     // log::warn!("TextureSlot01 {:?}", slotname);
            //     match effect.textures.binary_search_by(|a| { a.slotname.cmp(slotname) }) {
            //         Ok(index) => {
            //             if index == 0 {
            //                 // log::warn!("TextureSlot01");
            //                 entitycmd.insert(TextureSlot01(value.clone()));
            //                 if let Some(samp) = BindDataSampler::create(value.sample.clone(), &device, &asset_samp) {
            //                     entitycmd.insert(EffectBindSampler2D01Comp(EffectBindSampler2D01(samp)));
            //                 }
            //             } else if index == 1 {
            //                 entitycmd.insert(TextureSlot02(value.clone()));
            //                 if let Some(samp) = BindDataSampler::create(value.sample.clone(), &device, &asset_samp) {
            //                     entitycmd.insert(EffectBindSampler2D02Comp(EffectBindSampler2D02(samp)));
            //                 }
            //             } else if index == 2 {
            //                 entitycmd.insert(TextureSlot03(value.clone()));
            //                 if let Some(samp) = BindDataSampler::create(value.sample.clone(), &device, &asset_samp) {
            //                     entitycmd.insert(EffectBindSampler2D03Comp(EffectBindSampler2D03(samp)));
            //                 }
            //             } else if index == 3 {
            //                 entitycmd.insert(TextureSlot04(value.clone()));
            //                 if let Some(samp) = BindDataSampler::create(value.sample.clone(), &device, &asset_samp) {
            //                     entitycmd.insert(EffectBindSampler2D04Comp(EffectBindSampler2D04(samp)));
            //                 }
            //             }
            //         },
            //         _ => {}
            //     }
            // });
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