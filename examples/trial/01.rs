

use pi_animation::loop_mode::ELoopMode;
use pi_curves::{curve::frame_curve::FrameCurve, easing::EEasingMode};
use pi_engine_shell::prelude::*;
use pi_gltf2_load::{TypeAnimeContexts, TypeAnimeAssetMgrs};
use pi_node_materials::prelude::BlockMainTexture;
use pi_scene_context::prelude::*;
use pi_scene_math::*;
use pi_mesh_builder::cube::*;
use pi_trail_renderer::{ActionSetTrailRenderer, OpsTrail, OpsTrailAgeControl};
use rand::Rng;
use unlit_material::*;
use pi_particle_system::prelude::*;

fn setup(
    mut commands: Commands,
    mut scenecmds: ActionSetScene,
    mut cameracmds: ActionSetCamera,
    mut transformcmds: ActionSetTransform,
    mut meshcmds: ActionSetMesh,
    mut geometrycmd: ActionSetGeometry,
    mut matcmds: ActionSetMaterial,
    mut final_render: ResMut<WindowRenderer>,
    mut renderercmds: ActionSetRenderer,
    mut animegroupcmd: ActionSetAnimationGroup,
    anime_assets: TypeAnimeAssetMgrs,
    mut anime_contexts: TypeAnimeContexts,
    mut trailcmds: ActionSetTrailRenderer,
) {
    let tes_size = 50;
    let (scene, camera01) = base::DemoScene::new(&mut commands, &mut scenecmds, &mut cameracmds, &mut transformcmds, &mut animegroupcmd, &mut final_render, &mut renderercmds, tes_size as f32, 0.7, (0., 10., -50.), true);
    cameracmds.size.push(OpsCameraOrthSize::ops(camera01, tes_size as f32));

    let idmat = commands.spawn_empty().id();
    matcmds.create.push(OpsMaterialCreate::ops(idmat, UnlitShader::KEY, EPassTag::Transparent));
    matcmds.texture.push(OpsUniformTexture::ops(idmat, UniformTextureWithSamplerParam {
        slotname: Atom::from(BlockMainTexture::KEY_TEX),
        filter: true,
        sample: KeySampler::linear_repeat(),
        url: EKeyTexture::from("E:/Rust/PI/pi_3d/assets/images/eff_daoguang_lf_004.png"),
    }));

    let source = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(source, scene));
    let instancestate = 0;
    meshcmds.create.push(OpsMeshCreation::ops(scene, source, MeshInstanceState { state: instancestate, use_single_instancebuffer: false }));
    transformcmds.localpos.push(OpsTransformNodeLocalPosition::ops(source, 0., 10., 0.));
    matcmds.usemat.push(OpsMaterialUse::ops(source, idmat));
    let id_geo = commands.spawn_empty().id();
    geometrycmd.create.push(OpsGeomeryCreate::ops(source, id_geo, CubeBuilder::attrs_meta(), Some(CubeBuilder::indices_meta())));
    
    let node = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(node, scene));
    transformcmds.create.push(OpsTransformNode::ops(scene, node));
    // let key_group = pi_atom::Atom::from("key_group");
    let id_group = animegroupcmd.scene_ctxs.create_group(scene).unwrap();
    animegroupcmd.global.record_group(node, id_group);
    animegroupcmd.attach.push(OpsAnimationGroupAttach::ops(scene, node, id_group));
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
        animegroupcmd.scene_ctxs.add_target_anime(scene, node, id_group.clone(), animation);
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
        animegroupcmd.scene_ctxs.add_target_anime(scene, node, id_group.clone(), animation);
    }

    let mut param = AnimationGroupParam::default(); param.fps = 60; param.speed = 1.;param.loop_mode = ELoopMode::PositivePly(None);
    animegroupcmd.scene_ctxs.start_with_progress(scene, id_group.clone(), param, 0., pi_animation::base::EFillMode::NONE);
    // engine.start_animation_group(source, &key_group, 1.0, ELoopMode::OppositePly(None), 0., 1., 60, AnimationAmountCalc::default());

    let mut random = pi_wy_rng::WyRng::default();
    for idx in 0..200 {
        // let scalescalar = if idx % 2 == 0 { 1. } else { -1. };

        let source = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(source, node));
        // if idx == 0 {
        //     meshcmds.create.push(OpsMeshCreation::ops(scene, source));
        //     matcmds.usemat.push(OpsMaterialUse::ops(source, idmat));
        //     let id_geo = commands.spawn_empty().id();
        //     let instancestate = 0;
        //     geometrycmd.create.push(OpsGeomeryCreate::ops(source, id_geo, CubeBuilder::attrs_meta(), Some(CubeBuilder::indices_meta()), instancestate));
        // } else {
            transformcmds.create.push(OpsTransformNode::ops(scene, source));
        // }
        transformcmds.localpos.push(OpsTransformNodeLocalPosition::ops(source, random.gen_range(-20.0..20.0), random.gen_range(-20.0..20.0), random.gen_range(-20.0..20.0)));
        transformcmds.localscl.push(OpsTransformNodeLocalScaling::ops(source, 4., 4., 4.));
        transformcmds.localrot.push(OpsTransformNodeLocalEuler::ops(source, 3., 0., 0.));

        let trail = commands.spawn_empty().id();
        trailcmds.create.push(OpsTrail::ops(scene, source, idmat, trail));
        trailcmds.age.push(OpsTrailAgeControl::ops(trail, 500));
        let mut blend = ModelBlend::default(); blend.combine();
        meshcmds.blend.push(OpsRenderBlend::ops(trail, blend));
        meshcmds.depth_compare.push(OpsDepthCompare::ops(trail, CompareFunction::Always));
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



#[path = "../base.rs"]
mod base;
pub fn main() {
    let mut app = base::test_plugins_with_gltf();
    
    app.add_plugins(PluginTest);
    // app.add_systems(Update, base::sys_nodeinfo);

    app.add_systems(Update, pi_3d::sys_info_node);
    app.add_systems(Update, pi_3d::sys_info_resource);
    app.world.get_resource_mut::<StateRecordCfg>().unwrap().write_state = false;

    app.add_systems(Startup, setup);
    // bevy_mod_debugdump::print_main_schedule(&mut app);
    
    app.run()

}

#[test]
fn test() {
    let key1 = KeyShaderFromAttributes(vec![]);
    let key2 = KeyShaderFromAttributes(vec![]);
    println!("{:?}", key1 == key2);
}