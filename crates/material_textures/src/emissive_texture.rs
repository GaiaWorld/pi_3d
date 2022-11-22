use std::mem::replace;

use derive_deref::{Deref, DerefMut};
use pi_assets::asset::Handle;
use pi_ecs::{prelude::{ResMut, Query, Setup}, query::Write};
use pi_ecs_macros::setup;
use pi_engine_shell::{object::{ObjectID, GameObject}, engine_shell, plugin::Plugin};
use pi_render::rhi::{asset::TextureRes, texture::Sampler};
use pi_scene_context::{shaders::{FragmentUniformBindTexture, FragmentUniformBindTextureSampler}};
use render_resource::ImageAssetKey;

use crate::image_texture_load::CalcImageLoad;

#[derive(Debug, Deref, DerefMut, Clone, Default, Hash)]
pub struct EmissiveTextureKey(pub ImageAssetKey);

#[derive(Deref, DerefMut)]
pub struct EmissiveTextureRes(pub Handle<TextureRes>);
impl From<Handle<TextureRes>> for EmissiveTextureRes {
    fn from(h: Handle<TextureRes>) -> Self { Self(h) }
}
impl FragmentUniformBindTexture for EmissiveTextureRes {
    const TEXTURE_BIND: u8 = 0;

    const TEXTURE_SAMPLER_TYPE: wgpu::TextureSampleType = wgpu::TextureSampleType::Uint;

    const DIM: wgpu::TextureViewDimension = wgpu::TextureViewDimension::D2;

    const MULTI: bool = false;
}

pub struct EmissiveTextureSampler(pub Sampler);
impl FragmentUniformBindTextureSampler for EmissiveTextureSampler {
    const SAMPLER_BIND: u8 = 1;

    const SAMPLER_TYPE: wgpu::SamplerBindingType = wgpu::SamplerBindingType::Filtering;

}

pub type EmissiveTextureLoad = CalcImageLoad<EmissiveTextureKey, EmissiveTextureRes>;

#[derive(Debug, Default)]
struct SingleCommands {
    pub list: Vec<(ObjectID, Option<ImageAssetKey>)>,
}
#[setup]
impl SingleCommands {
    #[system]
    pub fn sys(
        mut cmds: ResMut<SingleCommands>,
        mut materials: Query<GameObject, Write<EmissiveTextureKey>>,
    ) {
        let mut list = replace(&mut cmds.list, vec![]);

        list.drain(..).for_each(|(entity, imagekey)| {
            if let Some(mut material) = materials.get_mut(entity) {
                match imagekey {
                    Some(imagekey) => {
                        material.write(EmissiveTextureKey(imagekey));
                    },
                    None => {
                        material.remove();
                    },
                }
            }
        });
    }   
}

pub trait InterfaceEmissiveTexture {
    fn set_main_texture(
        &self,
        material: ObjectID,
        url: Option<ImageAssetKey>,
    ) -> &Self;
}

impl InterfaceEmissiveTexture for engine_shell::EnginShell {
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

pub struct PluginEmissiveTexture;
impl Plugin for PluginEmissiveTexture {
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