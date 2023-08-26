

use pi_3d::PluginBundleDefault;
use pi_3d_state::StateGlobal;
use pi_bevy_ecs_extend::system_param::layer_dirty::ComponentEvent;
use pi_bevy_render_plugin::PiRenderPlugin;
use pi_engine_shell::{prelude::*, frame_time::PluginFrameTime};
use pi_node_materials::prelude::*;
use pi_particle_system::PluginParticleSystem;
use pi_scene_context::prelude::*;
use pi_mesh_builder::{cube::*, quad::PluginQuadBuilder};
use unlit_material::*;

use std::sync::Arc;
use pi_async_rt::rt::AsyncRuntime;
use pi_hal::{init_load_cb, runtime::MULTI_MEDIA_RUNTIME, on_load};

pub struct PluginLocalLoad;
impl Plugin for PluginLocalLoad {
    fn build(&self, _: &mut App) {
        init_load_cb(Arc::new(|path: String| {
            MULTI_MEDIA_RUNTIME
                .spawn(async move {
                    log::debug!("Load {}", path);
                    let r = std::fs::read(path.clone()).unwrap();
                    on_load(&path, r);
                })
                .unwrap();
        }));
    }
}

pub fn main() {
    
}

pub struct DemoScene;
impl DemoScene {
    pub fn new(
        commands: &mut Commands,
        scenecmds: &mut ActionSetScene,
        cameracmds: &mut ActionSetCamera,
        transformcmds: &mut ActionSetTransform,
        animegroupcmd: &mut ActionSetAnimationGroup,
        final_render: &mut WindowRenderer,
        renderercmds: &mut ActionSetRenderer,
        camera_size: f32,
        camera_fov: f32,
        camera_position: (f32, f32, f32),
        orthographic_camera: bool
    ) -> (Entity, Entity) {
        final_render.cleardepth = 0.0;

        let scene = commands.spawn_empty().id();
        animegroupcmd.scene_ctxs.init_scene(scene);
        scenecmds.create.push(OpsSceneCreation::ops(scene, ScenePassRenderCfg::default(), SceneBoundingPool::MODE_LIST, [0, 0, 0, 0,0 ,0 ,0 ,0 ,0]));

        let camera01 = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(camera01, scene));
        cameracmds.create.push(OpsCameraCreation::ops(scene, camera01, true));
        transformcmds.localpos.push(OpsTransformNodeLocalPosition::ops(camera01, camera_position.0, camera_position.1, camera_position.2));
        cameracmds.mode.push(OpsCameraMode::ops(camera01, orthographic_camera));
        cameracmds.active.push(OpsCameraActive::ops(camera01, true));
        cameracmds.size.push(OpsCameraOrthSize::ops(camera01, camera_size));
        cameracmds.fov.push(OpsCameraFov::ops(camera01, camera_fov));
        
        let desc = RendererGraphicDesc {
            pre: Some(final_render.clear_entity),
            curr: String::from("TestCamera"),
            next: Some(final_render.render_entity),
            passorders: PassTagOrders::new(vec![EPassTag::Opaque, EPassTag::Water, EPassTag::Sky, EPassTag::Transparent])
        };
        let id_renderer = commands.spawn_empty().id(); renderercmds.create.push(OpsRendererCreate::ops(id_renderer, desc.curr.clone()));
        renderercmds.modify.push(OpsRendererCommand::AutoClearColor(id_renderer, true));
        renderercmds.modify.push(OpsRendererCommand::AutoClearDepth(id_renderer, true));
        renderercmds.modify.push(OpsRendererCommand::AutoClearStencil(id_renderer, true));
        renderercmds.modify.push(OpsRendererCommand::DepthClear(id_renderer, RenderDepthClear(0.)));
        renderercmds.modify.push(OpsRendererCommand::ColorClear(id_renderer, RenderColorClear(0, 0, 0, 0)));
        renderercmds.connect.push(OpsRendererConnect::ops(final_render.clear_entity, id_renderer));
        renderercmds.connect.push(OpsRendererConnect::ops(id_renderer, final_render.render_entity));
        cameracmds.render.push(OpsCameraRendererInit::ops(camera01, id_renderer, desc.curr, desc.passorders, ColorFormat::Rgba8Unorm, DepthStencilFormat::None));

        (scene, camera01)
    }
}

pub fn sys_scene_time_from_frame(
    mut scenes: Query<&mut SceneTime>,
    frame: Res<SingleFrameTimeCommand>,
) {
    scenes.iter_mut().for_each(|mut comp| {
        let time = comp.time_ms + frame.delta_ms();
        // log::warn!("Time: {:?}, Delta MS: {:?}", time, frame.delta_ms());
        comp.reset(time);
    });
}

pub struct PluginSceneTimeFromPluginFrame;
impl Plugin for PluginSceneTimeFromPluginFrame {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            sys_scene_time_from_frame.after(pi_engine_shell::frame_time::sys_frame_time).in_set(ERunStageChap::Initial)
        );
    }
}

pub trait AddEvent {
	// 添加事件， 该实现每帧清理一次
	fn add_frame_event<T: Event>(&mut self) -> &mut Self;
}

impl AddEvent for App {
	fn add_frame_event<T: Event>(&mut self) -> &mut Self {
		if !self.world.contains_resource::<Events<T>>() {
			self.init_resource::<Events<T>>()
				.add_systems(Update, Events::<T>::update_system);
		}
		self
	}
}

pub fn test_plugins() -> App {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

    let mut app = App::default();

	let mut window_plugin = WindowPlugin::default();
    if let Some(primary_window) = &mut window_plugin.primary_window {
        primary_window.resolution.set_physical_resolution(800, 600);
    }

    app.insert_resource(AssetMgrConfigs::default());
    app.add_plugins(
        (
            InputPlugin::default(),
            window_plugin,
            AccessibilityPlugin,
            bevy::winit::WinitPlugin::default(),
            pi_bevy_asset::PiAssetPlugin::default(),
            PiRenderPlugin::default(),
            PluginLocalLoad,
            PluginFrameTime,
            PluginWindowRender,
        )
    );
            
    app.add_plugins(PluginBundleDefault);
    app.add_plugins(
        (
            PluginCubeBuilder,
            PluginQuadBuilder,
            PluginStateToFile,
            PluginNodeMaterial,
            PluginUnlitMaterial,
        )
    );
    app.add_plugins(PluginGroupNodeMaterialAnime);
    app.add_plugins(
        PluginSceneTimeFromPluginFrame
    );
    app.add_plugins(
        (
            PluginParticleSystem,
            pi_gltf2_load::PluginGLTF2Res,
            pi_trail_renderer::PluginTrail
        )
    );
    app.add_frame_event::<ComponentEvent<Changed<Layer>>>();

    app.world.get_resource_mut::<StateGlobal>().unwrap().debug = true;

    app.world.get_resource_mut::<WindowRenderer>().unwrap().active = true;
    
    app
}

pub fn test_plugins_with_gltf() -> App {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

    let mut app = App::default();

	let mut window_plugin = WindowPlugin::default();
    if let Some(primary_window) = &mut window_plugin.primary_window {
        primary_window.resolution.set_physical_resolution(800, 600);
    }

    app.insert_resource(AssetMgrConfigs::default());
    app.add_plugins(
        (
            InputPlugin::default(),
            window_plugin,
            AccessibilityPlugin,
            bevy::winit::WinitPlugin::default(),
            pi_bevy_asset::PiAssetPlugin::default(),
            PiRenderPlugin::default(),
            PluginLocalLoad,
            PluginFrameTime,
            PluginWindowRender,
        )
    );
            
    app.add_plugins(PluginBundleDefault);
    app.add_plugins(
        (
            PluginCubeBuilder,
            PluginQuadBuilder,
            PluginStateToFile,
            PluginNodeMaterial,
            PluginUnlitMaterial,
        )
    );
    app.add_plugins(PluginGroupNodeMaterialAnime);
    app.add_plugins(
        PluginSceneTimeFromPluginFrame
    );
    app.add_plugins(
        (
            PluginParticleSystem,
            pi_gltf2_load::PluginGLTF2Res,
            pi_trail_renderer::PluginTrail
        )
    );
    app.add_frame_event::<ComponentEvent<Changed<Layer>>>();

    app.world.get_resource_mut::<StateGlobal>().unwrap().debug = true;

    app.world.get_resource_mut::<WindowRenderer>().unwrap().active = true;
    
    app
}