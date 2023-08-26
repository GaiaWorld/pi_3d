#![feature(box_into_inner)]

use pi_engine_shell::{prelude::*, frame_time::SingleFrameTimeCommand};
use pi_scene_context::prelude::*;
use pi_mesh_builder::cube::*;
use pi_wy_rng::WyRng;
use rand::Rng;

#[derive(Resource)]
pub struct ListTestData(Vec<Entity>, Option<Entity>, WyRng);

// pub struct SysTest;
// impl TSystemStageInfo for SysTest {}
// #[setup]
// impl SysTest {
//     #[system]
    pub fn sys(
        mut commands: Commands,
        mut testdata: ResMut<ListTestData>,
        mut disposereadylist: ResMut<ActionListDisposeReady>,
        mut transformcmds: ActionSetTransform,
        mut meshcmds: ActionSetMesh,
        mut geometrycmd: ActionSetGeometry,
        mut matuse: ActionSetMaterial,
        defaultmat: Res<SingleIDBaseDefaultMaterial>,
    ) {
        if let Some(entity) = testdata.0.pop() {
            disposereadylist.push(OpsDisposeReady::ops(entity));
        }
        
        // if testdata.0.len() % 2 != 0 {
        //     if let Some(entity) = testdata.0.pop() {
        //         disposereadylist.push(OpsDisposeReady::ops(entity));
        //         // transformcmds.enable.push(OpsNodeEnable::ops(entity, false));
        //     }
        //     return;
        // }

        if let Some(scene) = testdata.1.clone() {
            let random = &mut testdata.2;
            // log::warn!("Random: {:?}", random.gen_range(-5.0f32..5.0f32));
            let cube: Entity = commands.spawn_empty().id();
            let instancestate = 0;
            meshcmds.create.push(OpsMeshCreation::ops(scene, cube, MeshInstanceState { state: instancestate, use_single_instancebuffer: false }));
            transformcmds.tree.push(OpsTransformNodeParent::ops(cube, scene));
            transformcmds.localpos.push(OpsTransformNodeLocalPosition::ops(cube, random.gen_range(-5.0f32..5.0f32) as f32, random.gen_range(-5.0f32..5.0f32), random.gen_range(-5.0f32..5.0f32)));
            testdata.0.insert(0, cube);

            let id_geo = commands.spawn_empty().id();
            let attrs = CubeBuilder::attrs_meta();
            // attrs.push(VertexBufferDesc::instance_world_matrix());
            geometrycmd.create.push(OpsGeomeryCreate::ops(cube, id_geo, attrs, Some(CubeBuilder::indices_meta())));
    
            let idmat = defaultmat.0;
            matuse.usemat.push(OpsMaterialUse::ops(cube, idmat));
        }
    }
// }

// #[derive(Debug)]
pub struct PluginTest;
impl Plugin for PluginTest {
    fn build(&self, app: &mut App) {
        app.insert_resource(ListTestData(vec![], None, pi_wy_rng::WyRng::default()));
    }
}

fn setup(
    mut commands: Commands,
    mut scenecmds: ActionSetScene,
    mut cameracmds: ActionSetCamera,
    mut transformcmds: ActionSetTransform,
    mut meshcmds: ActionSetMesh,
    mut instancemeshcmds: ActionSetInstanceMesh,
    mut geometrycmd: ActionSetGeometry,
    mut matuse: ActionSetMaterial,
    mut animegroupcmd: ActionSetAnimationGroup,
    mut fps: ResMut<SingleFrameTimeCommand>,
    mut final_render: ResMut<WindowRenderer>,
    mut renderercmds: ActionSetRenderer,
    defaultmat: Res<SingleIDBaseDefaultMaterial>,
    mut testdata: ResMut<ListTestData>,
) {
    let tes_size = 6;
    fps.frame_ms = 16;

    let (scene, camera01) = base::DemoScene::new(&mut commands, &mut scenecmds, &mut cameracmds, &mut transformcmds, &mut animegroupcmd, &mut final_render, &mut renderercmds, tes_size as f32, 1., (0., 10., -40.), true);

    cameracmds.target.push(OpsCameraTarget::ops(camera01, 0., -1., 4.));

    let source = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(source, scene));
    let instancestate = InstanceState::INSTANCE_BASE;
    meshcmds.create.push(OpsMeshCreation::ops(scene, source, MeshInstanceState { state: instancestate, use_single_instancebuffer: false }));
    testdata.0.push(source);
    // meshcmds.render_alignment.push(OpsMeshRenderAlignment::ops(source, ERenderAlignment::StretchedBillboard));
    
    let id_geo = commands.spawn_empty().id();
    let attrs = CubeBuilder::attrs_meta();
    geometrycmd.create.push(OpsGeomeryCreate::ops(source, id_geo, attrs, Some(CubeBuilder::indices_meta())));

    let idmat = defaultmat.0;
    matuse.usemat.push(OpsMaterialUse::ops(source, idmat));


    for i in 0..tes_size {
        for j in 0..tes_size {
            for _k in 0..1 {
                let cube: Entity = commands.spawn_empty().id();
                instancemeshcmds.create.push(OpsInstanceMeshCreation::ops(source, cube));
                transformcmds.tree.push(OpsTransformNodeParent::ops(cube, source));
                transformcmds.localpos.push(OpsTransformNodeLocalPosition::ops(cube, i as f32 * 2. - (tes_size) as f32, 0., j as f32 * 2. - (tes_size) as f32));
                transformcmds.localscl.push(OpsTransformNodeLocalScaling::ops(cube, 0.2, 0.2, 0.2));
                testdata.0.push(cube);
            }
        }
    }

    testdata.1 = Some(scene);
}

#[path = "../base.rs"]
mod base;
pub fn main() {
    let mut app = base::test_plugins();
    
    app.add_plugins(PluginTest);
    app.add_systems(Update, pi_3d::sys_info_node);
    app.add_systems(Update, pi_3d::sys_info_resource);
    app.add_systems(Update, pi_3d::sys_info_draw);
    
    app.add_systems(Update, sys.in_set(ERunStageChap::Command));
    
    app.add_systems(Startup, setup);
    // bevy_mod_debugdump::print_main_schedule(&mut app);
    
    app.run()

}