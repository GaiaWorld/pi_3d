#![feature(box_into_inner)]

use std::sync::Arc;

use base::DemoScene;
use pi_node_materials::prelude::*;
use pi_scene_shell::{prelude::*, frame_time::SingleFrameTimeCommand};
use pi_scene_context::{prelude::*, scene::StageScene};
use pi_mesh_builder::cube::*;
use pi_wy_rng::WyRng;
use rand::Rng;

#[path = "../base.rs"]
mod base;
#[path = "../copy.rs"]
mod copy;

#[derive(Resource)]
pub struct ListTestData(Vec<(Entity, Entity)>, Option<Entity>, WyRng);

// pub struct SysTest;
// impl TSystemStageInfo for SysTest {}
// #[setup]
// impl SysTest {
//     #[system]
    pub fn sys(
        mut commands: Commands,
        mut testdata: ResMut<ListTestData>,
        mut actions: pi_3d::ActionSets,
        defaultmat: Res<SingleIDBaseDefaultMaterial>,
    ) {
        if let Some((entity, idmat)) = testdata.0.pop() {
            actions.obj_dispose.push(OpsDisposeReady::ops(entity));
            actions.obj_dispose.push(OpsDisposeReady::ops(idmat));
        }
        
        // if testdata.0.len() % 2 != 0 {
        //     if let Some(entity) = testdata.0.pop() {
        //         disposereadylist.push(OpsDisposeReady::ops(entity));
        //         // actions.transform.enable.push(OpsNodeEnable::ops(entity, false));
        //     }
        //     return;
        // }

        if let Some(scene) = testdata.1.clone() {
            let random = &mut testdata.2;
            // log::warn!("Random: {:?}", random.gen_range(-5.0f32..5.0f32));
            let cube: Entity = commands.spawn_empty_id();
            actions.mesh.create.push(OpsMeshCreation::ops(scene, cube, MeshInstanceState::default()));
            actions.transform.tree.push(OpsTransformNodeParent::ops(cube, scene));
            actions.transform.localsrt.push(OpsTransformNodeLocal::ops(cube, ETransformSRT::Translation(random.gen_range(-5.0f32..5.0f32) as f32 * 0.5, random.gen_range(-5.0f32..5.0f32) * 0.5, random.gen_range(-5.0f32..5.0f32) * 0.5)));

            let id_geo = commands.spawn_empty_id();
            let attrs = CubeBuilder::attrs_meta();
            // attrs.push(VertexBufferDesc::instance_world_matrix());
            actions.geometry.create.push(OpsGeomeryCreate::ops(cube, id_geo, attrs, Some(CubeBuilder::indices_meta())));
    
            let idmat = commands.spawn_empty_id();
            actions.material.create.push(OpsMaterialCreate::ops(idmat, DefaultShader::KEY));
            // let idmat = defaultmat.0;
            actions.material.usemat.push(OpsMaterialUse::ops(cube, idmat, DemoScene::PASS_OPAQUE));

            testdata.0.insert(0, (cube, idmat));
        }
    }
// }

// #[derive(Debug)]
pub struct PluginTest;
impl Plugin for PluginTest {
    fn build(&self, app: &mut App) {
        app.insert_resource(ListTestData(vec![], None, pi_wy_rng::WyRng::default()));
        // app.configure_set(Update, StageTest::Cmd.before(StageScene::Create));
        // app.add_systems(Update, sys.in_set(StageTest::Cmd));
        
        app.insert_resource(SimpleList::default());
        app.add_startup_system(Update, simple_setup);
        app.add_systems(Update, sys_simple);
    }
}

#[derive(Resource, Default)]
pub struct SimpleList(Vec<(Entity, Number, Number, Number)>);

fn simple_setup(
    mut commands: Commands,
    mut list: ResMut<SimpleList>,
) {
    let count = 10;
    for i in 0..count {
        for j in 0..count {
            for k in 0..count {
                let entity = commands.spawn((LocalMatrix::default())).id();
                list.0.push((entity, i as Number, j as Number, k as Number));
            }
        }
    }
}
fn sys_simple(
    mut items: Query<&mut LocalMatrix>,
    mut list: ResMut<SimpleList>,
) {
    list.0.iter_mut().for_each(|(entity, x, y, z)| {
        if let Ok(mut matrix) = items.get_mut(*entity) {
            let mut temp = Matrix::identity();
            temp.append_scaling_mut(*x + *y + *z);
            matrix.0 = temp;
            *x += 0.01;
            *y += 0.01;
            *z += 0.01;
        }
    });
    log::warn!("SimpleList {:?}", list.0.capacity());
}

fn setup(
    mut commands: Commands,
    mut actions: pi_3d::ActionSets,
    mut animegroupres: ResourceAnimationGroup,
    mut fps: ResMut<SingleFrameTimeCommand>,
    defaultmat: Res<SingleIDBaseDefaultMaterial>,
    mut testdata: ResMut<ListTestData>,
    mut assets: (ResMut<CustomRenderTargets>, Res<PiRenderDevice>, Res<ShareAssetMgr<SamplerRes>>, Res<PiSafeAtlasAllocator>,),
    demooption: Res<base::DemoOption>,
) {
    let (demopass, scene, camera01, copyrenderer, copyrendercamera) = if let (Some(demo), Some(copyrenderer), Some(copyrendercamera)) = (&demooption.demo, &demooption.copyrenderer, &demooption.copyrendercamera) {
        (demo, demo.scene, demo.camera, *copyrenderer, *copyrendercamera)
    } else { return; };

    let tes_size = 6;
    fps.frame_ms = 16;

    actions.camera.target.push(OpsCameraTarget::ops(camera01, 0., -1., 4.));

    // actions.mesh.render_alignment.push(OpsMeshRenderAlignment::ops(source, ERenderAlignment::StretchedBillboard));
    


    for i in 0..tes_size {
        for j in 0..tes_size {
            for _k in 0..1 {
                let source = commands.spawn_empty_id(); actions.transform.tree.push(OpsTransformNodeParent::ops(source, scene));
                actions.mesh.create.push(OpsMeshCreation::ops(scene, source, MeshInstanceState { instance_matrix: true, ..Default::default() }));

                let id_geo = commands.spawn_empty_id();
                let attrs = CubeBuilder::attrs_meta();
                actions.geometry.create.push(OpsGeomeryCreate::ops(source, id_geo, attrs, Some(CubeBuilder::indices_meta())));
                
                let idmat = commands.spawn_empty_id();
                actions.material.create.push(OpsMaterialCreate::ops(idmat, DefaultShader::KEY));
                // let idmat = defaultmat.0;
                actions.material.usemat.push(OpsMaterialUse::ops(source, idmat, DemoScene::PASS_OPAQUE));

                actions.transform.localsrt.push(OpsTransformNodeLocal::ops(source, ETransformSRT::Translation(i as f32 * 2. - (tes_size) as f32, 0., j as f32 * 2. - (tes_size) as f32)));
                actions.transform.localsrt.push(OpsTransformNodeLocal::ops(source, ETransformSRT::Scaling(0.2, 0.2, 0.2)));

                testdata.0.push((source, idmat));
            }
        }
    }

    testdata.1 = Some(scene);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet, PartialOrd, Ord)]
pub enum StageTest {
    Cmd
}

pub fn main() {
    let (mut app, window, event_loop) = base::test_plugins();
    
    app.insert_resource(crate::base::DemoOption {
        orthographic_camera: false,
        camera_size: 10.,
        camera_fov: 0.7,
        camera_position: (0., 10., -40.),
        ..Default::default()
    });
    app.add_startup_system(Update, base::setup_demoinit);

    // env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

    // let mut app = App::new();

    // let width = 800;
    // let height = 600;

    // let mut opt = PiRenderOptions::default();
    // // opt.backends = wgpu::Backends::VULKAN;
    // app.insert_resource(opt);

	// let (window, event_loop) = {
	// 	use pi_winit::platform::windows::EventLoopBuilderExtWindows;
	// 	let event_loop = pi_winit::event_loop::EventLoopBuilder::new().with_any_thread(true).build();
	// 	let window = pi_winit::window::Window::new(&event_loop).unwrap();
	// 	(Arc::new(window), event_loop)
	// };
    // app.insert_resource(AssetMgrConfigs::default());
    
    app.add_plugins(PluginTest);
    // app.add_systems(Update, pi_3d::sys_info_node);
    // app.add_systems(Update, pi_3d::sys_info_resource);
    // app.add_systems(Update, pi_3d::sys_info_draw);
    
    
    //     #[cfg(feature = "use_bevy")]
    // app.add_systems(Startup, setup.after(base::setup_default_mat));
    // #[cfg(not(feature = "use_bevy"))]
    // app.add_startup_system(Update, setup.after(base::setup_default_mat));
    
    
    // app.run()
    crate::base::run_loop(app, window, event_loop)

}