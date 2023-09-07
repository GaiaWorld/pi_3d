
use pi_engine_shell::prelude::*;

use crate::materials::shader_effect::*;

use super::texture::*;

// pub type PluginTextureSlot01Load = PluginImageTextureViewLoad<TextureSlot01, EffectBindTexture2D01Comp>;
// pub type PluginTextureSlot02Load = PluginImageTextureViewLoad<TextureSlot02, EffectBindTexture2D02Comp>;
// pub type PluginTextureSlot03Load = PluginImageTextureViewLoad<TextureSlot03, EffectBindTexture2D03Comp>;
// pub type PluginTextureSlot04Load = PluginImageTextureViewLoad<TextureSlot04, EffectBindTexture2D04Comp>;
// pub type PluginTextureSlot05Load = PluginImageTextureViewLoad<TextureSlot05, EffectBindTexture2D05Comp>;
// pub type PluginTextureSlot06Load = PluginImageTextureViewLoad<TextureSlot06, EffectBindTexture2D06Comp>;

    // pub fn sys_texture_ready01(
    //     mut items: Query<
    //         (
    //             ObjectID
    //             , &AssetResShaderEffectMeta
    //             , (&EffectBindTexture2D01Comp, &EffectBindSampler2D01Comp)
    //             , &mut EffectTextureSamplersComp
    //         ),
    //         Or<(
    //             Changed<EffectBindTexture2D01Comp>, Changed<EffectBindSampler2D01Comp>,
    //         )>
    //     >,
    // ) {
    //     items.iter_mut().for_each(|(
    //         entity
    //         , binddesc
    //         , (tex01, sampl01)
    //         , mut comp
    //     )| {
    //         // log::debug!("SysTextureResReady1 >");
    //         if binddesc.textures.len() == 1 {
    //             comp.0 = Some(
    //                 EffectTextureSamplers {
    //                     textures: (
    //                         Some(tex01.0.clone()), None, None,
    //                         None, None, None,
    //                         None, None
    //                     ),
    //                     samplers: (
    //                         Some(sampl01.0.clone()), None, None,
    //                         None, None, None,
    //                         None, None
    //                     ),
    //                     binding_count: 2
    //                 }
    //             );
    //         }
    //     });
    // }

    // pub fn sys_texture_ready02(
    //     mut items: Query<
    //         (
    //             ObjectID,
    //             &AssetResShaderEffectMeta
    //             , (&EffectBindTexture2D01Comp, &EffectBindSampler2D01Comp)
    //             , (&EffectBindTexture2D02Comp, &EffectBindSampler2D02Comp)
    //             , &mut EffectTextureSamplersComp
    //         ),
    //         Or<(
    //             Changed<EffectBindTexture2D01Comp>, Changed<EffectBindSampler2D01Comp>
    //             , Changed<EffectBindTexture2D02Comp>, Changed<EffectBindSampler2D02Comp>
    //         )>
    //     >,
    //     mut commands: Commands,
    // ) {
    //     items.iter_mut().for_each(|(
    //         entity, binddesc
    //         , (tex01, sampl01)
    //         , (tex02, sampl02)
    //         , mut comp
    //     )| {
    //         log::debug!("texture_ready02: {:?}", binddesc.textures.len());
    //         if binddesc.textures.len() == 2 {
    //             comp.0 = Some(
    //                 EffectTextureSamplers {
    //                     textures: (
    //                         Some(tex01.0.clone()), Some(tex02.0.clone()), None,
    //                         None, None, None,
    //                         None, None
    //                     ),
    //                     samplers: (
    //                         Some(sampl01.0.clone()), Some(sampl02.0.clone()), None,
    //                         None, None, None,
    //                         None, None
    //                     ),
    //                     binding_count: 4
    //                 }
    //             );
    //         }
    //     });
    // }


    // pub fn sys_texture_ready03(
    //     mut items: Query<
    //         (
    //             ObjectID,
    //             &AssetResShaderEffectMeta
    //             , (&EffectBindTexture2D01Comp, &EffectBindSampler2D01Comp)
    //             , (&EffectBindTexture2D02Comp, &EffectBindSampler2D02Comp)
    //             , (&EffectBindTexture2D03Comp, &EffectBindSampler2D03Comp)
    //             , &mut EffectTextureSamplersComp
    //         ),
    //         Or<(
    //             Changed<EffectBindTexture2D01Comp>, Changed<EffectBindSampler2D01Comp>
    //             , Changed<EffectBindTexture2D02Comp>, Changed<EffectBindSampler2D02Comp>
    //             , Changed<EffectBindTexture2D03Comp>, Changed<EffectBindSampler2D03Comp>
    //         )>
    //     >,
    //     mut commands: Commands,
    // ) {
    //     items.iter_mut().for_each(|(
    //         entity, binddesc
    //         , (tex01, sampl01)
    //         , (tex02, sampl02)
    //         , (tex03, sampl03)
    //         , mut comp
    //     )| {
    //         log::debug!("texture_ready03: {:?}", binddesc.textures.len());
    //         if binddesc.textures.len() == 3 {
    //             comp.0 = Some(
    //                 EffectTextureSamplers {
    //                     textures: (
    //                         Some(tex01.0.clone()), Some(tex02.0.clone()), Some(tex03.0.clone()),
    //                         None, None, None,
    //                         None, None
    //                     ),
    //                     samplers: (
    //                         Some(sampl01.0.clone()), Some(sampl02.0.clone()), Some(sampl03.0.clone()),
    //                         None, None, None,
    //                         None, None
    //                     ),
    //                     binding_count: 6
    //                 }
    //             );
    //         }
    //     });
    // }
    
    // pub fn sys_texture_ready04(
    //     mut items: Query<
    //         (
    //             ObjectID,
    //             &AssetResShaderEffectMeta
    //             , (&EffectBindTexture2D01Comp, &EffectBindSampler2D01Comp)
    //             , (&EffectBindTexture2D02Comp, &EffectBindSampler2D02Comp)
    //             , (&EffectBindTexture2D03Comp, &EffectBindSampler2D03Comp)
    //             , (&EffectBindTexture2D04Comp, &EffectBindSampler2D04Comp)
    //             , &mut EffectTextureSamplersComp
    //         ),
    //         Or<(
    //             Changed<EffectBindTexture2D01Comp>, Changed<EffectBindSampler2D01Comp>
    //             , Changed<EffectBindTexture2D02Comp>, Changed<EffectBindSampler2D02Comp>
    //             , Changed<EffectBindTexture2D03Comp>, Changed<EffectBindSampler2D03Comp>
    //             , Changed<EffectBindTexture2D04Comp>, Changed<EffectBindSampler2D04Comp>
    //         )>
    //     >,
    //     mut commands: Commands,
    // ) {
    //     items.iter_mut().for_each(|(
    //         entity, binddesc
    //         , (tex01, sampl01)
    //         , (tex02, sampl02)
    //         , (tex03, sampl03)
    //         , (tex04, sampl04)
    //         , mut comp
    //     )| {
    //         if binddesc.textures.len() == 4 {
    //             comp.0 = Some(
    //                 EffectTextureSamplers {
    //                     textures: (
    //                         Some(tex01.0.clone()), Some(tex02.0.clone()), Some(tex03.0.clone()),
    //                         Some(tex04.0.clone()), None, None,
    //                         None, None
    //                     ),
    //                     samplers: (
    //                         Some(sampl01.0.clone()), Some(sampl02.0.clone()), Some(sampl03.0.clone()),
    //                         Some(sampl04.0.clone()), None, None,
    //                         None, None
    //                     ),
    //                     binding_count: 8
    //                 }
    //             );
    //         }
    //     });
    // }
    
    // pub fn sys_texture_ready05(
    //     mut items: Query<
    //         (
    //             ObjectID,
    //             &AssetResShaderEffectMeta
    //             , (&EffectBindTexture2D01Comp, &EffectBindSampler2D01Comp)
    //             , (&EffectBindTexture2D02Comp, &EffectBindSampler2D02Comp)
    //             , (&EffectBindTexture2D03Comp, &EffectBindSampler2D03Comp)
    //             , (&EffectBindTexture2D04Comp, &EffectBindSampler2D04Comp)
    //             , (&EffectBindTexture2D05Comp, &EffectBindSampler2D05Comp)
    //             , &mut EffectTextureSamplersComp
    //         ),
    //         Or<(
    //             Changed<EffectBindTexture2D01Comp>, Changed<EffectBindSampler2D01Comp>
    //             , Changed<EffectBindTexture2D02Comp>, Changed<EffectBindSampler2D02Comp>
    //             , Changed<EffectBindTexture2D03Comp>, Changed<EffectBindSampler2D03Comp>
    //             , Changed<EffectBindTexture2D04Comp>, Changed<EffectBindSampler2D04Comp>
    //             , Changed<EffectBindTexture2D05Comp>, Changed<EffectBindSampler2D05Comp>
    //         )>
    //     >,
    //     mut commands: Commands,
    // ) {
    //     items.iter_mut().for_each(|(
    //         entity, binddesc
    //         , (tex01, sampl01)
    //         , (tex02, sampl02)
    //         , (tex03, sampl03)
    //         , (tex04, sampl04)
    //         , (tex05, sampl05)
    //         , mut comp
    //     )| {
    //         if binddesc.textures.len() == 5 {
    //             comp.0 = Some(
    //                 EffectTextureSamplers {
    //                     textures: (
    //                         Some(tex01.0.clone()), Some(tex02.0.clone()), Some(tex03.0.clone()),
    //                         Some(tex04.0.clone()), Some(tex05.0.clone()), None,
    //                         None, None
    //                     ),
    //                     samplers: (
    //                         Some(sampl01.0.clone()), Some(sampl02.0.clone()), Some(sampl03.0.clone()),
    //                         Some(sampl04.0.clone()), Some(sampl05.0.clone()), None,
    //                         None, None
    //                     ),
    //                     binding_count: 10
    //                 }
    //             );
    //         }
    //     });
    // }
    
    // pub fn sys_texture_ready06(
    //     mut items: Query<
    //         (
    //             ObjectID,
    //             &AssetResShaderEffectMeta
    //             , (&EffectBindTexture2D01Comp, &EffectBindSampler2D01Comp)
    //             , (&EffectBindTexture2D02Comp, &EffectBindSampler2D02Comp)
    //             , (&EffectBindTexture2D03Comp, &EffectBindSampler2D03Comp)
    //             , (&EffectBindTexture2D04Comp, &EffectBindSampler2D04Comp)
    //             , (&EffectBindTexture2D05Comp, &EffectBindSampler2D05Comp)
    //             , (&EffectBindTexture2D06Comp, &EffectBindSampler2D06Comp)
    //             , &mut EffectTextureSamplersComp
    //         ),
    //         Or<(
    //             Changed<EffectBindTexture2D01Comp>, Changed<EffectBindSampler2D01Comp>
    //             , Changed<EffectBindTexture2D02Comp>, Changed<EffectBindSampler2D02Comp>
    //             , Changed<EffectBindTexture2D03Comp>, Changed<EffectBindSampler2D03Comp>
    //             , Changed<EffectBindTexture2D04Comp>, Changed<EffectBindSampler2D04Comp>
    //             , Changed<EffectBindTexture2D05Comp>, Changed<EffectBindSampler2D05Comp>
    //             , Changed<EffectBindTexture2D06Comp>, Changed<EffectBindSampler2D06Comp>
    //         )>
    //     >,
    //     mut commands: Commands,
    // ) {
    //     items.iter_mut().for_each(|(
    //         entity, binddesc
    //         , (tex01, sampl01)
    //         , (tex02, sampl02)
    //         , (tex03, sampl03)
    //         , (tex04, sampl04)
    //         , (tex05, sampl05)
    //         , (tex06, sampl06)
    //         , mut comp
    //     )| {
    //         if binddesc.textures.len() == 6 {
    //             comp.0 = Some(
    //                 EffectTextureSamplers {
    //                     textures: (
    //                         Some(tex01.0.clone()), Some(tex02.0.clone()), Some(tex03.0.clone()),
    //                         Some(tex04.0.clone()), Some(tex05.0.clone()), Some(tex06.0.clone()),
    //                         None, None
    //                     ),
    //                     samplers: (
    //                         Some(sampl01.0.clone()), Some(sampl02.0.clone()), Some(sampl03.0.clone()),
    //                         Some(sampl04.0.clone()), Some(sampl05.0.clone()), Some(sampl06.0.clone()),
    //                         None, None
    //                     ),
    //                     binding_count: 12
    //                 }
    //             );
    //         }
    //     });
    // }
    
    pub fn sys_texture_ready07(
        mut items: Query<
            (
                ObjectID,
                &AssetResShaderEffectMeta
                , (&EffectBindTexture2D01Comp, &EffectBindSampler2D01Comp)
                , (&EffectBindTexture2D02Comp, &EffectBindSampler2D02Comp)
                , (&EffectBindTexture2D03Comp, &EffectBindSampler2D03Comp)
                , (&EffectBindTexture2D04Comp, &EffectBindSampler2D04Comp)
                , (&EffectBindTexture2D05Comp, &EffectBindSampler2D05Comp)
                , (&EffectBindTexture2D06Comp, &EffectBindSampler2D06Comp)
                , (&EffectBindTexture2D07Comp, &EffectBindSampler2D07Comp)
                , &mut EffectTextureSamplersComp
            ),
            Or<(
                Changed<EffectBindTexture2D01Comp>, Changed<EffectBindSampler2D01Comp>
                , Changed<EffectBindTexture2D02Comp>, Changed<EffectBindSampler2D02Comp>
                , Changed<EffectBindTexture2D03Comp>, Changed<EffectBindSampler2D03Comp>
                , Changed<EffectBindTexture2D04Comp>, Changed<EffectBindSampler2D04Comp>
                , Changed<EffectBindTexture2D05Comp>, Changed<EffectBindSampler2D05Comp>
                , Changed<EffectBindTexture2D06Comp>, Changed<EffectBindSampler2D06Comp>
                , Changed<EffectBindTexture2D07Comp>, Changed<EffectBindSampler2D07Comp>
            )>
        >,
    ) {
        items.iter_mut().for_each(|(
            _entity, binddesc
            , (tex00, sampl00)
            , (tex01, sampl01)
            , (tex02, sampl02)
            , (tex03, sampl03)
            , (tex04, sampl04)
            , (tex05, sampl05)
            , (tex06, sampl06)
            , mut comp
        )| {
            let need = binddesc.textures.len() as u32;
            let mut textures = (None, None, None, None, None, None, None, None);
            let mut samplers = (None, None, None, None, None, None, None, None);

            if tex00.0.is_some() && sampl00.0.is_some() {
                if 0 < need {
                    textures.0 = tex00.0.clone(); samplers.0 = sampl00.0.clone();
                }
            } else if 0 < need {
                comp.0 = None;
                return;
            }
            
            if tex01.0.is_some() && sampl01.0.is_some() {
                if 1 < need {
                    textures.1 = tex01.0.clone(); samplers.1 = sampl01.0.clone();
                }
            } else if 1 < need {
                comp.0 = None;
                return;
            }
            
            if tex02.0.is_some() && sampl02.0.is_some() {
                if 2 < need {
                    textures.2 = tex02.0.clone(); samplers.2 = sampl02.0.clone();
                }
            } else if 2 < need {
                comp.0 = None;
                return;
            }
            
            if tex03.0.is_some() && sampl03.0.is_some() {
                if 3 < need {
                    textures.3 = tex03.0.clone(); samplers.3 = sampl03.0.clone();
                }
            } else if 3 < need {
                comp.0 = None;
                return;
            }
            
            if tex04.0.is_some() && sampl04.0.is_some() {
                if 4 < need {
                    textures.4 = tex04.0.clone(); samplers.4 = sampl04.0.clone();
                }
            } else if 4 < need {
                comp.0 = None;
                return;
            }
            
            if tex05.0.is_some() && sampl05.0.is_some() {
                if 5 < need {
                    textures.5 = tex05.0.clone(); samplers.5 = sampl05.0.clone();
                }
            } else if 5 < need {
                comp.0 = None;
                return;
            }
            
            if tex06.0.is_some() && sampl06.0.is_some() {
                if 6 < need {
                    textures.6 = tex06.0.clone(); samplers.6 = sampl06.0.clone();
                }
            } else if 6 < need {
                comp.0 = None;
                return;
            }

            comp.0 = Some(
                EffectTextureSamplers {
                    textures,
                    samplers,
                    binding_count: need * 2
                }
            );
        });
    }
    
    // pub fn sys_texture_ready08(
        // items: Query<
        //     (
        //         ObjectID,
        //         &AssetResShaderEffectMeta
        //         , (&EffectBindTexture2D01Comp, &EffectBindSampler2D01Comp)
        //         , (&EffectBindTexture2D02Comp, &EffectBindSampler2D02Comp)
        //         , (&EffectBindTexture2D03Comp, &EffectBindSampler2D03Comp)
        //         , (&EffectBindTexture2D04Comp, &EffectBindSampler2D04Comp)
        //         , (&EffectBindTexture2D05Comp, &EffectBindSampler2D05Comp)
        //         , (&EffectBindTexture2D06Comp, &EffectBindSampler2D06Comp)
        //         , (&EffectBindTexture2D07Comp, &EffectBindSampler2D07Comp)
        //         , (&EffectBindTexture2D08Comp, &EffectBindSampler2D08Comp)
        //     ),
        //     Or<(
        //         Changed<EffectBindTexture2D01Comp>, Changed<EffectBindSampler2D01Comp>
        //         , Changed<EffectBindTexture2D02Comp>, Changed<EffectBindSampler2D02Comp>
        //         , Changed<EffectBindTexture2D03Comp>, Changed<EffectBindSampler2D03Comp>
        //         , Changed<EffectBindTexture2D04Comp>, Changed<EffectBindSampler2D04Comp>
        //         , Changed<EffectBindTexture2D05Comp>, Changed<EffectBindSampler2D05Comp>
        //         , Changed<EffectBindTexture2D06Comp>, Changed<EffectBindSampler2D06Comp>
        //         , Changed<EffectBindTexture2D07Comp>, Changed<EffectBindSampler2D07Comp>
        //         , Changed<EffectBindTexture2D08Comp>, Changed<EffectBindSampler2D08Comp>
        //     )>
        // >,
        // mut commands: Commands,
    // ) {
        // items.iter().for_each(|(
        //     entity, binddesc
        //     , (tex01, sampl01)
        //     , (tex02, sampl02)
        //     , (tex03, sampl03)
        //     , (tex04, sampl04)
        //     , (tex05, sampl05)
        //     , (tex06, sampl06)
        //     , (tex07, sampl07)
        //     , (tex08, sampl08)
        // )| {
        //     if binddesc.textures.len() == 8 {
        //         let value = EffectTextureSamplersComp(
        //             EffectTextureSamplers {
        //                 textures: (
        //                     Some(tex01.0.clone()), Some(tex02.0.clone()), Some(tex03.0.clone()),
        //                     Some(tex04.0.clone()), Some(tex05.0.clone()), Some(tex06.0.clone()),
        //                     Some(tex07.0.clone()), Some(tex08.0.clone())
        //                 ),
        //                 samplers: (
        //                     Some(sampl01.0.clone()), Some(sampl02.0.clone()), Some(sampl03.0.clone()),
        //                     Some(sampl04.0.clone()), Some(sampl05.0.clone()), Some(sampl06.0.clone()),
        //                     Some(sampl07.0.clone()), Some(sampl08.0.clone())
        //                 ),
        //                 binding_count: 16
        //             }
        //         );
        //         if let Some(mut cmd) = commands.get_entity(entity) {
        //             cmd.insert(value);
        //         }
        //     }
        // });
    // }