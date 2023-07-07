#![feature(box_into_inner)]

use default_render::{SingleIDBaseDefaultMaterial, shader::DefaultShader};
use pi_3d::PluginBundleDefault;
use pi_animation::{loop_mode::ELoopMode, amount::AnimationAmountCalc};
use pi_atom::Atom;
use pi_bevy_ecs_extend::system_param::layer_dirty::ComponentEvent;
use pi_bevy_render_plugin::PiRenderPlugin;
use pi_curves::{curve::frame_curve::FrameCurve, easing::EEasingMode};
use pi_engine_shell::{prelude::*, frame_time::PluginFrameTime, };
use pi_node_materials::{NodeMaterialBlocks, PluginNodeMaterial};
use pi_scene_context::prelude::*;
use pi_scene_math::*;
use pi_mesh_builder::{cube::*, ball::*, quad::PluginQuadBuilder};
use unlit_material::{PluginUnlitMaterial, command::*, shader::UnlitShader};

use std::sync::Arc;
use pi_async::rt::AsyncRuntime;
use pi_hal::{init_load_cb, runtime::MULTI_MEDIA_RUNTIME, on_load};

pub struct PluginLocalLoad;
impl Plugin for PluginLocalLoad {
    fn build(&self, app: &mut App) {
        
        init_load_cb(Arc::new(|path: String| {
            MULTI_MEDIA_RUNTIME
                .spawn(MULTI_MEDIA_RUNTIME.alloc(), async move {
                    log::debug!("Load {}", path);
                    let r = std::fs::read(path.clone()).unwrap();
                    on_load(&path, r);
                })
                .unwrap();
        }));
    }
}

fn setup(
    mut commands: Commands,
    mut scenecmds: ActionSetScene,
    mut cameracmds: ActionSetCamera,
    mut transformcmds: ActionSetTransform,
    mut transformanime: ActionSetTransformNodeAnime,
    mut meshcmds: ActionSetMesh,
    mut instancemeshcmds: ActionSetInstanceMesh,
    mut skincmds: ActionSetSkeleton,
    mut geometrycmd: ActionSetGeometry,
    mut matcmds: ActionSetMaterial,
    mut animegroupcmd: ActionSetAnimationGroup,
    mut fps: ResMut<SingleFrameTimeCommand>,
    mut final_render: ResMut<WindowRenderer>,
    nodematblocks: Res<NodeMaterialBlocks>,
    defaultmat: Res<SingleIDBaseDefaultMaterial>,
    mut renderercmds: ActionSetRenderer,
) {
    let tes_size = 5;
    fps.frame_ms = 4;

    final_render.cleardepth = 0.0;

    let scene = commands.spawn_empty().id();
    animegroupcmd.scene_ctxs.init_scene(scene);
    scenecmds.create.push(OpsSceneCreation::ops(scene, ScenePassRenderCfg::default()));

    let camera01 = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(camera01, scene));
    cameracmds.create.push(OpsCameraCreation::ops(scene, camera01, String::from("TestCamera"), true));
    transformcmds.localpos.push(OpsTransformNodeLocalPosition::ops(camera01, 0., 0., -10.));
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

    let source = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(source, scene));
    meshcmds.create.push(OpsMeshCreation::ops(scene, source, String::from("TestCube")));
    transformcmds.tree.push(OpsTransformNodeParent::ops(source, scene));
    
    let key_group = pi_atom::Atom::from("key_group");
    let id_group = animegroupcmd.scene_ctxs.create_group(scene).unwrap();
    animegroupcmd.global.record_group(source, id_group);
    animegroupcmd.attach.push(OpsAnimationGroupAttach::ops(scene, source, id_group));
    
    let bone0 = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(bone0, scene));
    let bone1 = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(bone1, scene));
    let key_curve0 = pi_atom::Atom::from((1).to_string());
    let curve = FrameCurve::<LocalPosition>::curve_easing(LocalPosition(Vector3::new(0., 0., 0.)), LocalPosition(Vector3::new(1., 0., 0.)), 30, 30, EEasingMode::None);
    if let Ok(asset_curve) = transformanime.position.curves.insert(key_curve0, TypeFrameCurve(curve)) {
        let animation = transformanime.position.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
        animegroupcmd.scene_ctxs.add_target_anime(scene, bone1, id_group.clone(), animation);
    }
    let bone2 = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(bone2, scene));
    let key_curve0 = pi_atom::Atom::from((2).to_string());
    let curve = FrameCurve::<LocalPosition>::curve_easing(LocalPosition(Vector3::new(0., 0., 0.)), LocalPosition(Vector3::new(-1., 0., 0.)), 30, 30, EEasingMode::None);
    if let Ok(asset_curve) = transformanime.position.curves.insert(key_curve0, TypeFrameCurve(curve)) {
        let animation = transformanime.position.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
        animegroupcmd.scene_ctxs.add_target_anime(scene, bone2, id_group.clone(), animation);
    }
    let bone3 = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(bone3, scene));
    let key_curve0 = pi_atom::Atom::from((3).to_string());
    let curve = FrameCurve::<LocalPosition>::curve_easing(LocalPosition(Vector3::new(0., 0., 0.)), LocalPosition(Vector3::new(0., 1., 0.)), 30, 30, EEasingMode::None);
    if let Ok(asset_curve) = transformanime.position.curves.insert(key_curve0, TypeFrameCurve(curve)) {
        let animation = transformanime.position.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
        animegroupcmd.scene_ctxs.add_target_anime(scene, bone3, id_group.clone(), animation);
    }
    let bone4 = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(bone4, scene));
    let key_curve0 = pi_atom::Atom::from((4).to_string());
    let curve = FrameCurve::<LocalPosition>::curve_easing(LocalPosition(Vector3::new(0., 0., 0.)), LocalPosition(Vector3::new(0., -1., 0.)), 30, 30, EEasingMode::None);
    if let Ok(asset_curve) = transformanime.position.curves.insert(key_curve0, TypeFrameCurve(curve)) {
        let animation = transformanime.position.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
        animegroupcmd.scene_ctxs.add_target_anime(scene, bone4, id_group.clone(), animation);
    }
    animegroupcmd.scene_ctxs.start_with_progress(scene, id_group.clone(), AnimationGroupParam::default());

    skincmds.bone_create.push(OpsBoneCreation::ops(bone0, scene, scene, String::from("Bone00")));
    skincmds.bone_create.push(OpsBoneCreation::ops(bone1, bone0, scene, String::from("Bone01")));
    skincmds.bone_create.push(OpsBoneCreation::ops(bone2, bone0, scene, String::from("Bone02")));
    skincmds.bone_create.push(OpsBoneCreation::ops(bone3, bone0, scene, String::from("Bone03")));
    skincmds.bone_create.push(OpsBoneCreation::ops(bone4, bone0, scene, String::from("Bone04")));
    // transformcmds.tree.push(OpsTransformNodeParent::ops(bone0, scene));
    // transformcmds.tree.push(OpsTransformNodeParent::ops(bone1, bone0));
    // transformcmds.tree.push(OpsTransformNodeParent::ops(bone2, bone0));
    // transformcmds.tree.push(OpsTransformNodeParent::ops(bone3, bone0));
    // transformcmds.tree.push(OpsTransformNodeParent::ops(bone4, bone0));

    let data: [u16; 48] = [
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
        1, 1, 1, 1, 1, 1, 1, 1, 
        2, 2, 2, 2, 2, 2, 2, 2, 
        3, 3, 3, 3, 3, 3, 3, 3, 
        4, 4, 4, 4, 4, 4, 4, 4
    ];
    // normals
    let jointkey = KeyVertexBuffer::from("TestJoint");
    geometrycmd.vb_wait.add(&jointkey, bytemuck::cast_slice(&data).iter().map(|v| *v).collect::<Vec<u8>>());

    let format = wgpu::VertexFormat::Uint16x2;
    let jointdesc = VertexBufferDesc::vertices(jointkey.clone(), None, vec![VertexAttribute { kind: EVertexDataKind::MatricesIndices1, format }]);
    
    let id_geo = commands.spawn_empty().id();
    let mut attrs = CubeBuilder::attrs_meta();
    attrs.push(jointdesc);
    geometrycmd.create.push(OpsGeomeryCreate::ops(source, id_geo, attrs, Some(CubeBuilder::indices_meta())));

    let idmat = commands.spawn_empty().id();
    matcmds.usemat.push(OpsMaterialUse::ops(source, idmat));
    matcmds.create.push(OpsMaterialCreate::ops(idmat, UnlitShader::KEY, EPassTag::Opaque));
    matcmds.texture.push(OpsUniformTexture::ops(idmat, UniformTextureWithSamplerParam {
        slotname: Atom::from("_MainTex"),
        filter: true,
        sample: KeySampler::default(),
        url: EKeyTexture::from("E:/Rust/PI/pi_3d/assets/images/bubbles.png"),
    }));
    
    let skeleton = commands.spawn_empty().id();
    skincmds.skin_create.push(OpsSkinCreation::ops(skeleton, ESkinBonesPerVertex::One, bone0, &vec![bone0, bone1, bone2, bone3, bone4]));
    skincmds.skin_use.push(OpsSkinUse::ops(source, skeleton));

    transformcmds.localrot.push(OpsTransformNodeLocalEuler::ops(source, 1. as f32 * 0.2, 1. as f32 * 0.2, 1. as f32 * 0.2));
    meshcmds.cullmode.push(OpsCullMode::ops(source, CullMode::Off));
}

    // pub fn setup(
    //     engine: &pi_engine_shell::engine_shell::EnginShell,
    // ) {

    //     let tes_size = 5;
    //     let testdata = engine.world().get_resource_mut::<SingleTestData>().unwrap();

    //     engine.frame_time(4);

    //     // Test Code
    //     let scene01 = engine.create_scene();
    //     let camera01 = engine.create_free_camera(scene01);
    //     engine.active_camera(camera01, true);
    //     engine.layer_mask(camera01, LayerMask::default());
    //     engine.transform_position(camera01, Vector3::new(0., 0., -10.));
    //     engine.free_camera_orth_size(camera01, tes_size as f32);

    //     // let matid = engine.create_default_material();
    //     // engine.emissive_intensity(entity, intensity);
    //     let unlitmaterial = engine.create_unlit_material(EPassTag::Opaque);
    //     engine.set_texture(
    //         unlitmaterial, 
    //         UniformTextureWithSamplerParam {
    //             slotname: Atom::from("_MainTex"),
    //             filter: true,
    //             sample: KeySampler::default(),
    //             url: EKeyTexture::from("E:/Rust/PI/pi_3d/assets/images/top.jpg"),
    //         },
    //         false
    //     );

    //     let source = engine.create_mesh(scene01);
    //     let mut attrs = CubeBuilder::attrs_meta();

    //     let bone0 = engine.create_transform_node(scene01);
    //     testdata.transforms.push((bone0, 0., 0., 0.));
    //     let bone1 = engine.create_transform_node(scene01);
    //     testdata.transforms.push((bone1, 1., 0., 0.));
    //     let bone2 = engine.create_transform_node(scene01);
    //     testdata.transforms.push((bone2, -1., 0., 0.));
    //     let bone3 = engine.create_transform_node(scene01);
    //     testdata.transforms.push((bone3, 0., 1., 0.));
    //     let bone4 = engine.create_transform_node(scene01);
    //     testdata.transforms.push((bone4, 0., -1., 0.));
    //     engine.transform_parent(bone1, bone0);
    //     engine.transform_parent(bone2, bone0);
    //     engine.transform_parent(bone3, bone0);
    //     engine.transform_parent(bone4, bone0);

    //     let device = engine.world().get_resource::<RenderDevice>().unwrap();
    //     let queue = engine.world().get_resource::<RenderQueue>().unwrap();
    //     let data: [u16; 48] = [
    //         0, 0, 0, 0, 0, 0, 0, 0,
    //         0, 0, 0, 0, 0, 0, 0, 0,
    //         1, 1, 1, 1, 1, 1, 1, 1, 
    //         2, 2, 2, 2, 2, 2, 2, 2, 
    //         3, 3, 3, 3, 3, 3, 3, 3, 
    //         4, 4, 4, 4, 4, 4, 4, 4
    //     ];
    //     // normals
    //     let jointkey = pi_atom::Atom::from("TestJoint");
    //     engine.create_vertex_buffer(jointkey.clone(), bytemuck::cast_slice(&data).iter().map(|v| *v).collect::<Vec<u8>>());

    //     let format = wgpu::VertexFormat::Uint16x2;
    //     let jointdesc = VertexBufferDesc::vertices(jointkey.clone(), None, vec![VertexAttribute { kind: EVertexDataKind::MatricesIndices1, format }]);
    //     attrs.push(jointdesc);

    //     engine.use_geometry(source, attrs, Some(CubeBuilder::indices_meta()));
    //     engine.use_material(source, unlitmaterial);
    //     engine.layer_mask(source, LayerMask::default());
    //     engine.transform_rotation_euler(source, Vector3::new(1. as f32 * 0.2, 1. as f32 * 0.2, 1. as f32 * 0.2));
    //     engine.cull_mode(source, ECullMode::Off);

    //     let skeleton = engine.create_skeleton_ubo(ESkinBonesPerVertex::One, bone0, vec![bone0, bone1, bone2, bone3, bone4]);
    //     engine.use_skeleton(source, skeleton);
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
    app.add_plugin(pi_bevy_asset::PiAssetPlugin::default());
    app.add_plugin(PiRenderPlugin::default());
    app.add_plugin(PluginLocalLoad);
    app.add_plugin(PluginTest);
    app.add_plugin(PluginFrameTime);
    app.add_plugin(PluginWindowRender);
    app.add_plugins(PluginBundleDefault);
    app.add_plugin(PluginCubeBuilder);
    app.add_plugin(PluginQuadBuilder);
    app.add_plugin(PluginStateToFile);
    app.add_plugin(PluginNodeMaterial);
    app.add_plugin(PluginUnlitMaterial);
    app.add_plugin(pi_3d::PluginSceneTimeFromPluginFrame);

    app.world.get_resource_mut::<WindowRenderer>().unwrap().active = true;
    
    app.add_startup_system(setup);
    // bevy_mod_debugdump::print_main_schedule(&mut app);
    
    app.run()

}