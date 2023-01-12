#![feature(box_into_inner)]

use material_textures::{PluginMaterialTextures, main_texture::{PluginMainTexture, InterfaceMainTexture}};
use pi_3d::PluginBundleDefault;
use pi_engine_shell::{engine_shell::AppShell, frame_time::InterfaceFrameTime, assets::local_load::PluginLocalLoad};
use pi_mesh_builder::quad::{PluginQuadBuilder, InterfaceQuad};
use pi_render::rhi::options::RenderOptions;
use pi_scene_context::{plugin::Plugin,
    transforms::{interface::InterfaceTransformNode},
    scene::{interface::InterfaceScene},
    cameras::interface::InterfaceCamera,
    main_camera_render::interface::InterfaceMainCamera,
    layer_mask::{interface::InterfaceLayerMask, LayerMask},
    materials::{material::{InterfaceMaterial}, uniforms::sys_texture::InterfaceMaterialTexture}
};
use pi_scene_math::Vector3;
use render_resource::sampler::SamplerDesc;
use unlit_material::{interface::InterfaceUnlitMaterial, PluginUnlitMaterial};


#[derive(Debug)]
pub struct PluginTest;
impl Plugin for PluginTest {
    fn init(
        &mut self,
        engine: &mut pi_scene_context::engine::Engine,
        stages: &mut pi_scene_context::run_stage::RunStage,
    ) -> Result<(), pi_scene_context::plugin::ErrorPlugin> {
        PluginLocalLoad.init(engine, stages);
        PluginBundleDefault.init(engine, stages);
        PluginQuadBuilder.init(engine, stages);
        PluginUnlitMaterial.init(engine, stages);

        Ok(())
    }
}

impl PluginTest {
    pub fn setup(
        engine: &pi_engine_shell::engine_shell::EnginShell,
    ) {

        engine.frame_time(2);

        // Test Code
        let scene01 = engine.create_scene();
        let camera01 = engine.create_free_camera(scene01);
        engine.active_camera(camera01, true);
        engine.layer_mask(camera01, LayerMask::default());
        engine.transform_position(camera01, Vector3::new(0., 0., -10.));
        engine.free_camera_orth_size(camera01, 1 as f32);

        let unlitmaterial = engine.create_unlit_material();
		engine.set_texture_sampler(unlitmaterial, "_MainTex", SamplerDesc::default());
        engine.emissive_texture(unlitmaterial, render_resource::ImageAssetKey::from("E:/Rust/PI/pi_3d/assets/images/top.jpg"));

        
        let quad = engine.new_quad(scene01);
        engine.use_material(quad, unlitmaterial);
        engine.layer_mask(quad, LayerMask::default());
    }
}

pub fn main() {
    let mut shell = AppShell::new(
        RenderOptions {
            backends: wgpu::Backends::VULKAN,
            power_preference: wgpu::PowerPreference::HighPerformance,
            ..Default::default()
        }
    );
    shell.add_plugin(PluginTest);
    shell.ready();
    shell.setup(&PluginTest::setup);
    shell.run();
}