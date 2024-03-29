use pi_3d::PluginBundleDefault;
use pi_scene_shell::{engine_shell::{EnginShell, AppShell}, frame_time::InterfaceFrameTime};
use pi_render::rhi::options::RenderOptions;
use pi_scene_context::{
    plugin::Plugin,
    transforms::interface::InterfaceTransformNode,
    scene::interface::InterfaceScene,
    cameras::interface::InterfaceCamera,
    layer_mask::{interface::InterfaceLayerMask, LayerMask},
    materials::interface::InterfaceMaterial
};

use pi_scene_math::Vector3;
use procedural_texture::cloud::{PluginCloudMaterial, interface::InterfaceCloudMaterial};
use pi_mesh_builder::{cube::InterfaceCube, quad::{InterfaceQuad, PluginQuadBuilder}};


pub struct PluginTest;
impl Plugin for PluginTest {
    fn init(
        &mut self,
        engine: &mut pi_scene_context::engine::Engine,
        stages: &mut pi_scene_context::run_stage::RunStage,
    ) -> Result<(), pi_scene_context::plugin::ErrorPlugin> {
        PluginQuadBuilder.init(engine, stages);


        Ok(())
    }
}

impl PluginTest {
    fn setup(
        engine: &EnginShell
    ) {
        engine.frame_time(2000);
        // Test Code
        let scene01 = engine.create_scene();
        let camera01 = engine.create_free_camera(scene01);
        let node01 = engine.create_transform_node(scene01);
        // engine.set_parent(camera01, scene01, Some(node01));
        engine.active_camera(camera01, true);
        engine.transform_position(camera01, Vector3::new(0., 0., -5.));
        engine.free_camera_orth_size(camera01, 1 as f32);

        let sky_box = engine.new_quad(scene01);
        let material = engine.create_cloud_material();
        engine.use_material(sky_box, material);

        engine.layer_mask(camera01, LayerMask::default());
        engine.layer_mask(sky_box, LayerMask::default());

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
    shell.add_plugins(PluginBundleDefault);
    shell.add_plugins(PluginQuadBuilder);
    shell.add_plugins(PluginCloudMaterial);
    shell.add_plugins(PluginTest);
    shell.ready();
    shell.setup(&PluginTest::setup);
    shell.run();
}