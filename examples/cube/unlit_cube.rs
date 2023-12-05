#![feature(box_into_inner)]


use base::DemoScene;
use pi_atom::Atom;
use pi_engine_shell::prelude::*;
use pi_node_materials::prelude::*;
use pi_scene_context::prelude::*;
use pi_mesh_builder::cube::*;
use unlit_material::*;

#[path = "../base.rs"]
mod base;
#[path = "../copy.rs"]
mod copy;

// #[derive(Debug, Default)]
// pub struct SingleTestData {
//     pub transforms: Vec<(ObjectID, f32, f32, f32)>,
// }

// pub struct SysTest;
// impl TSystemStageInfo for SysTest {}
// #[setup]
// impl SysTest {
//     #[system]
//     pub fn sys(
//         mut list: ResMut<SingleTestData>,
//         mut transform_commands: ResMut<SingleTransformNodeModifyCommandList>,
//     ) {
//         list.transforms.iter_mut().for_each(|mut item| {
//             item.1 = item.1 + 16.0;
//             item.2 = item.2 + 16.0;
//             item.3 = item.3 + 16.0;
//             let x0 = item.1 % 4000.0 / 4000.0;
//             let x = x0 * 3.1415926 * 2.;
//             let y0 = item.2 % 4000.0 / 4000.0;
//             let y = y0 * 3.1415926 * 2.;
//             let z0 = item.3 % 4000.0 / 4000.0;
//             let z = z0 * 3.1415926 * 2.;
//             // transform_commands.list.push(TransformNodeCommand::ModifyPosition(item.0, Vector3::new(x.cos() * 3., 0., 0.)));
//             // transform_commands.list.push(TransformNodeCommand::ModifyScaling(item.0, Vector3::new(x.cos() + 0.5, x.sin() + 0.5, x + 0.5)));
//             transform_commands.list.push(ETransformNodeModifyCommand::ModifyRotation(item.0, Vector3::new(x, y, z)));
//         });
//     }
// }

// #[derive(Debug)]
// pub struct PluginTest;
// impl Plugin for PluginTest {
//     fn init(
//         &mut self,
//         engine: &mut pi_scene_context::engine::Engine,
//         stages: &mut pi_scene_context::run_stage::RunStage,
//     ) -> Result<(), pi_scene_context::plugin::ErrorPlugin> {
//         PluginLocalLoad.init(engine, stages);
//         PluginBundleDefault.init(engine, stages);
//         // PluginMaterialTextures.init(engine, stages);
//         // PluginMainTexture.init(engine, stages);
//         PluginUnlitMaterial.init(engine, stages);
//         PluginCubeBuilder.init(engine, stages);

//         let world = engine.world_mut();

//         SysTest::setup(world, stages.query_stage::<SysTest>(ERunStageChap::Command));

//         let testdata = SingleTestData::default();
//         world.insert_resource(testdata);

//         Ok(())
//     }
// }

fn setup(
    mut commands: Commands,
    mut actions: pi_3d::ActionSets,
    mut animegroupres: ResourceAnimationGroup,
    mut fps: ResMut<SingleFrameTimeCommand>,
    mut assets: (ResMut<CustomRenderTargets>, Res<PiRenderDevice>, Res<ShareAssetMgr<SamplerRes>>, Res<PiSafeAtlasAllocator>,),
) {
    let tes_size = 20;
    fps.frame_ms = 4;

    let demopass = DemoScene::new(&mut commands, &mut actions, &mut animegroupres, 
        &mut assets.0, &assets.1, &assets.2, &assets.3,
        tes_size as f32, 0.7, (0., 0., -10.), true
    );
    let (scene, camera01) = (demopass.scene, demopass.camera);

    let (copyrenderer, copyrendercamera) = copy::PluginImageCopy::toscreen(&mut commands, &mut actions, scene, demopass.transparent_renderer,demopass.transparent_target);
    actions.renderer.connect.push(OpsRendererConnect::ops(demopass.transparent_renderer, copyrenderer, false));

    actions.camera.size.push(OpsCameraOrthSize::ops(camera01, tes_size as f32));

    let vertices = CubeBuilder::attrs_meta();
    let indices = Some(CubeBuilder::indices_meta());
    let state = MeshInstanceState { state: 0, ..Default::default() };
    let source = base::DemoScene::mesh(&mut commands, scene, scene, &mut actions,  vertices, indices, state);
    let mut blend = ModelBlend::default(); blend.combine();
    actions.mesh.blend.push(OpsRenderBlend::ops(source, blend));

    let idmat = commands.spawn_empty().id();
    actions.material.create.push(OpsMaterialCreate::ops(idmat, UnlitShader::KEY));
    actions.material.texture.push(OpsUniformTexture::ops(idmat, UniformTextureWithSamplerParam {
        slotname: Atom::from(BlockMainTexture::KEY_TEX),
        filter: true,
        sample: KeySampler::linear_repeat(),
        url: EKeyTexture::from("E:/Rust/PI/pi_3d/assets/images/Q69L5MmgSNC2xbBiAwZcDw.png"),
    }));
    
    actions.material.usemat.push(OpsMaterialUse::ops(source, idmat, DemoScene::PASS_OPAQUE));
}

pub type ActionListTestData = ActionList<(ObjectID, f32, f32, f32)>;

pub struct PluginTest;
impl Plugin for PluginTest {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActionListTestData::default());
    }
}

pub fn main() {
    let mut app = base::test_plugins();
    
    app.add_plugins(PluginTest);
    
    app.add_systems(Startup, setup.after(base::setup_default_mat));
    // bevy_mod_debugdump::print_main_schedule(&mut app);
    
    app.run()
}