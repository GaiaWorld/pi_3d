

use pi_3d::PluginBundleDefault;
use pi_animation::{loop_mode::ELoopMode, amount::AnimationAmountCalc, animation_group::AnimationGroupID};
use pi_atom::Atom;
use pi_bevy_ecs_extend::system_param::layer_dirty::ComponentEvent;
use pi_bevy_render_plugin::PiRenderPlugin;
use pi_curves::{curve::frame_curve::FrameCurve, easing::EEasingMode};
use pi_engine_shell::{prelude::*, frame_time::PluginFrameTime};
use pi_gltf2_load::*;
use pi_node_materials::prelude::*;
use pi_particle_system::PluginParticleSystem;
use pi_scene_context::{prelude::*, light::base::Light};
use pi_scene_math::{Vector3, Vector4};
use pi_mesh_builder::{cube::*, ball::*, quad::PluginQuadBuilder};
use unlit_material::{PluginUnlitMaterial, command::*, shader::UnlitShader};

use std::{sync::Arc, mem::replace, ops::DerefMut};
use pi_async_rt::rt::AsyncRuntime;
use pi_hal::{init_load_cb, runtime::MULTI_MEDIA_RUNTIME, on_load};

pub struct PluginLocalLoad;
impl Plugin for PluginLocalLoad {
    fn build(&self, app: &mut App) {
        
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

pub fn sys_nodeinfo(
    materials: Query<&MaterialRefs>,
    geometries: Query<&GeometryDesc>,
    transformnodes: Query<&TransformNode>,
    meshes: Query<&Mesh>,
    instancemeshes: Query<&InstanceMesh>,
    cameras: Query<&Camera>,
    renderers: Query<&Renderer>,
    lights: Query<&Light>,
    passes: Query<&ModelPass>,
    skeletons: Query<&Skeleton>,
    bones: Query<&BoneParent>,
    pipeline_center: Res<AssetDataCenterPipeline3D>,
    asset_mgr_bindgroup: Res<ShareAssetMgr<BindGroup>>,
    asset_mgr_bindgroup_layout: Res<ShareAssetMgr<BindGroupLayout>>,
) {
    let count_material = materials.iter().count();
    let count_geometry = geometries.iter().count();
    let count_transform = transformnodes.iter().count();
    let count_mesh = meshes.iter().count();
    let count_instance = instancemeshes.iter().count();
    let count_camera = cameras.iter().count();
    let count_renderer = renderers.iter().count();
    let count_light = lights.iter().count();
    let count_pass = passes.iter().count();
    let count_skeleton = skeletons.iter().count();
    let count_bone = bones.iter().count();

    let count_pipeline = pipeline_center.0.asset_mgr().len();
    let count_bindgroup = asset_mgr_bindgroup.0.len();
    let count_bindgrouplayout = asset_mgr_bindgroup_layout.0.len();

    log::warn!(
        "Materials: {:?}, Geometry: {:?}, Transform: {:?}, Mesh: {:?}, InstanceMesh: {:?}, Camera: {:?}, Renderer: {:?}, Light: {:?}, Pass: {:?}, Skeleton: {:?}, Bone: {:?}, Pipeline: {:?}, BindGroup: {:?}, BindGroupLayout: {:?}",
        count_material,
        count_geometry,
        count_transform,
        count_mesh,
        count_instance,
        count_camera,
        count_renderer,
        count_light,
        count_pass,
        count_skeleton,
        count_bone,
        count_pipeline,
        count_bindgroup,
        count_bindgrouplayout,
    );
}

pub fn test_plugins() -> App {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

    let mut app = App::default();

	let mut window_plugin = WindowPlugin::default();
    if let Some(primary_window) = &mut window_plugin.primary_window {
        primary_window.resolution.set_physical_resolution(800, 600);
    }

    app.insert_resource(AssetMgrConfigs::default());
    app.add_plugin(InputPlugin::default());
    app.add_plugin(window_plugin);
    app.add_plugin(AccessibilityPlugin);
    app.add_plugin(bevy::winit::WinitPlugin::default());
    // .add_plugin(WorldInspectorPlugin::new())
    app.add_plugin(pi_bevy_asset::PiAssetPlugin::default());
    app.add_plugin(PiRenderPlugin::default());
    app.add_plugin(PluginLocalLoad);
    app.add_plugin(PluginFrameTime);
    app.add_plugin(PluginWindowRender);
    app.add_plugins(PluginBundleDefault);
    app.add_plugin(PluginCubeBuilder);
    app.add_plugin(PluginQuadBuilder);
    app.add_plugin(PluginStateToFile);
    app.add_plugin(PluginNodeMaterial);
    app.add_plugin(PluginUnlitMaterial);
    app.add_plugins(PluginGroupNodeMaterialAnime);
    app.add_plugin(pi_3d::PluginSceneTimeFromPluginFrame);

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
    app.add_plugin(InputPlugin::default());
    app.add_plugin(window_plugin);
    app.add_plugin(AccessibilityPlugin);
    app.add_plugin(bevy::winit::WinitPlugin::default());
    // .add_plugin(WorldInspectorPlugin::new())
    app.add_plugin(pi_bevy_asset::PiAssetPlugin::default());
    app.add_plugin(PiRenderPlugin::default());
    app.add_plugin(PluginLocalLoad);
    app.add_plugin(PluginFrameTime);
    app.add_plugin(PluginWindowRender);
    app.add_plugins(PluginBundleDefault);
    app.add_plugin(PluginCubeBuilder);
    app.add_plugin(PluginQuadBuilder);
    app.add_plugin(PluginStateToFile);
    app.add_plugin(PluginNodeMaterial);
    app.add_plugin(PluginUnlitMaterial);
    app.add_plugins(PluginGroupNodeMaterialAnime);
    app.add_plugin(pi_3d::PluginSceneTimeFromPluginFrame);
    app.add_plugin(PluginParticleSystem);
    app.add_plugin(pi_gltf2_load::PluginGLTF2Res);
    app.add_plugin(pi_trail_renderer::PluginTrail);

    app.world.get_resource_mut::<WindowRenderer>().unwrap().active = true;
    
    app
}