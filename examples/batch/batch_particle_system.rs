

use base::DemoScene;
use pi_curves::{curve::frame_curve::FrameCurve, easing::EEasingMode};
use pi_scene_shell::prelude::*;
use pi_scene_context::{prelude::{TypeAnimeAssetMgrs, TypeAnimeContexts}, scene::StageScene};
use pi_node_materials::prelude::BlockMainTexture;
use pi_scene_context::prelude::*;
use pi_mesh_builder::cube::*;
use rand::Rng;
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
) {
    let tes_size = 20;
    // frame.frame_ms = 200;

    let demopass = base::DemoScene::new(&mut commands, &mut actions, &mut animegroupres, 
        &mut assets.0, &assets.1, &assets.2, &assets.3,
        tes_size as f32, 0.7, (0., 0., -50.), true
    );
    let (scene, camera01) = (demopass.scene, demopass.camera);

    let (copyrenderer, copyrendercamera) = copy::PluginImageCopy::toscreen(&mut commands, &mut actions, scene, demopass.transparent_renderer,demopass.transparent_target);
    actions.renderer.connect.push(OpsRendererConnect::ops(demopass.transparent_renderer, copyrenderer, false));

    actions.camera.param.push(OpsCameraModify::ops( camera01, ECameraModify::OrthSize( tes_size as f32 )));

    let node = commands.spawn_empty().id(); actions.transform.tree.push(OpsTransformNodeParent::ops(node, scene));
    actions.transform.create.push(OpsTransformNode::ops(scene, node));

    let mut mats = vec![];
    {
        let idmattrail = commands.spawn_empty().id();
        actions.material.create.push(OpsMaterialCreate::ops(idmattrail, UnlitShader::KEY));
        actions.material.texture.push(OpsUniformTexture::ops(idmattrail, UniformTextureWithSamplerParam {
            slotname: Atom::from(BlockMainTexture::KEY_TEX),
            filter: true,
            sample: KeySampler::linear_repeat(),
            url: EKeyTexture::from("E:/Rust/PI/pi_3d/assets/images/eff_daoguang_lf_004.png"),
        }));
        mats.push(idmattrail);
    }

    {
        let idmattrail = commands.spawn_empty().id();
        actions.material.create.push(OpsMaterialCreate::ops(idmattrail, UnlitShader::KEY));
        actions.material.texture.push(OpsUniformTexture::ops(idmattrail, UniformTextureWithSamplerParam {
            slotname: Atom::from(BlockMainTexture::KEY_TEX),
            filter: true,
            sample: KeySampler::linear_repeat(),
            url: EKeyTexture::from("E:/Rust/PI/pi_3d/assets/images/4.png"),
        }));
        mats.push(idmattrail);
    }

    let mut random = pi_wy_rng::WyRng::default();
    let temp = 4;
    let size = -10.0..10.0;
    let euler = -3.0..3.0;
    for _i in 0..temp {
        for _j in 0..temp {
            for _k in 0..temp {
                let item = {
                    
                    let vertices = CubeBuilder::attrs_meta();
                    let indices = Some(CubeBuilder::indices_meta());
                    let state = base::particelsystem_mesh_state();
                    let source = base::DemoScene::mesh(&mut commands, scene, node, &mut actions,  vertices, indices, state);

                    let mut blend = ModelBlend::default(); blend.combine();
                    actions.mesh.blend.push(OpsRenderBlend::ops(source, DemoScene::PASS_TRANSPARENT, blend));
                    actions.mesh.render_queue.push(OpsRenderQueue::ops(source, 0, _k % 2));

                    //
                    let syskey = String::from("Test");
                    let syscfg = demo_cfg(10., 5.);
                    let calculator = commands.spawn_empty().id();
                    actions.parsys.calculator.push(OpsCPUParticleCalculator::ops(calculator, syscfg));
                    let particle_sys_calculator = ParticleSystemCalculatorID(calculator, 1024, particlesys_res.calculator_queue.queue());
                    let calculator = particlesys_res.calcultors.insert(syskey.asset_u64(), particle_sys_calculator).unwrap();
                    let trailmesh = commands.spawn_empty().id();
                    let trailgeo = commands.spawn_empty().id();
                    actions.parsys.create.push(OpsCPUParticleSystem::ops(scene, source, trailmesh, trailgeo, calculator, base::particelsystem_attrs()));
                    // actions.particlesys_cmds.particlesys_state_.push(OpsCPUParticleSystemState::ops_start(source));
                    // actions.particlesys_cmds.particlesys_state_.push(OpsCPUParticleSystemState::ops_stop(source));
                    //
                    let idmat = mats.get((_k as usize) % 2).unwrap().clone();
                    actions.material.usemat.push(OpsMaterialUse::ops(source, idmat, DemoScene::PASS_TRANSPARENT));
                    // actions.material.create.push(OpsMaterialCreate::ops(idmat, UnlitShader::KEY));
                    actions.parsys.trailmaterial.push(OpsCPUParticleSystemTrailMaterial::ops(source, mats.get(0).unwrap().clone(), DemoScene::PASS_TRANSPARENT));
                    source
                };

                actions.transform.localsrt.push(OpsTransformNodeLocal::ops(item, ETransformSRT::Translation(random.gen_range(size.clone()), random.gen_range(size.clone()), random.gen_range(size.clone()))));
                actions.transform.localsrt.push(OpsTransformNodeLocal::ops(item, ETransformSRT::Euler(random.gen_range(euler.clone()), random.gen_range(euler.clone()), random.gen_range(euler.clone()))));
                actions.transform.localsrt.push(OpsTransformNodeLocal::ops(item, ETransformSRT::Scaling(0.2, 0.2, 0.2)));
            }
        }
    }

    // actions.material.texture.push(OpsUniformTexture::ops(idmat, UniformTextureWithSamplerParam {
    //     slotname: Atom::from("_MainTex"),
    //     filter: true,
    //     sample: KeySampler::default(),
    //     url: EKeyTexture::from("E:/Rust/PI/pi_3d/assets/images/bubbles.png"),
    // }));

    
    // let key_group = pi_atom::Atom::from("key_group");
    let id_group = commands.spawn_empty().id();
    // animegroupres.scene_ctxs.create_group(scene).unwrap();
    // animegroupres.global.record_group(source, id_group);
    actions.anime.create.push(OpsAnimationGroupCreation::ops(scene, id_group));
    // actions.anime.attach.push(OpsAnimationGroupAttach::ops(scene, source, id_group));
    {
        let key_curve0 =  pi_atom::Atom::from("test2"); 
        let key_curve0 = key_curve0.asset_u64();
        let curve = FrameCurve::<LocalRotationQuaternion>::curve_easing(
            LocalRotationQuaternion::create(0., 0., 0., 1.), LocalRotationQuaternion::create(5., 0., 0., 1.),
            (60.) as FrameIndex, 30, EEasingMode::None);
        let asset_curve = if let Some(curve) = anime_assets.quaternion.get(&key_curve0) { curve } else {
            match anime_assets.quaternion.insert(key_curve0, TypeFrameCurve(curve)) {
                Ok(value) => { value },
                Err(_) => { return; },
            }
        };
        let animation = anime_contexts.quaternion.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
        actions.anime.add_target_anime.push(OpsAddTargetAnimation::ops(id_group.clone(), node, animation));
    }

    let mut param = AnimationGroupParam::default(); param.fps = 60; param.speed = 2.;
    // actions.anime.action.push(OpsAnimationGroupAction::Start(id_group, parma, 0., pi_animation::base::EFillMode::NONE));
    // engine.start_animation_group(source, &key_group, 1.0, ELoopMode::OppositePly(None), 0., 1., 60, AnimationAmountCalc::default());
}

fn demo_cfg(count: f32, speed: f32) -> IParticleSystemConfig {
    let mut cfg = IParticleSystemConfig::new();

    cfg.name = String::from("Test");
    cfg.prewarm = true;
    cfg.duration = 1.0;
    cfg.looping = 1;
    cfg.max_particles = count;
    cfg.emission = (count, None);
    cfg.start_speed = OneParamInfo::TInterpolateConstant(speed);
    cfg.start_color = FourGradientInfo::TInterpolateColor([1., 1., 1., 1.]);
    cfg.start_size = ParamInfo::OneParamInfo(OneParamInfo::TInterpolateConstant(1.));
    // cfg.color_over_lifetime = Some(FourGradientInfo::TInterpolateRandom);
    cfg.lifetime = OneParamInfo::TInterpolateConstant(1.);
    cfg.shape = IShape::ShapeCone(IShapeCone::default());
    // cfg.trail = Some(ITrail {
    //     ratio: 1.,
    //     mode: ETrailMode::Particles,
    //     lifetime: OneParamInfo::TInterpolateConstant(1.),
    //     ribbon_count: 5.,
    //     attach_rtt: 10,
    //     min_dist: 0.5,
    //     world_space: 0,
    //     die_with: 1,
    //     tex_mode: ETrailTextureMode::Stretch,
    //     size_awidth: 1,
    //     size_alifetime: 1,
    //     inherit_color: 1,
    //     color_over_life: FourGradientInfo::TInterpolateColor([1., 1., 1., 1.]),
    //     width_over_trail: OneParamInfo::TInterpolateConstant(1.),
    //     color_over_trail: FourGradientInfo::TInterpolateColor([1., 1., 1., 1.]),
    //     material: 0.,
    // });

    cfg
}

pub type ActionListTestData = ActionList<(ObjectID, f32, f32, f32)>;

pub struct PluginTest;
impl Plugin for PluginTest {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActionListTestData::default());
    }
}


pub fn main() {
    let mut app = base::test_plugins_with_gltf();
    
    app.add_plugins(PluginTest);
    app.add_systems(Update, pi_3d::sys_info_node        .run_if(should_run).in_set(StageScene::Create));
    app.add_systems(Update, pi_3d::sys_info_draw        .run_if(should_run).in_set(StageScene::Create));
    app.add_systems(Update, pi_3d::sys_info_resource    .run_if(should_run).in_set(StageScene::Create));

    app.world.get_resource_mut::<StateRecordCfg>().unwrap().write_state = false;

    app.add_systems(Startup, setup.after(base::setup_default_mat));
    
    
    // app.run()
    loop { app.update(); }

}