
use std::mem::size_of;

use pi_assets::{mgr::AssetMgr, asset::GarbageEmpty, homogeneous::HomogeneousMgr};
use pi_render::{rhi::{asset::{RenderRes, TextureRes}, buffer::Buffer, texture::TextureView, bind_group::BindGroup, pipeline::RenderPipeline, shader::Shader}, components::view::target_alloc::UnuseTexture};
use pi_share::Share;

use crate::plugin::Plugin;

pub mod local_load;
pub mod sync_load;
pub mod image_texture_load;

pub struct PluginGeometryBufferAsstes;
impl Plugin for PluginGeometryBufferAsstes {
    fn init(
        &mut self,
        engine: &mut crate::engine_shell::EnginShell,
        _: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        if engine.world().get_resource::<Share<AssetMgr<RenderRes<render_data_container::VertexBuffer>>>>().is_none() {
            let asset = AssetMgr::<RenderRes<render_data_container::VertexBuffer>>::new(GarbageEmpty(), false, 20 * 1024 * 1024, 3 * 60 * 1000);
            engine.world_mut().insert_resource(asset);
        }
        
        Ok(())
    }
}

pub struct PluginTextureViewAsstes;
impl Plugin for PluginTextureViewAsstes {
    fn init(
        &mut self,
        engine: &mut crate::engine_shell::EnginShell,
        _: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        if engine.world().get_resource::<Share<AssetMgr<RenderRes<TextureView>>>>().is_none() {
            engine.world_mut().insert_resource(
                AssetMgr::<RenderRes<TextureView>>::new(GarbageEmpty(), false, 60 * 1024 * 1024, 3 * 60 * 1000)
            );
        }
        
        Ok(())
    }
}

pub struct PluginBingGroupAssets;
impl Plugin for PluginBingGroupAssets {
    fn init(
        &mut self,
        engine: &mut crate::engine_shell::EnginShell,
        _: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        if engine.world().get_resource::<Share<AssetMgr<RenderRes<BindGroup>>>>().is_none() {
            engine.world_mut().insert_resource(
                AssetMgr::<RenderRes<BindGroup>>::new(GarbageEmpty(), false, 5 * 1024, 3 * 60 * 1000)
            );
        }
        
        Ok(())
    }
}

pub struct PluginRenderPipelineAssets;
impl Plugin for PluginRenderPipelineAssets {
    fn init(
        &mut self,
        engine: &mut crate::engine_shell::EnginShell,
        _: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        if engine.world().get_resource::<Share<AssetMgr<RenderRes<RenderPipeline>>>>().is_none() {
            engine.world_mut().insert_resource(
                AssetMgr::<RenderRes<RenderPipeline>>::new(GarbageEmpty(), false, 5 * 1024, 60 * 1000)
            );
        }
        
        Ok(())
    }
}

pub struct PluginUnuseTextureAssets;
impl Plugin for PluginUnuseTextureAssets {
    fn init(
        &mut self,
        engine: &mut crate::engine_shell::EnginShell,
        _: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        if engine.world().get_resource::<Share<HomogeneousMgr<RenderRes<UnuseTexture>>>>().is_none() {
            engine.world_mut().insert_resource(
                HomogeneousMgr::<RenderRes<UnuseTexture>>::new(pi_assets::homogeneous::GarbageEmpty(), 10 * size_of::<UnuseTexture>(), size_of::<UnuseTexture>(), 60 * 1000)
            );
        }
        
        Ok(())
    }
}

pub struct PluginShaderAssets;
impl Plugin for PluginShaderAssets {
    fn init(
        &mut self,
        engine: &mut crate::engine_shell::EnginShell,
        _: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        if engine.world().get_resource::<Share<AssetMgr<RenderRes<Shader>>>>().is_none() {
            engine.world_mut().insert_resource(
                AssetMgr::<RenderRes<Shader>>::new(GarbageEmpty(), false, 10 * 1024, 60 * 1000)
            );
        }
        
        Ok(())
    }
}