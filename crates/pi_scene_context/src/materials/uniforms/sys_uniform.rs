

use std::sync::Arc;

use pi_engine_shell::prelude::*;



use crate::materials::shader_effect::AssetResShaderEffectMeta;

use super::{
    uniform::*,
    texture::*,
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
//                      
// #[setup]
// impl<T: TPassID + Component> SysMaterialMetaChange<T> {
//     #[system]
    // pub fn sys_material_init(
    //     mut materials: Query<
    //         (
    //             ObjectID, &AssetKeyShaderEffect, &AssetResShaderEffectMeta, &EPassTag, &mut BindEffect, &mut BindEffectValueDirty
    //         ),
    //         Changed<AssetResShaderEffectMeta>
    //     >,
    //     mut allocator: ResMut<ResBindBufferAllocator>,
    //     device: Res<PiRenderDevice>,
    // ) {
    //     log::debug!("SysMaterialMetaInit: ");
    //     materials.iter_mut().for_each(|(
    //         matid,
    //         effect_key, effect, passtag, mut bindeffect, mut flag
    //     )| {
    //         if let Some(effect_val_bind) = BindEffectValues::new(&device, effect_key.0.clone(), effect.0.clone(), &mut allocator) {
    //             log::warn!("SysMaterialMetaInit: 1");
    //             *bindeffect = BindEffect(Some(effect_val_bind));
    //             *flag = BindEffectValueDirty(true);
    //         } else {
    //             *flag = BindEffectValueDirty(true);
    //         }
    //     });
    // }
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
            let mut entitycmd = if let Some(cmd) = commands.get_entity(matid) {
                cmd
            } else {
                return;
            };

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
                // log::error!("Texture {:?} {:?}", index, &param.url);
                
                if index == 0 {
                    // log::warn!("Texture 0 {:?}", &param.url);
                    entitycmd.insert(TextureSlot01(param.clone()));
                    if let Some(samp) = BindDataSampler::create(param.sample.clone(), &device, &asset_samp) {
                        entitycmd.insert(EffectBindSampler2D01Comp(EffectBindSampler2D01(samp)));
                    }
                } else if index == 1 {
                    // log::warn!("Texture 1 {:?}", &param.url);
                    entitycmd.insert(TextureSlot02(param.clone()));
                    if let Some(samp) = BindDataSampler::create(param.sample.clone(), &device, &asset_samp) {
                        entitycmd.insert(EffectBindSampler2D02Comp(EffectBindSampler2D02(samp)));
                    }
                } else if index == 2 {
                    // log::warn!("Texture 2 {:?}", &param.url);
                    entitycmd.insert(TextureSlot03(param.clone()));
                    if let Some(samp) = BindDataSampler::create(param.sample.clone(), &device, &asset_samp) {
                        entitycmd.insert(EffectBindSampler2D03Comp(EffectBindSampler2D03(samp)));
                    }
                } else if index == 3 {
                    // log::warn!("Texture 3 {:?}", &param.url);
                    entitycmd.insert(TextureSlot04(param.clone()));
                    if let Some(samp) = BindDataSampler::create(param.sample.clone(), &device, &asset_samp) {
                        entitycmd.insert(EffectBindSampler2D04Comp(EffectBindSampler2D04(samp)));
                    }
                } else if index == 4 {
                    entitycmd.insert(TextureSlot05(param.clone()));
                    if let Some(samp) = BindDataSampler::create(param.sample.clone(), &device, &asset_samp) {
                        entitycmd.insert(EffectBindSampler2D05Comp(EffectBindSampler2D05(samp)));
                    }
                } else if index == 5 {
                    entitycmd.insert(TextureSlot06(param.clone()));
                    if let Some(samp) = BindDataSampler::create(param.sample.clone(), &device, &asset_samp) {
                        entitycmd.insert(EffectBindSampler2D06Comp(EffectBindSampler2D06(samp)));
                    }
                } else if index == 6 {
                    entitycmd.insert(TextureSlot07(param.clone()));
                    if let Some(samp) = BindDataSampler::create(param.sample.clone(), &device, &asset_samp) {
                        entitycmd.insert(EffectBindSampler2D07Comp(EffectBindSampler2D07(samp)));
                    }
                } else if index == 7 {
                    entitycmd.insert(TextureSlot08(param.clone()));
                    if let Some(samp) = BindDataSampler::create(param.sample.clone(), &device, &asset_samp) {
                        entitycmd.insert(EffectBindSampler2D08Comp(EffectBindSampler2D08(samp)));
                    }
                }
            }
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
            &BindEffect,
            Changed<BindEffectValueDirty>,
        >,
    ) {
        items.iter_mut().for_each(|bind| {
            bind.update();
        });
    }
// }