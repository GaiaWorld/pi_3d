#![feature(box_into_inner)]

use base::DemoScene;
use pi_bevy_ecs_extend::{prelude::Layer, system_param::layer_dirty::ComponentEvent};
use pi_engine_shell::prelude::*;
use pi_gltf2_load::{TypeAnimeContexts, TypeAnimeAssetMgrs};
use pi_node_materials::NodeMaterialBlocks;
use pi_scene_context::prelude::*;
use pi_mesh_builder::cube::*;

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
    mut scenecmds: ActionSetScene,
    mut cameracmds: ActionSetCamera,
    mut transformcmds: ActionSetTransform,
    mut meshcmds: ActionSetMesh,
    mut instancemeshcmds: ActionSetInstanceMesh,
    mut geometrycmd: ActionSetGeometry,
    mut matcmds: ActionSetMaterial,
    mut animegroupcmd: ActionSetAnimationGroup,
    mut fps: ResMut<SingleFrameTimeCommand>,
    mut final_render: ResMut<WindowRenderer>,
    nodematblocks: Res<NodeMaterialBlocks>,
    defaultmat: Res<SingleIDBaseDefaultMaterial>,
    mut renderercmds: ActionSetRenderer,
    anime_assets: TypeAnimeAssetMgrs,
    
    mut testdata: ResMut<ActionListTestData>,
) {
    fps.frame_ms = 200;

    let tes_size = 4;
    let (scene, camera01) = DemoScene::new(&mut commands, &mut scenecmds, &mut cameracmds, &mut transformcmds, &mut animegroupcmd, &mut final_render, &mut renderercmds, tes_size as f32, 0.7, (0., 0., -10.), true);

    let cube = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(cube, scene));
    meshcmds.create.push(OpsMeshCreation::ops(scene, cube, String::from("TestCube")));
    meshcmds.indexrange.push(OpsMeshRenderIndiceRange::ops(cube, Some(3), Some(12)));
    meshcmds.cullmode.push(OpsCullMode::ops(cube, CullMode::Off));
    
    let id_geo = commands.spawn_empty().id();
    geometrycmd.create.push(OpsGeomeryCreate::ops(cube, id_geo, CubeBuilder::attrs_meta(), Some(CubeBuilder::indices_meta())));

    matcmds.usemat.push(OpsMaterialUse::ops(cube, defaultmat.0));

    testdata.push((cube, 0., 0., 0.));

}

#[path = "../base.rs"]
mod base;
pub fn main() {
    let mut app = base::test_plugins();
    
    app.add_plugin(PluginTest);
    app.add_system(pi_3d::sys_info_node);
    
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