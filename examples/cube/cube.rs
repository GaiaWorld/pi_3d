#![feature(box_into_inner)]


use default_render::{interface::*, SingleIDBaseDefaultMaterial, PluginDefaultMaterial};
use pi_3d::PluginBundleDefault;
use pi_atom::Atom;
use pi_bevy_ecs_extend::{prelude::Layer, system_param::layer_dirty::ComponentEvent};
use pi_bevy_render_plugin::PiRenderPlugin;
use pi_engine_shell::{prelude::*, frame_time::{SingleFrameTimeCommand, PluginFrameTime}};
use pi_render::rhi::options::RenderOptions;
use pi_scene_context::prelude::*;
use pi_scene_math::Vector3;
use pi_mesh_builder::{cube::*, ball::PluginBallBuilder, quad::PluginQuadBuilder};
use unlit_material::PluginUnlitMaterial;

pub trait AddEvent {
	// 添加事件， 该实现每帧清理一次
	fn add_frame_event<T: Event>(&mut self) -> &mut Self;
}

impl AddEvent for App {
	fn add_frame_event<T: Event>(&mut self) -> &mut Self {
		if !self.world.contains_resource::<Events<T>>() {
			self.init_resource::<Events<T>>()
				.add_system(Events::<T>::update_system);
		}
		self
	}
}

pub type ActionListTestData = ActionList<(ObjectID, f32, f32, f32)>;

// pub struct SysTest;
// impl TSystemStageInfo for SysTest {}
// #[setup]
// impl SysTest {
//     #[system]
    pub fn sys(
        mut list: ResMut<ActionListTestData>,
        mut transform_commands: ResMut<ActionListTransformNodeLocalEuler>,
    ) {
        list.drain().drain(..).for_each(|mut item| {
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
            transform_commands.push(OpsTransformNodeLocalEuler::ops(item.0, x, y, z));

            list.push(item);
        });
    }
// }

// #[derive(Debug)]
pub struct PluginTest;
impl Plugin for PluginTest {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActionListTestData::default());
        app.add_frame_event::<ComponentEvent<Changed<Layer>>>();
    }
//     fn init(
//         &mut self,
//         engine: &mut pi_scene_context::engine::Engine,
//         stages: &mut pi_scene_context::run_stage::RunStage,
//     ) -> Result<(), pi_scene_context::plugin::ErrorPlugin> {
//         PluginLocalLoad.init(engine, stages);
//         PluginBundleDefault.init(engine, stages);
//         PluginUnlitMaterial.init(engine, stages);

//         PluginCubeBuilder.init(engine, stages);

//         let world = engine.world_mut();

//         SysTest::setup(world, stages.query_stage::<SysTest>(ERunStageChap::Command));

//         let testdata = SingleTestData::default();
//         world.insert_resource(testdata);

//         Ok(())
//     }
}

// impl PluginTest {
//     fn setup(
//         engine: &pi_engine_shell::engine_shell::EnginShell,
//     ) {
//         let testdata = engine.world().get_resource_mut::<SingleTestData>().unwrap();

//         engine.frame_time(2000);

//         // Test Code
//         let scene01 = engine.create_scene();
//         let camera01 = engine.create_free_camera(scene01);
//         engine.active_camera(camera01, true);
//         engine.layer_mask(camera01, LayerMask::default());
//         engine.transform_position(camera01, Vector3::new(0., 0., -10.));
//         engine.free_camera_orth_size(camera01, 4 as f32);
//         engine.camera_renderer(camera01, RendererGraphicDesc { pre: Some(Atom::from("Clear")), curr: Atom::from("MainCamera"), next: None, passorders: PassTagOrders::new(vec![EPassTag::Opaque, EPassTag::Water, EPassTag::Sky, EPassTag::Transparent]) });


//         // let matid = engine.create_default_material();
//         // engine.emissive_intensity(entity, intensity);

//         let cube = engine.new_cube(scene01);
//         let mat = engine.create_default_material(EPassTag::Opaque);
//         engine.use_material(cube, mat);
//         engine.layer_mask(cube, LayerMask::default());
//         testdata.transforms.push((cube, 0., 0., 0.));
//     }
// }

fn setup(
    mut commands: Commands,
    mut testdata: ResMut<ActionListTestData>,
    mut scenecmds: ResMut<ActionListSceneCreate>,
    mut treecmds: ResMut<ActionListTransformNodeParent>,
    mut cameracmds: ActionSetCamera,
    mut localpositioncmds: ResMut<ActionListTransformNodeLocalPosition>,
    mut meshcreate: ResMut<ActionListMeshCreate>,
    mut geometrycreate: ResMut<ActionListGeometryCreate>,
    mut matuse: ResMut<ActionListMaterialUse>,
    mut fps: ResMut<SingleFrameTimeCommand>,
    mut final_render: ResMut<WindowRenderer>,
    defaultmat: Res<SingleIDBaseDefaultMaterial>,
    mut renderercmds: ActionSetRenderer,
) {
    fps.frame_ms = 200;

    final_render.cleardepth = 0.0;

    let scene = commands.spawn_empty().id();
    scenecmds.push(OpsSceneCreation::ops(scene, ScenePassRenderCfg::default()));

    let camera01 = commands.spawn_empty().id(); treecmds.push(OpsTransformNodeParent::ops(camera01, scene));
    cameracmds.create.push(OpsCameraCreation::ops(scene, camera01, String::from("TestCamera"), true));
    cameracmds.active.push(OpsCameraActive::ops(camera01, true));
    cameracmds.size.push(OpsCameraOrthSize::ops(camera01, 4.));
    localpositioncmds.push(OpsTransformNodeLocalPosition::ops(camera01, 0., 0., -10.));

    let desc = RendererGraphicDesc {
        pre: Some(final_render.clear_entity),
        curr: String::from("TestCamera"),
        next: Some(final_render.render_entity),
        passorders: PassTagOrders::new(vec![EPassTag::Opaque, EPassTag::Water, EPassTag::Sky, EPassTag::Transparent])
    };
    let id_renderer = commands.spawn_empty().id(); renderercmds.create.push(OpsRendererCreate::ops(id_renderer, desc.curr.clone()));
    renderercmds.connect.push(OpsRendererConnect::ops(final_render.clear_entity, id_renderer));
    renderercmds.connect.push(OpsRendererConnect::ops(id_renderer, final_render.render_entity));
    cameracmds.render.push(OpsCameraRendererInit::ops(camera01, id_renderer, desc.curr, desc.passorders, ColorFormat::Rgba8Unorm, DepthStencilFormat::None));

    let cube = commands.spawn_empty().id(); treecmds.push(OpsTransformNodeParent::ops(cube, scene));
    meshcreate.push(OpsMeshCreation::ops(scene, cube, String::from("TestCube")));
    
    let id_geo = commands.spawn_empty().id();
    geometrycreate.push(OpsGeomeryCreate::ops(cube, id_geo, CubeBuilder::attrs_meta(), Some(CubeBuilder::indices_meta())));

    matuse.push(OpsMaterialUse::ops(cube, defaultmat.0));

    testdata.push((cube, 0., 0., 0.));

}


pub fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let mut app = App::default();

	let mut window_plugin = WindowPlugin::default();
    if let Some(primary_window) = &mut window_plugin.primary_window {
        primary_window.resolution.set_physical_resolution(800, 600);
    }

    app.add_plugin(InputPlugin::default());
    app.add_plugin(window_plugin);
    app.add_plugin(AccessibilityPlugin);
    app.add_plugin(bevy::winit::WinitPlugin::default());
    // .add_plugin(WorldInspectorPlugin::new())
    app.add_plugin(PiRenderPlugin::default());
    app.add_plugin(PluginTest);
    app.add_plugin(PluginFrameTime);
    app.add_plugin(PluginWindowRender);
    app.add_plugins(PluginBundleDefault);
    app.add_plugin(PluginCubeBuilder);
    app.add_plugin(PluginQuadBuilder);
    app.add_plugin(PluginStateToFile);
    app.add_plugin(pi_3d::PluginSceneTimeFromPluginFrame);

    app.world.get_resource_mut::<WindowRenderer>().unwrap().active = true;
    
    app.add_startup_system(setup);
    // bevy_mod_debugdump::print_main_schedule(&mut app);
    
    app.run()

    // let mut shell = App::new(
    //     RenderOptions {
    //         backends: wgpu::Backends::VULKAN,
    //         power_preference: wgpu::PowerPreference::HighPerformance,
    //         ..Default::default()
    //     }
    // );
    // shell.add_plugin(PluginTest);
    // shell.ready();
    // shell.setup(&PluginTest::setup);
    // shell.run();
}