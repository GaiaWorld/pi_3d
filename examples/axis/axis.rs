

use pi_3d::PluginBundleDefault;
use pi_atom::Atom;
use pi_ecs::prelude::{ResMut, Setup};
use pi_ecs_macros::setup;
use pi_scene_shell::{
    engine_shell::{AppShell, EnginShell},
    frame_time::InterfaceFrameTime,
    object::ObjectID,
    plugin::Plugin, run_stage::{TSystemStageInfo, ERunStageChap},
};
use pi_render::rhi::options::RenderOptions;
use pi_scene_context::{
    cameras::interface::InterfaceCamera,
    layer_mask::{interface::InterfaceLayerMask, LayerMask},
    materials::interface::InterfaceMaterial,
    scene::interface::InterfaceScene,
    transforms::{interface::InterfaceTransformNode, command::{SingleTransformNodeModifyCommandList, ETransformNodeModifyCommand}}, renderers::graphic::RendererGraphicDesc, pass::{EPassTag, PassTagOrders},
};
use pi_scene_math::*;
use axis::{axis::{InterfaceAxis, PluginAxisBuilder}, interface::InterfaceAxisMaterial, PluginAxis};

#[derive(Default)]
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
            // // log::debug!("=========== item.1: {}, item.2: {}, item.3: {}", item.1, item.2, item.3);
            let x = item.1 % 4000.0 / 4000.0 * 3.1415926 * 2.;
            let y = item.2 % 4000.0 / 4000.0 * 3.1415926 * 2.;
            let z = item.3 % 4000.0 / 4000.0 * 3.1415926 * 2.;
            // log::debug!("=========== x: {}, y: {}, z: {}", x, y, z);
            // transform_commands.list.push(TransformNodeCommand::ModifyPosition(item.0, Vector3::new(x.cos() * 20., 0., 5.)));
            // transform_commands.list.push(TransformNodeCommand::ModifyRotation(item.0, Vector3::new(2.7394686, 2.7394686, 2.7394686)));
            transform_commands.list.push(ETransformNodeModifyCommand::ModifyRotation(item.0, Vector3::new(x, y, z)));
        });
    }
}

pub struct PluginTest;
impl Plugin for PluginTest {
    fn init(
        &mut self,
        engine: &mut pi_scene_context::engine::Engine,
        stages: &mut pi_scene_context::run_stage::RunStage,
    ) -> Result<(), pi_scene_context::plugin::ErrorPlugin> {
        // PluginQuadBuilder.init(engine, stages);
        let mut world = engine.world_mut().clone();
        
        PluginAxisBuilder.init(engine, stages);

        SysTest::setup(&mut world, stages.query_stage::<SysTest>(ERunStageChap::Command));
        let testdata = SingleTestData::default();
        world.insert_resource(testdata);

        Ok(())
    }
}

impl PluginTest {
    fn setup(engine: &EnginShell) {
        engine.frame_time(16);
        // Test Code
        let scene01 = engine.create_scene();
        let camera01 = engine.create_free_camera(scene01);
        let node01 = engine.create_transform_node(scene01);
        // engine.set_parent(camera01, scene01, Some(node01));
        engine.active_camera(camera01, true);
        engine.transform_position(camera01, Vector3::new(0., 0., -5.));
        engine.free_camera_orth_size(camera01, 1 as f32);
        engine.camera_renderer(camera01, RendererGraphicDesc { pre: Some(Atom::from("Clear")), curr: Atom::from("MainCamera"), next: None, passorders: PassTagOrders::new(vec![EPassTag::Opaque]) });

        let axis_box = engine.new_axis(scene01);
        let material = engine.create_axis_material();
        engine.use_material(axis_box, material);

        engine.layer_mask(camera01, LayerMask::default());
        engine.layer_mask(axis_box, LayerMask::default());

        let mut testdata = engine.world().get_resource_mut::<SingleTestData>().unwrap();
        testdata.transforms.push((axis_box, 0., 0., 0.));
    }
}

pub fn main() {
    let mut shell = AppShell::new(RenderOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        ..Default::default()
    });
    shell.add_plugins(PluginBundleDefault);
    shell.add_plugins(PluginAxis);
    shell.add_plugins(PluginTest);
    shell.ready();
    shell.setup(&PluginTest::setup);
    shell.run();
}
