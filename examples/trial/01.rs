

use base::DemoScene;
use pi_animation::loop_mode::ELoopMode;
use pi_curves::{curve::frame_curve::FrameCurve, easing::EEasingMode};
use pi_postprocess::prelude::TImageEffect;
use pi_scene_shell::prelude::*;
use pi_scene_context::prelude::{TypeAnimeAssetMgrs, TypeAnimeContexts};
use pi_node_materials::prelude::BlockMainTexture;
use pi_scene_context::prelude::*;
use pi_scene_math::*;
use pi_mesh_builder::cube::*;
use pi_trail_renderer::{ActionSetTrailRenderer, OpsTrail, OpsTrailAgeControl};
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
    mut animegroupres: ResourceAnimationGroup,
    anime_assets: TypeAnimeAssetMgrs,
    mut anime_contexts: TypeAnimeContexts,
    mut assets: (ResMut<CustomRenderTargets>, Res<PiRenderDevice>, Res<ShareAssetMgr<SamplerRes>>, Res<PiSafeAtlasAllocator>,),
    mut graphic: ResMut<PiRenderGraph>,
    mut resources: ResMut<pi_postprocess::image_effect::SingleImageEffectResource>,
    demooption: Res<base::DemoOption>,
) {
    let (demopass, scene, camera01, copyrenderer, copyrendercamera) = if let (Some(demo), Some(copyrenderer), Some(copyrendercamera)) = (&demooption.demo, &demooption.copyrenderer, &demooption.copyrendercamera) {
        (demo, demo.scene, demo.camera, *copyrenderer, *copyrendercamera)
    } else { return; };

    let tes_size = 50;
    
    pi_postprocess::image_effect::EffectBlurBokeh::setup(&assets.1, &mut resources, &assets.2);
    pi_postprocess::image_effect::EffectBlurDirect::setup(&assets.1, &mut resources, &assets.2);
    pi_postprocess::image_effect::EffectBlurDual::setup(&assets.1, &mut resources, &assets.2);
    pi_postprocess::image_effect::EffectBlurRadial::setup(&assets.1, &mut resources, &assets.2);
    pi_postprocess::image_effect::EffectColorEffect::setup(&assets.1, &mut resources, &assets.2);
    pi_postprocess::image_effect::EffectCopy::setup(&assets.1, &mut resources, &assets.2);
    pi_postprocess::image_effect::EffectFilterBrightness::setup(&assets.1, &mut resources, &assets.2);
    pi_postprocess::image_effect::EffectFilterSobel::setup(&assets.1, &mut resources, &assets.2);
    pi_postprocess::image_effect::EffectHorizonGlitch::setup(&assets.1, &mut resources, &assets.2);
    pi_postprocess::image_effect::EffectRadialWave::setup(&assets.1, &mut resources, &assets.2);
    pi_postprocess::image_effect::EffectBlurGauss::setup(&assets.1, &mut resources, &assets.2);
    pi_postprocess::image_effect::EffectImageMask::setup(&assets.1, &mut resources, &assets.2);
    pi_postprocess::image_effect::EffectClipSdf::setup(&assets.1, &mut resources, &assets.2);
    let copyrenderer = commands.spawn_empty_id();
    let render_node = pi_postprocess::graphic::RenderNode::new(copyrenderer);
    match graphic.add_node("TestPostProcess", render_node, NodeId::null()) {
        Ok(nodeid) => {
            let dst = pi_postprocess::component::PostProcessDst {
                width: 10,
                height: 1,
                vtype: None,
                format: wgpu::TextureFormat::Bgra8Unorm,
                depth: 0.,
                screen: true,
            };
            match demopass.transparent_target.clone().unwrap() {
                KeyCustomRenderTarget::Custom(key) => {
                    log::error!("PostProcess Ok");
                    let target = assets.0.get(key).unwrap();
                    let bundle = (
                        GraphId(nodeid),
                        pi_postprocess::postprocess::PostProcess::default(),
                        pi_postprocess::postprocess::PostProcessAnalyzer::default(),
                        dst,
                        pi_postprocess::component::PostProcessMatrix(Matrix::identity()),
                        pi_postprocess::component::PostprocessDrawList::default(),
                        pi_postprocess::component::PostProcessResult::default(),
                        pi_postprocess::component::PostProcessSrc(Some(pi_postprocess::temprory_render_target::PostprocessTexture::from_share_target(target.rt, ColorFormat::Rgba8Unorm.val())))
                    );
                    commands.add_components::<(
                        GraphId,
                        pi_postprocess::postprocess::PostProcess,
                        pi_postprocess::postprocess::PostProcessAnalyzer,
                        pi_postprocess::component::PostProcessDst,
                        pi_postprocess::component::PostProcessMatrix,
                        pi_postprocess::component::PostprocessDrawList,
                        pi_postprocess::component::PostProcessResult,
                        pi_postprocess::component::PostProcessSrc
                    )>(copyrenderer, bundle);
                    
                    if let Err(err) = graphic.set_finish(nodeid, true) {
                        // error.graphic(entity, err);
                    }
                },
                KeyCustomRenderTarget::FinalRender(_) => todo!(),
            }
        },
        Err(err) => {
            // log::error!("CreateRenderer Fail Graphic Error");
            // error.graphic(entity, err);
        },
    }

    // let (copyrenderer, copyrendercamera) = copy::PluginImageCopy::toscreen(&mut commands, &mut actions, scene, demopass.transparent_renderer, demopass.transparent_target);
    actions.renderer.connect.push(OpsRendererConnect::ops(demopass.transparent_renderer, copyrenderer, false));

    actions.camera.param.push(OpsCameraModify::ops( camera01, ECameraModify::OrthSize( tes_size as f32 )));

    let idmat = commands.spawn_empty_id();
    actions.material.create.push(OpsMaterialCreate::ops_with_matarray(idmat, UnlitShader::KEY));
    actions.material.valb.push(OpsUniformValB::texture(idmat, UniformTextureWithSamplerParam {
        slotname: Atom::from(BlockMainTexture::KEY_TEX),
        sample: KeySampler::linear_repeat(),
        url: EKeyTexture::from("assets/images/eff_daoguang_lf_004.png"),
        ..Default::default()
    }));

    let source = commands.spawn_empty_id(); actions.transform.tree.push(OpsTransformNodeParent::ops(source, scene));
    actions.mesh.create.push(OpsMeshCreation::ops(scene, source, MeshInstanceState::default()));
    actions.transform.localsrt.push(OpsTransformNodeLocal::ops(source, ETransformSRT::Translation(0., 10., 0.)));
    actions.material.usemat.push(OpsMaterialUse::ops(source, idmat, DemoScene::PASS_TRANSPARENT));
    let id_geo = commands.spawn_empty_id();
    actions.geometry.create.push(OpsGeomeryCreate::ops(source, id_geo, CubeBuilder::attrs_meta(), CubeBuilder::indices_meta()));
    
    let node = commands.spawn_empty_id(); actions.transform.tree.push(OpsTransformNodeParent::ops(node, scene));
    actions.transform.create.push(OpsTransformNode::ops(scene, node));

    // let key_group = pi_atom::Atom::from("key_group");
    let id_group = commands.spawn_empty_id();
    // animegroupres.scene_ctxs.create_group(scene).unwrap();
    // animegroupres.global.record_group(source, id_group);
    actions.anime.create.push(OpsAnimationGroupCreation::ops(scene, id_group));
    // actions.anime.attach.push(OpsAnimationGroupAttach::ops(scene, source, id_group));
    {
        let key_curve0 =  pi_atom::Atom::from("test2"); 
        let key_curve0 = key_curve0.asset_u64();
        let curve = FrameCurve::<LocalEulerAngles>::curve_easing(LocalEulerAngles(Vector3::new(0., 0., 0.)), LocalEulerAngles(Vector3::new(0., 3.1415926 * 4., 3.1415926 * 2.)), (5. * 60.) as FrameIndex, 30, EEasingMode::None);
        
        let asset_curve = if let Some(curve) = anime_assets.euler.get(&key_curve0) { curve } else {
            match anime_assets.euler.insert(key_curve0, TypeFrameCurve(curve)) {
                Ok(value) => { value  },
                Err(_) => { return; },
            }
        };

        let animation = anime_contexts.euler.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
        actions.anime.action.push(OpsAnimationGroupAction::addtarget(id_group.clone(), node, animation));
    }
    {
        let key_curve0 =  pi_atom::Atom::from("test0"); 
        let key_curve0 = key_curve0.asset_u64();
        let curve = FrameCurve::<LocalPosition>::curve_easing(LocalPosition(Vector3::new(-10., -10., 0.)), LocalPosition(Vector3::new(0., 20., 0.)), (300.) as FrameIndex, 30, EEasingMode::SineInOut);
        
        let asset_curve = if let Some(curve) = anime_assets.position.get(&key_curve0) { curve } else {
            match anime_assets.position.insert(key_curve0, TypeFrameCurve(curve)) {
                Ok(value) => { value  },
                Err(_) => { return; },
            }
        };

        let animation = anime_contexts.position.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
        actions.anime.action.push(OpsAnimationGroupAction::addtarget(id_group.clone(), node, animation));
    }
    {
        let key_curve0 =  pi_atom::Atom::from("test1"); 
        let key_curve0 = key_curve0.asset_u64();
        let curve = FrameCurve::<LocalScaling>::curve_easing(LocalScaling(Vector3::new(0.2, 0.2, 0.2)), LocalScaling(Vector3::new(2., 2., 2.)), (300.) as FrameIndex, 30, EEasingMode::SineInOut);
        
        let asset_curve = if let Some(curve) = anime_assets.scaling.get(&key_curve0) { curve } else {
            match anime_assets.scaling.insert(key_curve0, TypeFrameCurve(curve)) {
                Ok(value) => { value  },
                Err(_) => { return; },
            }
        };

        let animation = anime_contexts.scaling.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
        actions.anime.action.push(OpsAnimationGroupAction::addtarget(id_group.clone(), node, animation));
    }


    let mut random = pi_wy_rng::WyRng::default();
    for idx in 0..1000 {
        // let scalescalar = if idx % 2 == 0 { 1. } else { -1. };

        let source = commands.spawn_empty_id(); actions.transform.tree.push(OpsTransformNodeParent::ops(source, node));
        // if idx == 0 {
        //     actions.mesh.create.push(OpsMeshCreation::ops(scene, source));
        //     actions.material.usemat.push(OpsMaterialUse::ops(source, idmat));
        //     let id_geo = commands.spawn_empty_id();
        //     let instancestate = 0;
        //     actions.geometry.create.push(OpsGeomeryCreate::ops(source, id_geo, CubeBuilder::attrs_meta(), CubeBuilder::indices_meta(), instancestate));
        // } else {
            actions.transform.create.push(OpsTransformNode::ops(scene, source));
        // }
        actions.transform.localsrt.push(OpsTransformNodeLocal::ops(source, ETransformSRT::Translation(random.gen_range(-20.0..20.0), random.gen_range(-20.0..20.0), random.gen_range(-20.0..20.0))));
        actions.transform.localsrt.push(OpsTransformNodeLocal::ops(source, ETransformSRT::Scaling(4., 4., 4.)));
        actions.transform.localsrt.push(OpsTransformNodeLocal::ops(source, ETransformSRT::Euler(3., 0., 0.)));

        let trail = commands.spawn_empty_id();
        actions.transform.tree.push(OpsTransformNodeParent::ops(trail, scene));
        actions.mesh.create.push(OpsMeshCreation::ops(scene, trail, MeshInstanceState::default()));
        actions.trail.create.push(OpsTrail::ops(scene, source, trail));
        actions.trail.age.push(OpsTrailAgeControl::ops(trail, 500));
        actions.material.usemat.push(OpsMaterialUse::ops(trail, idmat, DemoScene::PASS_TRANSPARENT));
        let mut blend = ModelBlend::default(); blend.combine();
        actions.mesh.render_state.push(OpsRenderState::blend(trail, DemoScene::PASS_TRANSPARENT, blend));
        actions.mesh.render_state.push(OpsRenderState::depth_state(trail, DemoScene::PASS_TRANSPARENT, EDepthState::Compare(CompareFunction::Always)));
    }
    
    let mut param = AnimationGroupParam::default(); param.fps = 60; param.speed = 2.;param.loop_mode = ELoopMode::PositivePly(None);
    actions.anime.action.push(OpsAnimationGroupAction::Start(id_group, param, 0., pi_animation::base::EFillMode::NONE));
    // engine.start_animation_group(source, &key_group, 1.0, ELoopMode::OppositePly(None), 0., 1., 60, AnimationAmountCalc::default());
}

fn _demo_cfg(count: f32, speed: f32) -> IParticleSystemConfig {
    let mut cfg = IParticleSystemConfig::new();

    cfg.name = String::from("Test");
    cfg.duration = 1.0;
    cfg.looping = 1;
    cfg.max_particles = count;
    cfg.emission = (count, None);
    cfg.start_speed = OneParamInfo::TInterpolateConstant(speed);
    cfg.start_color = FourGradientInfo::TInterpolateRandom;
    cfg.color_over_lifetime = Some(FourGradientInfo::TInterpolateRandom);
    cfg.lifetime = OneParamInfo::TInterpolateConstant(1.);
    cfg.shape = IShape::ShapeCone(IShapeCone::default());

    cfg
}


pub type ActionListTestData = ActionList<(ObjectID, f32, f32, f32)>;

pub struct PluginTest;
impl Plugin for PluginTest {
    fn build(&self, app: &mut App) {
        let device = app.world.get_resource::<PiRenderDevice>().unwrap().0.clone();
        let queue = app.world.get_resource::<PiRenderQueue>().unwrap().0.clone();
        let buffer = app.world.get_resource_mut::<VertexBufferAllocator3D>().unwrap();
        let temp = pi_postprocess::image_effect::SingleImageEffectResource::new(&device, &queue, buffer);
        app.insert_resource(temp);
        app.insert_resource(ActionListTestData::default());
    }
}



pub fn main() {
    let (mut app, window, event_loop) = base::test_plugins_with_gltf();

    app.insert_resource(crate::base::DemoOption {
        orthographic_camera: true,
        camera_size: 50.,
        camera_fov: 0.7,
        camera_position: (0., 10., -50.),
        ..Default::default()
    });
    app.add_startup_system(Update, base::setup_demoinit);

    app.add_plugins(PluginTest);
    // app.add_systems(StageD3, base::sys_nodeinfo);

    app.add_systems(StageD3, pi_3d::sys_info_node);
    app.add_systems(StageD3, pi_3d::sys_info_resource);
    app.world.get_resource_mut::<StateRecordCfg>().unwrap().write_state = false;

        #[cfg(feature = "use_bevy")]
    app.add_systems(Startup, setup.after(base::setup_default_mat));
    #[cfg(not(feature = "use_bevy"))]
    app.add_startup_system(Update, setup.after(base::setup_default_mat));

    crate::base::run_loop(app, window, event_loop)

}

#[test]
fn test() {
    let key1 = KeyShaderFromAttributes(vec![]);
    let key2 = KeyShaderFromAttributes(vec![]);
    println!("{:?}", key1 == key2);
}