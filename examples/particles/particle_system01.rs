

use axis::{PluginAxis, axis::AxisBuilder};
use pi_bevy_ecs_extend::system_param::layer_dirty::ComponentEvent;
use pi_curves::{curve::frame_curve::FrameCurve, easing::EEasingMode};
use pi_engine_shell::prelude::*;
use pi_gltf2_load::{TypeAnimeContexts, TypeAnimeAssetMgrs};
use pi_node_materials::prelude::BlockMainTexture;
use pi_scene_context::prelude::*;
use pi_scene_math::*;
use pi_mesh_builder::cube::*;
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
    mut frame: ResMut<SingleFrameTimeCommand>,
) {
    let tes_size = 20;
    // frame.frame_ms = 200;

    final_render.cleardepth = 0.0;

    let scene = commands.spawn_empty().id();
    animegroupcmd.scene_ctxs.init_scene(scene);
    scenecmds.create.push(OpsSceneCreation::ops(scene, ScenePassRenderCfg::default()));

    let node = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(node, scene));
    transformcmds.create.push(OpsTransformNode::ops(scene, node, String::from("A")));

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

    let idmattrail = commands.spawn_empty().id();
    matcmds.create.push(OpsMaterialCreate::ops(idmattrail, UnlitShader::KEY, EPassTag::Transparent));
    matcmds.texture.push(OpsUniformTexture::ops(idmattrail, UniformTextureWithSamplerParam {
        slotname: Atom::from(BlockMainTexture::KEY_TEX),
        filter: true,
        sample: KeySampler::linear_repeat(),
        url: EKeyTexture::from("E:/Rust/PI/pi_3d/assets/images/eff_daoguang_lf_004.png"),
    }));

    let temp = 1;
    for i in 0..temp {
        for j in 0..temp {
            for k in 0..temp {
                let item = {
                    let source = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(source, node));
                    meshcmds.create.push(OpsMeshCreation::ops(scene, source, String::from("TestCube")));
                    let id_geo = commands.spawn_empty().id();
                    let mut attrs = CubeBuilder::attrs_meta();
                    // ParticleSystem Add
                    attrs.push(VertexBufferDesc::instance_world_matrix());
                    attrs.push(VertexBufferDesc::instance_color());
                    attrs.push(VertexBufferDesc::instance_tilloff());
                    geometrycmd.create.push(OpsGeomeryCreate::ops(source, id_geo, attrs, Some(CubeBuilder::indices_meta())));
                    //
                    let syskey = String::from("Test");
                    let syscfg = demo_cfg(10., 5.);
                    let calculator = commands.spawn_empty().id();
                    particlesys_cmds.calculator_cmds.push(OpsCPUParticleCalculator::ops(calculator, syscfg));
                    let particle_sys_calculator = ParticleSystemCalculatorID(calculator, 1024, particlesys_cmds.calculator_queue.queue());
                    let calculator = particlesys_cmds.calcultors.insert(syskey.asset_u64(), particle_sys_calculator).unwrap();
                    let trailmesh = commands.spawn_empty().id();
                    let trailgeo = commands.spawn_empty().id();
                    particlesys_cmds.particlesys_cmds.push(OpsCPUParticleSystem::ops(scene, source, trailmesh, trailgeo, calculator));
                    particlesys_cmds.particlesys_state_cmds.push(OpsCPUParticleSystemState::ops_start(source));
                    //
                    let idmat = commands.spawn_empty().id();
                    matcmds.usemat.push(OpsMaterialUse::ops(source, idmat));
                    matcmds.create.push(OpsMaterialCreate::ops(idmat, UnlitShader::KEY, EPassTag::Opaque));
                    particlesys_cmds.trail_material.push(OpsCPUParticleSystemTrailMaterial::ops(source, idmattrail));
                    source
                };

                transformcmds.localpos.push(OpsTransformNodeLocalPosition::ops(item, (i - temp / 2) as f32 * 0., (j - temp / 2) as f32 * 0., (k - temp / 2) as f32 * 0.));
                // transformcmds.localrot.push(OpsTransformNodeLocalEuler::ops(item, (i - temp) as f32, (j - temp) as f32, (k - temp) as f32));
                // transformcmds.localscl.push(OpsTransformNodeLocalScaling::ops(item, 0.2, 0.2, 0.2));
            }
        }
    }

    // matcmds.texture.push(OpsUniformTexture::ops(idmat, UniformTextureWithSamplerParam {
    //     slotname: Atom::from("_MainTex"),
    //     filter: true,
    //     sample: KeySampler::default(),
    //     url: EKeyTexture::from("E:/Rust/PI/pi_3d/assets/images/bubbles.png"),
    // }));

    
    let key_group = pi_atom::Atom::from("key_group");
    let id_group = animegroupcmd.scene_ctxs.create_group(scene).unwrap();
    animegroupcmd.global.record_group(node, id_group);
    animegroupcmd.attach.push(OpsAnimationGroupAttach::ops(scene, node, id_group));
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
        animegroupcmd.scene_ctxs.add_target_anime(scene, node, id_group.clone(), animation);
    }

    let mut param = AnimationGroupParam::default(); param.fps = 60; param.speed = 2.;
    animegroupcmd.scene_ctxs.start_with_progress(scene, id_group.clone(), param, 0., pi_animation::base::EFillMode::NONE);
    // engine.start_animation_group(source, &key_group, 1.0, ELoopMode::OppositePly(None), 0., 1., 60, AnimationAmountCalc::default());
}

fn demo_cfg(count: f32, speed: f32) -> IParticleSystemConfig {
    let mut cfg = IParticleSystemConfig::new();

    cfg.name = String::from("Test");
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
    cfg.trail = Some(ITrail {
        ratio: 1.,
        mode: ETrailMode::Particles,
        lifetime: OneParamInfo::TInterpolateConstant(1.),
        ribbon_count: 5.,
        attach_rtt: 10,
        min_dist: 0.5,
        world_space: 0,
        die_with: 1,
        tex_mode: ETrailTextureMode::Stretch,
        size_awidth: 1,
        size_alifetime: 1,
        inherit_color: 1,
        color_over_life: FourGradientInfo::TInterpolateColor([1., 1., 1., 1.]),
        width_over_trail: OneParamInfo::TInterpolateConstant(1.),
        color_over_trail: FourGradientInfo::TInterpolateColor([1., 1., 1., 1.]),
        material: 0.,
    });

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
    app.add_plugin(PluginAxis);
    // app.add_system(base::sys_nodeinfo);

    app.world.get_resource_mut::<StateRecordCfg>().unwrap().write_state = false;

    app.add_startup_system(setup);
    // bevy_mod_debugdump::print_main_schedule(&mut app);
    
    app.run()

}