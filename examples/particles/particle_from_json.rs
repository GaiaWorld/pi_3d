

use base::DemoScene;
use pi_curves::{curve::frame_curve::FrameCurve, easing::EEasingMode};
use pi_gltf::json::{self, deserialize};
use pi_gltf2_load::particle_system::gltf_format_particle_cfg;
use pi_scene_shell::prelude::*;
use pi_scene_context::prelude::{TypeAnimeAssetMgrs, TypeAnimeContexts};
use pi_node_materials::prelude::BlockMainTexture;
use pi_scene_context::prelude::*;
use pi_mesh_builder::quad::QuadBuilder;
use unlit_material::*;
use pi_particle_system::prelude::*;

#[path = "../base.rs"]
mod base;
#[path = "../copy.rs"]
mod copy;

fn setup(
    mut commands: Commands,
    mut actions: pi_3d::ActionSets,
    mut particlesys_res: ResourceParticleSystem,
    mut animegroupres: ResourceAnimationGroup,
    anime_assets: TypeAnimeAssetMgrs,
    mut anime_contexts: TypeAnimeContexts,
    mut assets: (ResMut<CustomRenderTargets>, Res<PiRenderDevice>, Res<ShareAssetMgr<SamplerRes>>, Res<PiSafeAtlasAllocator>,),
    demooption: Res<base::DemoOption>,
) {
    let (demopass, scene, camera01, copyrenderer, copyrendercamera) = if let (Some(demo), Some(copyrenderer), Some(copyrendercamera)) = (&demooption.demo, &demooption.copyrenderer, &demooption.copyrendercamera) {
        (demo, demo.scene, demo.camera, *copyrenderer, *copyrendercamera)
    } else { return; };

    let tes_size = 20;
    // frame.frame_ms = 200;

    actions.camera.param.push(OpsCameraModify::ops( camera01, ECameraModify::OrthSize( tes_size as f32 )));
    actions.camera.target.push(OpsCameraTarget::ops(camera01, 0.0, -2.0, 1.0));

    let node = commands.spawn_empty_id(); actions.transform.tree.push(OpsTransformNodeParent::ops(node, scene));
    actions.transform.create.push(OpsTransformNode::ops(scene, node));

    let idmattrail = commands.spawn_empty_id();
    actions.material.create.push(OpsMaterialCreate::ops_with_matarray(idmattrail, UnlitShader::KEY));
    // actions.material.valb.push(OpsUniformValB::texture(idmattrail, UniformTextureWithSamplerParam {
    //     slotname: Atom::from(BlockMainTexture::KEY_TEX),
    //     sample: KeySampler::linear_repeat(),
    //     url: EKeyTexture::from("assets/images/4.png"),
    //     ..Default::default()
    // }));

    let mut _random = pi_wy_rng::WyRng::default();
    let temp = 1;
    let _size = -10.0..10.0;
    let _euler = -3.0..3.0;
    for _i in 0..temp {
        for _j in 0..temp {
            for _k in 0..temp {
                let _item = {
                    let vertices = QuadBuilder::attrs_meta();
                    let indices = None;
                    let state = base::particelsystem_mesh_state();
                    let source = base::DemoScene::mesh(&mut commands, scene, node, &mut actions,  vertices, indices, state);
                    // actions.transform.localsrt.push(OpsTransformNodeLocal::ops(source, ETransformSRT::Euler(-0.5*3.1415926, 0., 0.)));

                    let mut blend = ModelBlend::one_one();

                    actions.mesh.render_state.push(OpsRenderState::blend(source, DemoScene::PASS_TRANSPARENT, blend));
                    actions.mesh.render_state.push(OpsRenderState::primitive_state(source, DemoScene::PASS_TRANSPARENT, EPrimitiveState::CCullMode(CullMode::Off)));
                    actions.mesh.render_state.push(OpsRenderState::depth_state(source, DemoScene::PASS_TRANSPARENT, EDepthState::Compare(CompareFunction::Always)));
                    //
                    let syskey = String::from("Test");
                    let syscfg = demo_cfg(10., 1.);
                    let calculator = commands.spawn_empty_id();
                    actions.parsys.calculator.push(OpsCPUParticleCalculator::ops(calculator, syscfg));
                    let particle_sys_calculator = ParticleSystemCalculatorID(calculator, 1024, particlesys_res.calculator_queue.queue());
                    let calculator = particlesys_res.calcultors.insert(syskey.asset_u64(), particle_sys_calculator).unwrap();
                    let trailmesh = commands.spawn_empty_id();
                    let trailgeo = commands.spawn_empty_id();
                    actions.parsys.create.push(OpsCPUParticleSystem::ops(scene, source, trailmesh, trailgeo, *calculator.key(), base::particelsystem_attrs(), 0));
                    actions.parsys.state.push(OpsCPUParticleSystemState::ops_start(source));
                    actions.parsys.state.push(OpsCPUParticleSystemState::ops_speed(source, 0.7));
                    // actions.particlesys_cmds.particlesys_state_.push(OpsCPUParticleSystemState::ops_stop(source));
                    //
                    actions.material.usemat.push(OpsMaterialUse::ops(source, idmattrail, DemoScene::PASS_TRANSPARENT));
                    source
                };
            }
        }
    }
}

fn demo_cfg(count: f32, speed: f32) -> IParticleSystemConfig {

    let v = include_bytes!("../../assets/particles/eff_sz_ui_zhandoushengli1.gltf");
    let v = deserialize::from_slice::<json::Node>(v.as_slice());
    match v {
        Ok(node) => {
            if let Some(extras) = &node.extras {
                if let Some(cfg) = extras.get("meshParticle") {
                    let v = gltf_format_particle_cfg(cfg);
                    log::error!("{:?}", cfg);
                    return v;
                }
            }
        },
        Err(_) => {},
    }

    return IParticleSystemConfig::new();
}

pub type ActionListTestData = ActionList<(ObjectID, f32, f32, f32)>;

pub struct PluginTest;
impl Plugin for PluginTest {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActionListTestData::default());
    }
}



pub fn main() {
    let (mut app, window, event_loop) = base::test_plugins_with_gltf();

    app.insert_resource(crate::base::DemoOption {
        orthographic_camera: true,
        camera_size: 10.,
        camera_fov: 0.7,
        camera_position: (0., 34., -20.),
        ..Default::default()
    });
    app.add_startup_system(Update, base::setup_demoinit);

    app.add_plugins(PluginTest);
    // app.add_systems(StageD3, pi_3d::sys_info_node);
    // app.add_systems(StageD3, pi_3d::sys_info_draw);
    // app.add_systems(StageD3, pi_3d::sys_info_resource);

    app.world.get_resource_mut::<StateRecordCfg>().unwrap().write_state = false;

        #[cfg(feature = "use_bevy")]
    app.add_systems(Startup, setup.after(base::setup_default_mat));
    #[cfg(not(feature = "use_bevy"))]
    app.add_startup_system(Update, setup.after(base::setup_default_mat));
    
    
    // app.run()
    crate::base::run_loop(app, window, event_loop)

}