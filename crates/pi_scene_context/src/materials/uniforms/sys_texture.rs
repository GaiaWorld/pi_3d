use std::{ sync::Arc};

use pi_ecs::{prelude::{Query, Res, Commands}, query::{Changed, Or}};
use pi_ecs_macros::setup;
use pi_engine_shell::{object::{GameObject, ObjectID}, assets::image_texture_load::{CalcImageLoad}, run_stage::{TSystemStageInfo}};
use pi_render::{rhi::{device::RenderDevice}, render_3d::{binds::{effect_texture2d::{EffectBindTexture2D01, EffectBindTexture2D02, EffectBindTexture2D03, EffectBindTexture2D04}, effect_sampler2d::{EffectBindSampler2D01, EffectBindSampler2D02}}, bind_groups::texture_sampler::EffectTextureSamplers}};

use crate::{
    materials::{shader_effect::AssetResShaderEffectMeta},
};

use super::{
    texture::{ValueTextureKey, TextureSlot01, TextureSlot02, TextureSlot03, TextureSlot04},
    sys_uniform::SysMaterialTexturesChange
};

// #[derive(Debug, Clone)]
// enum ECommand {
//     Texture(ObjectID, UniformPropertyName, Option<UniformTextureWithSamplerParam>),
// }

// #[derive(Debug, Default)]
// struct SingleTextureCommands(pub Vec<ECommand>);

// pub struct SysTextureCommand;
// impl TSystemStageInfo for SysTextureCommand {
// }
// #[setup]
// impl SysTextureCommand {
//     #[system]
//     fn cmd(
//         mut cmds: ResMut<SingleTextureCommands>,
//         mut items: Query<
//             GameObject,
//             &AssetResShaderEffectMeta,
//         >,
//         device: Res<RenderDevice>,
//         mut slot01_cmd: Commands<GameObject, TextureSlot01>,
//         // mut samp01_cmd: Commands<GameObject, SamplerSlot01>,
//         mut slot02_cmd: Commands<GameObject, TextureSlot02>,
//         // mut samp02_cmd: Commands<GameObject, SamplerSlot02>,
//         mut slot03_cmd: Commands<GameObject, TextureSlot03>,
//         // mut samp03_cmd: Commands<GameObject, SamplerSlot03>,
//         mut slot04_cmd: Commands<GameObject, TextureSlot04>,
//         // mut samp04_cmd: Commands<GameObject, SamplerSlot04>,

//     ) {
//         let mut list = replace(&mut cmds.0, vec![]);
//         list.drain(..).for_each(|cmd| {
//             match cmd.clone() {
//                 ECommand::Texture(entity, slotname, param) => {
//                     if let Some(item) = items.get_mut(entity) {
//                         if let Ok(index) = item.query_tex_slot(&slotname) {
//                             let param = if let Some(param) = param {
//                                 param
//                             } else {
//                                 let item = item.textures.descs.get(index).unwrap();
//                                 UniformTextureWithSamplerParam {
//                                     slotname: slotname.clone(),
//                                     filter: true,
//                                     sample: SamplerDesc::default(),
//                                     url: Atom::from(DefaultTexture::path(item.initial, wgpu::TextureDimension::D2)),
//                                 }
//                             };
//                             if index == 0 {
//                                 slot01_cmd.insert(entity.clone(), TextureSlot01::new(param));
//                             }
//                             else if index == 1 {
//                                 slot02_cmd.insert(entity.clone(), TextureSlot02::new(param));
//                             }
//                             else if index == 2 {
//                                 slot03_cmd.insert(entity.clone(), TextureSlot03::new(param));
//                             }
//                             else if index == 3 {
//                                 slot04_cmd.insert(entity.clone(), TextureSlot04::new(param));
//                             }
//                         }
//                     } else {
//                         cmds.0.push(cmd);
//                     }
//                 },
//             }
//         });
//     }
// }

pub struct SysTextureLoad;
impl TSystemStageInfo for SysTextureLoad {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysMaterialTexturesChange::key()
        ]
    }
}

pub type SysTextureSlot01Load = CalcImageLoad<TextureSlot01, EffectBindTexture2D01>;
pub type SysTextureSlot02Load = CalcImageLoad<TextureSlot02, EffectBindTexture2D02>;
pub type SysTextureSlot03Load = CalcImageLoad<TextureSlot03, EffectBindTexture2D03>;
pub type SysTextureSlot04Load = CalcImageLoad<TextureSlot04, EffectBindTexture2D04>;

pub struct SysTextureResReady1;
impl TSystemStageInfo for SysTextureResReady1 {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysTextureLoad::key(),
        ]
    }
}
#[setup]
impl SysTextureResReady1 {
    #[system]
    pub fn sys(
        items: Query<
            GameObject,
            (
                ObjectID,
                &AssetResShaderEffectMeta,
                &EffectBindTexture2D01, &TextureSlot01,
                &EffectBindSampler2D01,
            ),
            Or<(
                Changed<EffectBindTexture2D01>, Changed<EffectBindSampler2D01>,
            )>
        >,
        mut params: Commands<GameObject, EffectTextureSamplers>,
        device: Res<RenderDevice>,
    ) {
        items.iter().for_each(|(
            id_obj,
            binddesc,
            tex01, slot01,
            sampl01
        )| {
            // log::debug!("SysTextureResReady1 >");
            if binddesc.textures.len() == 1 {
                let value = EffectTextureSamplers {
                    textures: (
                        Some(tex01.clone()), None, None,
                        None, None, None
                    ),
                    samplers: (
                        Some(sampl01.clone()), None, None,
                        None, None, None
                    )
                };

                log::debug!("SysTextureResReady1 >>");
                // let mut list = vec![];
                // list.push(slot01.param());

                // let useinfo = binddesc.textures.use_info(list);
                // let bind = EffectTextureAndSamplerBinds::new(&useinfo);

                params.insert(id_obj, value);
            }
        });
    }
}

pub struct SysTextureResReady2;
impl TSystemStageInfo for SysTextureResReady2 {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysTextureLoad::key(),
        ]
    }
}
#[setup]
impl SysTextureResReady2 {
    #[system]
    pub fn sys(
        items: Query<
            GameObject,
            (
                ObjectID,
                &AssetResShaderEffectMeta
                , &EffectBindTexture2D01, &TextureSlot01
                , &EffectBindTexture2D02, &TextureSlot02
                , &EffectBindSampler2D01
                , &EffectBindSampler2D02
            ),
            Or<(
                Changed<EffectBindTexture2D01>
                , Changed<EffectBindTexture2D02>
            )>
        >,
        mut params: Commands<GameObject, EffectTextureSamplers>,
        device: Res<RenderDevice>,
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
                
                let value = EffectTextureSamplers {
                    textures: (
                        Some(tex01.clone()), Some(tex02.clone()), None,
                        None, None, None
                    ),
                    samplers: (
                        Some(sampl01.clone()), Some(sampl02.clone()), None,
                        None, None, None
                    )
                };
                params.insert(id_obj, value);
            }
        });
    }
}
