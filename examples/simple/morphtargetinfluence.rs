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
        mut transform_commands: ResMut<ActionListAbstructMeshValueStateModify>,
    ) {
        list.exchange_empty().drain(..).for_each(|mut item| {
            item.1 = (item.1 + 0.025).fract();
            item.2 = item.2 + 0.01;
            item.3 = item.3 + 0.01;
            log::error!("{:?}", (item.1));
            transform_commands.push(OpsAbstructMeshValueStateModify::ops(item.0, EMeshValueStateModify::MorphInfluence(item.1, 0., 0., 0.)));

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
    mut assets: (ResMut<CustomRenderTargets>, Res<PiRenderDevice>, Res<ShareAssetMgr<SamplerRes>>, Res<PiSafeAtlasAllocator>, Res<PiRenderQueue>, ),
    mut testdata: ResMut<ActionListTestData>,
    demooption: Res<base::DemoOption>,
    mut allocator: ResMut<VertexBufferAllocator3D>,
    mut asset_mgr: ResMut<ShareAssetMgr<EVertexBufferRange>>,
) {
    let (demopass, scene, camera01, copyrenderer, copyrendercamera) = if let (Some(demo), Some(copyrenderer), Some(copyrendercamera)) = (&demooption.demo, &demooption.copyrenderer, &demooption.copyrendercamera) {
        (demo, demo.scene, demo.camera, *copyrenderer, *copyrendercamera)
    } else { return; };

    fps.frame_ms = 16;

    let tes_size = 4;

    actions.camera.param.push(OpsCameraModify::ops( camera01, ECameraModify::OrthSize( tes_size as f32 )));

    let mut vertices = CubeBuilder::attrs_meta();
    {
        let device = &assets.1;
        let queue = &assets.4;
        let mut singequad = SingleCube::default();
        let key = KeyVertexBuffer::from("key_morph");
        let morphdata: [f32; 288] = 
        [
            // z = 1
             0.5, -0.5,  0.5,       0.,  0.,  1.,      1., 0.,
            -0.5,  0.5,  0.5,       0.,  0.,  1.,      0., 1.,
             0.5,  0.5,  0.5,       0.,  0.,  1.,      1., 1., 
             0.5, -0.5,  0.5,       0.,  0.,  1.,      1., 0.,
            -0.5, -0.5,  0.5,       0.,  0.,  1.,      0., 0.,
            -0.5,  0.5,  0.5,       0.,  0.,  1.,      0., 1.,
            // z = -1
             0.5,  0.5, -0.5,       0.,  0., -1.,      1., 1.,
            -0.5, -0.5, -0.5,       0.,  0., -1.,      0., 0.,
             0.5, -0.5, -0.5,       0.,  0., -1.,      1., 0., 
             0.5,  0.5, -0.5,       0.,  0., -1.,      1., 1.,
            -0.5,  0.5, -0.5,       0.,  0., -1.,      0., 1.,
            -0.5, -0.5, -0.5,       0.,  0., -1.,      0., 0.,
            // x = 1
             0.5,  0.5, -0.5,       1.,  0.,  0.,      1., 0.,
             0.5, -0.5,  0.5,       1.,  0.,  0.,      0., 1.,
             0.5,  0.5,  0.5,       1.,  0.,  0.,      1., 1.,
             0.5,  0.5, -0.5,       1.,  0.,  0.,      1., 0.,
             0.5, -0.5, -0.5,       1.,  0.,  0.,      0., 0.,
             0.5, -0.5,  0.5,       1.,  0.,  0.,      0., 1.,
            // x = -1
            -0.5,  0.5,  0.5,      -1.,  0.,  0.,      1., 1.,
            -0.5, -0.5, -0.5,      -1.,  0.,  0.,      0., 0.,
            -0.5,  0.5, -0.5,      -1.,  0.,  0.,      1., 0.,
            -0.5,  0.5,  0.5,      -1.,  0.,  0.,      1., 1.,
            -0.5, -0.5,  0.5,      -1.,  0.,  0.,      0., 1.,
            -0.5, -0.5, -0.5,      -1.,  0.,  0.,      0., 0.,
            // y = 1
            -0.5,  0.5,  0.5,       0.,  1.,  0.,      0., 1.,
             0.5,  0.5, -0.5,       0.,  1.,  0.,      1., 0.,
             0.5,  0.5,  0.5,       0.,  1.,  0.,      1., 1.,
             -0.5,  0.5,  0.5,       0.,  1.,  0.,      0., 1.,
             -0.5,  0.5, -0.5,       0.,  1.,  0.,      0., 0.,
              0.5,  0.5, -0.5,       0.,  1.,  0.,      1., 0.,
             // y = -1
             0.5, -0.5,  0.5,       0., -1.,  0.,      1., 1.,
            -0.5, -0.5, -0.5,       0., -1.,  0.,      0., 0.,
            -0.5, -0.5,  0.5,       0., -1.,  0.,      0., 1.,
             0.5, -0.5,  0.5,       0., -1.,  0.,      1., 1.,
             0.5, -0.5, -0.5,       0., -1.,  0.,      1., 0.,
            -0.5, -0.5, -0.5,       0., -1.,  0.,      0., 0.,
        ];
        if let Some(bufferrange) = allocator.create_not_updatable_buffer(&device, &queue, &bytemuck::cast_slice(&morphdata).iter().map(|v| *v).collect::<Vec<u8>>(), None) {
            if let Ok(range) = asset_mgr.insert(key.asset_u64(), bufferrange) {
                singequad.0 = Some(range);
            }
        }
        vertices.push(
            VertexBufferDesc::vertices(
                key,
                VertexBufferDescRange::new(CubeBuilder::POSITION_OFFSET as VertexBufferRangeVType, CubeBuilder::POSITION_SIZE  as VertexBufferRangeVType ),
                vec![
                    EVertexAttribute::Buildin(EBuildinVertexAtribute::PositionM, wgpu::VertexFormat::Float32x3),
                    EVertexAttribute::Buildin(EBuildinVertexAtribute::NormalM, wgpu::VertexFormat::Float32x3),
                    EVertexAttribute::Buildin(EBuildinVertexAtribute::UVM, wgpu::VertexFormat::Float32x2),
                    // VertexAttribute { kind: EVertexDataKind::Position, format: wgpu::VertexFormat::Float32x3 },
                    // VertexAttribute { kind: EVertexDataKind::Normal, format: wgpu::VertexFormat::Float32x3 },
                    // VertexAttribute { kind: EVertexDataKind::UV, format: wgpu::VertexFormat::Float32x2 }
                ]
            ),
        );
    }
    let indices = CubeBuilder::indices_meta();
    let state = MeshInstanceState::default();
    let source = base::DemoScene::mesh(&mut commands, scene, scene, &mut actions,  vertices, indices, state);

    actions.mesh.value_state.push(OpsAbstructMeshValueStateModify::ops(source, EMeshValueStateModify::IndiceRange(Some((3, 12)))));
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
    app.add_systems(Update, pi_3d::sys_info_node);
    app.add_systems(Update, sys);
    
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