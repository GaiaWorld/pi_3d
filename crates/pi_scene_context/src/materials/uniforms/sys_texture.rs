use std::{marker::PhantomData, mem::replace};

use pi_assets::{mgr::AssetMgr, asset::GarbageEmpty};
use pi_ecs::{prelude::{Query, Res, ResMut, Setup, Commands}, query::{Write, Changed, Or}, entity};
use pi_ecs_macros::setup;
use pi_engine_shell::{object::{GameObject, ObjectID}, assets::image_texture_load::{CalcImageLoad, ImageAwait, SysImageLoad}, engine_shell::EnginShell, run_stage::{TSystemStageInfo, ERunStageChap}};
use pi_render::rhi::{device::RenderDevice, asset::TextureRes};
use pi_share::Share;
use render_resource::{sampler::{SamplerPool, SamplerDesc}, ImageAssetKey};
use render_shader::unifrom_code::{UniformPropertyName, MaterialTextureBindDesc};

use crate::{
    materials::{shader_effect::AssetResShaderEffectMeta},
    bindgroup::{RenderBindGroupKey, RenderBindGroupPool}
};

use super::{
    texture_uniform::{MaterialTextureBindGroupID, TForTextureBindGroup},
    texture::{ValueTextureKey, TextureSlot01, TextureResSlot01, TextureResSlot03, TextureResSlot04, TextureResSlot02, TextureSlot02, TextureSlot03, TextureSlot04, UniformSampler, SamplerSlot01, UniformTexture, SamplerSlot02, SamplerSlot03, SamplerSlot04, ETextureSlot}
};

#[derive(Debug, Clone)]
enum ECommand {
    TexturePath(ObjectID, UniformPropertyName, Option<ImageAssetKey>),
    Sampler(ObjectID, UniformPropertyName, SamplerDesc),
}

#[derive(Debug, Default)]
struct SingleTextureCommands(pub Vec<ECommand>);

pub struct SysTextureCommand;
impl TSystemStageInfo for SysTextureCommand {
}
#[setup]
impl SysTextureCommand {
    #[system]
    fn cmd(
        mut cmds: ResMut<SingleTextureCommands>,
        mut items: Query<
            GameObject,
            &AssetResShaderEffectMeta
        >,
        device: Res<RenderDevice>,
        mut samplerpool: ResMut<SamplerPool>,
        mut slot01_cmd: Commands<GameObject, TextureSlot01>,
        mut samp01_cmd: Commands<GameObject, SamplerSlot01>,
        mut slot02_cmd: Commands<GameObject, TextureSlot02>,
        mut samp02_cmd: Commands<GameObject, SamplerSlot02>,
        mut slot03_cmd: Commands<GameObject, TextureSlot03>,
        mut samp03_cmd: Commands<GameObject, SamplerSlot03>,
        mut slot04_cmd: Commands<GameObject, TextureSlot04>,
        mut samp04_cmd: Commands<GameObject, SamplerSlot04>,
    ) {
        let mut list = replace(&mut cmds.0, vec![]);
        list.drain(..).for_each(|cmd| {
            match cmd.clone() {
                ECommand::TexturePath(entity, slotname, path) => {
                    if let Some(mut item) = items.get_mut(entity) {
                        if let Ok(index) = item.query_tex_slot(&slotname) {
                            if index == 0 {
                                if let Some(path) = path { slot01_cmd.insert(entity.clone(), TextureSlot01::new(path)); } else { slot01_cmd.delete(entity.clone()); };
                            }
                            else if index == 1 {
                                if let Some(path) = path { slot02_cmd.insert(entity.clone(), TextureSlot02::new(path)); } else { slot02_cmd.delete(entity.clone()); };
                            }
                            else if index == 2 {
                                if let Some(path) = path { slot03_cmd.insert(entity.clone(), TextureSlot03::new(path)); } else { slot03_cmd.delete(entity.clone()); };
                            }
                            else if index == 3 {
                                if let Some(path) = path { slot04_cmd.insert(entity.clone(), TextureSlot04::new(path)); } else { slot04_cmd.delete(entity.clone()); };
                            }
                        }
                    } else {
                        cmds.0.push(cmd);
                    }
                },
                ECommand::Sampler(entity, slotname, sampler) => {
                    if let Some(mut item) = items.get_mut(entity) {
                        if let Ok(index) = item.query_tex_slot(&slotname) {
                            if index == 0 {
                                samp01_cmd.insert(entity.clone(), SamplerSlot01::new(&sampler, &device, &mut samplerpool));
                            }
                            else if index == 1 {
                                samp02_cmd.insert(entity.clone(), SamplerSlot02::new(&sampler, &device, &mut samplerpool));
                            }
                            else if index == 2 {
                                samp03_cmd.insert(entity.clone(), SamplerSlot03::new(&sampler, &device, &mut samplerpool));
                            }
                            else if index == 3 {
                                samp04_cmd.insert(entity.clone(), SamplerSlot04::new(&sampler, &device, &mut samplerpool));
                            }
                        }
                    } else {
                        cmds.0.push(cmd);
                    }
                },
            }
        });
    }
}

pub type SysTextureSlot01Load = CalcImageLoad<TextureSlot01, TextureResSlot01>;
pub type SysTextureSlot02Load = CalcImageLoad<TextureSlot02, TextureResSlot02>;
pub type SysTextureSlot03Load = CalcImageLoad<TextureSlot03, TextureResSlot03>;
pub type SysTextureSlot04Load = CalcImageLoad<TextureSlot04, TextureResSlot04>;

pub struct SysTextureReady;
impl TSystemStageInfo for SysTextureReady {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysTextureCommand::key(), SysImageLoad::key(), 
        ]
    }
}

pub struct SysTextureResReady1;
#[setup]
impl SysTextureResReady1 {
    #[system]
    pub fn sys(
        items: Query<
            GameObject,
            (
                &MaterialTextureBindGroupID, &AssetResShaderEffectMeta,
                &TextureResSlot01, &SamplerSlot01
            ),
            Or<(
                Changed<TextureResSlot01>, Changed<SamplerSlot01>
            )>
        >,
        mut bindgrouppool: ResMut<RenderBindGroupPool>,
        device: Res<RenderDevice>,
    ) {
        items.iter().for_each(|(bindgroup, binddesc, tex1, sampler1)| {
            log::debug!("SysTextureResReady1 >");
            if let Some(binddesc) = &binddesc.textures {
                if binddesc.list.len() == 1 {
                    log::debug!("SysTextureResReady1 >>");
                    if let Some(group) = bindgrouppool.get_mut(&bindgroup.0) {
                        log::debug!("SysTextureResReady1 >>>");
                        // binddesc.bind_group(&device, group, &[tex1.texture()], &[sampler1.sampler()]);
                    }
                }
            }
        });
    }
}

pub struct SysTextureResReady2;
#[setup]
impl SysTextureResReady2 {
    #[system]
    pub fn sys(
        items: Query<
            GameObject,
            (
                &MaterialTextureBindGroupID, &MaterialTextureBindDesc
                , &TextureResSlot01, &SamplerSlot01
                , &TextureResSlot02, &SamplerSlot02
            ),
            Or<(
                Changed<TextureResSlot01>, Changed<SamplerSlot01>
                , Changed<TextureResSlot02>, Changed<SamplerSlot02>
            )>
        >,
        mut bindgrouppool: ResMut<RenderBindGroupPool>,
        device: Res<RenderDevice>,
    ) {
        items.iter().for_each(|(
            bindgroup, binddesc
            , tex1, sampler1
            , tex2, sampler2
        )| {
            if binddesc.list.len() == 2 {
                if let Some(group) = bindgrouppool.get_mut(&bindgroup.0) {
                    binddesc.bind_group(&device, group, &[tex1.texture(), tex2.texture()], &[sampler1.sampler(), sampler2.sampler()]);
                }
            }
        });
    }
}

pub trait InterfaceMaterialTexture {
    fn set_texture(
        &self,
        entity: ObjectID,
        slot: &str,
        image: Option<ImageAssetKey>,
    ) -> &Self;
    fn set_texture_sampler(
        &self,
        entity: ObjectID,
        slot: &str,
        sampler: SamplerDesc,
    ) -> &Self;
}

impl InterfaceMaterialTexture for EnginShell {
    fn set_texture(
        &self,
        entity: ObjectID,
        slot: &str,
        image: Option<ImageAssetKey>,
    ) -> &Self {
        let commands = self.world().get_resource_mut::<SingleTextureCommands>().unwrap();
        commands.0.push(ECommand::TexturePath(entity, UniformPropertyName::from(slot), image));
        
        self
    }

    fn set_texture_sampler(
        &self,
        entity: ObjectID,
        slot: &str,
        sampler: SamplerDesc,
    ) -> &Self {
        let commands = self.world().get_resource_mut::<SingleTextureCommands>().unwrap();
        commands.0.push(ECommand::Sampler(entity, UniformPropertyName::from(slot), sampler));

        self
    }
}

pub struct PluginTextureSlot;
impl pi_engine_shell::plugin::Plugin for PluginTextureSlot {
    fn init(
        &mut self,
        engine: &mut pi_engine_shell::engine_shell::EnginShell,
        stages: &mut pi_engine_shell::run_stage::RunStage,
    ) -> Result<(), pi_engine_shell::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        SysTextureCommand::setup(world, stages.query_stage::<SysTextureCommand>(ERunStageChap::Command));

        if world.get_resource::<Share<AssetMgr<TextureRes>>>().is_none() {
            world.insert_resource(
                AssetMgr::<TextureRes>::new(GarbageEmpty(), false, 60 * 1024 * 1024, 60 * 1000)
            );
        }
        world.insert_resource(SingleTextureCommands::default());
        world.insert_resource(ImageAwait::<TextureSlot01>::default());
        world.insert_resource(ImageAwait::<TextureSlot02>::default());
        world.insert_resource(ImageAwait::<TextureSlot03>::default());
        world.insert_resource(ImageAwait::<TextureSlot04>::default());

        SysTextureSlot01Load::setup(world, stages.query_stage::<SysImageLoad>(ERunStageChap::Command));
        SysTextureSlot02Load::setup(world, stages.query_stage::<SysImageLoad>(ERunStageChap::Command));
        SysTextureSlot03Load::setup(world, stages.query_stage::<SysImageLoad>(ERunStageChap::Command));
        SysTextureSlot04Load::setup(world, stages.query_stage::<SysImageLoad>(ERunStageChap::Command));

        // SysTextureResReady1::setup(world, stages.query_stage::<SysTextureReady>(ERunStageChap::Command));
        // SysTextureResReady2::setup(world, stages.query_stage::<SysTextureReady>(ERunStageChap::Command));

        Ok(())
    }
}