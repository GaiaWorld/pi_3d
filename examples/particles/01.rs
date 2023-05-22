
use default_render::{SingleIDBaseDefaultMaterial, shader::DefaultShader};
use pi_3d::PluginBundleDefault;
use pi_animation::{loop_mode::ELoopMode, amount::AnimationAmountCalc};
use pi_atom::Atom;
use pi_bevy_ecs_extend::system_param::layer_dirty::ComponentEvent;
use pi_bevy_render_plugin::PiRenderPlugin;
use pi_curves::{curve::frame_curve::FrameCurve, easing::EEasingMode};
use pi_engine_shell::{prelude::*, frame_time::PluginFrameTime,};
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
    let tes_size = 50;
    fps.frame_ms = 4;

    final_render.cleardepth = 0.0;

    let scene = commands.spawn_empty().id();
    scenecmds.push(OpsSceneCreation::ops(scene, ScenePassRenderCfg::default()));

    let camera01 = commands.spawn_empty().id();
    cameracmds.0.push(OpsCameraCreation::ops(scene, camera01, String::from("TestCamera"), true));
    transformcmds.1.push(OpsTransformNodeLocalPosition::ops(camera01, 0., 0., -10.));
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
    cameracmds.3.push(OpsCameraRendererInit::ops(camera01, id_renderer, desc, ColorFormat::Rgba8Unorm, None));

    let source = commands.spawn_empty().id();
    transformcmds.3.push(OpsMeshCreation::ops(scene, source, String::from("TestCube")));
    
    let id_geo = commands.spawn_empty().id();
    let mut attrs = CubeBuilder::attrs_meta();
    attrs.push(VertexBufferDesc::instance_world_matrix());
    attrs.push(VertexBufferDesc::instance_color());
    geometrycreate.push(OpsGeomeryCreate::ops(source, id_geo, attrs, Some(CubeBuilder::indices_meta())));

    let idmat = commands.spawn_empty().id();
    matcmds.0.push(OpsMaterialUse::ops(source, idmat));
    matcmds.1.push(OpsMaterialCreate::ops(idmat, UnlitShader::KEY, EPassTag::Opaque));
    // matcmds.2.push(EUniformCommand::Texture(idmat, UniformTextureWithSamplerParam {
    //     slotname: Atom::from("_MainTex"),
    //     filter: true,
    //     sample: KeySampler::default(),
    //     url: KeyTexture::from("E:/Rust/PI/pi_3d/assets/images/bubbles.png"),
    // }, true));
    
    commands.entity(source).insert(Particle);
}

#[derive(Component)]
pub struct Particle;

fn sys_demo_particle(
    particles: Query<(&SceneID, &GeometryID), With<Particle>>,
    scenes: Query<&SceneTime>,
    mut geometrys: Query<(&mut InstanceBufferWorldMatrix, &mut InstanceBufferColor)>,
    mut geoloader: ResMut<GeometryVBLoader>,
    mut vb_data_map: ResMut<VertexBufferDataMap3D>,
) {
    particles.iter().for_each(|(idscene, idgeo)| {
        if let Ok(scenetime) = scenes.get(idscene.0) {
            if let Ok((mut wm, mut colors)) = geometrys.get_mut(idgeo.0) {
                let mut buffermatrix = vec![];
                let mut buffercolor = vec![];
            
                for z in 0..20 {
                    let ringcount = (z + 1) * 10;
                    let tt = if z % 2 == 0 {
                        scenetime.time_ms as f32 * 0.002
                    } else {
                        scenetime.time_ms as f32 * 0.002 * -1.
                    };
                    for x in 0..ringcount {
                        let t: f32 = (tt + x as f32 * (1. / ringcount as f32)) * 3.1415926 * 2.;
                        let mut wm = Matrix::identity();
                        wm.append_translation_mut(
                            &Vector3::new(
                                f32::cos(t) * 2. * ( z as f32 + 1.0),
                                f32::sin(t) * 2. * ( z as f32 + 1.0),
                                0.,
                            )
                        );
                        buffermatrix.push(wm);

                        buffercolor.push(Vector4::new(
                            f32::cos(tt + x as f32) * 0.5 + 0.5,
                            f32::sin(tt) * 0.5 + 0.5,
                            f32::sin(tt + z as f32) * 0.5 + 0.5,
                            f32::cos(tt) * 0.5 + 0.5,
                        ));
                    }
                }

                let mut colordata : Vec<u8> = vec![];
                buffercolor.iter().for_each(|v| {
                    bytemuck::cast_slice(v.as_slice()).iter().for_each(|v| {
                        colordata.push(*v);
                    })
                });
                
                let mut wmdata: Vec<u8> = vec![];
                buffermatrix.iter().for_each(|v| {
                    bytemuck::cast_slice(v.as_slice()).iter().for_each(|v| {
                        wmdata.push(*v);
                    })
                });

                geometry_update_instance_buffer::<InstanceBufferWorldMatrix>(Some(wmdata), idgeo.0, &mut wm, &mut geoloader, &mut vb_data_map);
                geometry_update_instance_buffer::<InstanceBufferColor>(Some(colordata), idgeo.0, &mut colors, &mut geoloader, &mut vb_data_map);
            }
        }
    });
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
    app.add_plugin(PluginUnlitMaterial);
    
    app.add_system(
        sys_demo_particle.in_set(ERunStageChap::CalcRenderMatrix)
    );

    app.add_startup_system(setup);
    // bevy_mod_debugdump::print_main_schedule(&mut app);
    
    app.run()

}