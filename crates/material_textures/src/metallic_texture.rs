use std::mem::replace;

use derive_deref::{Deref, DerefMut};
use pi_assets::asset::Handle;
use pi_ecs::{prelude::{ResMut, Query, Setup, Res}, query::Write};
use pi_ecs_macros::setup;
use pi_engine_shell::{object::{ObjectID, GameObject}, engine_shell, plugin::Plugin, assets::image_texture_load::CalcImageLoad};
use pi_render::rhi::{asset::TextureRes, texture::Sampler, device::RenderDevice};
use pi_scene_context::{shaders::{FragmentUniformBindTexture, FragmentUniformBindTextureSampler}};
use render_resource::{ImageAssetKey, sampler::{SamplerDesc, SamplerPool}};

#[derive(Debug, Deref, DerefMut, Clone, Default, Hash)]
pub struct MetallicTextureKey(pub ImageAssetKey);

#[derive(Deref, DerefMut)]
pub struct MetallicTextureRes(pub Handle<TextureRes>);
impl From<Handle<TextureRes>> for MetallicTextureRes {
    fn from(h: Handle<TextureRes>) -> Self { Self(h) }
}
impl FragmentUniformBindTexture for MetallicTextureRes {
    const TEXTURE_BIND: u8 = 0;

    const TEXTURE_SAMPLER_TYPE: wgpu::TextureSampleType = wgpu::TextureSampleType::Float { filterable: true };

    const DIM: wgpu::TextureViewDimension = wgpu::TextureViewDimension::D2;

    const MULTI: bool = false;
}

pub struct MetallicTextureSampler(pub Sampler);
impl FragmentUniformBindTextureSampler for MetallicTextureSampler {
    const SAMPLER_BIND: u8 = 1;
    const SAMPLER_TYPE: wgpu::SamplerBindingType = wgpu::SamplerBindingType::Filtering;
}

pub type MetallicTextureLoad = CalcImageLoad<MetallicTextureKey, MetallicTextureRes>;

#[derive(Debug)]
enum ECommand {
    Texture(ObjectID, Option<ImageAssetKey>),
    Sampler(ObjectID, SamplerDesc),
}

#[derive(Debug, Default)]
struct SingleCommands {
    pub list: Vec<ECommand>,
}
#[setup]
impl SingleCommands {
    #[system]
    pub fn sys(
        mut cmds: ResMut<SingleCommands>,
        mut materials: Query<GameObject, Write<MetallicTextureKey>>,
        mut samplers: Query<GameObject, Write<MetallicTextureSampler>>,
        mut samplerpool: ResMut<SamplerPool>,
        device: Res<RenderDevice>,
    ) {
        let mut list = replace(&mut cmds.list, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd
            {
                ECommand::Texture(entity, imagekey) => {
                    if let Some(mut material) = materials.get_mut(entity) {
                        match imagekey {
                            Some(imagekey) => {
                                material.write(MetallicTextureKey(imagekey));
                            },
                            None => {
                                material.remove();
                            },
                        }
                    }
                },
                ECommand::Sampler(entity, samplerdesc) => {
                    if let Some(mut sampler) = samplers.get_mut(entity) {
                        let key = SamplerPool::cacl_key(&samplerdesc);
                        samplerpool.create(&samplerdesc, &device);
                        sampler.write(MetallicTextureSampler(samplerpool.get(key).unwrap()));
                        println!("MetallicTextureSampler Write");
                    }
                },
            }
        });
    }   
}

pub trait InterfaceMetallicTexture {
    fn set_metallic_texture(
        &self,
        material: ObjectID,
        url: Option<ImageAssetKey>,
    ) -> &Self;
    fn set_metallic_texture_sampler(
        &self,
        material: ObjectID,
        sampler: SamplerDesc,
    ) -> &Self;
}

impl InterfaceMetallicTexture for engine_shell::EnginShell {
    fn set_metallic_texture(
        &self,
        material: ObjectID,
        url: Option<ImageAssetKey>,
    ) -> &Self {
        let world = self.world();

        world.get_resource_mut::<SingleCommands>().unwrap().list.push(ECommand::Texture(material, url));
        world.get_resource_mut::<SingleCommands>().unwrap().list.push(ECommand::Sampler(material, SamplerDesc::default()));
        
        self
    }
    fn set_metallic_texture_sampler(
        &self,
        material: ObjectID,
        sampler: SamplerDesc,
    ) -> &Self {
        let world = self.world();

        world.get_resource_mut::<SingleCommands>().unwrap().list.push(ECommand::Sampler(material, sampler));
        
        self
    }
}

pub struct PluginMetallicTexture;
impl Plugin for PluginMetallicTexture {
    fn init(
        &mut self,
        engine: &mut engine_shell::EnginShell,
        stages: &mut pi_engine_shell::run_stage::RunStage,
    ) -> Result<(), pi_engine_shell::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        world.insert_resource(SingleCommands::default());

        MetallicTextureLoad::setup(world, stages.command_stage());
        SingleCommands::setup(world, stages.command_stage());
        
        Ok(())
    }
}