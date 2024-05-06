

use base::DemoScene;
use pi_animation::loop_mode::ELoopMode;
use pi_curves::{curve::frame_curve::FrameCurve, easing::EEasingMode};
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
) {
    let tes_size = 50;
    let demopass = base::DemoScene::new(&mut commands, &mut actions, &mut animegroupres,
        &mut assets.0, &assets.1, &assets.2, &assets.3,
        tes_size as f32, 0.7, (0., 10., -50.), true
    );
    let (scene, camera01) = (demopass.scene, demopass.camera);

    let (copyrenderer, copyrendercamera) = copy::PluginImageCopy::toscreen(&mut commands, &mut actions, scene, demopass.transparent_renderer, demopass.transparent_target);
    actions.renderer.connect.push(OpsRendererConnect::ops(demopass.transparent_renderer, copyrenderer, false));

    actions.camera.size.push(OpsCameraOrthSize::ops(camera01, tes_size as f32));

    let idmat = commands.spawn_empty().id();
    actions.material.create.push(OpsMaterialCreate::ops(idmat, UnlitShader::KEY));
    actions.material.texture.push(OpsUniformTexture::ops(idmat, UniformTextureWithSamplerParam {
        slotname: Atom::from(BlockMainTexture::KEY_TEX),
        filter: true,
        sample: KeySampler::linear_repeat(),
        url: EKeyTexture::from("E:/Rust/PI/pi_3d/assets/images/eff_daoguang_lf_004.png"),
    }));

    let source = commands.spawn_empty().id(); actions.transform.tree.push(OpsTransformNodeParent::ops(source, scene));
    actions.mesh.create.push(OpsMeshCreation::ops(scene, source, MeshInstanceState::default()));
    actions.transform.localsrt.push(OpsTransformNodeLocal::ops(source, ETransformSRT::Translation(0., 10., 0.)));
    actions.material.usemat.push(OpsMaterialUse::ops(source, idmat, DemoScene::PASS_TRANSPARENT));
    let id_geo = commands.spawn_empty().id();
    actions.geometry.create.push(OpsGeomeryCreate::ops(source, id_geo, CubeBuilder::attrs_meta(), Some(CubeBuilder::indices_meta())));
    
    let node = commands.spawn_empty().id(); actions.transform.tree.push(OpsTransformNodeParent::ops(node, scene));
    actions.transform.create.push(OpsTransformNode::ops(scene, node));

    // let key_group = pi_atom::Atom::from("key_group");
    let id_group = commands.spawn_empty().id();
    // animegroupres.scene_ctxs.create_group(scene).unwrap();
    // animegroupres.global.record_group(source, id_group);
    actions.anime.create.push(OpsAnimationGroupCreation::ops(scene, id_group));
    // actions.anime.attach.push(OpsAnimationGroupAttach::ops(scene, source, id_group));
    {
        let key_curve0 =  pi_atom::Atom::from("test2"); 
        let key_curve0 = key_curve0.asset_u64();
        let curve = FrameCurve::<LocalEulerAngles>::curve_easing(LocalEulerAngles(Vector3::new(0., 0., 0.)), LocalEulerAngles(Vector3::new(0., 0. * 4., 0. * 2.)), (60.) as FrameIndex, 30, EEasingMode::None);
        
        let asset_curve = if let Some(curve) = anime_assets.euler.get(&key_curve0) { curve } else {
            match anime_assets.euler.insert(key_curve0, TypeFrameCurve(curve)) {
                Ok(value) => { value  },
                Err(_) => { return; },
            }
        };

        let animation = anime_contexts.euler.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
        actions.anime.add_target_anime.push(OpsAddTargetAnimation::ops(id_group.clone(), node, animation));
    }
    {
        let key_curve0 =  pi_atom::Atom::from("test0"); 
        let key_curve0 = key_curve0.asset_u64();
        let curve = FrameCurve::<LocalPosition>::curve_easing(LocalPosition(Vector3::new(-10., -10., 0.)), LocalPosition(Vector3::new(0., 20., 0.)), (60.) as FrameIndex, 30, EEasingMode::SineInOut);
        
        let asset_curve = if let Some(curve) = anime_assets.position.get(&key_curve0) { curve } else {
            match anime_assets.position.insert(key_curve0, TypeFrameCurve(curve)) {
                Ok(value) => { value  },
                Err(_) => { return; },
            }
        };

        let animation = anime_contexts.position.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
        actions.anime.add_target_anime.push(OpsAddTargetAnimation::ops(id_group.clone(), node, animation));
    }

    let mut param = AnimationGroupParam::default(); param.fps = 60; param.speed = 1.;param.loop_mode = ELoopMode::PositivePly(None);
    actions.anime.action.push(OpsAnimationGroupAction::Start(id_group, param, 0., pi_animation::base::EFillMode::NONE));
    // engine.start_animation_group(source, &key_group, 1.0, ELoopMode::OppositePly(None), 0., 1., 60, AnimationAmountCalc::default());

    let mut random = pi_wy_rng::WyRng::default();
    for idx in 0..10 {
        // let scalescalar = if idx % 2 == 0 { 1. } else { -1. };

        let source = commands.spawn_empty().id(); actions.transform.tree.push(OpsTransformNodeParent::ops(source, node));
        // if idx == 0 {
        //     actions.mesh.create.push(OpsMeshCreation::ops(scene, source));
        //     actions.material.usemat.push(OpsMaterialUse::ops(source, idmat));
        //     let id_geo = commands.spawn_empty().id();
        //     let instancestate = 0;
        //     actions.geometry.create.push(OpsGeomeryCreate::ops(source, id_geo, CubeBuilder::attrs_meta(), Some(CubeBuilder::indices_meta()), instancestate));
        // } else {
            actions.transform.create.push(OpsTransformNode::ops(scene, source));
        // }
        actions.transform.localsrt.push(OpsTransformNodeLocal::ops(source, ETransformSRT::Translation(random.gen_range(-20.0..20.0), random.gen_range(-20.0..20.0), random.gen_range(-20.0..20.0))));
        actions.transform.localsrt.push(OpsTransformNodeLocal::ops(source, ETransformSRT::Scaling(4., 4., 4.)));
        actions.transform.localsrt.push(OpsTransformNodeLocal::ops(source, ETransformSRT::Euler(3., 0., 0.)));

        let trail = commands.spawn_empty().id();
        actions.trail.create.push(OpsTrail::ops(scene, source, trail));
        actions.trail.age.push(OpsTrailAgeControl::ops(trail, 500));
        actions.material.usemat.push(OpsMaterialUse::ops(trail, idmat, DemoScene::PASS_TRANSPARENT));
        let mut blend = ModelBlend::default(); blend.combine();
        actions.mesh.blend.push(OpsRenderBlend::ops(trail, DemoScene::PASS_TRANSPARENT, blend));
        actions.mesh.depth_state.push(OpsDepthState::ops(trail, DemoScene::PASS_TRANSPARENT, EDepthState::Compare(CompareFunction::Always)));
    }
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
        app.insert_resource(ActionListTestData::default());
    }
}



pub fn main() {
    let mut app = base::test_plugins_with_gltf();
    
    app.add_plugins(PluginTest);
    // app.add_system(Update, base::sys_nodeinfo);

    app.add_system(Update, pi_3d::sys_info_node);
    app.add_system(Update, pi_3d::sys_info_resource);
    app.world.get_resource_mut::<StateRecordCfg>().unwrap().write_state = false;

    app.add_system(Startup, setup.after(base::setup_default_mat));
    
    
    // app.run()
    loop { app.update(); }

}

#[test]
fn test() {
    let key1 = KeyShaderFromAttributes(vec![]);
    let key2 = KeyShaderFromAttributes(vec![]);
    println!("{:?}", key1 == key2);
}