#![feature(box_into_inner)]

use std::{result, sync::Arc};

use base::DemoScene;
use crossbeam::queue::SegQueue;
use pi_node_materials::prelude::*;
use pi_scene_shell::{prelude::*, frame_time::SingleFrameTimeCommand};
use pi_scene_context::{geometry::instance::{instanced_buffer::CombineBuffer, types::ModelInstanceAttributes}, prelude::*, scene::StageScene};
use pi_mesh_builder::cube::*;
use pi_wy_rng::WyRng;
use rand::Rng;
use unlit_material::MainOpacityShader;

#[path = "../base.rs"]
mod base;
#[path = "../copy.rs"]
mod copy;


const TEST_SIZE: usize = 1000;

#[derive(Resource)]
pub struct ListTestData(SegQueue<(Entity, Entity, Vec<Entity>)>, Option<Entity>, WyRng, usize);

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
        scenes: Query<&SceneBoundingPool>,
        materials: Query<&MaterialRefs>,
        viewers: Query<(&ModelList, &ModelListAfterCulling)>,
        instancedatas: Query<&ModelInstanceAttributes>,
        instancesource: Query<&InstancedMeshTransparentSortCollection>,
        combinebuffer: Res<CombineBuffer>,
        meshes: Query<&InstanceSourceRefs>,
        tempvecs: Res<TmpCommonVec>,
    ) {
        let _ = actions.disposeref.drain();
        let mut instancedatalen = 0;
        instancedatas.iter().for_each(|item| {
            instancedatalen += item.bytes().len();
        });
        instancesource.iter().for_each(|item| {
            instancedatalen += item.data.len();
        });
        instancedatalen += combinebuffer.data.size();

        // return;
        let mut result = vec![instancedatalen];
        // scenes.iter().for_each(|item| {
        //     result.push(item.size() * 40);
        // });
        // materials.iter().for_each(|item| {
        //     result.push(item.capacity() * 8);
        // });
        meshes.iter().for_each(|item| {
            result.push(item.capacity() * 8);
        });
        // viewers.iter().for_each(|item| {
        //     result.push(item.0.0.capacity() * 8);
        //     result.push(item.1.0.capacity() * 8);
        // });
        // log::error!("{:?}", (result, testdata.0.len(), tempvecs.instancesort.capacity() * 64));
        if let Some(scene) = testdata.1.clone() {
            if let Some((source, idmat, mut instances)) = testdata.0.pop() {
                actions.obj_dispose.push(OpsDisposeReady::ops(source));
                actions.obj_dispose.push(OpsDisposeReady::ops(idmat));

                instances.drain(..).for_each(|item: Entity| {
                    actions.obj_dispose.push(OpsDisposeReady::ops(item));
                });

                let mut temp = vec![];
                let source = commands.spawn_empty_id();
                actions.transform.tree.push(OpsTransformNodeParent::ops(source, scene));
                actions.mesh.create.push(OpsMeshCreation::ops(scene, source, MeshInstanceState { instance_matrix: true, ..Default::default() }));
                let id_geo = commands.spawn_empty_id();
                let attrs = CubeBuilder::attrs_meta();
                actions.geometry.create.push(OpsGeomeryCreate::ops(source, id_geo, attrs, CubeBuilder::indices_meta()));
                let idmat = commands.spawn_empty_id();
                actions.material.usemat.push(OpsMaterialUse::ops(source, idmat, DemoScene::PASS_OPAQUE));
                actions.material.create.push(OpsMaterialCreate::ops_with_matarray(idmat, MainOpacityShader::KEY));
                actions.material.valb.push(OpsUniformValB::texture(idmat, UniformTextureWithSamplerParam {
                    slotname: Atom::from(BlockMainTexture::KEY_TEX),
                    sample: KeySampler::linear_repeat(),
                    url: EKeyTexture::from("assets/images/fractal.png"),
                    ..Default::default()
                }));
    
                for _ in 0..TEST_SIZE {
                    let random = &mut testdata.2;
                    let instance = commands.spawn_empty_id();
                    actions.instance.create.push(OpsInstanceMeshCreation::ops(source, instance));
                    actions.transform.tree.push(OpsTransformNodeParent::ops(instance, scene));
                    actions.transform.localsrt.push(OpsTransformNodeLocal::ops(instance, ETransformSRT::Translation(random.gen_range(-0.5f32..0.5f32) as f32 * (TEST_SIZE as f32), random.gen_range(-0.5f32..0.5f32) * (TEST_SIZE as f32), random.gen_range(0f32..0.5f32) * (TEST_SIZE as f32))));
                    actions.transform.localsrt.push(OpsTransformNodeLocal::ops(instance, ETransformSRT::Scaling(0.5, 0.5, 0.5)));
                    
                    // let instance = commands.spawn_empty_id();
                    // actions.transform.tree.push(OpsTransformNodeParent::ops(instance, scene));
                    // actions.transform.create.push(OpsTransformNode::ops(scene, instance));
                    // actions.transform.localsrt.push(OpsTransformNodeLocal::ops(instance, ETransformSRT::Translation(random.gen_range(-0.5f32..0.5f32) as f32 * (TEST_SIZE as f32), random.gen_range(-0.5f32..0.5f32) * (TEST_SIZE as f32), random.gen_range(0f32..0.5f32) * (TEST_SIZE as f32))));
                    // actions.transform.localsrt.push(OpsTransformNodeLocal::ops(instance, ETransformSRT::Scaling(0.5, 0.5, 0.5)));
                    // // actions.transform.tree.push(OpsTransformNodeParent::ops(instance, scene));
                    // // actions.mesh.create.push(OpsMeshCreation::ops(scene, instance, MeshInstanceState { instance_matrix: true, ..Default::default() }));
    
    
                    temp.push(instance);
                }
                testdata.0.push((source, idmat, temp));
            }
            
        }
    }
// }

// #[derive(Debug)]
pub struct PluginTest;
impl Plugin for PluginTest {
    fn build(&self, app: &mut App) {
        log::error!("Okkkk");
        app.insert_resource(ListTestData(SegQueue::default(), None, pi_wy_rng::WyRng::default(), 0));
        app.configure_set(Update, StageTest::Cmd.before(StageScene::SceneCreate));
        app.add_systems(Update, sys.in_set(StageTest::Cmd));
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
    demooption: Res<base::DemoOption>,
) {
    let (demopass, scene, camera01, copyrenderer, copyrendercamera) = if let (Some(demo), Some(copyrenderer), Some(copyrendercamera)) = (&demooption.demo, &demooption.copyrenderer, &demooption.copyrendercamera) {
        (demo, demo.scene, demo.camera, *copyrenderer, *copyrendercamera)
    } else { return; };

    let tes_size = TEST_SIZE;
    fps.frame_ms = 16;

    actions.camera.target.push(OpsCameraTarget::ops(camera01, 0., -1., 4.));

    for i in 0..2 {
        let source = commands.spawn_empty_id();
        actions.transform.tree.push(OpsTransformNodeParent::ops(source, scene));
        actions.mesh.create.push(OpsMeshCreation::ops(scene, source, MeshInstanceState { instance_matrix: true, ..Default::default() }));
        let id_geo = commands.spawn_empty_id();
        let attrs = CubeBuilder::attrs_meta();
        actions.geometry.create.push(OpsGeomeryCreate::ops(source, id_geo, attrs, CubeBuilder::indices_meta()));
        let idmat = commands.spawn_empty_id();
        actions.material.create.push(OpsMaterialCreate::ops_with_matarray(idmat, DefaultShader::KEY));
        actions.material.usemat.push(OpsMaterialUse::ops(source, idmat, DemoScene::PASS_OPAQUE));

        let mut tmp = vec![];
        for j in 0..tes_size {
            for _k in 0..1 {
                let instance = commands.spawn_empty_id();
                actions.instance.create.push(OpsInstanceMeshCreation::ops(source, instance));
                actions.transform.tree.push(OpsTransformNodeParent::ops(instance, scene));
                actions.transform.localsrt.push(OpsTransformNodeLocal::ops(instance, ETransformSRT::Translation(i as f32 * 2. - (tes_size) as f32, 0., j as f32 * 2. - (tes_size) as f32)));
                actions.transform.localsrt.push(OpsTransformNodeLocal::ops(instance, ETransformSRT::Scaling(0.2, 0.2, 0.2)));
                tmp.push(instance);
            }
        }
        testdata.0.push((source, idmat, tmp));
    }

    testdata.1 = Some(scene);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet, PartialOrd, Ord)]
pub enum StageTest {
    Cmd
}

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

pub fn main() {

    let (mut app, window, event_loop) = base::test_plugins();
    
    app.insert_resource(crate::base::DemoOption {
        orthographic_camera: true,
        camera_size: TEST_SIZE as f32,
        camera_fov: 0.9,
        camera_position: (0., 20., -40.),
        camera_nearfar: (0.1, TEST_SIZE as f32 + 0.1),
        ..Default::default()
    });
    app.add_startup_system(Update, base::setup_demoinit);
    
    app.add_plugins(PluginTest);
    // app.add_systems(Update, pi_3d::sys_info_node);
    // app.add_systems(Update, pi_3d::sys_info_resource);
    // app.add_systems(Update, pi_3d::sys_info_draw);
    
    
    //     #[cfg(feature = "use_bevy")]
    // app.add_systems(Startup, setup.after(base::setup_default_mat));
    // #[cfg(not(feature = "use_bevy"))]
    app.add_startup_system(Update, setup.after(base::setup_default_mat));
    
    
    // app.run()
    crate::base::run_loop(app, window, event_loop)

}