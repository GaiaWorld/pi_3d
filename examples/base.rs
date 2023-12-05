#[allow(dead_code)]
#[allow(unused_imports)]

use default_render::shader::DefaultShader;
use pi_3d::PluginBundleDefault;
use pi_3d_state::StateResource;
use pi_bevy_ecs_extend::system_param::layer_dirty::ComponentEvent;
use pi_bevy_render_plugin::PiRenderPlugin;
use pi_engine_shell::{prelude::*, frame_time::PluginFrameTime};
use pi_node_materials::prelude::*;
use pi_particle_system::{PluginParticleSystem, prelude::{ResParticleCommonBuffer, ActionSetParticleSystem}};
use pi_scene_context::{prelude::*, shadow::PluginShadowGenerator};
use pi_mesh_builder::{cube::*, quad::{PluginQuadBuilder, QuadBuilder}, ball::PluginBallBuilder};
use pi_shadow_mapping::PluginShadowMapping;
use pi_standard_material::PluginStandardMaterial;
use pi_trail_renderer::ActionSetTrailRenderer;
use unlit_material::*;
use wgpu1::Backends;

use std::sync::Arc;
use pi_async_rt::rt::AsyncRuntime;
use pi_hal::{init_load_cb, runtime::MULTI_MEDIA_RUNTIME, on_load};

#[path = "./copy.rs"]
mod copy;

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

pub struct DemoScene {
    pub scene: Entity,
    pub camera: Entity,
    pub opaque_renderer: Entity,
    pub transparent_renderer: Entity,
    pub opaque_target: Option<KeyCustomRenderTarget>,
    pub transparent_target: Option<KeyCustomRenderTarget>,
    pub shadowtarget: Option<KeyRenderTarget>,
}
impl DemoScene {
    pub const PASS_SHADOW: PassTag          = PassTag::PASS_TAG_01;
    pub const PASS_PRE_DEPTH: PassTag       = PassTag::PASS_TAG_02;
    pub const PASS_OPAQUE: PassTag          = PassTag::PASS_TAG_03;
    pub const PASS_HIGHLIGHT: PassTag       = PassTag::PASS_TAG_04;
    pub const PASS_SKY_WATER: PassTag       = PassTag::PASS_TAG_06;
    pub const PASS_TRANSPARENT: PassTag     = PassTag::PASS_TAG_07;
    pub fn new(
        commands: &mut Commands,
        actions: &mut pi_3d::ActionSets,
        animegroupres: &mut ResourceAnimationGroup,
        targets: &mut CustomRenderTargets,
        device: &RenderDevice,
        asset_samp: &ShareAssetMgr<SamplerRes>,
        atlas_allocator: &PiSafeAtlasAllocator,
        camera_size: f32,
        camera_fov: f32,
        camera_position: (f32, f32, f32),
        orthographic_camera: bool
    ) -> Self {
        
        let keytarget =  match targets.create(device, KeySampler::linear_clamp(), asset_samp, atlas_allocator, ColorFormat::Rgba8Unorm, DepthStencilFormat::Depth32Float, 800 * 2, 600 * 2) {
            Some(key) => { Some(KeyCustomRenderTarget::Custom(key)) },
            None => None,
        };
        
        let shadowtarget = targets.create(device, KeySampler::linear_clamp(), asset_samp, atlas_allocator, ColorFormat::Rgba16Float, DepthStencilFormat::Depth32Float, 2048, 2048);

        let scene = commands.spawn_empty().id();
        animegroupres.scene_ctxs.init_scene(scene);
        actions.scene.create.push(OpsSceneCreation::ops(scene, SceneBoundingPool::MODE_LIST, [0, 0, 0, 0,0 ,0 ,0 ,0 ,0]));

        let camera = commands.spawn_empty().id(); actions.transform.tree.push(OpsTransformNodeParent::ops(camera, scene));
        actions.camera.create.push(OpsCameraCreation::ops(scene, camera, true));
        actions.transform.localpos.push(OpsTransformNodeLocalPosition::ops(camera, camera_position.0, camera_position.1, camera_position.2));
        actions.camera.mode.push(OpsCameraMode::ops(camera, orthographic_camera));
        actions.camera.active.push(OpsCameraActive::ops(camera, true));
        actions.camera.size.push(OpsCameraOrthSize::ops(camera, camera_size));
        actions.camera.fov.push(OpsCameraFov::ops(camera, camera_fov));
        actions.camera.aspect.push(OpsCameraAspect::ops(camera, 800. / 600.) );
        actions.camera.nearfar.push(OpsCameraNearFar::ops(camera, 0.1, 100.));
        actions.camera.target.push(OpsCameraTarget::ops(camera, 0., -1., 1.));

        let opaque_renderer = commands.spawn_empty().id(); actions.renderer.create.push(OpsRendererCreate::ops(opaque_renderer, String::from("TestCameraOpaque"), camera, DemoScene::PASS_OPAQUE, false));
        actions.renderer.modify.push(OpsRendererCommand::AutoClearColor(opaque_renderer, true));
        actions.renderer.modify.push(OpsRendererCommand::AutoClearDepth(opaque_renderer, true));
        actions.renderer.modify.push(OpsRendererCommand::AutoClearStencil(opaque_renderer, true));
        actions.renderer.modify.push(OpsRendererCommand::DepthClear(opaque_renderer, RenderDepthClear(1.)));
        actions.renderer.modify.push(OpsRendererCommand::ColorClear(opaque_renderer, RenderColorClear(0, 0, 0, 0)));
        actions.renderer.target.push(OpsRendererTarget::Custom(opaque_renderer, keytarget.clone().unwrap()));
        // actions.camera.render.push(OpsCameraRendererInit::ops(camera, opaque_renderer, desc.curr, desc.passorders, ColorFormat::Rgba8Unorm, DepthStencilFormat::None, RenderTargetMode::Window));

        let transparent_renderer = commands.spawn_empty().id(); actions.renderer.create.push(OpsRendererCreate::ops(transparent_renderer, String::from("TestCameraTransparent"), camera, DemoScene::PASS_TRANSPARENT, true));
        actions.renderer.modify.push(OpsRendererCommand::AutoClearColor(transparent_renderer, false));
        actions.renderer.modify.push(OpsRendererCommand::AutoClearDepth(transparent_renderer, false));
        actions.renderer.modify.push(OpsRendererCommand::AutoClearStencil(transparent_renderer, false));
        actions.renderer.connect.push(OpsRendererConnect::ops(opaque_renderer, transparent_renderer, false));
        actions.renderer.target.push(OpsRendererTarget::Custom(transparent_renderer, keytarget.clone().unwrap()));
        // actions.camera.render.push(OpsCameraRendererInit::ops(camera, transparent_renderer, desc.curr, desc.passorders, ColorFormat::Rgba8Unorm, DepthStencilFormat::None, RenderTargetMode::Window));

        Self { scene, camera, opaque_renderer, transparent_renderer, opaque_target: keytarget.clone(), transparent_target: keytarget, shadowtarget }
    }

    pub fn mesh(
        commands: &mut Commands,
        scene: Entity,
        parent: Entity,
        actions: &mut pi_3d::ActionSets,
        vertices: Vec<VertexBufferDesc>,
        indices: Option<IndicesBufferDesc>,
        state: MeshInstanceState,
    ) -> Entity {
        let id_geo = commands.spawn_empty().id();
        let mesh = commands.spawn_empty().id(); actions.transform.tree.push(OpsTransformNodeParent::ops(mesh, parent));
        actions.mesh.create.push(OpsMeshCreation::ops(scene, mesh, state));
        actions.geometry.create.push(OpsGeomeryCreate::ops(mesh, id_geo, vertices, indices));
        actions.mesh.depth_compare.push(OpsDepthCompare::ops(mesh, CompareFunction::LessEqual));
        mesh
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

    let width = 800;
    let height = 600;

    let mut opt = PiRenderOptions::default();
    opt.backends = Backends::VULKAN;
    app.insert_resource(opt);

	let mut window_plugin = WindowPlugin::default();
    if let Some(primary_window) = &mut window_plugin.primary_window {
        primary_window.resolution.set_physical_resolution(width, height);
    }
	let (w, eventloop) = {
		use pi_winit::platform::windows::EventLoopBuilderExtWindows;
		let event_loop = pi_winit::event_loop::EventLoopBuilder::new().with_any_thread(true).build();
		let window = pi_winit::window::Window::new(&event_loop).unwrap();
		(window, event_loop)
	};

    app.insert_resource(AssetMgrConfigs::default());
    app.add_plugins(
        (
            InputPlugin::default(),
            window_plugin,
        )
    );
    app.add_plugins(
        (
            AccessibilityPlugin,
            pi_bevy_winit_window::WinitPlugin::new(Arc::new(w)).with_size(width, height),
            pi_bevy_asset::PiAssetPlugin::default(),
            PiRenderPlugin::default(),
            PluginLocalLoad,
            PluginFrameTime,
        )
    );
            
    app.add_plugins(PluginBundleDefault);
    app.add_plugins(
        (
            PluginCubeBuilder,
            PluginQuadBuilder,
            PluginBallBuilder,
            PluginStateToFile,
            PluginUnlitMaterial,
            PluginStandardMaterial,
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

    app.add_plugins(copy::PluginImageCopy);
    app.add_frame_event::<ComponentEvent<Changed<Layer>>>();

    app.world.get_resource_mut::<StateResource>().unwrap().debug = true;
    
    app.add_systems(Startup, setup_default_mat);
    
    app
}

pub fn test_plugins_with_gltf() -> App {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

    let mut app = App::default();
    let width = 800;
    let height = 600;

    let mut opt = PiRenderOptions::default();
    opt.backends = Backends::VULKAN;
    app.insert_resource(opt);

    app.insert_resource(SceneLightLimit(LightLimitInfo { max_direct_light_count: 8, max_point_light_count: 256, max_spot_light_count: 128, max_hemi_light_count: 16 }));
    app.insert_resource(ModelLightLimit(LightLimitInfo { max_direct_light_count: 4, max_point_light_count: 16, max_spot_light_count: 16, max_hemi_light_count: 4 }));
    app.insert_resource(SceneShadowLimit(
        ShadowLimitInfo { max_count: 1, max_width: 2048, max_height: 2048, color_format: ColorFormat::Rgba16Float, depth_stencil_format: DepthStencilFormat::Depth32Float }
    ));

	let mut window_plugin = WindowPlugin::default();
    if let Some(primary_window) = &mut window_plugin.primary_window {
        primary_window.resolution.set_physical_resolution(width, height);
    }
    
	let w = {
		use pi_winit::platform::windows::EventLoopBuilderExtWindows;
		let event_loop = pi_winit::event_loop::EventLoopBuilder::new().with_any_thread(true).build();
		let window = pi_winit::window::Window::new(&event_loop).unwrap();
		window
	};

    let mut cfg = AssetMgrConfigs::default();
    cfg.insert(String::from(ResParticleCommonBuffer::ASSET_TYPE), AssetCapacity { flag: false, min: 10 * 1024 * 1024, max: 10 * 1024 * 1024, timeout: 100  });
    app.insert_resource(cfg);
    app.add_plugins(
        (
            InputPlugin::default(),
            window_plugin,
        )
    );
    app.add_plugins(
        (
            AccessibilityPlugin,
            pi_bevy_winit_window::WinitPlugin::new(Arc::new(w)).with_size(width, height),
            pi_bevy_asset::PiAssetPlugin::default(),
            PiRenderPlugin::default(),
            PluginLocalLoad,
            PluginFrameTime,
        )
    );
            
    app.add_plugins(PluginBundleDefault);
    app.add_plugins(
        (
            PluginCubeBuilder,
            PluginQuadBuilder,
            PluginBallBuilder,
            PluginStateToFile,
            PluginUnlitMaterial,
            PluginStandardMaterial,
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

    app.add_plugins(copy::PluginImageCopy);
    app.add_frame_event::<ComponentEvent<Changed<Layer>>>();

    app.world.get_resource_mut::<StateResource>().unwrap().debug = true;

    app.add_systems(Startup, setup_default_mat);
    
    app
}

pub fn setup_default_mat(
    mat: Res<SingleIDBaseDefaultMaterial>,
    mut actionsmat: ResMut<ActionListMaterialCreate>,
) {
    let entity = mat.0;
    actionsmat.push(OpsMaterialCreate(entity, KeyShaderMeta::from(DefaultShader::KEY)));
}
