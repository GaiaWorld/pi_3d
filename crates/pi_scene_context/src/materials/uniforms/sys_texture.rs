use std::{marker::PhantomData, mem::replace};

use pi_assets::{mgr::AssetMgr, asset::GarbageEmpty};
use pi_ecs::{prelude::{Query, Res, ResMut, Setup}, query::{Write, Changed, Or}, entity};
use pi_ecs_macros::setup;
use pi_engine_shell::{object::{GameObject, ObjectID}, assets::image_texture_load::{CalcImageLoad, ImageAwait}, engine_shell::EnginShell};
use pi_render::rhi::{device::RenderDevice, asset::TextureRes};
use pi_share::Share;
use render_resource::{sampler::{SamplerPool, SamplerDesc}, ImageAssetKey};
use render_shader::unifrom_code::{UniformPropertyName, MaterialTextureBindDesc};

use crate::materials::{bind_group::{RenderBindGroupKey, RenderBindGroupPool}, material_meta::AssetResMaterailMeta};

use super::{texture_uniform::{MaterialTextureBindGroupID, TForTextureBindGroup}, texture::{ValueTextureKey, TextureSlot1, TextureResSlot1, TextureResSlot3, TextureResSlot4, TextureResSlot2, TextureSlot2, TextureSlot3, TextureSlot4, UniformSampler, SamplerSlot1, UniformTexture, SamplerSlot2, SamplerSlot3, SamplerSlot4, ETextureSlot}};

#[derive(Debug)]
enum ECommand {
    TexturePath(ObjectID, UniformPropertyName, Option<ImageAssetKey>),
    Sampler(ObjectID, UniformPropertyName, SamplerDesc),
}

#[derive(Debug, Default)]
struct SingleTextureCommands(pub Vec<ECommand>);

struct SysTextureChange;
#[setup]
impl SysTextureChange {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingleTextureCommands>,
        mut items: Query<
            GameObject,
            (
                &AssetResMaterailMeta
                , Write<TextureSlot1>, Write<SamplerSlot1>
                , Write<TextureSlot2>, Write<SamplerSlot2>
                , Write<TextureSlot3>, Write<SamplerSlot3>
                , Write<TextureSlot4>, Write<SamplerSlot4>
                , Write<MaterialTextureBindGroupID>
            )
        >,
        device: Res<RenderDevice>,
        mut samplerpool: ResMut<SamplerPool>,
    ) {
        let mut list = replace(&mut cmds.0, vec![]);
        list.drain(..).for_each(|cmd| {
            match cmd {
                ECommand::TexturePath(entity, slotname, path) => {
                    if let Some(mut item) = items.get_mut(entity) {
                        if let Ok(index) = item.0.query_tex_slot(&slotname) {
                            if index == 0 {
                                if let Some(path) = path { item.1.write(TextureSlot1::new(path)); } else { item.1.remove(); };
                            }
                            else if index == 1 {
                                if let Some(path) = path { item.3.write(TextureSlot2::new(path)); } else { item.3.remove(); };
                            }
                            else if index == 2 {
                                if let Some(path) = path { item.5.write(TextureSlot3::new(path)); } else { item.5.remove(); };
                            }
                            else if index == 3 {
                                if let Some(path) = path { item.7.write(TextureSlot4::new(path)); } else { item.7.remove(); };
                            }
                        }
                    }
                },
                ECommand::Sampler(entity, slotname, sampler) => {
                    if let Some(mut item) = items.get_mut(entity) {
                        if let Ok(index) = item.0.query_tex_slot(&slotname) {
                            if index == 0 {
                                item.2.write(SamplerSlot1::new(&sampler, &device, &mut samplerpool));
                            }
                            else if index == 1 {
                                item.4.write(SamplerSlot2::new(&sampler, &device, &mut samplerpool));
                            }
                            else if index == 2 {
                                item.6.write(SamplerSlot3::new(&sampler, &device, &mut samplerpool));
                            }
                            else if index == 3 {
                                item.8.write(SamplerSlot4::new(&sampler, &device, &mut samplerpool));
                            }
                        }
                    }
                },
            }
        });
    }
}

struct SysTextureResReady1;
#[setup]
impl SysTextureResReady1 {
    #[system]
    pub fn sys(
        items: Query<
            GameObject,
            (
                &MaterialTextureBindGroupID, &AssetResMaterailMeta,
                &TextureResSlot1, &SamplerSlot1
            ),
            Or<(
                Changed<TextureResSlot1>, Changed<SamplerSlot1>
            )>
        >,
        mut bindgrouppool: ResMut<RenderBindGroupPool>,
        device: Res<RenderDevice>,
    ) {
        items.iter().for_each(|(bindgroup, binddesc, tex1, sampler1)| {
            println!("SysTextureResReady1 >");
            if let Some(binddesc) = &binddesc.textures {
                if binddesc.list.len() == 1 {
                    println!("SysTextureResReady1 >>");
                    if let Some(group) = bindgrouppool.get_mut(&bindgroup.0) {
                        println!("SysTextureResReady1 >>>");
                        binddesc.bind_group(&device, group, &[tex1.texture()], &[sampler1.sampler()]);
                    }
                }
            }
        });
    }
}

struct SysTextureResReady2;
#[setup]
impl SysTextureResReady2 {
    #[system]
    pub fn sys(
        items: Query<
            GameObject,
            (
                &MaterialTextureBindGroupID, &MaterialTextureBindDesc
                , &TextureResSlot1, &SamplerSlot1
                , &TextureResSlot2, &SamplerSlot2
            ),
            Or<(
                Changed<TextureResSlot1>, Changed<SamplerSlot1>
                , Changed<TextureResSlot2>, Changed<SamplerSlot2>
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

pub type SysTextureSlot1Load = CalcImageLoad<TextureSlot1, TextureResSlot1>;
pub type SysTextureSlot2Load = CalcImageLoad<TextureSlot2, TextureResSlot2>;
pub type SysTextureSlot3Load = CalcImageLoad<TextureSlot3, TextureResSlot3>;
pub type SysTextureSlot4Load = CalcImageLoad<TextureSlot4, TextureResSlot4>;

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

        SysTextureChange::setup(world, stages.command_stage());

        if world.get_resource::<Share<AssetMgr<TextureRes>>>().is_none() {
            world.insert_resource(
                AssetMgr::<TextureRes>::new(GarbageEmpty(), false, 60 * 1024 * 1024, 60 * 1000)
            );
        }
        world.insert_resource(SingleTextureCommands::default());
        world.insert_resource(ImageAwait::<TextureSlot1>::default());
        world.insert_resource(ImageAwait::<TextureSlot2>::default());
        world.insert_resource(ImageAwait::<TextureSlot3>::default());
        world.insert_resource(ImageAwait::<TextureSlot4>::default());

        SysTextureSlot1Load::setup(world, stages.uniform_update());
        SysTextureSlot2Load::setup(world, stages.uniform_update());
        SysTextureSlot3Load::setup(world, stages.uniform_update());
        SysTextureSlot4Load::setup(world, stages.uniform_update());

        SysTextureResReady1::setup(world, stages.between_uniform_update_and_filter_culling());
        SysTextureResReady2::setup(world, stages.between_uniform_update_and_filter_culling());

        Ok(())
    }
}