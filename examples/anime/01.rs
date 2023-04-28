#![feature(box_into_inner)]


use default_render::SingleIDBaseDefaultMaterial;
use pi_3d::PluginBundleDefault;
use pi_animation::{loop_mode::ELoopMode, amount::AnimationAmountCalc, curve_frame_event::CurveFrameEvent, animation_listener::{AnimationListener, EAnimationEventResult}};
use pi_atom::Atom;
use pi_bevy_ecs_extend::{prelude::Layer, system_param::layer_dirty::ComponentEvent};
use pi_bevy_render_plugin::{PiRenderPlugin, PiRenderSystemSet};
use pi_curves::{curve::frame_curve::FrameCurve, easing::EEasingMode};
use pi_engine_shell::{prelude::*, frame_time::{SingleFrameTimeCommand, PluginFrameTime}};

use pi_scene_context::{plugin::Plugin, object::ObjectID,
    transforms::{command::*, transform_node::LocalScaling},
    scene::{command::{ActionListSceneCreate, ActionScene}},
    cameras::{command::*, camera::EFreeCameraMode},
    layer_mask::{interface::*, LayerMask},
    renderers::graphic::RendererGraphicDesc,
    pass::{EPassTag, PassTagOrders},
    materials::{command::*},
    meshes::command::*,
    geometry::command::*,
    state::PluginStateToFile, animation::{command::{ActionAnime, ActionListAnimeGroupCreate, ActionListAddTargetAnime, OpsAddTargetAnimation, OpsAnimationGroupCreation, ActionListAnimeGroupStart, OpsAnimationGroupStart, AnimationGroupParam}, base::{TypeFrameCurve, TypeAnimeContext, AssetTypeFrameCurve}}
};
use pi_scene_math::{Vector3, Vector4};
use pi_mesh_builder::{cube::*, ball::*, quad::*};
use unlit_material::PluginUnlitMaterial;


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
    ),
    mut geometrycreate: ResMut<ActionListGeometryCreate>,
    mut matuse: ResMut<ActionListMaterialUse>,
    mut fps: ResMut<SingleFrameTimeCommand>,
    mut scaling_ctx: ResMut<TypeAnimeContext<LocalScaling>>,
    mut anime: (
        ResMut<ActionListAnimeGroupCreate>,
        ResMut<ActionListAddTargetAnime>,
        ResMut<ActionListAnimeGroupStart>,
    ),
    mut final_render: ResMut<WindowRenderer>,
    scaling_curves: Res<ShareAssetMgr<TypeFrameCurve<LocalScaling>>>,
    defaultmat: Res<SingleIDBaseDefaultMaterial>,
) {
    let tes_size = 20;
    fps.frame_ms = 100;

    final_render.cleardepth = 0.0;

    let scene = commands.spawn_empty().id();
    scenecmds.push(scene);

    let camera01 = commands.spawn_empty().id();
    cameracmds.0.push(OpsCameraCreation::ops(scene, camera01, String::from("TestCamera")));
    transformcmds.1.push(OpsTransformNodeLocalPosition(camera01, Vector3::new(0., 10., -40.)));
    cameracmds.1.push(OpsCameraTarget::ops(camera01, Vector3::new(0., -1., 4.)));
    cameracmds.2.push(OpsCameraMode::ops(camera01, false));
    cameracmds.4.push(OpsCameraActive::ops(camera01, true));
    cameracmds.7.push(OpsCameraOrthSize::ops(camera01, 4.));
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
    geometrycreate.push((source, id_geo, attrs, Some(CubeBuilder::indices_meta())));

    matuse.push(OpsMaterialUse::ops(source, defaultmat.0.unwrap()));
    
    let key_group = pi_atom::Atom::from("key_group");
    anime.0.push(OpsAnimationGroupCreation::ops(source, key_group.clone()));

    let cell_col = 4.;
    let cell_row = 4.;
    for i in 0..tes_size {
        for j in 0..tes_size {
            for k in 0..1 {
                
                let cube: Entity = commands.spawn_empty().id();
                transformcmds.4.push(OpsInstanceMeshCreation::ops(source, cube, String::from("a")));
                transformcmds.0.push(OpsTransformNodeParent::ops(cube, scene));

                let pos = Vector3::new(i as f32 * 2. - (tes_size) as f32, 0., j as f32 * 2. - (tes_size) as f32);
                transformcmds.1.push(OpsTransformNodeLocalPosition(cube, pos));
                
                let key_curve0 = pi_atom::Atom::from((i * tes_size + j).to_string());
                let curve = FrameCurve::<LocalScaling>::curve_easing(LocalScaling(Vector3::new(1., 1., 1.)), LocalScaling(Vector3::new(0., 2. * (1.1 + (i as f32).sin()), 0.)), (60. * (1.1 + ((i * j) as f32).cos())) as u16, 30, EEasingMode::None);
                
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
    // fn setup(
    //     engine: &pi_engine_shell::engine_shell::EnginShell,
    // ) {

    //     let tes_size = 20;
    //     engine.frame_time(100);

    //     // Test Code
    //     let scene01 = engine.create_scene();

    //     let root = engine.create_transform_node(scene01);

    //     let camera01 = engine.create_free_camera(scene01);
    //     engine.free_camera_mode(camera01, EFreeCameraMode::Perspective);
    //     engine.active_camera(camera01, true);
    //     engine.layer_mask(camera01, LayerMask::default());
    //     engine.transform_position(camera01, Vector3::new(0., 10., -40.));
    //     engine.camera_renderer(camera01, RendererGraphicDesc { pre: Some(Atom::from("Clear")), curr: Atom::from("MainCamera"), next: None, passorders: PassTagOrders::new(vec![EPassTag::Opaque]) });
    //     // engine.transform_parent(camera01, root);
    //     engine.camera_target(camera01, Vector3::new(0., -1., 4.));

    //     let source = engine.create_mesh(scene01);
    //     let mut attrs = CubeBuilder::attrs_meta();
    //     attrs.push(VertexBufferDesc::instance_world_matrix());
    //     engine.use_geometry(source, attrs, Some(CubeBuilder::indices_meta()));
    //     engine.use_default_material(source);
    //     engine.layer_mask(source, LayerMask::default());
    //     engine.depth_stencil(source, ModelDepthStencil::new(
    //         true,
    //         wgpu::CompareFunction::GreaterEqual,
    //         DepthBiasState {
    //             constant: 1,
    //             slope_scale: 1,
    //             clamp: 1,
    //         },
    //         wgpu::StencilState {
    //             front: wgpu::StencilFaceState::IGNORE,
    //             back: wgpu::StencilFaceState::IGNORE,
    //             read_mask: 0,
    //             write_mask: 0,
    //         },
    //     ));

    //     let key_group = pi_atom::Atom::from("key_group");
    //     engine.create_animation_group(source, &key_group);

    //     let cell_col = 4.;
    //     let cell_row = 4.;
    //     for i in 0..tes_size {
    //         for j in 0..tes_size {
    //             for k in 0..1 {
    //                 let cube = engine.create_instanced_mesh(scene01, source.clone());
    //                 let pos = Vector3::new(i as f32 * 2. - (tes_size) as f32, 0., j as f32 * 2. - (tes_size) as f32);
    //                 engine.transform_position(cube, pos.clone());
                    
    //                 let key_curve0 = pi_atom::Atom::from((i * tes_size + j).to_string());
    //                 let curve = FrameCurve::<LocalScaling>::curve_easing(LocalScaling(Vector3::new(1., 1., 1.)), LocalScaling(Vector3::new(0., 2. * (1.1 + (i as f32).sin()), 0.)), (60. * (1.1 + ((i * j) as f32).cos())) as u16, 30, EEasingMode::None);
    //                 let asset_curve = if let Some(curve) = engine.check_anim_curve::<LocalScaling>(&key_curve0) {
    //                     curve
    //                 } else {
    //                     engine.creat_anim_curve::<LocalScaling>(&key_curve0, curve)
    //                 };
    //                 let animation = engine.create_animation::<LocalScaling>(asset_curve);


    //                 engine.create_target_animation(source, cube, &key_group, animation);
    //             }
    //         }
    //     }

    //     engine.start_animation_group(source, &key_group, 1.0, ELoopMode::OppositePly(None), 0., 1., 60, AnimationAmountCalc::default());
            
    //     // 创建帧事件
    // }
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
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

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
    app.add_plugin(PluginTest);
    app.add_plugin(PluginFrameTime);
    app.add_plugin(PluginWindowRender);
    app.add_plugin(PluginCubeBuilder);
    app.add_plugin(PluginQuadBuilder);
    app.add_plugin(PluginStateToFile);
    app.add_plugins(PluginBundleDefault);
    
    app.add_startup_system(setup);
    // bevy_mod_debugdump::print_main_schedule(&mut app);
    
    app.run()

}