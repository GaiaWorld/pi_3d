#[cfg(feature = "use_bevy")]
use bevy_a11y::AccessibilityPlugin;
#[cfg(feature = "use_bevy")]
use bevy_input::*;
use distortion_material::ShaderDistortion;
use pbr_material::ShaderPBR;
#[allow(dead_code)]
#[allow(unused_imports)]

use pi_3d::*;
use pi_bevy_ecs_extend::action;
// use pi_bevy_ecs_extend::system_param::layer_dirty::ComponentEvent;
use pi_bevy_render_plugin::PiRenderPlugin;
use pi_render::components::view;
use pi_scene_shell::{prelude::*, frame_time::PluginFrameTime, run_stage::RunState3D};
use pi_node_materials::prelude::*;
use pi_particle_system::{PluginParticleSystem, prelude::{ActionSetParticleSystem, ParticleAttribute, EParticleAttributeType}};
use pi_scene_context::{prelude::*, shadow::PluginShadowGenerator, scene::StageScene};
use pi_mesh_builder::{cube::*, quad::{PluginQuadBuilder, QuadBuilder}, ball::PluginBallBuilder};
use pi_standard_material::PluginStandardMaterial;
use predepth::ShaderPreDepth;
use unlit_material::*;
use water::ShaderWater;
use wgpu::Backends;
use pi_winit::{event::WindowEvent, event_loop::EventLoop, window::Window};

use std::sync::Arc;
use pi_async_rt::rt::AsyncRuntime;
use pi_hal::{init_load_cb, runtime::MULTI_MEDIA_RUNTIME, on_load};

#[path = "./copy.rs"]
mod copy;
#[path = "./distortion_material.rs"]
mod distortion_material;
#[path = "./pbr_material.rs"]
mod pbr_material;
#[path = "./water.rs"]
mod water;
#[path = "./predepth.rs"]
mod predepth;

pub struct PluginLocalLoad;
impl Plugin for PluginLocalLoad {
    fn build(&self, _: &mut App) {
        init_load_cb(Arc::new(|module, func, hash, rag| {
            MULTI_MEDIA_RUNTIME
                .spawn(async move {
                    // log::debug!("Load {}", path);
                    match &rag[0] {
                        pi_hal::Arg::Number(_) => todo!(),
                        pi_hal::Arg::String(path) => {
                            let r = std::fs::read(path.clone()).unwrap();
                            on_load(hash.parse::<u64>().unwrap(), Ok(Share::new(r)));
                        },
                        pi_hal::Arg::Buffer(_) => todo!(),
                        pi_hal::Arg::None => todo!(),
                    }
                })
                .unwrap();
        }));
    }
}

pub fn main() {
    
}

pub fn instance_attr(matrix: bool, color: bool, tilloff: bool) -> MeshInstanceState {
    let mut instances = vec![];

    if color {
        instances.push(instance_color());
    }
    if tilloff {
        instances.push(instance_tilloff());
    }

    MeshInstanceState {
        instances,
        instance_matrix: matrix,
        use_single_instancebuffer: false,
    }
}
pub fn instance_color() -> CustomVertexAttribute {
    CustomVertexAttribute::new(Atom::from("InsColor4"), Atom::from("A_COLOR4 = InsColor4;"), ECustomVertexType::Vec4, None)
}
pub fn instance_tilloff() -> CustomVertexAttribute {
    CustomVertexAttribute::new(Atom::from("InsTilloff"), Atom::from("A_UV = A_UV * InsTilloff.xy + InsTilloff.zw;"), ECustomVertexType::Vec4, None)
}

pub fn particelsystem_mesh_state() -> MeshInstanceState {
    MeshInstanceState {
        instances: vec![instance_color(), instance_tilloff()],
        instance_matrix: true,
        use_single_instancebuffer: false,
    }
}
pub fn particelsystem_mesh_state_single() -> MeshInstanceState {
    MeshInstanceState {
        instances: vec![instance_color(), instance_tilloff()],
        instance_matrix: true,
        use_single_instancebuffer: true,
    }
}
pub fn particelsystem_attrs() -> Vec<ParticleAttribute> {
    vec![
        ParticleAttribute { vtype: EParticleAttributeType::Matrix, attr: Atom::from("") },
        ParticleAttribute { vtype: EParticleAttributeType::Color, attr: Atom::from("InsColor4") },
        ParticleAttribute { vtype: EParticleAttributeType::Tilloff, attr: Atom::from("InsTilloff") },
    ]
}

pub struct DemoScene {
    pub scene: Entity,
    pub camera: Entity,
    pub opaque_renderer: Entity,
    pub skywater_renderer: Entity,
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
        camera_nearfar: (f32, f32),
        camera_position: (f32, f32, f32),
        orthographic_camera: bool
    ) -> Self {
        let freemode = if orthographic_camera {
            EFreeCameraMode::Orthograhic
        } else { EFreeCameraMode::Perspective };
        
        let keytarget =  match targets.create_sync(device, asset_samp, atlas_allocator, KeySampler::linear_clamp(), ColorFormat::Rgba8Unorm, DepthStencilFormat::Depth32Float, 800, 600) {
            Some(key) => { Some(KeyCustomRenderTarget::Custom(key)) },
            None => None,
        };
        
        let shadowtarget = targets.create(KeySampler::linear_clamp(), ColorFormat::Rgba16Float, DepthStencilFormat::Depth32Float, 1024, 1024);

        let scene = commands.spawn_empty_id();
        // animegroupres.scene_ctxs.init_scene(scene);
        actions.scene.create.push(OpsSceneCreation::ops(scene, SceneBoundingPool::MODE_LIST, SceneColliderPool::MODE_LIST, [-9999, -9999, -9999, 9999, 9999, 9999, 0, 0, 0]));

        let camera = commands.spawn_empty_id(); actions.transform.tree.push(OpsTransformNodeParent::ops(camera, scene));
        actions.camera.create.push(OpsCameraCreation::ops(scene, camera));
        actions.transform.localsrt.push(OpsTransformNodeLocal::ops(camera, ETransformSRT::Translation(camera_position.0, camera_position.1, camera_position.2)));
        actions.camera.param.push(OpsCameraModify::ops(camera, ECameraModify::FreeMode( freemode ) ));
        actions.camera.param.push(OpsCameraModify::ops( camera, ECameraModify::Active( true )));
        actions.camera.param.push(OpsCameraModify::ops( camera, ECameraModify::OrthSize( camera_size )));
        actions.camera.param.push(OpsCameraModify::ops( camera, ECameraModify::Fov( camera_fov )));
        actions.camera.param.push(OpsCameraModify::ops( camera, ECameraModify::Aspect( 800. / 600. )) );
        actions.camera.param.push(OpsCameraModify::ops( camera, ECameraModify::NearFar( camera_nearfar.0,  camera_nearfar.1)));
        actions.camera.target.push(OpsCameraTarget::ops(camera, 0., -1., 1.));

        let opaque_renderer = commands.spawn_empty_id(); actions.renderer.create.push(OpsRendererCreate::ops(opaque_renderer, String::from("TestCameraOpaque"), camera, DemoScene::PASS_OPAQUE, false));
        actions.renderer.modify.push(OpsRendererCommand::AutoClearColor(opaque_renderer, true));
        actions.renderer.modify.push(OpsRendererCommand::AutoClearDepth(opaque_renderer, true));
        actions.renderer.modify.push(OpsRendererCommand::AutoClearStencil(opaque_renderer, true));
        actions.renderer.modify.push(OpsRendererCommand::DepthClear(opaque_renderer, RenderDepthClear(1.)));
        actions.renderer.modify.push(OpsRendererCommand::ColorClear(opaque_renderer, RenderColorClear(0, 0, 0, 0)));
        actions.renderer.target.push(OpsRendererTarget::Custom(opaque_renderer, keytarget.clone().unwrap(), false));
        // actions.camera.render.push(OpsCameraRendererInit::ops(camera, opaque_renderer, desc.curr, desc.passorders, ColorFormat::Rgba8Unorm, DepthStencilFormat::None, RenderTargetMode::Window));
        
        let skywater_renderer = commands.spawn_empty_id(); actions.renderer.create.push(OpsRendererCreate::ops(skywater_renderer, String::from("TestCameraSkyWater"), camera, DemoScene::PASS_SKY_WATER, false));
        actions.renderer.modify.push(OpsRendererCommand::AutoClearColor(skywater_renderer, false));
        actions.renderer.modify.push(OpsRendererCommand::AutoClearDepth(skywater_renderer, false));
        actions.renderer.modify.push(OpsRendererCommand::AutoClearStencil(skywater_renderer, false));
        actions.renderer.target.push(OpsRendererTarget::Custom(skywater_renderer, keytarget.clone().unwrap(), false));
        actions.renderer.connect.push(OpsRendererConnect::ops(opaque_renderer, skywater_renderer, false));

        let transparent_renderer = commands.spawn_empty_id(); actions.renderer.create.push(OpsRendererCreate::ops(transparent_renderer, String::from("TestCameraTransparent"), camera, DemoScene::PASS_TRANSPARENT, true));
        actions.renderer.modify.push(OpsRendererCommand::AutoClearColor(transparent_renderer, false));
        actions.renderer.modify.push(OpsRendererCommand::AutoClearDepth(transparent_renderer, false));
        actions.renderer.modify.push(OpsRendererCommand::AutoClearStencil(transparent_renderer, false));
        actions.renderer.connect.push(OpsRendererConnect::ops(skywater_renderer, transparent_renderer, false));
        actions.renderer.target.push(OpsRendererTarget::Custom(transparent_renderer, keytarget.clone().unwrap(), false));
        // actions.camera.render.push(OpsCameraRendererInit::ops(camera, transparent_renderer, desc.curr, desc.passorders, ColorFormat::Rgba8Unorm, DepthStencilFormat::None, RenderTargetMode::Window));

        Self { scene, camera, skywater_renderer, opaque_renderer, transparent_renderer, opaque_target: keytarget.clone(), transparent_target: keytarget, shadowtarget }
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
        let id_geo = commands.spawn_empty_id();
        let mesh = commands.spawn_empty_id(); actions.transform.tree.push(OpsTransformNodeParent::ops(mesh, parent));
        actions.mesh.create.push(OpsMeshCreation::ops(scene, mesh, state));
        actions.geometry.create.push(OpsGeomeryCreate::ops(mesh, id_geo, vertices, indices));

        // actions.mesh.depth_compare.push(OpsDepthCompare::ops(mesh, CompareFunction::LessEqual));
        actions.mesh.render_state.push(OpsRenderState::depth_state(mesh, PassTag::PASS_TAG_01, EDepthState::Compare(CompareFunction::LessEqual)));
        actions.mesh.render_state.push(OpsRenderState::depth_state(mesh, PassTag::PASS_TAG_02, EDepthState::Compare(CompareFunction::LessEqual)));
        actions.mesh.render_state.push(OpsRenderState::depth_state(mesh, PassTag::PASS_TAG_03, EDepthState::Compare(CompareFunction::LessEqual)));
        actions.mesh.render_state.push(OpsRenderState::depth_state(mesh, PassTag::PASS_TAG_04, EDepthState::Compare(CompareFunction::LessEqual)));
        actions.mesh.render_state.push(OpsRenderState::depth_state(mesh, PassTag::PASS_TAG_05, EDepthState::Compare(CompareFunction::LessEqual)));
        actions.mesh.render_state.push(OpsRenderState::depth_state(mesh, PassTag::PASS_TAG_06, EDepthState::Compare(CompareFunction::LessEqual)));
        actions.mesh.render_state.push(OpsRenderState::depth_state(mesh, PassTag::PASS_TAG_07, EDepthState::Compare(CompareFunction::LessEqual)));
        actions.mesh.render_state.push(OpsRenderState::depth_state(mesh, PassTag::PASS_TAG_08, EDepthState::Compare(CompareFunction::LessEqual)));

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
            sys_scene_time_from_frame.after(pi_scene_shell::frame_time::sys_frame_time).in_set(StageScene::SceneCreate)
        );
    }
}

#[derive(Resource)]
pub struct DemoOption {
    pub width: u32,
    pub height: u32,
    pub orthographic_camera: bool,
    pub camera_fov: f32,
    pub camera_size: f32,
    pub camera_nearfar: (f32, f32),
    pub camera_position: (f32, f32, f32),
    pub demo: Option<DemoScene>,
    pub copyrenderer: Option<Entity>,
    pub copyrendercamera: Option<Entity>,
}
impl Default for DemoOption {
    fn default() -> Self {
        Self {
            width: 800,
            height: 600,
            orthographic_camera: true,
            camera_fov: 0.7,
            camera_size: 1.0,
            camera_nearfar: (1., 1001.),
            camera_position: (0., 10., -40.),
            demo: None,
            copyrenderer: None,
            copyrendercamera: None,
        }
    }
}

pub fn setup_demoinit(
    mut commands: Commands,
    mut actions: pi_3d::ActionSets,
    mut animegroupres: ResourceAnimationGroup,
    mut demooption: ResMut<DemoOption>,
    mut assets: (ResMut<CustomRenderTargets>, Res<PiRenderDevice>, Res<ShareAssetMgr<SamplerRes>>, Res<PiSafeAtlasAllocator>,),
    mut errors: ResMut<ErrorRecord>,
    engineopt: Res<EngineCustomPlugins>,
    asset_mgr: Res<ShareAssetMgr<ShaderEffectMeta>>,
    mut nodematblocks: ResMut<NodeMaterialBlocks>,
) {
    
    ActionMaterial::regist_material_meta(&asset_mgr, KeyShaderMeta::from(ShaderDistortion::KEY), ShaderDistortion::meta(&mut nodematblocks, &engineopt));
    ActionMaterial::regist_material_meta(&asset_mgr, KeyShaderMeta::from(MainOpacityShader::KEY), MainOpacityShader::meta(&engineopt));
    ActionMaterial::regist_material_meta(&asset_mgr, KeyShaderMeta::from(copy::ShaderImageCopy::KEY), copy::ShaderImageCopy::res(&engineopt));
    ActionMaterial::regist_material_meta(&asset_mgr, KeyShaderMeta::from(unlit_material::PlanarShadow::KEY), unlit_material::PlanarShadow::meta(&engineopt));
    ActionMaterial::regist_material_meta(&asset_mgr, KeyShaderMeta::from(ShaderPBR::KEY), ShaderPBR::meta(&mut nodematblocks, &engineopt));
    ActionMaterial::regist_material_meta(&asset_mgr, KeyShaderMeta::from(ShaderWater::KEY), ShaderWater::meta(&mut nodematblocks, &engineopt));
    ActionMaterial::regist_material_meta(&asset_mgr, KeyShaderMeta::from(ShaderPreDepth::KEY), ShaderPreDepth::meta(&mut nodematblocks, &engineopt));

    // errors.1 = true;
    let demopass = DemoScene::new(&mut commands, &mut actions, &mut animegroupres, 
        &mut assets.0, &assets.1, &assets.2, &assets.3,
        demooption.camera_size, demooption.camera_fov, demooption.camera_nearfar, demooption.camera_position, demooption.orthographic_camera
    );
    let (scene, camera01) = (demopass.scene, demopass.camera);

    let (copyrenderer, copyrendercamera) = copy::PluginImageCopy::toscreen(&mut commands, &mut actions, scene, demopass.transparent_renderer,demopass.transparent_target.clone());
    actions.renderer.connect.push(OpsRendererConnect::ops(demopass.transparent_renderer, copyrenderer, false));

    demooption.demo = Some(demopass);
    demooption.copyrenderer = Some(copyrenderer);
    demooption.copyrendercamera = Some(copyrendercamera);
    log::warn!("setup_demoinit");
}

pub fn test_plugins() -> (App, Arc<pi_winit::window::Window>,EventLoop<()>) {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

    let mut app = App::new();

    let width = 800;
    let height = 600;

    let mut opt = PiRenderOptions::default();
    opt.backends = wgpu::Backends::GL;
    app.insert_resource(opt);

	let (w, eventloop) = {
		use pi_winit::platform::windows::EventLoopBuilderExtWindows;
		let event_loop = pi_winit::event_loop::EventLoopBuilder::new().with_any_thread(true).build();
		let window = pi_winit::window::Window::new(&event_loop).unwrap();
		(Arc::new(window), event_loop)
	};
    app.insert_resource(AssetMgrConfigs::default());

    #[cfg(feature = "use_bevy")]
    {
        let mut window_plugin = bevy_window::WindowPlugin::default();
        if let Some(primary_window) = &mut window_plugin.primary_window {
            primary_window.resolution.set_physical_resolution(width, height);
        }
        app.add_plugins(
            (
                InputPlugin::default(),
                window_plugin,
            )
        );
        app.add_plugins(AccessibilityPlugin);
    }


    app.add_plugins(pi_bevy_winit_window::WinitPlugin::new(w.clone()).with_size(width, height));
    app.add_plugins(pi_bevy_asset::PiAssetPlugin::default());
    app.add_plugins(PiRenderPlugin::default());
    app.add_plugins(PluginLocalLoad);
    app.add_plugins(PluginFrameTime);
            
    app.insert_resource(EngineCustomPlugins::default());
    PluginBundleDefault::add(&mut app);
    
    app.add_plugins(PluginNodeMaterial);
    app.add_plugins(PluginShadowGenerator);
    app.add_plugins(PluginShadowMapping);
        
    app.add_plugins(PluginCubeBuilder);
    app.add_plugins(PluginQuadBuilder);
    app.add_plugins(PluginBallBuilder);
    app.add_plugins(PluginStateToFile);
    app.add_plugins(PluginUnlitMaterial);
    app.add_plugins(PluginStandardMaterial);
    
    app.add_plugins(
        PluginSceneTimeFromPluginFrame
    );
    
    app.add_plugins(PluginParticleSystem);
    app.add_plugins(pi_gltf2_load::PluginGLTF2Res);
    app.add_plugins(pi_trail_renderer::PluginTrail);
    
    app.insert_resource(SceneLightLimit(LightLimitInfo { max_direct_light_count: 4, max_point_light_count: 64, max_spot_light_count: 64, max_hemi_light_count: 4 }));
    app.insert_resource(ModelLightLimit(LightLimitInfo { max_direct_light_count: 4, max_point_light_count: 8, max_spot_light_count: 4, max_hemi_light_count: 4 }));
    app.insert_resource(SceneShadowLimit(
        ShadowLimitInfo { max_count: 1, max_width: 2048, max_height: 2048, color_format: ColorFormat::Rgba16Float, depth_stencil_format: DepthStencilFormat::Depth32Float }
    ));

    app.add_plugins(copy::PluginImageCopy);
    // app.add_frame_event::<ComponentEvent<Changed<Layer>>>();

    app.world.get_resource_mut::<StateResource>().unwrap().debug = true;

    #[cfg(feature = "use_bevy")]
    app.add_systems(Startup, setup_default_mat);
    #[cfg(not(feature = "use_bevy"))]
    app.add_startup_system(Update, setup_default_mat);
    
    (app, w, eventloop)
}

pub fn test_plugins_with_gltf() -> (App, Arc<Window>, EventLoop<()>) {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

    let mut app = App::new();
    let width = 800;
    let height = 600;

    let mut opt = PiRenderOptions::default();
    opt.backends = Backends::VULKAN;
    app.insert_resource(opt);
    
	let (w, event_loop) = {
		use pi_winit::platform::windows::EventLoopBuilderExtWindows;
		let event_loop = pi_winit::event_loop::EventLoopBuilder::new().with_any_thread(true).build();
		let window = pi_winit::window::Window::new(&event_loop).unwrap();
		(Arc::new(window), event_loop)
	};

    let mut cfg = AssetMgrConfigs::default();
    // cfg.insert(String::from(ResParticleCommonBuffer::ASSET_TYPE), AssetCapacity { flag: false, min: 10 * 1024 * 1024, max: 4, timeout: 100  });
    app.insert_resource(cfg);

    #[cfg(feature = "use_bevy")]
    {
        let mut window_plugin = bevy_window::WindowPlugin::default();
        if let Some(primary_window) = &mut window_plugin.primary_window {
            primary_window.resolution.set_physical_resolution(width, height);
        }
        app.add_plugins(
            (
                InputPlugin::default(),
                window_plugin,
            )
        );
        app.add_plugins(AccessibilityPlugin);
    }


    app.add_plugins(pi_bevy_winit_window::WinitPlugin::new(w.clone()).with_size(width, height));
    app.add_plugins(pi_bevy_asset::PiAssetPlugin::default());
    app.add_plugins(PiRenderPlugin::default());
    app.add_plugins(PluginLocalLoad);
    app.add_plugins(PluginFrameTime);

    app.insert_resource(EngineCustomPlugins::default());
    PluginBundleDefault::add(&mut app);

    app.add_plugins(PluginNodeMaterial);
    app.add_plugins(PluginShadowGenerator);
    app.add_plugins(PluginShadowMapping);

    app.add_plugins(PluginCubeBuilder);
    app.add_plugins(PluginQuadBuilder);
    app.add_plugins(PluginBallBuilder);
    app.add_plugins(PluginStateToFile);
    app.add_plugins(PluginUnlitMaterial);
    app.add_plugins(PluginStandardMaterial);

    app.add_plugins(
        PluginSceneTimeFromPluginFrame
    );
    app.add_plugins(PluginParticleSystem);
    app.add_plugins(pi_gltf2_load::PluginGLTF2Res);
    app.add_plugins(pi_trail_renderer::PluginTrail);
    
    app.insert_resource(SceneLightLimit(LightLimitInfo { max_direct_light_count: 4, max_point_light_count: 64, max_spot_light_count: 64, max_hemi_light_count: 4 }));
    app.insert_resource(ModelLightLimit(LightLimitInfo { max_direct_light_count: 4, max_point_light_count: 8, max_spot_light_count: 4, max_hemi_light_count: 4 }));
    app.insert_resource(SceneShadowLimit(
        ShadowLimitInfo { max_count: 1, max_width: 2048, max_height: 2048, color_format: ColorFormat::Rgba16Float, depth_stencil_format: DepthStencilFormat::Depth32Float }
    ));

    app.add_plugins(copy::PluginImageCopy);
    // app.add_frame_event::<ComponentEvent<Changed<Layer>>>();

    app.world.get_resource_mut::<StateResource>().unwrap().debug = true;

    #[cfg(feature = "use_bevy")]
    app.add_systems(Startup, setup_default_mat);
    #[cfg(not(feature = "use_bevy"))]
    app.add_startup_system(Update, setup_default_mat);
    
    (app, w, event_loop)
}

pub fn setup_default_mat(
    mat: Res<SingleIDBaseDefaultMaterial>,
    mut actionsmat: ResMut<ActionListMaterialCreate>,
) {
    let entity = mat.0;
    actionsmat.push(OpsMaterialCreate::ops_with_matarray(entity, DefaultShader::KEY));
}

pub fn active_lighting_shadow(mut state3d: ResMut<RunState3D>) {
    state3d.with_lighting(true);
    state3d.with_shadow(true);
}

#[derive(Resource, Default)]
pub struct DemoWindowEvent {
    pub cursormoved: Option<(f32, f32)>,
    pub viewer: Option<Entity>,
    pub raybox: Option<Entity>,
}

pub fn sys_move_ray_collider(
    mut events: ResMut<DemoWindowEvent>,
    scenes: Query<(&SceneColliderPool, &SceneBoundingPool)>,
    viewers: Query<(&SceneID, &ViewerTransformMatrix)>,
    window: Res<PiRenderWindow>,
    items: Query<&Collider>,
    matrixs: Query<&GlobalMatrix>,
    mut actions: pi_3d::ActionSets,
    mut commands: Commands,
    defaultmat: Res<SingleIDBaseDefaultMaterial>,
    sortparam: Query<&GlobalEnable>,
) {

    if let (Some((x, y)), Some(viewer)) = (events.cursormoved, events.viewer) {
        if let Ok((sceneid, transformatrix)) = viewers.get(viewer) {
                
            if events.raybox.is_none() {
                let (vertices, indices) = (CubeBuilder::attrs_meta(), CubeBuilder::indices_meta());
                let state: MeshInstanceState = MeshInstanceState::default();
                let source = DemoScene::mesh(&mut commands, sceneid.0, sceneid.0, &mut actions,  vertices, indices, state);
                actions.material.usemat.push(OpsMaterialUse::ops(source, defaultmat.0, DemoScene::PASS_TRANSPARENT));
                actions.mesh.state.push(OpsMeshStateModify::ops(source, EMeshStateModify::BoundingCullingMode(ECullingStrategy::None)));
                actions.mesh.render_state.push(OpsRenderState::depth_state(source, DemoScene::PASS_TRANSPARENT, EDepthState::Write(false)));
                actions.mesh.render_state.push(OpsRenderState::primitive_state(source, DemoScene::PASS_TRANSPARENT, EPrimitiveState::CPolygonMode(PolygonMode::Line)));
                actions.mesh.render_state.push(OpsRenderState::render_queue(source, 0, i32::MAX));
                events.raybox = Some(source);
                log::error!("Collider: {:?}", source);
            }

            let raybox = events.raybox.unwrap();
    
            let x = 0. + ((x / window.width  as f32) * 2. - 1.);
            let y = 0. - ((y / window.height as f32) * 2. - 1.);
            let ray = transformatrix.ray(x, y);
            let result = ray_cast(
                &scenes,
                &ray,
                sceneid.0,
                false,
                &sortparam
            );

            if let Some(result) = &result {
                if let (Ok(collider), Ok(nodematrix)) = (items.get(result.target), matrixs.get(result.target)) {
                    let px = collider.maximum.x + collider.minimum.x;
                    let py = collider.maximum.y + collider.minimum.y;
                    let pz = collider.maximum.z + collider.minimum.z;
                    let sx = (collider.maximum.x - collider.minimum.x);
                    let sy = (collider.maximum.y - collider.minimum.y);
                    let sz = (collider.maximum.z - collider.minimum.z);
                    let mut temp = Matrix::identity();
                    let mut rendermatrix = Matrix::identity();
                    CoordinateSytem3::matrix4_compose_no_rotation(&Vector3::new(sx, sy, sz), &Vector3::new(px, py, pz),&mut temp);
                    nodematrix.matrix.mul_to(&temp, &mut rendermatrix);
                    actions.mesh.pose.push(OpsAbstractMeshPose::ops(raybox, rendermatrix));
                }
            }

            actions.transform.enable.push(OpsNodeEnable::ops(raybox, result.is_some()));
        }
    }
    events.cursormoved = None;
}

pub fn run_loop<T>(mut app:  App, window: Arc<Window>, event_loop: EventLoop<T>) {
    app.insert_resource(DemoWindowEvent::default());
    app.add_systems(Update, sys_move_ray_collider);

    event_loop.run(move |event, elwt, flow| {
        match event {
            pi_winit::event::Event::NewEvents(_) => {},
            pi_winit::event::Event::WindowEvent { window_id, event } => {
                match event {
                    WindowEvent::CloseRequested => {
                        flow.set_exit()
                    },
                    WindowEvent::Resized(_) => {},
                    WindowEvent::Moved(ev) => {
                        log::error!("Moved Point: {:?}", (ev.x, ev.y));
                    },
                    WindowEvent::Destroyed => {},
                    WindowEvent::DroppedFile(_) => {},
                    WindowEvent::HoveredFile(_) => {},
                    WindowEvent::HoveredFileCancelled => {},
                    WindowEvent::ReceivedCharacter(_) => {},
                    WindowEvent::Focused(_) => {},
                    WindowEvent::KeyboardInput { device_id, input, is_synthetic } => {},
                    WindowEvent::ModifiersChanged(_) => {},
                    WindowEvent::Ime(_) => {},
                    WindowEvent::CursorMoved { device_id, position, modifiers } => {
                        // log::error!("CursorMoved: {:?}", (position.x, position.y));
                        if let Some(events) = app.world.get_resource_mut::<DemoWindowEvent>() {
                            events.cursormoved = Some((position.x as f32, position.y as f32));
                        }
                    },
                    WindowEvent::CursorEntered { device_id } => {},
                    WindowEvent::CursorLeft { device_id } => {},
                    WindowEvent::MouseWheel { device_id, delta, phase, modifiers } => {},
                    WindowEvent::MouseInput { device_id, state, button, modifiers } => {
                    },
                    WindowEvent::TouchpadPressure { device_id, pressure, stage } => {},
                    WindowEvent::AxisMotion { device_id, axis, value } => {},
                    WindowEvent::Touch(ev) => {
                        log::error!("Touch Point: {:?}", (ev.location.x, ev.location.y));
                    },
                    WindowEvent::ScaleFactorChanged { scale_factor, new_inner_size } => {},
                    WindowEvent::ThemeChanged(_) => {},
                    WindowEvent::Occluded(_) => {},
                }
            },
            pi_winit::event::Event::DeviceEvent { device_id, event } => {
                match event {
                    pi_winit::event::DeviceEvent::Added => {
                        log::error!("DeviceEvent::Added");
                    },
                    pi_winit::event::DeviceEvent::Removed => {
                        log::error!("DeviceEvent::Removed");
                    },
                    pi_winit::event::DeviceEvent::MouseMotion { delta } => {
                        // log::error!("MouseMotion: {:?}", delta);
                    },
                    pi_winit::event::DeviceEvent::MouseWheel { delta } => {
                        
                    },
                    pi_winit::event::DeviceEvent::Motion { axis, value } => {
                        
                    },
                    pi_winit::event::DeviceEvent::Button { button, state } => {
                        
                    },
                    pi_winit::event::DeviceEvent::Key(_) => {

                    },
                    pi_winit::event::DeviceEvent::Text { codepoint } => {
                        
                    },
                }
            },
            pi_winit::event::Event::UserEvent(event) => {
                
            },
            pi_winit::event::Event::Suspended => {},
            pi_winit::event::Event::Resumed => {
            },
            pi_winit::event::Event::MainEventsCleared => {
                window.request_redraw();
            },
            pi_winit::event::Event::RedrawRequested(_) => {
    
                #[cfg(feature = "dhat-heap")]
                let _profiler = dhat::Profiler::new_heap();
                app.update();
            },
            pi_winit::event::Event::RedrawEventsCleared => {},
            pi_winit::event::Event::LoopDestroyed => {},
        }
        
    })
}