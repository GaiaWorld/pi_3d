use default_render::{shader::DefaultShader, SingleIDBaseDefaultMaterial};
use particle::{
    emitter::ishape_emitter_type::EShapeEmitterArcMode,
    extend::format_mesh_particle,
    iparticle_system_config::{
        FourGradientInfo, IParticleSystemConfig, IShape, IShapeArc, IShapeArcRandom, IShapeCone,
        OneParamInfo, ParamInfo, ThreeParamInfo,
    },
    mesh_particle_system::MeshParticleSystem,
    particle_system_tool::{
        EMeshParticleScaleMode, EMeshParticleSpaceMode, ERenderAlignment, ERenderMode,
    },
};
use pi_3d::PluginBundleDefault;
use pi_animation::{amount::AnimationAmountCalc, loop_mode::ELoopMode};
use pi_async_rt::rt::AsyncRuntime;
use pi_atom::Atom;
use pi_bevy_ecs_extend::system_param::layer_dirty::ComponentEvent;
use pi_bevy_render_plugin::PiRenderPlugin;
use pi_curves::{curve::frame_curve::FrameCurve, easing::EEasingMode};
use pi_engine_shell::{frame_time::PluginFrameTime, prelude::*};
use pi_hal::{init_load_cb, on_load, runtime::MULTI_MEDIA_RUNTIME};
use pi_mesh_builder::{ball::*, cube::*, quad::PluginQuadBuilder};
use pi_node_materials::{NodeMaterialBlocks, PluginNodeMaterial, prelude::{MainColor, BlockMainTexture}};
use pi_scene_context::{
    prelude::*,
    viewer::prelude::{ViewerGlobalPosition, ViewerViewMatrix},
};
use pi_scene_math::*;
use std::sync::Arc;
use unlit_material::{command::*, shader::UnlitShader, PluginUnlitMaterial};

pub struct PluginLocalLoad;
impl Plugin for PluginLocalLoad {
    fn build(&self, app: &mut App) {
        init_load_cb(Arc::new(|path: String| {
            MULTI_MEDIA_RUNTIME
                .spawn(MULTI_MEDIA_RUNTIME.alloc(), async move {
                    log::warn!("Load {}", path);
                    if let Ok(r) = std::fs::read(path.clone()) {
                        on_load(&path, r);
                    } else {
                        log::error!("Load Error: {:?}", path);
                    }
                    // let r = std::fs::read(path.clone()).unwrap();
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
    mut geometrycmd: ActionSetGeometry,
    mut matcmds: ActionSetMaterial,
    mut animegroupcmd: ActionSetAnimationGroup,
    mut fps: ResMut<SingleFrameTimeCommand>,
    mut final_render: ResMut<WindowRenderer>,
    nodematblocks: Res<NodeMaterialBlocks>,
    defaultmat: Res<SingleIDBaseDefaultMaterial>,
    mut renderercmds: ActionSetRenderer,
) {
    let tes_size = 10;
    fps.frame_ms = 16;

    final_render.cleardepth = 0.0;

    let scene = commands.spawn_empty().id();
    scenecmds
        .create
        .push(OpsSceneCreation::ops(scene, ScenePassRenderCfg::default()));

    let camera01 = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(camera01, scene));
    cameracmds.create.push(OpsCameraCreation::ops(
        scene,
        camera01,
        String::from("TestCamera"),
        true,
    ));
    transformcmds
        .localpos
        .push(OpsTransformNodeLocalPosition::ops(camera01, 0., 0., -10.));
    cameracmds.active.push(OpsCameraActive::ops(camera01, true));
    cameracmds
        .size
        .push(OpsCameraOrthSize::ops(camera01, tes_size as f32));
    // localrulercmds.push(OpsTransformNodeLocalEuler(camera01, Vector3::new(3.1415926 / 4., 0., 0.)));

    let desc = RendererGraphicDesc {
        pre: Some(final_render.clear_entity),
        curr: String::from("TestCamera"),
        next: Some(final_render.render_entity),
        passorders: PassTagOrders::new(vec![
            EPassTag::Opaque,
            EPassTag::Water,
            EPassTag::Sky,
            EPassTag::Transparent,
        ]),
    };
    let id_renderer = commands.spawn_empty().id(); renderercmds.create.push(OpsRendererCreate::ops(id_renderer, desc.curr.clone()));
    renderercmds.connect.push(OpsRendererConnect::ops(final_render.clear_entity, id_renderer));
    renderercmds.connect.push(OpsRendererConnect::ops(id_renderer, final_render.render_entity));
    cameracmds.render.push(OpsCameraRendererInit::ops(camera01, id_renderer, desc.curr, desc.passorders,
        ColorFormat::Rgba8Unorm,
        DepthStencilFormat::None,
    ));

    let source = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(source, scene));
    meshcmds.create.push(OpsMeshCreation::ops(
        scene,
        source,
        String::from("TestCube"),
    ));

    let id_geo = commands.spawn_empty().id();
    let mut attrs = CubeBuilder::attrs_meta();
    attrs.push(VertexBufferDesc::instance_world_matrix());
    attrs.push(VertexBufferDesc::instance_color());
    attrs.push(VertexBufferDesc::instance_tilloff());

    geometrycmd.create.push(OpsGeomeryCreate::ops(
        source,
        id_geo,
        attrs,
        Some(CubeBuilder::indices_meta()),
    ));

    let idmat = commands.spawn_empty().id();
    matcmds.usemat.push(OpsMaterialUse::ops(source, idmat));
    matcmds.create.push(OpsMaterialCreate::ops(
        idmat,
        UnlitShader::KEY,
        EPassTag::Opaque,
    ));
    matcmds.texture.push(OpsUniformTexture::ops(
        idmat,
        UniformTextureWithSamplerParam {
            slotname: Atom::from("_MainTex"),
            filter: true,
            sample: KeySampler::default(),
            url: EKeyTexture::from("assets/images/bubbles.png"),
        },
    ));
    matcmds.vec4.push(
        OpsUniformVec4::ops(
            idmat,
            Atom::from(BlockMainTexture::KEY_COLOR),
            1., 1., 1., 1.
        )
    );

    let mut config = IParticleSystemConfig {
        name: "MP".to_string(),
        duration: 5.,
        start_delay: 0.,
        looping: 1,
        prewarm: true,
        simulation_space_is_world: EMeshParticleSpaceMode::Local,
        scaling_mode: EMeshParticleScaleMode::Hierarchy,
        render_alignment: ERenderAlignment::Local,
        render_mode: ERenderMode::Billboard,
        stretched_velocity_scale: 0.,
        stretched_length_scale: 0.,
        max_particles: 10.,
        start_speed: OneParamInfo::TInterpolateConstant(8.0),
        lifetime: OneParamInfo::TInterpolateConstant(2.0),
        delay: vec![0.],
        start_color: FourGradientInfo::TInterpolateColor([1., 1., 1., 1.]),
        start_size: ParamInfo::OneParamInfo(OneParamInfo::TInterpolateConstant(1.0)),
        start_rotation: ParamInfo::OneParamInfo(OneParamInfo::TInterpolateConstant(0.0)),
        gravity: OneParamInfo::TInterpolateConstant(0.0),
        emission: (
            5.0,
            Some(vec![
                [0., 100., 2., 0.1],
                [1000., 100., 100., 0.1],
                [2000., 100., 100., 0.1],
            ]),
        ),
        shape: IShape::ShapeCone(IShapeCone {
            _type: 0,
            radius: 0.01,
            angle: 90.0,
            radius_thickness: 0.,
            arc: IShapeArc::IShapeArcRandom(IShapeArcRandom {
                mode: EShapeEmitterArcMode::Random,
                value: 360.,
                spread: 0.,
                speed: 1.,
            }),
            emit_as_volume: false,
            height: 0.,

            scale: default(),
            position: default(),
            rotation: default(),
            align_dir: default(),
            randomize: None,
        }),
        velocity_over_lifetime: Some(ParamInfo::ThreeParamInfo(
            ThreeParamInfo::TInterpolateConstant([0., 0., 0.]),
        )),
        limit_velocity_over_lifetime: Some(OneParamInfo::TInterpolateConstant(100.)),
        velocity_over_lifetime_is_local: default(),
        limit_velocity_over_lifetime_dampen: Some(0.9),
        force_over_lifetime: Some(ParamInfo::OneParamInfo(OneParamInfo::TInterpolateConstant(
            0.,
        ))),
        force_space_is_local: default(),
        color_over_lifetime: Some(FourGradientInfo::TInterpolateTwoColors(
            [0.5, 0.5, 0.5, 1.],
            [0.5, 0.5, 0., 1.],
        )),
        color_by_speed: default(),
        size_over_lifetime: Some(ParamInfo::OneParamInfo(OneParamInfo::TInterpolateConstant(
            0.25,
        ))),
        size_by_speed: default(),
        rotation_over_lifetime: Some(ParamInfo::ThreeParamInfo(
            ThreeParamInfo::TInterpolateConstant([0., 0., 0.]),
        )),
        rotation_by_speed: default(),
        texture_sheet: default(),
        texture: default(),
        trail: default(),
        orbtial_velocity: default(),
        orbital_offset: default(),
        orbital_radial: default(),
        speed_modifier: default(),
        render_pivot: default(),
        custom1: default(),
    };
    let mut mp = MeshParticleSystem::new();
    format_mesh_particle(&mut config, &mut mp);

    // println!("============= IParticleSystemConfig End: {:?}", c);
    mp.build();
    mp.start();
    commands.entity(source).insert(Particle(mp));
    println!("============= IParticleSystemConfig End:");

    // mp
}

#[derive(Component)]
pub struct Particle(MeshParticleSystem);

fn sys_demo_particle(
    mut particles: Query<(
        &SceneID,
        &GeometryID,
        &mut Particle,
        &WorldMatrix,
        &LocalMatrix,
    )>,
    scenes: Query<(&SceneTime, &SceneMainCameraID)>,
    cameras: Query<(&ViewerGlobalPosition, &ViewerViewMatrix)>,
    mut geometrys: Query<(
        &mut InstanceBufferWorldMatrix,
        &mut InstanceBufferColor,
        &mut InstanceBufferTillOff,
    )>,
    mut geoloader: ResMut<GeometryVBLoader>,
    mut vb_data_map: ResMut<VertexBufferDataMap3D>,
) {
    particles.iter_mut().for_each(
        |(idscene, idgeo, mut particle, world_matrix, local_matrix)| {

            let (mut camerapos, mut camera_rotation_matrix) =
                (Vector3::new(0., 0., -1.), Matrix::identity());

            if let Ok((v1, main_camera)) = scenes.get(idscene.0) {
                if let Some(main_crame) = main_camera.0 {
                    if let Ok((viewpos, wiewmat)) = cameras.get(main_crame) {
                        camerapos = viewpos.0;
                        camera_rotation_matrix = wiewmat.get_rotation_matrix();
                        if let Some(inverse) = camera_rotation_matrix.try_inverse(){
                            // println!("相机旋转矩阵有逆");
                            camera_rotation_matrix = inverse;
                        }
                    };
                }
            }
            // if let Ok(scenetime) = scenes.get(idscene.0) {
            if let Ok((
                mut wm,
                mut colors,
                mut uv, // mut uv
            )) = geometrys.get_mut(idgeo.0)
            {
                println!("============= 1111111111111");
                particle.as_mut().0.compute_call(world_matrix.0, local_matrix.0);
                particle.as_mut().0.update_call(world_matrix.0, local_matrix.0, camerapos, camera_rotation_matrix);

                let buffercolor = particle.0.ps_tool.get_mp_color_data().unwrap();
                let bufferuv = particle.0.ps_tool.get_mp_uvdata().unwrap();
                let buffermatrix = particle.0.ps_tool.get_mp_matrix_list().unwrap();

                println!("============= buffercolor: {:?}", buffercolor);
                println!("============= buffermatrix: {:?}", buffermatrix);
                println!("============= uvdata: {:?}", bufferuv);

                let colordata: Vec<u8> = bytemuck::cast_slice(&buffercolor).to_vec();
                let uvdata: Vec<u8> = bytemuck::cast_slice(&bufferuv).to_vec();
                let wmdata: Vec<u8> = bytemuck::cast_slice(&buffermatrix).to_vec();

                // let mut buffermatrix = vec![];
                // let mut buffercolor = vec![];

                // for z in 0..20 {
                //     let ringcount = (z + 1) * 10;
                //     let tt = if z % 2 == 0 {
                //         scenetime.time_ms as f32 * 0.002
                //     } else {
                //         scenetime.time_ms as f32 * 0.002 * -1.
                //     };
                //     for x in 0..ringcount {
                //         let t: f32 = (tt + x as f32 * (1. / ringcount as f32)) * 3.1415926 * 2.;
                //         let mut wm = Matrix::identity();
                //         wm.append_translation_mut(&Vector3::new(
                //             f32::cos(t) * 2. * (z as f32 + 1.0),
                //             f32::sin(t) * 2. * (z as f32 + 1.0),
                //             0.,
                //         ));
                //         buffermatrix.push(wm);

                //         buffercolor.push(Vector4::new(
                //             f32::cos(tt + x as f32) * 0.5 + 0.5,
                //             f32::sin(tt) * 0.5 + 0.5,
                //             f32::sin(tt + z as f32) * 0.5 + 0.5,
                //             f32::cos(tt) * 0.5 + 0.5,
                //         ));
                //     }
                // }

                // let mut colordata: Vec<u8> = vec![];
                // buffercolor.iter().for_each(|v| {
                //     bytemuck::cast_slice(v.as_slice()).iter().for_each(|v| {
                //         colordata.push(*v);
                //     })
                // });

                // let mut wmdata: Vec<u8> = vec![];
                // buffermatrix.iter().for_each(|v| {
                //     bytemuck::cast_slice(v.as_slice()).iter().for_each(|v| {
                //         wmdata.push(*v);
                //     })
                // });

                geometry_update_instance_buffer::<InstanceBufferWorldMatrix>(
                    Some(wmdata),
                    idgeo.0,
                    &mut wm,
                    &mut geoloader,
                    &mut vb_data_map,
                );
                geometry_update_instance_buffer::<InstanceBufferColor>(
                    Some(colordata),
                    idgeo.0,
                    &mut colors,
                    &mut geoloader,
                    &mut vb_data_map,
                );
                geometry_update_instance_buffer::<InstanceBufferTillOff>(
                    Some(uvdata),
                    idgeo.0,
                    &mut uv,
                    &mut geoloader,
                    &mut vb_data_map,
                );
            }
            // }
        },
    );
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

    app.insert_resource(AssetMgrConfigs::default());
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
    app.add_plugin(PluginUnlitMaterial);
    app.add_plugin(PluginNodeMaterial);
    app.add_plugin(pi_3d::PluginSceneTimeFromPluginFrame);

    app.add_system(sys_demo_particle.in_set(ERunStageChap::CalcRenderMatrix));

    app.add_startup_system(setup);
    // bevy_mod_debugdump::print_main_schedule(&mut app);

    app.run()
}
