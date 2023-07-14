use std::{ sync::Arc};

use pi_engine_shell::prelude::*;

use crate::{
    materials::{shader_effect::*},
};

use super::{
    texture::*,
};

pub type PluginTextureSlot01Load = PluginImageLoad<TextureSlot01, EffectBindTexture2D01Comp>;
pub type PluginTextureSlot02Load = PluginImageLoad<TextureSlot02, EffectBindTexture2D02Comp>;
pub type PluginTextureSlot03Load = PluginImageLoad<TextureSlot03, EffectBindTexture2D03Comp>;
pub type PluginTextureSlot04Load = PluginImageLoad<TextureSlot04, EffectBindTexture2D04Comp>;
pub type PluginTextureSlot05Load = PluginImageLoad<TextureSlot05, EffectBindTexture2D05Comp>;
pub type PluginTextureSlot06Load = PluginImageLoad<TextureSlot06, EffectBindTexture2D06Comp>;

    pub fn sys_texture_ready01(
        items: Query<
            (
                ObjectID
                , &AssetResShaderEffectMeta
                , (&EffectBindTexture2D01Comp, &TextureSlot01, &EffectBindSampler2D01Comp)
            ),
            Or<(
                Changed<EffectBindTexture2D01Comp>, Changed<EffectBindSampler2D01Comp>,
            )>
        >,
        mut commands: Commands,
    ) {
        items.iter().for_each(|(
            id_obj
            , binddesc
            , (tex01, slot01, sampl01)
        )| {
            // log::debug!("SysTextureResReady1 >");
            if binddesc.textures.len() == 1 {
                let value = EffectTextureSamplersComp(
                    EffectTextureSamplers {
                        textures: (
                            Some(tex01.0.clone()), None, None,
                            None, None, None,
                            None, None
                        ),
                        samplers: (
                            Some(sampl01.0.clone()), None, None,
                            None, None, None,
                            None, None
                        ),
                        binding_count: 2
                    }
                );

                // log::warn!("SysTextureResReady1 >>");
                // let mut list = vec![];
                // list.push(slot01.param());

                // let useinfo = binddesc.textures.use_info(list);
                // let bind = EffectTextureAndSamplerBinds::new(&useinfo);

                if let Some(mut cmd) = commands.get_entity(id_obj) {
                    cmd.insert(value);
                }
            }
        });
    }

    pub fn sys_texture_ready02(
        items: Query<
            (
                ObjectID,
                &AssetResShaderEffectMeta
                , (&EffectBindTexture2D01Comp, &TextureSlot01, &EffectBindSampler2D01Comp)
                , (&EffectBindTexture2D02Comp, &TextureSlot02, &EffectBindSampler2D02Comp)
            ),
            Or<(
                Changed<EffectBindTexture2D01Comp>
                , Changed<EffectBindTexture2D02Comp>
            )>
        >,
        mut commands: Commands,
    ) {
        items.iter().for_each(|(
            id_obj, binddesc
            , (tex01, slot01, sampl01)
            , (tex02, slot02, sampl02)
        )| {
            if binddesc.textures.len() == 2 {
                let value = EffectTextureSamplersComp(
                    EffectTextureSamplers {
                        textures: (
                            Some(tex01.0.clone()), Some(tex02.0.clone()), None,
                            None, None, None,
                            None, None
                        ),
                        samplers: (
                            Some(sampl01.0.clone()), Some(sampl02.0.clone()), None,
                            None, None, None,
                            None, None
                        ),
                        binding_count: 4
                    }
                );
                if let Some(mut cmd) = commands.get_entity(id_obj) {
                    cmd.insert(value);
                }
            }
        });
    }


    pub fn sys_texture_ready03(
        items: Query<
            (
                ObjectID,
                &AssetResShaderEffectMeta
                , (&EffectBindTexture2D01Comp, &TextureSlot01, &EffectBindSampler2D01Comp)
                , (&EffectBindTexture2D02Comp, &TextureSlot02, &EffectBindSampler2D02Comp)
                , (&EffectBindTexture2D03Comp, &TextureSlot03, &EffectBindSampler2D03Comp)
            ),
            Or<(
                Changed<EffectBindTexture2D01Comp>
                , Changed<EffectBindTexture2D02Comp>
                , Changed<EffectBindTexture2D03Comp>
            )>
        >,
        mut commands: Commands,
    ) {
        items.iter().for_each(|(
            id_obj, binddesc
            , (tex01, slot01, sampl01)
            , (tex02, slot02, sampl02)
            , (tex03, slot03, sampl03)
        )| {
            if binddesc.textures.len() == 3 {
                let value = EffectTextureSamplersComp(
                    EffectTextureSamplers {
                        textures: (
                            Some(tex01.0.clone()), Some(tex02.0.clone()), Some(tex03.0.clone()),
                            None, None, None,
                            None, None
                        ),
                        samplers: (
                            Some(sampl01.0.clone()), Some(sampl02.0.clone()), Some(sampl03.0.clone()),
                            None, None, None,
                            None, None
                        ),
                        binding_count: 6
                    }
                );
                if let Some(mut cmd) = commands.get_entity(id_obj) {
                    cmd.insert(value);
                }
            }
        });
    }
    
    pub fn sys_texture_ready04(
        items: Query<
            (
                ObjectID,
                &AssetResShaderEffectMeta
                , (&EffectBindTexture2D01Comp, &TextureSlot01, &EffectBindSampler2D01Comp)
                , (&EffectBindTexture2D02Comp, &TextureSlot02, &EffectBindSampler2D02Comp)
                , (&EffectBindTexture2D03Comp, &TextureSlot03, &EffectBindSampler2D03Comp)
                , (&EffectBindTexture2D04Comp, &TextureSlot04, &EffectBindSampler2D04Comp)
            ),
            Or<(
                Changed<EffectBindTexture2D01Comp>
                , Changed<EffectBindTexture2D02Comp>
                , Changed<EffectBindTexture2D03Comp>
                , Changed<EffectBindTexture2D04Comp>
            )>
        >,
        mut commands: Commands,
    ) {
        items.iter().for_each(|(
            id_obj, binddesc
            , (tex01, slot01, sampl01)
            , (tex02, slot02, sampl02)
            , (tex03, slot03, sampl03)
            , (tex04, slot04, sampl04)
        )| {
            if binddesc.textures.len() == 4 {
                let value = EffectTextureSamplersComp(
                    EffectTextureSamplers {
                        textures: (
                            Some(tex01.0.clone()), Some(tex02.0.clone()), Some(tex03.0.clone()),
                            Some(tex04.0.clone()), None, None,
                            None, None
                        ),
                        samplers: (
                            Some(sampl01.0.clone()), Some(sampl02.0.clone()), Some(sampl03.0.clone()),
                            Some(sampl04.0.clone()), None, None,
                            None, None
                        ),
                        binding_count: 8
                    }
                );
                if let Some(mut cmd) = commands.get_entity(id_obj) {
                    cmd.insert(value);
                }
            }
        });
    }
    
    pub fn sys_texture_ready05(
        items: Query<
            (
                ObjectID,
                &AssetResShaderEffectMeta
                , (&EffectBindTexture2D01Comp, &TextureSlot01, &EffectBindSampler2D01Comp)
                , (&EffectBindTexture2D02Comp, &TextureSlot02, &EffectBindSampler2D02Comp)
                , (&EffectBindTexture2D03Comp, &TextureSlot03, &EffectBindSampler2D03Comp)
                , (&EffectBindTexture2D04Comp, &TextureSlot04, &EffectBindSampler2D04Comp)
                , (&EffectBindTexture2D05Comp, &TextureSlot05, &EffectBindSampler2D05Comp)
            ),
            Or<(
                Changed<EffectBindTexture2D01Comp>
                , Changed<EffectBindTexture2D02Comp>
                , Changed<EffectBindTexture2D03Comp>
                , Changed<EffectBindTexture2D04Comp>
                , Changed<EffectBindTexture2D05Comp>
            )>
        >,
        mut commands: Commands,
    ) {
        items.iter().for_each(|(
            id_obj, binddesc
            , (tex01, slot01, sampl01)
            , (tex02, slot02, sampl02)
            , (tex03, slot03, sampl03)
            , (tex04, slot04, sampl04)
            , (tex05, slot05, sampl05)
        )| {
            if binddesc.textures.len() == 5 {
                let value = EffectTextureSamplersComp(
                    EffectTextureSamplers {
                        textures: (
                            Some(tex01.0.clone()), Some(tex02.0.clone()), Some(tex03.0.clone()),
                            Some(tex04.0.clone()), Some(tex05.0.clone()), None,
                            None, None
                        ),
                        samplers: (
                            Some(sampl01.0.clone()), Some(sampl02.0.clone()), Some(sampl03.0.clone()),
                            Some(sampl04.0.clone()), Some(sampl05.0.clone()), None,
                            None, None
                        ),
                        binding_count: 10
                    }
                );
                if let Some(mut cmd) = commands.get_entity(id_obj) {
                    cmd.insert(value);
                }
            }
        });
    }
    
    pub fn sys_texture_ready06(
        items: Query<
            (
                ObjectID,
                &AssetResShaderEffectMeta
                , (&EffectBindTexture2D01Comp, &TextureSlot01, &EffectBindSampler2D01Comp)
                , (&EffectBindTexture2D02Comp, &TextureSlot02, &EffectBindSampler2D02Comp)
                , (&EffectBindTexture2D03Comp, &TextureSlot03, &EffectBindSampler2D03Comp)
                , (&EffectBindTexture2D04Comp, &TextureSlot04, &EffectBindSampler2D04Comp)
                , (&EffectBindTexture2D05Comp, &TextureSlot05, &EffectBindSampler2D05Comp)
                , (&EffectBindTexture2D06Comp, &TextureSlot06, &EffectBindSampler2D06Comp)
            ),
            Or<(
                Changed<EffectBindTexture2D01Comp>
                , Changed<EffectBindTexture2D02Comp>
                , Changed<EffectBindTexture2D03Comp>
                , Changed<EffectBindTexture2D04Comp>
                , Changed<EffectBindTexture2D05Comp>
                , Changed<EffectBindTexture2D06Comp>
            )>
        >,
        mut commands: Commands,
    ) {
        items.iter().for_each(|(
            id_obj, binddesc
            , (tex01, slot01, sampl01)
            , (tex02, slot02, sampl02)
            , (tex03, slot03, sampl03)
            , (tex04, slot04, sampl04)
            , (tex05, slot05, sampl05)
            , (tex06, slot06, sampl06)
        )| {
            if binddesc.textures.len() == 6 {
                let value = EffectTextureSamplersComp(
                    EffectTextureSamplers {
                        textures: (
                            Some(tex01.0.clone()), Some(tex02.0.clone()), Some(tex03.0.clone()),
                            Some(tex04.0.clone()), Some(tex05.0.clone()), Some(tex06.0.clone()),
                            None, None
                        ),
                        samplers: (
                            Some(sampl01.0.clone()), Some(sampl02.0.clone()), Some(sampl03.0.clone()),
                            Some(sampl04.0.clone()), Some(sampl05.0.clone()), Some(sampl06.0.clone()),
                            None, None
                        ),
                        binding_count: 12
                    }
                );
                if let Some(mut cmd) = commands.get_entity(id_obj) {
                    cmd.insert(value);
                }
            }
        });
    }
    
    pub fn sys_texture_ready07(
        items: Query<
            (
                ObjectID,
                &AssetResShaderEffectMeta
                , (&EffectBindTexture2D01Comp, &TextureSlot01, &EffectBindSampler2D01Comp)
                , (&EffectBindTexture2D02Comp, &TextureSlot02, &EffectBindSampler2D02Comp)
                , (&EffectBindTexture2D03Comp, &TextureSlot03, &EffectBindSampler2D03Comp)
                , (&EffectBindTexture2D04Comp, &TextureSlot04, &EffectBindSampler2D04Comp)
                , (&EffectBindTexture2D05Comp, &TextureSlot05, &EffectBindSampler2D05Comp)
                , (&EffectBindTexture2D06Comp, &TextureSlot06, &EffectBindSampler2D06Comp)
                , (&EffectBindTexture2D07Comp, &TextureSlot07, &EffectBindSampler2D07Comp)
            ),
            Or<(
                Changed<EffectBindTexture2D01Comp>
                , Changed<EffectBindTexture2D02Comp>
                , Changed<EffectBindTexture2D03Comp>
                , Changed<EffectBindTexture2D04Comp>
                , Changed<EffectBindTexture2D05Comp>
                , Changed<EffectBindTexture2D06Comp>
                , Changed<EffectBindTexture2D07Comp>
            )>
        >,
        mut commands: Commands,
    ) {
        items.iter().for_each(|(
            id_obj, binddesc
            , (tex01, slot01, sampl01)
            , (tex02, slot02, sampl02)
            , (tex03, slot03, sampl03)
            , (tex04, slot04, sampl04)
            , (tex05, slot05, sampl05)
            , (tex06, slot06, sampl06)
            , (tex07, slot07, sampl07)
        )| {
            if binddesc.textures.len() == 7 {
                let value = EffectTextureSamplersComp(
                    EffectTextureSamplers {
                        textures: (
                            Some(tex01.0.clone()), Some(tex02.0.clone()), Some(tex03.0.clone()),
                            Some(tex04.0.clone()), Some(tex05.0.clone()), Some(tex06.0.clone()),
                            Some(tex07.0.clone()), None
                        ),
                        samplers: (
                            Some(sampl01.0.clone()), Some(sampl02.0.clone()), Some(sampl03.0.clone()),
                            Some(sampl04.0.clone()), Some(sampl05.0.clone()), Some(sampl06.0.clone()),
                            Some(sampl07.0.clone()), None
                        ),
                        binding_count: 14
                    }
                );
                if let Some(mut cmd) = commands.get_entity(id_obj) {
                    cmd.insert(value);
                }
            }
        });
    }
    
    pub fn sys_texture_ready08(
        items: Query<
            (
                ObjectID,
                &AssetResShaderEffectMeta
                , (&EffectBindTexture2D01Comp, &TextureSlot01, &EffectBindSampler2D01Comp)
                , (&EffectBindTexture2D02Comp, &TextureSlot02, &EffectBindSampler2D02Comp)
                , (&EffectBindTexture2D03Comp, &TextureSlot03, &EffectBindSampler2D03Comp)
                , (&EffectBindTexture2D04Comp, &TextureSlot04, &EffectBindSampler2D04Comp)
                , (&EffectBindTexture2D05Comp, &TextureSlot05, &EffectBindSampler2D05Comp)
                , (&EffectBindTexture2D06Comp, &TextureSlot06, &EffectBindSampler2D06Comp)
                , (&EffectBindTexture2D07Comp, &TextureSlot07, &EffectBindSampler2D07Comp)
                , (&EffectBindTexture2D08Comp, &TextureSlot08, &EffectBindSampler2D08Comp)
            ),
            Or<(
                Changed<EffectBindTexture2D01Comp>
                , Changed<EffectBindTexture2D02Comp>
                , Changed<EffectBindTexture2D03Comp>
                , Changed<EffectBindTexture2D04Comp>
                , Changed<EffectBindTexture2D05Comp>
                , Changed<EffectBindTexture2D06Comp>
                , Changed<EffectBindTexture2D07Comp>
                , Changed<EffectBindTexture2D08Comp>
            )>
        >,
        mut commands: Commands,
    ) {
        items.iter().for_each(|(
            id_obj, binddesc
            , (tex01, slot01, sampl01)
            , (tex02, slot02, sampl02)
            , (tex03, slot03, sampl03)
            , (tex04, slot04, sampl04)
            , (tex05, slot05, sampl05)
            , (tex06, slot06, sampl06)
            , (tex07, slot07, sampl07)
            , (tex08, slot08, sampl08)
        )| {
            if binddesc.textures.len() == 8 {
                let value = EffectTextureSamplersComp(
                    EffectTextureSamplers {
                        textures: (
                            Some(tex01.0.clone()), Some(tex02.0.clone()), Some(tex03.0.clone()),
                            Some(tex04.0.clone()), Some(tex05.0.clone()), Some(tex06.0.clone()),
                            Some(tex07.0.clone()), Some(tex08.0.clone())
                        ),
                        samplers: (
                            Some(sampl01.0.clone()), Some(sampl02.0.clone()), Some(sampl03.0.clone()),
                            Some(sampl04.0.clone()), Some(sampl05.0.clone()), Some(sampl06.0.clone()),
                            Some(sampl07.0.clone()), Some(sampl08.0.clone())
                        ),
                        binding_count: 16
                    }
                );
                if let Some(mut cmd) = commands.get_entity(id_obj) {
                    cmd.insert(value);
                }
            }
        });
    }