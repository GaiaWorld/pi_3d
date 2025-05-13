#![feature(box_into_inner)]

use base::DemoScene;
use pi_scene_shell::prelude::*;
use pi_scene_context::prelude::*;
use pi_mesh_builder::cube::*;

#[path = "../base.rs"]
mod base;
#[path = "../copy.rs"]
mod copy;

pub type ActionListTestData = ActionList<(ObjectID, f32, f32, f32)>;

// pub struct SysTest;
// impl TSystemStageInfo for SysTest {}
// #[setup]
// impl SysTest {
//     #[system]
    pub fn sys(
        mut list: ResMut<ActionListTestData>,
        mut transform_commands: ResMut<ActionListTransformNodeLocal>,
    ) {
        list.exchange_empty().drain(..).for_each(|mut item| {
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
            transform_commands.push(OpsTransformNodeLocal::ops(item.0, ETransformSRT::Euler(x, y, z)));

            list.push(item);
        });
    }
// }

// #[derive(Debug)]
pub struct PluginTest;
impl Plugin for PluginTest {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActionListTestData::default());
    }
}

fn setup(
    mut commands: Commands,
    mut actions: pi_3d::ActionSets,
    mut animegroupres: ResourceAnimationGroup,
    mut fps: ResMut<SingleFrameTimeCommand>,
    defaultmat: Res<SingleIDBaseDefaultMaterial>,
    mut assets: (ResMut<CustomRenderTargets>, Res<PiRenderDevice>, Res<ShareAssetMgr<SamplerRes>>, Res<PiSafeAtlasAllocator>,),
    mut testdata: ResMut<ActionListTestData>,
    demooption: Res<base::DemoOption>,
) {
    let (demopass, scene, camera01, copyrenderer, copyrendercamera) = if let (Some(demo), Some(copyrenderer), Some(copyrendercamera)) = (&demooption.demo, &demooption.copyrenderer, &demooption.copyrendercamera) {
        (demo, demo.scene, demo.camera, *copyrenderer, *copyrendercamera)
    } else { return; };

    fps.frame_ms = 200;

    let tes_size = 4;

    actions.camera.param.push(OpsCameraModify::ops( camera01, ECameraModify::OrthSize( tes_size as f32 )));

    let vertices = CubeBuilder::attrs_meta();
    let indices = CubeBuilder::indices_meta();
    let state = MeshInstanceState::default();
    let source = base::DemoScene::mesh(&mut commands, scene, scene, &mut actions,  vertices, indices, state);

    actions.mesh.value_state.push(OpsAbstructMeshValueStateModify::ops(source, EMeshValueStateModify::IndiceRange(Some((3, 12)))));
    // actions.mesh.vertexrange.push(OpsMeshRenderVertexRange::ops(cube, Some(0), Some(12)));
    actions.mesh.render_state.push(OpsRenderState::primitive_state(source, DemoScene::PASS_OPAQUE, EPrimitiveState::CCullMode(CullMode::Off)));

    actions.material.usemat.push(OpsMaterialUse::ops(source, defaultmat.0, DemoScene::PASS_OPAQUE));

    testdata.push((source, 0., 0., 0.));

}

pub fn main() {
    let (mut app, window, event_loop) = base::test_plugins();

    app.insert_resource(crate::base::DemoOption {
        orthographic_camera: true,
        camera_position: (0., 0., -40.),
        ..Default::default()
    });
    app.add_startup_system(Update, base::setup_demoinit);

    app.add_plugins(PluginTest);
    app.add_systems(StageD3, pi_3d::sys_info_node);
    
        #[cfg(feature = "use_bevy")]
    app.add_systems(Startup, setup.after(base::setup_default_mat));
    #[cfg(not(feature = "use_bevy"))]
    app.add_startup_system(Update, setup.after(base::setup_default_mat));
    
    
    // app.run()
    crate::base::run_loop(app, window, event_loop)

    // let mut shell = App::new(
    //     RenderOptions {
    //         backends: wgpu::Backends::VULKAN,
    //         power_preference: wgpu::PowerPreference::HighPerformance,
    //         ..Default::default()
    //     }
    // );
    // shell.add_plugins(PluginTest);
    // shell.ready();
    // shell.setup(&PluginTest::setup);
    // shell.run();
}