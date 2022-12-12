use pi_3d::PluginBundleDefault;
use pi_engine_shell::engine_shell::{EnginShell, AppShell};
use pi_render::rhi::options::RenderOptions;
use pi_scene_context::{plugin::Plugin, object::ObjectID, transforms::{command::{SingleTransformNodeCommandList, TransformNodeCommand}, interface::InterfaceTransformNode}, scene::{interface::InterfaceScene}, cameras::interface::InterfaceCamera, meshes::{cube::InterfaceCube, quad::{InterfaceQuad, PluginQuadBuilder}}, main_camera_render::interface::InterfaceMainCamera, layer_mask::{interface::InterfaceLayerMask, LayerMask}};
use pi_ecs::prelude::{ResMut, Setup};
use pi_ecs_macros::setup;
use pi_scene_math::Vector3;
use procedural_texture::{InterfaceTestPerlinNoise, cloud::interface::{InterfaceCloudMaterial, PluginCloudMaterial}, PluginTestPerlinNoise};


pub struct PluginTest;
impl Plugin for PluginTest {
    fn init(
        &mut self,
        engine: &mut pi_scene_context::engine::Engine,
        stages: &mut pi_scene_context::run_stage::RunStage,
    ) -> Result<(), pi_scene_context::plugin::ErrorPlugin> {
        PluginTestPerlinNoise.init(engine, stages);
        PluginCloudMaterial.init(engine, stages);
        
        PluginQuadBuilder.init(engine, stages);


        Ok(())
    }
}

impl PluginTest {
    fn setup(
        engine: &EnginShell
    ) {
        // Test Code
        let scene01 = engine.create_scene();
        let camera01 = engine.create_free_camera(scene01);
        let node01 = engine.create_transform_node(scene01);
        // engine.set_parent(camera01, scene01, Some(node01));
        engine.active_camera(camera01, true);
        engine.transform_position(camera01, Vector3::new(0., 0., -5.));
        engine.free_camera_orth_size(camera01, 1 as f32);

        let sky_box = engine.new_quad(scene01);
        engine.use_cloud_material(sky_box);

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
    shell.add_plugin(PluginBundleDefault);
    shell.add_plugin(PluginTest);
    shell.ready();
    shell.setup(&PluginTest::setup);
    shell.run();
}