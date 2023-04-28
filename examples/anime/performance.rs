#![feature(box_into_inner)]

use default_render::SingleIDBaseDefaultMaterial;
use pi_3d::PluginBundleDefault;
use pi_animation::{loop_mode::ELoopMode, amount::AnimationAmountCalc};
use pi_atom::Atom;
use pi_bevy_ecs_extend::system_param::layer_dirty::ComponentEvent;
use pi_bevy_render_plugin::PiRenderPlugin;
use pi_curves::{curve::frame_curve::FrameCurve, easing::EEasingMode};
use pi_engine_shell::{prelude::*, frame_time::PluginFrameTime, assets::local_load::PluginLocalLoad};
use pi_scene_context::{prelude::*, materials::uniforms::sys_uniform::{ActionListUniform, EUniformCommand}};
use pi_scene_math::{Vector3, Vector4};
use pi_mesh_builder::{cube::*, ball::*, quad::PluginQuadBuilder};
use unlit_material::{PluginUnlitMaterial, command::*, shader::UnlitShader};


fn setup(
    mut commands: Commands,
    mut scenecmds: ResMut<ActionListSceneCreate>,
    mut cameracmds: (
        ResMut<ActionListCameraCreate>,
        ResMut<ActionListCameraTarget>,
        ResMut<ActionListCameraMode>,
        ResMut<ActionListCameraRenderer>,
        ResMut<ActionListCameraActive>,
        ResMut<ActionListCameraFixedMode>,
        ResMut<ActionListCameraFov>,
        ResMut<ActionListCameraOrthSize>,
        ResMut<ActionListCameraNearFar>,
    ),
    mut transformcmds: (
        ResMut<ActionListTransformNodeParent>,
        ResMut<ActionListTransformNodeLocalPosition>,
        ResMut<ActionListTransformNodeLocalEuler>,
        ResMut<ActionListMeshCreate>,
        ResMut<ActionListInstanceMeshCreate>,
        ResMut<ActionListInstanceTillOff>
    ),
    mut geometrycreate: ResMut<ActionListGeometryCreate>,
    mut matcmds: (
        ResMut<ActionListMaterialUse>,
        ResMut<ActionListMaterialCreate>,
        ResMut<ActionListUniform>,
    ),
    mut fps: ResMut<SingleFrameTimeCommand>,
    mut anime: (
        ResMut<ActionListAnimeGroupCreate>,
        ResMut<ActionListAddTargetAnime>,
        ResMut<ActionListAnimeGroupStart>,
    ),
    mut final_render: ResMut<WindowRenderer>,
    mut scaling_ctx: ResMut<TypeAnimeContext<LocalEulerAngles>>,
    scaling_curves: Res<ShareAssetMgr<TypeFrameCurve<LocalEulerAngles>>>,
) {
    let tes_size = 100;
    fps.frame_ms = 4;

    final_render.cleardepth = 0.0;

    let scene = commands.spawn_empty().id();
    scenecmds.push(scene);

    let camera01 = commands.spawn_empty().id();
    cameracmds.0.push(OpsCameraCreation::ops(scene, camera01, String::from("TestCamera")));
    transformcmds.1.push(OpsTransformNodeLocalPosition(camera01, Vector3::new(0., 0., -10.)));
    cameracmds.4.push(OpsCameraActive::ops(camera01, true));
    cameracmds.7.push(OpsCameraOrthSize::ops(camera01, tes_size as f32));
    // localrulercmds.push(OpsTransformNodeLocalEuler(camera01, Vector3::new(3.1415926 / 4., 0., 0.)));

    let desc = RendererGraphicDesc {
        pre: Some(Atom::from(WindowRenderer::CLEAR_KEY)),
        curr: Atom::from("TestCamera"),
        next: Some(Atom::from(WindowRenderer::KEY)),
        passorders: PassTagOrders::new(vec![EPassTag::Opaque, EPassTag::Water, EPassTag::Sky, EPassTag::Transparent])
    };
    cameracmds.3.push(OpsCameraRendererInit::ops(camera01, desc, wgpu::TextureFormat::Rgba8Unorm, None));

    let source = commands.spawn_empty().id();
    transformcmds.3.push(OpsMeshCreation(scene, source, String::from("TestCube")));
    
    let id_geo = commands.spawn_empty().id();
    let mut attrs = CubeBuilder::attrs_meta();
    attrs.push(VertexBufferDesc::instance_world_matrix());
    attrs.push(VertexBufferDesc::instance_tilloff());
    geometrycreate.push((source, id_geo, attrs, Some(CubeBuilder::indices_meta())));

    let idmat = commands.spawn_empty().id();
    matcmds.0.push(OpsMaterialUse::ops(source, idmat));
    matcmds.1.push(OpsMaterialCreate::ops(idmat, UnlitShader::KEY, EPassTag::Opaque));
    matcmds.2.push(EUniformCommand::Texture(idmat, UniformTextureWithSamplerParam {
        slotname: Atom::from("_MainTex"),
        filter: true,
        sample: KeySampler::default(),
        url: KeyTexture::from("E:/Rust/PI/pi_3d/assets/images/bubbles.png"),
    }, true));
    
    let key_group = pi_atom::Atom::from("key_group");
    anime.0.push(OpsAnimationGroupCreation::ops(source, key_group.clone()));

    let cell_col = 4.;
    let cell_row = 4.;
    for i in 0..tes_size {
        for j in 0..tes_size {
            for k in 0..1 {
                
                let cube: Entity = commands.spawn_empty().id();
                transformcmds.4.push(OpsInstanceMeshCreation::ops(source, cube, String::from("a")));

                let pos = Vector3::new(i as f32 * 2. - (tes_size) as f32, j as f32 * 2. - (tes_size) as f32, k as f32 * 2. - (tes_size) as f32);
                transformcmds.1.push(OpsTransformNodeLocalPosition(cube, pos.clone()));

                transformcmds.5.push(OpsInstanceTillOff::ops(cube, 1.0 / cell_col, 1.0 / cell_row, (i % 4) as f32 / cell_col, (j % 4) as f32 / cell_row));
                
                let key_curve0 = pi_atom::Atom::from((i * tes_size + j).to_string());
                let curve = FrameCurve::<LocalEulerAngles>::curve_easing(LocalEulerAngles(pos), LocalEulerAngles(Vector3::new(10., 10., 10.)), 30, 30, EEasingMode::None);
                
                let asset_curve = if let Some(curve) = scaling_curves.get(&key_curve0) {
                    curve
                } else {
                    match scaling_curves.insert(key_curve0, TypeFrameCurve(curve)) {
                        Ok(value) => {
                            value
                        },
                        Err(_) => {
                            break;
                        },
                    }
                };

                let animation = scaling_ctx.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
                anime.1.push(OpsAddTargetAnimation::ops(source, cube, key_group.clone(), animation));
                // engine.create_target_animation(source, cube, &key_group, animation);
            }
        }
    }

    anime.2.push(OpsAnimationGroupStart::ops(source, key_group.clone(), AnimationGroupParam::default()));
    // engine.start_animation_group(source, &key_group, 1.0, ELoopMode::OppositePly(None), 0., 1., 60, AnimationAmountCalc::default());
}

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
//         PluginUnlitMaterial.init(engine, stages);

//         PluginCubeBuilder.init(engine, stages);

//         Ok(())
//     }
// }

// impl PluginTest {
//     fn setup(
//         engine: &pi_engine_shell::engine_shell::EnginShell,
//     ) {

//         let tes_size = 100;
//         engine.frame_time(4);

//         // Test Code
//         let scene01 = engine.create_scene();
//         let camera01 = engine.create_free_camera(scene01);
//         engine.active_camera(camera01, true);
//         engine.layer_mask(camera01, LayerMask::default());
//         engine.transform_position(camera01, Vector3::new(0., 0., -10.));
//         engine.free_camera_orth_size(camera01, tes_size as f32);
//         engine.camera_renderer(camera01, RendererGraphicDesc { pre: Some(Atom::from("Clear")), curr: Atom::from("MainCamera"), next: None, passorders: PassTagOrders::new(vec![EPassTag::Opaque]) });

//         let unlitmaterial = engine.create_unlit_material(EPassTag::Opaque);
//         engine.set_texture(
//             unlitmaterial,
//             UniformTextureWithSamplerParam {
//                 slotname: Atom::from("_MainTex"),
//                 filter: true,
//                 sample: KeySampler::default(),
//                 url: KeyTexture::from("E:/Rust/PI/pi_3d/assets/images/bubbles.png"),
//             },
//             false
//         );

//         let source = engine.create_mesh(scene01);
//         let mut attrs = CubeBuilder::attrs_meta();
//         attrs.push(VertexBufferDesc::instance_world_matrix());
//         attrs.push(VertexBufferDesc::instance_tilloff());
//         engine.use_geometry(source, attrs, Some(CubeBuilder::indices_meta()));
//         engine.use_material(source, unlitmaterial);
//         engine.layer_mask(source, LayerMask::default());
        
//         let key_group = pi_atom::Atom::from("key_group");
//         engine.create_animation_group(source, &key_group);

//         let cell_col = 4.;
//         let cell_row = 4.;
//         for i in 0..tes_size {
//             for j in 0..tes_size {
//                 for k in 0..1 {
//                     let cube = engine.create_instanced_mesh(scene01, source.clone());
//                     let pos = Vector3::new(i as f32 * 2. - (tes_size) as f32, j as f32 * 2. - (tes_size) as f32, k as f32 * 2. - (tes_size) as f32);
//                     engine.transform_position(cube, pos.clone());
//                     engine.set_instance_tilloff(cube, Vector4::new(1.0 / cell_col, 1.0 / cell_row, (i % 4) as f32 / cell_col, (j % 4) as f32 / cell_row));
                    
//                     let key_curve0 = pi_atom::Atom::from((i * tes_size + j).to_string());
//                     let curve = FrameCurve::<LocalEulerAngles>::curve_easing(LocalEulerAngles(pos), LocalEulerAngles(Vector3::new(10., 10., 10.)), 30, 30, EEasingMode::None);
//                     let asset_curve = if let Some(curve) = engine.check_anim_curve::<LocalEulerAngles>(&key_curve0) {
//                         curve
//                     } else {
//                         engine.creat_anim_curve::<LocalEulerAngles>(&key_curve0, curve)
//                     };
//                     let animation = engine.create_animation::<LocalEulerAngles>(asset_curve);


//                     engine.create_target_animation(source, cube, &key_group, animation);
//                 }
//             }
//         }

//         engine.start_animation_group(source, &key_group, 1.0, ELoopMode::OppositePly(None), 0., 1., 60, AnimationAmountCalc::default());
//     }
// }

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


pub fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

    let mut app = App::default();

	let mut window_plugin = WindowPlugin::default();
    if let Some(primary_window) = &mut window_plugin.primary_window {
        primary_window.resolution.set_physical_resolution(800, 600);
    }

    app.add_plugin(InputPlugin::default());
    app.add_plugin(window_plugin);
    app.add_plugin(AccessibilityPlugin);
    app.add_plugin(bevy::winit::WinitPlugin::default());
    // .add_plugin(WorldInspectorPlugin::new())
    app.add_plugin(PiRenderPlugin::default());
    app.add_plugin(PluginLocalLoad);
    app.add_plugin(PluginTest);
    app.add_plugin(PluginFrameTime);
    app.add_plugin(PluginWindowRender);
    app.add_plugin(PluginCubeBuilder);
    app.add_plugin(PluginQuadBuilder);
    app.add_plugin(PluginStateToFile);
    app.add_plugins(PluginBundleDefault);
    app.add_plugin(PluginUnlitMaterial);
    
    app.add_startup_system(setup);
    // bevy_mod_debugdump::print_main_schedule(&mut app);
    
    app.run()

}