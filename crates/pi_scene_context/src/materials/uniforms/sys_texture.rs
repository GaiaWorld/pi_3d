use std::{ sync::Arc};

use pi_engine_shell::prelude::*;

use crate::{
    materials::{shader_effect::AssetResShaderEffectMeta},
};

use super::{
    texture::{TextureSlot01, TextureSlot02, TextureSlot03, TextureSlot04},
};

pub type PluginTextureSlot01Load = PluginImageLoad<TextureSlot01, EffectBindTexture2D01Comp>;
pub type PluginTextureSlot02Load = PluginImageLoad<TextureSlot02, EffectBindTexture2D02Comp>;
pub type PluginTextureSlot03Load = PluginImageLoad<TextureSlot03, EffectBindTexture2D03Comp>;
pub type PluginTextureSlot04Load = PluginImageLoad<TextureSlot04, EffectBindTexture2D04Comp>;

    pub fn sys_texture_ready01(
        items: Query<
            (
                ObjectID,
                &AssetResShaderEffectMeta,
                &EffectBindTexture2D01Comp, &TextureSlot01,
                &EffectBindSampler2D01Comp,
            ),
            Or<(
                Changed<EffectBindTexture2D01Comp>, Changed<EffectBindSampler2D01Comp>,
            )>
        >,
        mut commands: Commands,
    ) {
        items.iter().for_each(|(
            id_obj,
            binddesc,
            tex01, slot01,
            sampl01
        )| {
            // log::debug!("SysTextureResReady1 >");
            if binddesc.textures.len() == 1 {
                let value = EffectTextureSamplersComp(
                    EffectTextureSamplers {
                        textures: (
                            Some(tex01.0.clone()), None, None,
                            None, None, None
                        ),
                        samplers: (
                            Some(sampl01.0.clone()), None, None,
                            None, None, None
                        ),
                        binding_count: 2
                    }
                );

                log::warn!("SysTextureResReady1 >>");
                // let mut list = vec![];
                // list.push(slot01.param());

                // let useinfo = binddesc.textures.use_info(list);
                // let bind = EffectTextureAndSamplerBinds::new(&useinfo);

                commands.entity(id_obj).insert(value);
            }
        });
    }

    pub fn sys_texture_ready02(
        items: Query<
            (
                ObjectID,
                &AssetResShaderEffectMeta
                , &EffectBindTexture2D01Comp, &TextureSlot01
                , &EffectBindTexture2D02Comp, &TextureSlot02
                , &EffectBindSampler2D01Comp
                , &EffectBindSampler2D02Comp
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
            , tex01, slot01
            , tex02, slot02
            , sampl01
            , sampl02
        )| {
            if binddesc.textures.len() == 2 {
                // // log::debug!("SysTextureResReady1 >>");
                // let mut list = vec![];
                // list.push(slot1.param());
                // list.push(slot2.param());
               
                // let useinfo = binddesc.textures.use_info(list);
                // let bind = EffectTextureAndSamplerBinds::new(&useinfo);
                // params.insert(id_obj, BindEffectTextureAndSamplers(Arc::new(bind)));
                
                let value = EffectTextureSamplersComp(
                    EffectTextureSamplers {
                        textures: (
                            Some(tex01.0.clone()), Some(tex02.0.clone()), None,
                            None, None, None
                        ),
                        samplers: (
                            Some(sampl01.0.clone()), Some(sampl02.0.clone()), None,
                            None, None, None
                        ),
                        binding_count: 4
                    }
                );
                commands.entity(id_obj).insert(value);
            }
        });
    }

