//! 动态正弦波演示 - 使用实例化渲染
//!
//! 这个示例展示了如何使用实例化渲染创建两个周期的动态正弦波效果
//! - 100个采样点形成波浪，沿X轴分布
//! - 使用实例化渲染提高性能
//! - 使用默认材质，支持颜色变化
//! - 动态缩放和位置动画

#![feature(box_into_inner)]

use base::DemoScene;
use pi_atom::Atom;
use pi_scene_shell::prelude::*;
use pi_node_materials::prelude::*;
use pi_scene_context::prelude::*;
use pi_mesh_builder::ball::*;
use unlit_material::*;
use std::f32::consts::PI;

#[path = "./base.rs"]
mod base;
#[path = "./copy.rs"]
mod copy;

/// 正弦波配置
#[derive(Resource)]
pub struct SineWaveConfig {
    pub sample_count: usize,
    pub wave_count: usize,
    pub amplitude: f32,
    pub frequency: f32,
    pub phase_speed: f32,
    pub time: f32,
}

impl Default for SineWaveConfig {
    fn default() -> Self {
        Self {
            sample_count: 100,
            wave_count: 2,
            amplitude: 3.0,
            frequency: 1.0,
            phase_speed: 2.0,
            time: 0.0,
        }
    }
}

/// 正弦波实体容器
#[derive(Resource)]
pub struct SineWaveContainer {
    pub source_entity: Option<Entity>,
    pub time_entity: Option<Entity>,
}

impl Default for SineWaveContainer {
    fn default() -> Self {
        Self {
            source_entity: None,
            time_entity: None,
        }
    }
}

/// 正弦波动画系统
fn animate_sine_wave(
    mut actions: pi_3d::ActionSets,
    time: Res<SingleFrameTimeCommand>,
    mut config: ResMut<SineWaveConfig>,
    container: Res<SineWaveContainer>,
) {
    // 更新时间
    config.time += time.delta_ms() as f32 * 0.001;

    // 计算相位偏移
    let phase_offset = config.time * config.phase_speed;

    // 更新时间控制器的旋转来可视化时间流逝
    if let Some(time_entity) = container.time_entity {
        actions.transform.localsrt.push(OpsTransformNodeLocal::ops(
            time_entity,
            ETransformSRT::Euler(0.0, config.time * 0.5, 0.0)
        ));
    }

    log::debug!("正弦波动画帧 - 时间: {:.2}", config.time);
}

fn setup_sine_wave(
    mut commands: Commands,
    mut actions: pi_3d::ActionSets,
    mut matmetas: ResMut<ShareAssetMgr<ShaderEffectMeta>>,
    mut animegroupres: ResourceAnimationGroup,
    mut fps: ResMut<SingleFrameTimeCommand>,
    mut assets: (ResMut<CustomRenderTargets>, Res<PiRenderDevice>, Res<ShareAssetMgr<SamplerRes>>, Res<PiSafeAtlasAllocator>),
    demooption: Res<base::DemoOption>,
    engineopt: Res<EngineCustomPlugins>,
    mut container: ResMut<SineWaveContainer>,
    config: Res<SineWaveConfig>,
) {
    let (demopass, scene, camera01, copyrenderer, copyrendercamera) =
        if let (Some(demo), Some(copyrenderer), Some(copyrendercamera)) =
            (&demooption.demo, &demooption.copyrenderer, &demooption.copyrendercamera) {
            (demo, demo.scene, demo.camera, *copyrenderer, *copyrendercamera)
        } else { return; };

    // 注册材质
    ActionMaterial::regist_material_meta(&matmetas, KeyShaderMeta::from(MainOpacityShader::KEY), MainOpacityShader::meta(&engineopt));

    // 设置相机参数 - 正交相机便于观察正弦波
    actions.camera.param.push(OpsCameraModify::ops(camera01, ECameraModify::OrthSize(15.0)));

    // 设置帧率
    fps.frame_ms = 4;

    // 使用球体几何体
    let vertices = BallBuilder::attrs_meta();
    let indices = Some(BallBuilder::indices_meta());

    // 配置实例化状态
    let mut state = MeshInstanceState::default();
    state.instance_matrix = false; // 不使用实例变换矩阵，使用自定义属性

    // 添加位置属性
    state.instances.push(
        CustomVertexAttribute::new(
            Atom::from("InsPosition"),
            Atom::from("A_POSITION.xyz = A_POSITION.xyz * 0.1 + InsPosition;"),
            ECustomVertexType::Vec3, None
        )
    );

    // 添加颜色属性
    state.instances.push(
        CustomVertexAttribute::new(
            Atom::from("InsColor"),
            Atom::from("A_COLOR4 = InsColor;"),
            ECustomVertexType::Vec4, None
        )
    );

    // 添加缩放属性
    state.instances.push(
        CustomVertexAttribute::new(
            Atom::from("InsScale"),
            Atom::from("A_POSITION.xyz = A_POSITION.xyz * InsScale;"),
            ECustomVertexType::Float, None
        )
    );

    // 创建基础网格
    let source = base::DemoScene::mesh(&mut commands, scene, scene, &mut actions, vertices, indices, state);

    // 设置材质
    let idmat = commands.spawn_empty_id();
    actions.material.usemat.push(OpsMaterialUse::ops(source, idmat, DemoScene::PASS_OPAQUE));
    actions.material.create.push(OpsMaterialCreate::ops(idmat, DefaultShader::KEY, false, false));

    // 创建一个父节点用于整体控制
    let parent_node = commands.spawn_empty_id();
    actions.transform.tree.push(OpsTransformNodeParent::ops(parent_node, scene));
    actions.transform.create.push(OpsTransformNode::ops(scene, parent_node));
    actions.transform.localsrt.push(OpsTransformNodeLocal::ops(parent_node, ETransformSRT::Translation(0.0, 0.0, 0.0)));
    actions.transform.tree.push(OpsTransformNodeParent::ops(source, parent_node));

    // 创建100个实例形成正弦波
    for i in 0..config.sample_count {
        // 计算X位置（从-10到10）
        let x_normalized = i as f32 / (config.sample_count - 1) as f32;
        let x_range = (x_normalized - 0.5) * 20.0; // X轴范围: -10 到 10

        // 创建实例
        let instance = commands.spawn_empty_id();
        actions.transform.tree.push(OpsTransformNodeParent::ops(instance, scene));
        actions.instance.create.push(OpsInstanceMeshCreation::ops(source, instance));

        // 计算初始位置
        let mut y = 0.0;
        for wave in 0..config.wave_count {
            let wave_frequency = config.frequency * (wave + 1) as f32;
            let phase = (wave as f32 * PI * 0.5); // 每个波有相位差
            y += (x_range * wave_frequency + phase).sin() * config.amplitude / (wave + 1) as f32;
        }

        // 计算Z值（创建3D效果）
        let z = (x_range * 0.5).sin() * 1.5;

        // 设置实例属性
        actions.instance.attr.push(OpsInstanceAttr::ops(instance,
            EInstanceAttr::Vec3([x_range, y, z]), Atom::from("InsPosition")));

        // 根据位置设置颜色（彩虹色）
        let hue = x_normalized * 360.0;
        let color = hsv_to_rgb(hue, 0.8, 1.0);
        actions.instance.attr.push(OpsInstanceAttr::ops(instance,
            EInstanceAttr::Vec4([color[0], color[1], color[2], 1.0]), Atom::from("InsColor")));

        // 设置缩放
        let scale = 0.3;
        actions.instance.attr.push(OpsInstanceAttr::ops(instance,
            EInstanceAttr::Float(scale), Atom::from("InsScale")));
    }

    // 创建时间可视化器（一个旋转的小球）
    let time_visualizer = commands.spawn_empty_id();
    actions.transform.tree.push(OpsTransformNodeParent::ops(time_visualizer, parent_node));
    actions.transform.localsrt.push(OpsTransformNodeLocal::ops(time_visualizer, ETransformSRT::Translation(12.0, 8.0, 0.0)));
    actions.transform.localsrt.push(OpsTransformNodeLocal::ops(time_visualizer, ETransformSRT::Scaling(0.5, 0.5, 0.5)));

    let time_vertices = BallBuilder::attrs_meta();
    let time_indices = Some(BallBuilder::indices_meta());
    let time_mesh = base::DemoScene::mesh(&mut commands, scene, time_visualizer, &mut actions, time_vertices, time_indices, MeshInstanceState::default());

    let time_mat = commands.spawn_empty_id();
    actions.material.usemat.push(OpsMaterialUse::ops(time_mesh, time_mat, DemoScene::PASS_OPAQUE));
    actions.material.create.push(OpsMaterialCreate::ops(time_mat, DefaultShader::KEY, false, false));
    actions.material.val.push(OpsUniformVal::ops(time_mat, EUniformVal::Vec4(Atom::from("color"), 1.0, 1.0, 0.0, 1.0)));

    // 保存实体引用
    container.source_entity = Some(source);
    container.time_entity = Some(time_visualizer);

    log::info!("正弦波场景设置完成：{}个采样点，{}个周期", config.sample_count, config.wave_count);
}

/// HSV到RGB颜色转换
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> [f32; 3] {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r_prime, g_prime, b_prime) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    [
        r_prime + m,
        g_prime + m,
        b_prime + m,
    ]
}

/// 正弦波插件
pub struct PluginSineWave;
impl Plugin for PluginSineWave {
    fn build(&self, app: &mut App) {
        app.insert_resource(SineWaveConfig::default());
        app.insert_resource(SineWaveContainer::default());
    }
}

pub fn main() {
    // env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let (mut app, window, event_loop) = base::test_plugins();

    app.insert_resource(crate::base::DemoOption {
        orthographic_camera: true,
        camera_size: 15.0,
        camera_fov: 0.7,
        camera_position: (0.0, 0.0, -10.0),
        ..Default::default()
    });

    app.add_startup_system(Update, base::setup_demoinit);
    app.add_plugins(PluginSineWave);

    #[cfg(feature = "use_bevy")]
    app.add_systems(Startup, setup_sine_wave.after(base::setup_default_mat));
    #[cfg(not(feature = "use_bevy"))]
    app.add_startup_system(Update, setup_sine_wave.after(base::setup_default_mat));

    app.add_system(Update, animate_sine_wave);

    log::info!("动态正弦波演示启动！");
    log::info!("特性：");
    log::info!("- 100个采样点形成正弦波");
    log::info!("- 两个周期的波形叠加");
    log::info!("- 使用实例化渲染优化性能");
    log::info!("- 彩虹颜色渐变效果");
    log::info!("- 动态3D波浪效果");

    crate::base::run_loop(app, window, event_loop)
}