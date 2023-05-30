use default_render::{shader::DefaultShader, SingleIDBaseDefaultMaterial};
use particle::{
    emitter::ishape_emitter_type::EShapeEmitterArcMode,
    extend::formatMeshParticle,
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
use pi_async::rt::AsyncRuntime;
use pi_atom::Atom;
use pi_bevy_ecs_extend::system_param::layer_dirty::ComponentEvent;
use pi_bevy_render_plugin::PiRenderPlugin;
use pi_curves::{curve::frame_curve::FrameCurve, easing::EEasingMode};
use pi_engine_shell::{frame_time::PluginFrameTime, prelude::*};
use pi_hal::{init_load_cb, on_load, runtime::MULTI_MEDIA_RUNTIME};
use pi_mesh_builder::{ball::*, cube::*, quad::PluginQuadBuilder};
use pi_node_materials::{NodeMaterialBlocks, PluginNodeMaterial};
use pi_scene_context::prelude::*;
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
) {
    let tes_size = 50;
    fps.frame_ms = 16;

    final_render.cleardepth = 0.0;

    let scene = commands.spawn_empty().id();
    scenecmds
        .create
        .push(OpsSceneCreation::ops(scene, ScenePassRenderCfg::default()));

    let camera01 = commands.spawn_empty().id();
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
        pre: Some(Atom::from(WindowRenderer::CLEAR_KEY)),
        curr: Atom::from("TestCamera"),
        next: Some(Atom::from(WindowRenderer::KEY)),
        passorders: PassTagOrders::new(vec![
            EPassTag::Opaque,
            EPassTag::Water,
            EPassTag::Sky,
            EPassTag::Transparent,
        ]),
    };
    let id_renderer = commands.spawn_empty().id();
    cameracmds.render.push(OpsCameraRendererInit::ops(
        camera01,
        id_renderer,
        desc,
        ColorFormat::Rgba8Unorm,
        DepthStencilFormat::None,
    ));

    let source = commands.spawn_empty().id();
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

    let mut config = IParticleSystemConfig {
        name: "MP".to_string(),
        duration: 5.,
        startDelay: 0.,
        looping: 1,
        prewarm: true,
        simulationSpaceIsWorld: EMeshParticleSpaceMode::Local,
        scalingMode: EMeshParticleScaleMode::Hierarchy,
        renderAlignment: ERenderAlignment::Local,
        renderMode: ERenderMode::Billboard,
        stretchedVelocityScale: 0.,
        stretchedLengthScale: 0.,
        maxParticles: 10.,
        startSpeed: OneParamInfo::TInterpolateConstant(8.0),
        lifetime: OneParamInfo::TInterpolateConstant(2.0),
        delay: vec![0.],
        startColor: FourGradientInfo::TInterpolateColor([1., 1., 1., 1.]),
        startSize: ParamInfo::OneParamInfo(OneParamInfo::TInterpolateConstant(1.0)),
        startRotation: ParamInfo::OneParamInfo(OneParamInfo::TInterpolateConstant(0.0)),
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
            radiusThickness: 0.,
            arc: IShapeArc::IShapeArcRandom(IShapeArcRandom {
                mode: EShapeEmitterArcMode::Random,
                value: 360.,
                spread: 0.,
                speed: 1.,
            }),
            emitAsVolume: false,
            height: 0.,

            scale: default(),
            position: default(),
            rotation: default(),
            alignDir: default(),
            randomize: None,
        }),
        velocityOverLifetime: Some(ParamInfo::ThreeParamInfo(
            ThreeParamInfo::TInterpolateConstant([0., 0., 0.]),
        )),
        limitVelocityOverLifetime: Some(OneParamInfo::TInterpolateConstant(100.)),
        velocityOverLifetimeIsLocal: default(),
        limitVelocityOverLifetimeDampen: Some(0.9),
        forceOverLifetime: Some(ParamInfo::OneParamInfo(OneParamInfo::TInterpolateConstant(
            0.,
        ))),
        forceSpaceIsLocal: default(),
        colorOverLifetime: Some(FourGradientInfo::TInterpolateTwoColors(
            [1., 1., 1., 1.],
            [0., 0., 0., 1.],
        )),
        colorBySpeed: default(),
        sizeOverLifetime: Some(ParamInfo::OneParamInfo(OneParamInfo::TInterpolateConstant(
            0.25,
        ))),
        sizeBySpeed: default(),
        rotationOverLifetime: Some(ParamInfo::ThreeParamInfo(
            ThreeParamInfo::TInterpolateConstant([0., 0., 0.]),
        )),
        rotationBySpeed: default(),
        textureSheet: default(),
        texture: default(),
        trail: default(),
        orbtialVelocity: default(),
        orbitalOffset: default(),
        orbitalRadial: default(),
        speedModifier: default(),
        renderPivot: default(),
        custom1: default(),
    };
    let mut mp = MeshParticleSystem::new();
    formatMeshParticle(&mut config, &mut mp);
    mp.build();
    mp.start();
    commands.entity(source).insert(Particle(mp));

    // mp
}

#[derive(Component)]
pub struct Particle(MeshParticleSystem);

fn sys_demo_particle(
    mut particles: Query<(&GeometryID, &mut Particle, &WorldMatrix, &LocalMatrix)>,
    // scenes: Query<&SceneTime>,
    mut geometrys: Query<(
        &mut InstanceBufferWorldMatrix,
        &mut InstanceBufferColor,
        &mut InstanceBufferTillOff,
    )>,
    mut geoloader: ResMut<GeometryVBLoader>,
    mut vb_data_map: ResMut<VertexBufferDataMap3D>,
) {
    particles.iter_mut().for_each(|(idgeo, mut particle, world_matrix, local_matrix)| {
        // if let Ok(scenetime) = scenes.get(idscene.0) {
        if let Ok((mut wm, mut colors, mut uv
            // mut uv
        )) = geometrys.get_mut(idgeo.0) {
            particle.as_mut().0.updateCall();

            let buffercolor = particle.0.psTool.colorData.as_ref().unwrap();
            let bufferuv = particle.0.psTool.uvData.as_ref().unwrap();
            let buffermatrix = particle.0.psTool.get_mpMatrixList().unwrap();

            // println!("============= buffercolor: {:?}", buffercolor);
            // println!("============= buffermatrix: {:?}", buffermatrix);
            let colordata: Vec<u8> = bytemuck::cast_slice(buffercolor).to_vec();
            let uvdata: Vec<u8> = bytemuck::cast_slice(bufferuv).to_vec();
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
    app.add_plugin(PluginNodeMaterial);

    app.add_system(sys_demo_particle.in_set(ERunStageChap::CalcRenderMatrix));

    app.add_startup_system(setup);
    // bevy_mod_debugdump::print_main_schedule(&mut app);

    app.run()
}
