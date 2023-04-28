#![feature(box_into_inner)]


use axis::axis::{PluginAxisBuilder, InterfaceAxis};
use default_render::{interface::InterfaceDefaultMaterial, PluginDefaultMaterial};
use pi_3d::PluginBundleDefault;
use pi_atom::Atom;
use pi_engine_shell::{engine_shell::AppShell, frame_time::InterfaceFrameTime, assets::local_load::PluginLocalLoad};
use pi_render::rhi::options::RenderOptions;
use pi_scene_context::{
    plugin::Plugin,
    transforms::{interface::InterfaceTransformNode},
    scene::{interface::InterfaceScene},
    cameras::interface::InterfaceCamera,
    layer_mask::{interface::InterfaceLayerMask, LayerMask}, materials::{interface::{InterfaceMaterial}}, renderers::{render_primitive::{InterfaceRenderPrimitive, FrontFace}, graphic::RendererGraphicDesc}, pass::{EPassTag, PassTagOrders}
};
use pi_scene_math::Vector3;
use pi_mesh_builder::{quad::{InterfaceQuad}, cube::{PluginCubeBuilder, InterfaceCube}, ball::{PluginBallBuilder, InterfaceBall}};

/// 实现测试代码的方法
fn test(
    engine: &pi_engine_shell::engine_shell::EnginShell,
) {
    // 设置运行帧率
    engine.frame_time(2000);

    // 创建场景
    let scene01 = engine.create_scene();
    // 创建相机 - FreeCamera 默认为 Orthgraphic 模式
    let camera01 = engine.create_free_camera(scene01);
    // 激活相机
    engine.active_camera(camera01, true);
    // 修改相机位置
    engine.transform_position(camera01, Vector3::new(-5., 5., -10.));
    // 修改相机尺寸
    engine.free_camera_orth_size(camera01, 4 as f32);
    engine.camera_renderer(camera01, RendererGraphicDesc { pre: Some(Atom::from("Clear")), curr: Atom::from("MainCamera"), next: None, passorders: PassTagOrders::new(vec![EPassTag::Opaque, EPassTag::Water, EPassTag::Sky, EPassTag::Transparent]) });


    // 创建一个材质 (默认材质: 渲染顶点颜色;默认材质为PluginBundleDefault已注册的材质)
    let unlitmaterial = engine.create_default_material(EPassTag::Opaque);
    
    // 创建一个立方体(标准立方体,尺寸为1,中心点在原点)(接口内部已自动初始化立方体为 TransformNode、Mesh)
    let cube = engine.new_cube(scene01);
    // 使用材质
    engine.use_material(cube, unlitmaterial);
    // 设置立方体的渲染层级
    engine.layer_mask(cube, LayerMask::default());
    
    // 创建一个球体(标准球体,尺寸为1,中心点在原点)(接口内部已自动初始化球体为 TransformNode、Mesh、LayerMask)
    let ball = engine.new_ball(scene01, 20, 20);
    // 使用材质
    engine.use_material(ball, unlitmaterial);
    // 修改球体的偏移
    engine.transform_position(ball, Vector3::new(2., 2., 0.));
    engine.front_face(ball, FrontFace::Cw);
    
    // let axis_box = engine.new_axis(scene01);
    // // 使用材质
    // engine.use_material(axis_box, unlitmaterial);
}

pub fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let mut shell = AppShell::new(
        RenderOptions {
            backends: wgpu::Backends::VULKAN,
            power_preference: wgpu::PowerPreference::HighPerformance,
            ..Default::default()
        }
    );
    shell
        // 默认必须的功能插件集合
        .add_plugin(PluginBundleDefault)
        // 立方体网格
        .add_plugin(PluginCubeBuilder)
        // 球体网格
        .add_plugin(PluginBallBuilder)
        // 立方体网格
        .add_plugin(PluginAxisBuilder)
        .ready()
        .setup(&test)
        .run();
}