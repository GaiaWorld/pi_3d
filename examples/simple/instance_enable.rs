#![feature(box_into_inner)]

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
pub struct ListTestData(Vec<Entity>, Option<Entity>, WyRng, usize);

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
        let len = testdata.0.len();
        if testdata.3 >= len {
            testdata.3 = 0;
        }
        let mut idx = 0;
        testdata.0.iter().for_each(|entity| {
            idx += 1;
            let mut enable = false;
            if idx == testdata.3 {
                enable = true;
            }
            actions.transform.enable.push(OpsNodeEnable::ops(entity.clone(), enable));
        });

        testdata.3 += 1;
    }
// }

// #[derive(Debug)]
pub struct PluginTest;
impl Plugin for PluginTest {
    fn build(&self, app: &mut App) {
        app.insert_resource(ListTestData(vec![], None, pi_wy_rng::WyRng::default(), 0));
        app.configure_set(Update, StageTest::Cmd.before(StageScene::Create));
        app.add_system(Update, sys.in_set(StageTest::Cmd));
    }
}

fn setup(
    mut commands: Commands,
    mut actions: pi_3d::ActionSets,
    mut animegroupres: ResourceAnimationGroup,
    mut fps: ResMut<SingleFrameTimeCommand>,
    defaultmat: Res<SingleIDBaseDefaultMaterial>,
    mut testdata: ResMut<ListTestData>,
    mut assets: (ResMut<CustomRenderTargets>, Res<PiRenderDevice>, Res<ShareAssetMgr<SamplerRes>>, Res<PiSafeAtlasAllocator>,),
) {
    let tes_size = 6;
    fps.frame_ms = 16;

    let demopass = base::DemoScene::new(&mut commands, &mut actions, &mut animegroupres, 
        &mut assets.0, &assets.1, &assets.2, &assets.3,
        tes_size as f32, 1., (0., 10., -40.), true
    );
    let (scene, camera01, id_renderer) = (demopass.scene, demopass.camera, demopass.transparent_renderer);

    let (copyrenderer, copyrendercamera) = copy::PluginImageCopy::toscreen(&mut commands, &mut actions, scene, demopass.transparent_renderer,demopass.transparent_target);
    actions.renderer.connect.push(OpsRendererConnect::ops(demopass.transparent_renderer, copyrenderer, false));

    actions.camera.target.push(OpsCameraTarget::ops(camera01, 0., -1., 4.));

    // actions.mesh.render_alignment.push(OpsMeshRenderAlignment::ops(source, ERenderAlignment::StretchedBillboard));

    let source = commands.spawn_empty().id(); actions.transform.tree.push(OpsTransformNodeParent::ops(source, scene));
    actions.mesh.create.push(OpsMeshCreation::ops(scene, source, MeshInstanceState { instance_matrix: true, use_single_instancebuffer: true ,..Default::default() }));

    let id_geo = commands.spawn_empty().id();
    let attrs = CubeBuilder::attrs_meta();
    actions.geometry.create.push(OpsGeomeryCreate::ops(source, id_geo, attrs, Some(CubeBuilder::indices_meta())));
    
    let idmat = commands.spawn_empty().id();
    actions.material.create.push(OpsMaterialCreate::ops(idmat, DefaultShader::KEY));
    // let idmat = defaultmat.0;
    actions.material.usemat.push(OpsMaterialUse::ops(source, idmat, DemoScene::PASS_OPAQUE));


    for i in 0..tes_size {
        for j in 0..tes_size {
            for _k in 0..1 {
                let instance = commands.spawn_empty().id(); actions.instance.create.push(OpsInstanceMeshCreation::ops(source, instance));
                let node = commands.spawn_empty().id(); actions.transform.create.push(OpsTransformNode::ops(scene, node));
                actions.transform.localsrt.push(OpsTransformNodeLocal::ops(node, ETransformSRT::Translation(i as f32 * 2. - (tes_size) as f32, 0., j as f32 * 2. - (tes_size) as f32)));
                actions.transform.localsrt.push(OpsTransformNodeLocal::ops(node, ETransformSRT::Scaling(0.2, 0.2, 0.2)));

                actions.transform.tree.push(OpsTransformNodeParent::ops(instance, node));

                testdata.0.push(node);
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
    let mut app = base::test_plugins();

    app.add_plugins(PluginTest);
    app.add_system(Update, pi_3d::sys_info_node);
    app.add_system(Update, pi_3d::sys_info_resource);
    app.add_system(Update, pi_3d::sys_info_draw);

    app.add_system(Startup, setup.after(base::setup_default_mat));
    
    
    // app.run()
    loop { app.update(); }

}