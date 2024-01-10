
use pi_3d::PluginBundleDefault;
use pi_atom::Atom;
use pi_scene_shell::{engine_shell::{EnginShell, AppShell}, run_stage::{TSystemStageInfo, ERunStageChap}, frame_time::InterfaceFrameTime};
use pi_mesh_builder::cube::{InterfaceCube, PluginCubeBuilder};
use pi_render::rhi::options::RenderOptions;
use pi_scene_context::{
    plugin::Plugin,
    object::ObjectID,
    transforms::{command::{SingleTransformNodeModifyCommandList, ETransformNodeModifyCommand}, interface::InterfaceTransformNode},
    scene::interface::InterfaceScene,
    cameras::interface::InterfaceCamera,
    layer_mask::{interface::InterfaceLayerMask, LayerMask}, renderers::graphic::RendererGraphicDesc, pass::{PassTagOrders, EPassTag}, materials::interface::InterfaceMaterial
};
use pi_ecs::prelude::{ResMut, Setup};
use pi_ecs_macros::setup;
use pi_scene_math::Vector3;
use skybox::{InterfaceSkybox, PluginSkybox};

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
            log::debug!("=========== item.1: {}, item.2: {}, item.3: {}", item.1, item.2, item.3);
            let x = item.1 % 4000.0 / 4000.0 * 3.1415926 * 2.;
            let y = item.2 % 4000.0 / 4000.0 * 3.1415926 * 2.;
            let z = item.3 % 4000.0 / 4000.0 * 3.1415926 * 2.;
            log::debug!("=========== x: {}, y: {}, z: {}", x, y, z);
            // transform_commands.list.push(TransformNodeCommand::ModifyPosition(item.0, Vector3::new(x.cos() * 20., 0., 5.)));
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
        let mut world = engine.world_mut().clone();

        SysTest::setup(&mut world, stages.query_stage::<SysTest>(ERunStageChap::Command));

        let testdata = SingleTestData::default();
        world.insert_resource(testdata);

        Ok(())
    }
}

impl PluginTest {
    fn setup(
        engine: &EnginShell
    ) {

        engine.frame_time(10);

        // Test Code
        let scene01 = engine.create_scene();
        let camera01 = engine.create_free_camera(scene01);
        let node01 = engine.create_transform_node(scene01);
        // engine.set_parent(camera01, scene01, Some(node01));
        engine.layer_mask(camera01, LayerMask::default());
        engine.active_camera(camera01, true);
        engine.transform_position(camera01, Vector3::new(0., 0., -5.));
        engine.camera_renderer(camera01, RendererGraphicDesc { pre: Some(Atom::from("Clear")), curr: Atom::from("MainCameraSky"), next: None, passorders: PassTagOrders::new(vec![EPassTag::Sky]) });

        // let mat = engine.create_default_material(EPassTag::Opaque);
    
        let sky_box = engine.new_skybox(scene01);
        // let sky_box = engine.new_cube(scene01);
        // engine.use_material(sky_box, mat);
        engine.transform_position(sky_box, Vector3::new(0., 0., 0.));
        engine.layer_mask(sky_box, LayerMask::default());

        let mut testdata = engine.world().get_resource_mut::<SingleTestData>().unwrap();
        testdata.transforms.push((sky_box, 0., 0., 0.));
        // testdata.transforms.push((camera01, 0., 0., 0.));
    }
}

pub fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let mut shell = AppShell::new(
        RenderOptions {
            backends: wgpu::Backends::VULKAN,
            power_preference: wgpu::PowerPreference::HighPerformance,
            ..Default::default()
        }
    );
    shell.add_plugins(PluginBundleDefault);
    shell.add_plugins(PluginCubeBuilder);
    shell.add_plugins(PluginSkybox);
    shell.add_plugins(PluginTest);
    shell.ready();
    shell.setup(&PluginTest::setup);
    shell.run();
}