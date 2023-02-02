#![feature(box_into_inner)]


use default_render::interface::InterfaceDefaultMaterial;
use pi_3d::PluginBundleDefault;
use pi_engine_shell::{engine_shell::AppShell, frame_time::InterfaceFrameTime, run_stage::{TSystemStageInfo, ERunStageChap}};
use pi_render::rhi::options::RenderOptions;
use pi_scene_context::{plugin::Plugin, object::ObjectID,
    transforms::{command::{SingleTransformNodeModifyCommandList, ETransformNodeModifyCommand}, interface::InterfaceTransformNode},
    scene::{interface::InterfaceScene},
    cameras::interface::InterfaceCamera,
    main_camera_render::interface::InterfaceMainCamera,
    layer_mask::{interface::InterfaceLayerMask, LayerMask}
};
use pi_ecs::prelude::{ResMut, Setup};
use pi_ecs_macros::setup;
use pi_scene_math::Vector3;
use pi_mesh_builder::{ball::{InterfaceBall, PluginBallBuilder}};

#[derive(Debug, Default)]
pub struct SingleTestData {
    pub transforms: Vec<(ObjectID, f32, f32, f32)>,
}

pub struct SysTest;
impl TSystemStageInfo for SysTest {}
#[setup]
impl SysTest {
    #[system]
    pub fn sys(
        mut list: ResMut<SingleTestData>,
        mut transform_commands: ResMut<SingleTransformNodeModifyCommandList>,
    ) {
        list.transforms.iter_mut().for_each(|mut item| {
            item.1 = item.1 + 16.0;
            item.2 = item.2 + 16.0;
            item.3 = item.3 + 16.0;
            let x0 = item.1 % 4000.0 / 4000.0;
            let x = x0 * 3.1415926 * 2.;
            let y0 = item.2 % 4000.0 / 4000.0;
            let y = y0 * 3.1415926 * 2.;
            let z0 = item.3 % 4000.0 / 4000.0;
            let z = z0 * 3.1415926 * 2.;
            // transform_commands.list.push(TransformNodeCommand::ModifyPosition(item.0, Vector3::new(x.cos() * 3., 0., 0.)));
            // transform_commands.list.push(TransformNodeCommand::ModifyScaling(item.0, Vector3::new(x.cos() + 0.5, x.sin() + 0.5, x + 0.5)));
            transform_commands.list.push(ETransformNodeModifyCommand::ModifyRotation(item.0, Vector3::new(x, y, z)));
        });
    }
}

#[derive(Debug)]
pub struct PluginTest;
impl Plugin for PluginTest {
    fn init(
        &mut self,
        engine: &mut pi_scene_context::engine::Engine,
        stages: &mut pi_scene_context::run_stage::RunStage,
    ) -> Result<(), pi_scene_context::plugin::ErrorPlugin> {

        PluginBundleDefault.init(engine, stages);
        PluginBallBuilder.init(engine, stages);

        let world = engine.world_mut();

        SysTest::setup(world, stages.query_stage::<SysTest>(ERunStageChap::Command));

        let testdata = SingleTestData::default();
        world.insert_resource(testdata);

        Ok(())
    }
}

impl PluginTest {
    pub fn setup(
        engine: &pi_engine_shell::engine_shell::EnginShell,
    ) {

        let tes_size = 10;
        let testdata = engine.world().get_resource_mut::<SingleTestData>().unwrap();

        engine.frame_time(1);

        // Test Code
        let scene01 = engine.create_scene();
        let camera01 = engine.create_free_camera(scene01);
        let node01 = engine.create_transform_node(scene01);
        engine.active_camera(camera01, true);
        engine.layer_mask(camera01, LayerMask::default());
        engine.transform_position(camera01, Vector3::new(0., 0., -10.));
        engine.free_camera_orth_size(camera01, tes_size as f32);

        for i in 0..tes_size {
            for j in 0..tes_size {
                for k in 0..1 {
                    let ball = engine.new_ball(scene01, 36, 18);
                    engine.use_default_material(ball);
                    engine.transform_position(ball, Vector3::new(i as f32 * 2. - (tes_size) as f32, j as f32 * 2. - (tes_size) as f32, k as f32 * 2. - (tes_size) as f32));
                    // engine.transform_rotation_euler(cube, Vector3::new(i as f32 * 0.2, j as f32 * 0.2, k as f32 * 0.2));
                    engine.layer_mask(ball, LayerMask::default());
                    testdata.transforms.push((ball, i as f32 * 100., j as f32 * 100., k as f32 * 100.));
                }
            }
        }
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