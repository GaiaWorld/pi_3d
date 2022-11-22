use std::mem::replace;

use derive_deref::{Deref, DerefMut};
use pi_assets::asset::Handle;
use pi_ecs::{prelude::{ResMut, Query, Setup}, query::Write};
use pi_ecs_macros::setup;
use pi_engine_shell::{object::{ObjectID, GameObject}, image_texture_load::CalcImageLoad, engine_shell, plugin::Plugin};
use pi_render::rhi::{asset::TextureRes, texture::Sampler};
use pi_scene_context::{texture::{texture_sampler::TextureSamplerDesc}, shaders::{FragmentUniformBindTexture, FragmentUniformBindTextureSampler}};
use render_resource::ImageAssetKey;


#[derive(Debug, Deref, DerefMut, Clone, Default, Hash)]
pub struct AlbedoTextureKey(pub ImageAssetKey);

#[derive(Deref, DerefMut)]
pub struct AlbedoTextureRes(pub Handle<TextureRes>);
impl From<Handle<TextureRes>> for AlbedoTextureRes {
    fn from(h: Handle<TextureRes>) -> Self { Self(h) }
}
impl FragmentUniformBindTexture for AlbedoTextureRes {
    const TEXTURE_BIND: u8 = 0;

    const TEXTURE_SAMPLER_TYPE: wgpu::TextureSampleType = wgpu::TextureSampleType::Uint;

    const DIM: wgpu::TextureViewDimension = wgpu::TextureViewDimension::D2;

    const MULTI: bool = false;
}

pub struct AlbedoTextureSampler(pub Sampler);
impl FragmentUniformBindTextureSampler for AlbedoTextureSampler {
    const SAMPLER_BIND: u8 = 1;

    const SAMPLER_TYPE: wgpu::SamplerBindingType = wgpu::SamplerBindingType::Filtering;

}

pub type AlbedoTextureLoad = CalcImageLoad<AlbedoTextureKey, AlbedoTextureRes>;

#[derive(Debug, Default)]
struct SingleCommands {
    pub list: Vec<(ObjectID, Option<ImageAssetKey>)>,
}
#[setup]
impl SingleCommands {
    #[system]
    pub fn sys(
        mut cmds: ResMut<SingleCommands>,
        mut materials: Query<GameObject, Write<AlbedoTextureKey>>,
    ) {
        let mut list = replace(&mut cmds.list, vec![]);

        list.drain(..).for_each(|(entity, imagekey)| {
            if let Some(mut material) = materials.get_mut(entity) {
                match imagekey {
                    Some(imagekey) => {
                        material.write(AlbedoTextureKey(imagekey));
                    },
                    None => {
                        material.remove();
                    },
                }
            }
        });
    }   
}

pub trait InterfaceAlbedoTexture {
    fn set_main_texture(
        &self,
        material: ObjectID,
        url: Option<ImageAssetKey>,
    ) -> &Self;
}

impl InterfaceAlbedoTexture for engine_shell::EnginShell {
    fn set_main_texture(
        &self,
        material: ObjectID,
        url: Option<ImageAssetKey>,
    ) -> &Self {
        let world = self.world();

        world.get_resource_mut::<SingleCommands>().unwrap().list.push((material, url));
        
        self
    }
}

pub struct PluginAlbedoTexture;
impl Plugin for PluginAlbedoTexture {
    fn init(
        &mut self,
        engine: &mut engine_shell::EnginShell,
        stages: &mut pi_engine_shell::run_stage::RunStage,
    ) -> Result<(), pi_engine_shell::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        world.insert_resource(SingleCommands::default());

        SingleCommands::setup(world, stages.command_stage());
        
        Ok(())
    }
}