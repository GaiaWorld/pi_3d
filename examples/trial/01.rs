

use pi_animation::loop_mode::ELoopMode;
use pi_bevy_ecs_extend::system_param::layer_dirty::ComponentEvent;
use pi_curves::{curve::frame_curve::FrameCurve, easing::EEasingMode};
use pi_engine_shell::prelude::*;
use pi_gltf2_load::{TypeAnimeContexts, TypeAnimeAssetMgrs};
use pi_node_materials::prelude::BlockMainTexture;
use pi_scene_context::prelude::*;
use pi_scene_math::*;
use pi_mesh_builder::cube::*;
use pi_trail_renderer::{ActionSetTrailRenderer, OpsTrail, OpsTrailAgeControl};
use rand::Rng;
use unlit_material::shader::UnlitShader;
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
    mut particlesys_cmds: ParticleSystemActionSet,
    mut animegroupcmd: ActionSetAnimationGroup,
    anime_assets: TypeAnimeAssetMgrs,
    mut anime_contexts: TypeAnimeContexts,
    mut trailcmds: ActionSetTrailRenderer,
) {
    let tes_size = 50;

    final_render.cleardepth = 0.0;

    let scene = commands.spawn_empty().id();
    animegroupcmd.scene_ctxs.init_scene(scene);
    scenecmds.create.push(OpsSceneCreation::ops(scene, ScenePassRenderCfg::default()));

    let camera01 = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(camera01, scene));
    cameracmds.create.push(OpsCameraCreation::ops(scene, camera01, String::from("TestCamera"), true));
    transformcmds.localpos.push(OpsTransformNodeLocalPosition::ops(camera01, 0., 0., -50.));
    cameracmds.active.push(OpsCameraActive::ops(camera01, true));
    cameracmds.size.push(OpsCameraOrthSize::ops(camera01, tes_size as f32));
    // localrulercmds.push(OpsTransformNodeLocalEuler(camera01, Vector3::new(3.1415926 / 4., 0., 0.)));

    let desc = RendererGraphicDesc {
        pre: Some(final_render.clear_entity),
        curr: String::from("TestCamera"),
        next: Some(final_render.render_entity),
        passorders: PassTagOrders::new(vec![EPassTag::Opaque, EPassTag::Water, EPassTag::Sky, EPassTag::Transparent])
    };
    let id_renderer = commands.spawn_empty().id(); renderercmds.create.push(OpsRendererCreate::ops(id_renderer, desc.curr.clone()));
    renderercmds.connect.push(OpsRendererConnect::ops(final_render.clear_entity, id_renderer));
    renderercmds.connect.push(OpsRendererConnect::ops(id_renderer, final_render.render_entity));
    cameracmds.render.push(OpsCameraRendererInit::ops(camera01, id_renderer, desc.curr, desc.passorders, ColorFormat::Rgba8Unorm, DepthStencilFormat::None));

    let idmat = commands.spawn_empty().id();
    matcmds.create.push(OpsMaterialCreate::ops(idmat, UnlitShader::KEY, EPassTag::Transparent));
    matcmds.texture.push(OpsUniformTexture::ops(idmat, UniformTextureWithSamplerParam {
        slotname: Atom::from(BlockMainTexture::KEY_TEX),
        filter: true,
        sample: KeySampler::linear_repeat(),
        url: EKeyTexture::from("E:/Rust/PI/pi_3d/assets/images/eff_daoguang_lf_004.png"),
    }));

    let source = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(source, scene));
    meshcmds.create.push(OpsMeshCreation::ops(scene, source, String::from("TestCube")));
    transformcmds.localpos.push(OpsTransformNodeLocalPosition::ops(source, 0., 10., 0.));
    matcmds.usemat.push(OpsMaterialUse::ops(source, idmat));
    let id_geo = commands.spawn_empty().id();
    geometrycmd.create.push(OpsGeomeryCreate::ops(source, id_geo, CubeBuilder::attrs_meta(), Some(CubeBuilder::indices_meta())));
    
    let node = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(node, scene));
    transformcmds.create.push(OpsTransformNode::ops(scene, node, String::from("A")));
    let key_group = pi_atom::Atom::from("key_group");
    let id_group = animegroupcmd.scene_ctxs.create_group(scene).unwrap();
    animegroupcmd.global.record_group(node, id_group);
    animegroupcmd.attach.push(OpsAnimationGroupAttach::ops(scene, node, id_group));
    {
        let key_curve0 =  pi_atom::Atom::from("test2"); 
        let key_curve0 = key_curve0.asset_u64();
        let curve = FrameCurve::<LocalEulerAngles>::curve_easing(LocalEulerAngles(Vector3::new(0., 0., 1.)), LocalEulerAngles(Vector3::new(0., 3.1415926 * 4., 3.1415926 * 2.)), (60.) as FrameIndex, 30, EEasingMode::None);
        
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
        let curve = FrameCurve::<LocalPosition>::curve_easing(LocalPosition(Vector3::new(-10., -10., 0.)), LocalPosition(Vector3::new(20., 20., 0.)), (60.) as FrameIndex, 30, EEasingMode::SineInOut);
        
        let asset_curve = if let Some(curve) = anime_assets.position.get(&key_curve0) { curve } else {
            match anime_assets.position.insert(key_curve0, TypeFrameCurve(curve)) {
                Ok(value) => { value  },
                Err(_) => { return; },
            }
        };

        let animation = anime_contexts.position.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
        animegroupcmd.scene_ctxs.add_target_anime(scene, node, id_group.clone(), animation);
    }

    let mut param = AnimationGroupParam::default(); param.fps = 60; param.speed = 0.2;param.loop_mode = ELoopMode::PositivePly(None);
    animegroupcmd.scene_ctxs.start_with_progress(scene, id_group.clone(), param, 0., pi_animation::base::EFillMode::NONE);
    // engine.start_animation_group(source, &key_group, 1.0, ELoopMode::OppositePly(None), 0., 1., 60, AnimationAmountCalc::default());

    let mut random = pi_wy_rng::WyRng::default();
    for idx in 0..6000 {
        let scalescalar = if idx % 2 == 0 { 1. } else { -1. };

        let source = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(source, node));
        if idx == 0 {
            meshcmds.create.push(OpsMeshCreation::ops(scene, source, String::from("TestCube")));
            matcmds.usemat.push(OpsMaterialUse::ops(source, idmat));
            let id_geo = commands.spawn_empty().id();
            geometrycmd.create.push(OpsGeomeryCreate::ops(source, id_geo, CubeBuilder::attrs_meta(), Some(CubeBuilder::indices_meta())));
        } else {
            transformcmds.create.push(OpsTransformNode::ops(scene, source, String::from("TestCube")));
        }
        transformcmds.localpos.push(OpsTransformNodeLocalPosition::ops(source, random.gen_range(-20.0..20.0), random.gen_range(-20.0..20.0), random.gen_range(-20.0..20.0)));

        let trail = commands.spawn_empty().id();
        trailcmds.create.push(OpsTrail::ops(scene, source, idmat, trail));
        trailcmds.age.push(OpsTrailAgeControl::ops(trail, 200));
        let mut blend = ModelBlend::default(); blend.combine();
        meshcmds.blend.push(OpsRenderBlend::ops(trail, blend));
        meshcmds.depth_compare.push(OpsDepthCompare::ops(trail, CompareFunction::Always));
    }
}

fn demo_cfg(count: f32, speed: f32) -> IParticleSystemConfig {
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

pub trait AddEvent {
	// 添加事件， 该实现每帧清理一次
	fn add_frame_event<T: Event>(&mut self) -> &mut Self;
}

impl AddEvent for App {
	fn add_frame_event<T: Event>(&mut self) -> &mut Self {
		if !self.world.contains_resource::<Events<T>>() {
			self.init_resource::<Events<T>>()
				.add_system(Events::<T>::update_system);
		}
		self
	}
}

pub type ActionListTestData = ActionList<(ObjectID, f32, f32, f32)>;

pub struct PluginTest;
impl Plugin for PluginTest {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActionListTestData::default());
        app.add_frame_event::<ComponentEvent<Changed<Layer>>>();
    }
}



#[path = "../base.rs"]
mod base;
pub fn main() {
    let mut app = base::test_plugins_with_gltf();
    
    app.add_plugin(PluginTest);
    // app.add_system(base::sys_nodeinfo);

    app.world.get_resource_mut::<StateRecordCfg>().unwrap().write_state = false;

    app.add_startup_system(setup);
    // bevy_mod_debugdump::print_main_schedule(&mut app);
    
    app.run()

}

#[test]
fn test() {
    let key1 = KeyShaderFromAttributes(vec![]);
    let key2 = KeyShaderFromAttributes(vec![]);
    println!("{:?}", key1 == key2);
}