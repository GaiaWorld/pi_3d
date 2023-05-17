#![feature(box_into_inner)]

use default_render::SingleIDBaseDefaultMaterial;
use pi_3d::PluginBundleDefault;
use pi_animation::{loop_mode::ELoopMode, amount::AnimationAmountCalc};
use pi_atom::Atom;
use pi_bevy_ecs_extend::system_param::layer_dirty::ComponentEvent;
use pi_bevy_render_plugin::PiRenderPlugin;
use pi_curves::{curve::frame_curve::FrameCurve, easing::EEasingMode};
use pi_engine_shell::{prelude::*, frame_time::PluginFrameTime, };
use pi_node_materials::{prelude::*, NodeMaterialBlocks, PluginNodeMaterial};
use pi_scene_context::{prelude::*, materials::{uniforms::sys_uniform::{ActionListUniform, EUniformCommand, ActionListUniformByName, OpsUniformByName}, shader_effect::{AssetKeyShaderEffect, AssetResShaderEffectMeta}}, renderers::render_blend::{ActionListBlend, OpsRenderBlend, ModelBlend}, geometry::ActionVertexBuffer};
use pi_scene_math::{Vector3, Vector4, Vector2};
use pi_mesh_builder::{cube::*, ball::*, quad::{PluginQuadBuilder, QuadBuilder}};
use unlit_material::{PluginUnlitMaterial, command::*, shader::UnlitShader, effects::{main_opacity::MainOpacityShader, main_opacity_fresnel::MainOpacityFresnelShader, two_opacity_mix::TwoOpacityMixShader}};

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
        ResMut<ActionListInstanceTillOff>,
    ),
    mut geometrycreate: ResMut<ActionListGeometryCreate>,
    mut matcmds: (
        ResMut<ActionListMaterialUse>,
        ResMut<ActionListMaterialCreate>,
        ResMut<ActionListUniformByName>,
        Res<ShareAssetMgr<ShaderEffectMeta>>,
        ResMut<AssetSyncWait<KeyShaderMeta, AssetKeyShaderEffect, ShaderEffectMeta, AssetResShaderEffectMeta>>,
        ResMut<ActionListBlend>,
        ResMut<NodeMaterialBlocks>,
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
    ActionMaterial::regist_material_meta(&matcmds.3, &mut matcmds.4, KeyShaderMeta::from(TwoOpacityMixShader::KEY), TwoOpacityMixShader::create(&matcmds.6));

    let tes_size = 5;
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
    let id_renderer = commands.spawn_empty().id();
    cameracmds.3.push(OpsCameraRendererInit::ops(camera01, id_renderer, desc, wgpu::TextureFormat::Rgba8Unorm, None));

    let source = commands.spawn_empty().id();
    transformcmds.3.push(OpsMeshCreation(scene, source, String::from("TestCube")));
    let mut blend = ModelBlend::default(); blend.combine();
    matcmds.5.push(OpsRenderBlend::ops(source, blend));
    
    let id_geo = commands.spawn_empty().id();
    geometrycreate.push(OpsGeomeryCreate::ops(source, id_geo, 
        CubeBuilder::attrs_meta(),
        Some(CubeBuilder::indices_meta())
    ));

    let idmat = commands.spawn_empty().id();
    matcmds.0.push(OpsMaterialUse::ops(source, idmat));
    matcmds.1.push(OpsMaterialCreate::ops(idmat, TwoOpacityMixShader::KEY, EPassTag::Transparent));
    matcmds.2.push(OpsUniformByName::Texture(idmat, UniformTextureWithSamplerParam {
        slotname: Atom::from(BlockMainTexture::KEY_TEX),
        filter: true,
        sample: KeySampler::linear_repeat(),
        url: KeyTexture::from("E:/Rust/PI/pi_3d/assets/images/fractal.png"),
    }, true));
    matcmds.2.push(OpsUniformByName::Texture(idmat, UniformTextureWithSamplerParam {
        slotname: Atom::from(BlockOpacityTexture::KEY_TEX),
        filter: true,
        sample: KeySampler::linear_repeat(),
        url: KeyTexture::from("E:/Rust/PI/pi_3d/assets/images/eff_ui_ll_085.png"),
    }, true));
    matcmds.2.push(OpsUniformByName::Texture(idmat, UniformTextureWithSamplerParam {
        slotname: Atom::from(BlockOpacity2Texture::KEY_TEX),
        filter: true,
        sample: KeySampler::linear_repeat(),
        url: KeyTexture::from("E:/Rust/PI/pi_3d/assets/images/eff_uv_lf_002.png"),
    }, true));
    matcmds.2.push(OpsUniformByName::Texture(idmat, UniformTextureWithSamplerParam {
        slotname: Atom::from(BlockMixTexture::KEY_TEX),
        filter: true,
        sample: KeySampler::linear_repeat(),
        url: KeyTexture::from("E:/Rust/PI/pi_3d/assets/images/icon_city.png"),
    }, true));
    matcmds.2.push(
        OpsUniformByName::Vec4(
            idmat, 
            String::from(BlockEmissiveBase::KEY_INFO), 
            Vector4::new(1., 1., 1., 1.),
            true
        )
    );
    matcmds.2.push(
        OpsUniformByName::Vec2(
            idmat, 
            String::from(BlockOpacityTextureUVOffsetSpeed::KEY_PARAM), 
            Vector2::new(1., 1.),
            true
        )
    );
    matcmds.2.push(
        OpsUniformByName::Vec2(
            idmat, 
            String::from(BlockOpacity2TextureUVOffsetSpeed::KEY_PARAM), 
            Vector2::new(1., 1.),
            true
        )
    );
    matcmds.2.push(
        OpsUniformByName::Float(
            idmat, 
            String::from(TwoOpacityMixShader::KEY_MIX_CONTROL), 
            1.0,
            true
        )
    );
}

fn sys_setup_ball(
    asset_mgr: Res<ShareAssetMgr<EVertexBufferRange>>,
    mut data_map: ResMut<VertexBufferDataMap3D>,
) {
    let param = BallParam { sectors: 20, stacks: 20 };

    let (positions, normals, indices, uvs) = generate_sphere(&param);
    ActionVertexBuffer::create(&mut data_map, KeyVertexBuffer::from("BallPos#20#20"), bytemuck::cast_slice(&positions).iter().map(|v| *v).collect::<Vec<u8>>());
    ActionVertexBuffer::create(&mut data_map, KeyVertexBuffer::from("BallNor#20#20"), bytemuck::cast_slice(&normals).iter().map(|v| *v).collect::<Vec<u8>>());
    ActionVertexBuffer::create(&mut data_map, KeyVertexBuffer::from("BallUV#20#20"), bytemuck::cast_slice(&uvs).iter().map(|v| *v).collect::<Vec<u8>>());
    ActionVertexBuffer::create_indices(&mut data_map, KeyVertexBuffer::from("BallInd#20#20"), bytemuck::cast_slice(&indices).iter().map(|v| *v).collect::<Vec<u8>>());
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
    app.add_plugin(PluginNodeMaterial);
    app.add_plugin(PluginUnlitMaterial);
    
    app.add_startup_system(sys_setup_ball);
    app.add_startup_system(setup);
    // bevy_mod_debugdump::print_main_schedule(&mut app);
    
    app.run()

}