#![feature(box_into_inner)]

use base::DemoScene;
use pi_node_materials::prelude::*;
use pi_scene_shell::{prelude::*, frame_time::SingleFrameTimeCommand};
use pi_scene_context::{prelude::*, scene::StageScene};
use pi_mesh_builder::cube::*;
use pi_wy_rng::WyRng;
use rand::Rng;
use pi_winit::event::{Event, WindowEvent};
use pi_world::editor::EntityEditor;
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
        mut editor: EntityEditor,
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
            let cube: Entity = editor.alloc_entity();
            actions.mesh.create.push(OpsMeshCreation::ops(scene, cube, MeshInstanceState::default()));
            actions.transform.tree.push(OpsTransformNodeParent::ops(cube, scene));
            actions.transform.localsrt.push(OpsTransformNodeLocal::ops(cube, ETransformSRT::Translation(random.gen_range(-5.0f32..5.0f32) as f32 * 0.5, random.gen_range(-5.0f32..5.0f32) * 0.5, random.gen_range(-5.0f32..5.0f32) * 0.5)));

            let id_geo = editor.alloc_entity();
            let attrs = CubeBuilder::attrs_meta();
            // attrs.push(VertexBufferDesc::instance_world_matrix());
            actions.geometry.create.push(OpsGeomeryCreate::ops(cube, id_geo, attrs, Some(CubeBuilder::indices_meta())));
    
            let idmat = editor.alloc_entity();
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
        app.world.insert_single_res(ListTestData(vec![], None, pi_wy_rng::WyRng::default()));
        app.configure_set(Update, StageTest::Cmd.before(StageScene::Create));
        app.add_system(Update, sys.in_set(StageTest::Cmd));
    }
}

fn setup(
    mut editor: EntityEditor,
    mut actions: pi_3d::ActionSets,
    mut animegroupres: ResourceAnimationGroup,
    mut fps: ResMut<SingleFrameTimeCommand>,
    defaultmat: Res<SingleIDBaseDefaultMaterial>,
    mut testdata: ResMut<ListTestData>,
    mut assets: (ResMut<CustomRenderTargets>, Res<PiRenderDevice>, Res<ShareAssetMgr<SamplerRes>>, Res<PiSafeAtlasAllocator>,),
) {
    let tes_size = 6;
    fps.frame_ms = 16;

    let demopass = base::DemoScene::new(&mut editor, &mut actions, &mut animegroupres, 
        &mut assets.0, &assets.1, &assets.2, &assets.3,
        tes_size as f32, 1., (0., 10., -40.), true
    );
    let (scene, camera01, id_renderer) = (demopass.scene, demopass.camera, demopass.transparent_renderer);

    let (copyrenderer, copyrendercamera) = copy::PluginImageCopy::toscreen(&mut editor, &mut actions, scene, demopass.transparent_renderer,demopass.transparent_target);
    actions.renderer.connect.push(OpsRendererConnect::ops(demopass.transparent_renderer, copyrenderer, false));

    actions.camera.target.push(OpsCameraTarget::ops(camera01, 0., -1., 4.));

    // actions.mesh.render_alignment.push(OpsMeshRenderAlignment::ops(source, ERenderAlignment::StretchedBillboard));
    


    for i in 0..tes_size {
        for j in 0..tes_size {
            for _k in 0..1 {
                let source = editor.alloc_entity(); actions.transform.tree.push(OpsTransformNodeParent::ops(source, scene));
                actions.mesh.create.push(OpsMeshCreation::ops(scene, source, MeshInstanceState { instance_matrix: true, ..Default::default() }));

                let id_geo = editor.alloc_entity();
                let attrs = CubeBuilder::attrs_meta();
                actions.geometry.create.push(OpsGeomeryCreate::ops(source, id_geo, attrs, Some(CubeBuilder::indices_meta())));
                
                let idmat = editor.alloc_entity();
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
    let  (mut app, window, event_loop) = base::test_plugins();
    
    app.add_plugins(PluginTest);
    app.add_system(Update, pi_3d::sys_info_node);
    app.add_system(Update, pi_3d::sys_info_resource);
    app.add_system(Update, pi_3d::sys_info_draw);
    
    
    app.add_startup_system(Update, setup.after(base::setup_default_mat));
    
    
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    control_flow.set_exit();
                }
                
                _ => (),
            },
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(_window_id) => {
                app.run();
            }
            
            _ => (),
        }
    });

}