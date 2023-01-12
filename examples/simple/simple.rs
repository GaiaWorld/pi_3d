#![feature(box_into_inner)]


use default_render::{interface::InterfaceDefaultMaterial, PluginDefaultMaterial};
use pi_3d::PluginBundleDefault;
use pi_engine_shell::{engine_shell::AppShell, frame_time::InterfaceFrameTime, assets::local_load::PluginLocalLoad};
use pi_render::rhi::options::RenderOptions;
use pi_scene_context::{
    plugin::Plugin,
    transforms::{interface::InterfaceTransformNode},
    scene::{interface::InterfaceScene},
    cameras::interface::InterfaceCamera,
    main_camera_render::interface::InterfaceMainCamera,
    layer_mask::{interface::InterfaceLayerMask, LayerMask}, materials::{material::{InterfaceMaterial}}
};
use pi_scene_math::Vector3;
use pi_mesh_builder::{quad::{InterfaceQuad}, cube::{PluginCubeBuilder, InterfaceCube}};

/// 创建一个插件集合,将需要的插件在此处打包
#[derive(Debug)]
pub struct PluginTest;
impl Plugin for PluginTest {
    fn init(
        &mut self,
        engine: &mut pi_scene_context::engine::Engine,
        stages: &mut pi_scene_context::run_stage::RunStage,
    ) -> Result<(), pi_scene_context::plugin::ErrorPlugin> {
        PluginLocalLoad.init(engine, stages);
        /// 默认必须的功能插件集合
        PluginBundleDefault.init(engine, stages);
        /// 立方体网格
        PluginCubeBuilder.init(engine, stages);

        Ok(())
    }
}

impl PluginTest {
    /// 实现测试代码的方法
    pub fn setup(
        engine: &pi_engine_shell::engine_shell::EnginShell,
    ) {
        // 设置运行帧率
        engine.frame_time(2000);

        // 创建场景
        let scene01 = engine.create_scene();
        // 创建相机
        let camera01 = engine.create_free_camera(scene01);
        // 激活相机
        engine.active_camera(camera01, true);
        // 设置相机层级
        engine.layer_mask(camera01, LayerMask::default());
        // 修改相机位置
        engine.transform_position(camera01, Vector3::new(0., 0., -10.));
        // 修改相机尺寸
        engine.free_camera_orth_size(camera01, 2 as f32);

        // 创建一个材质 (默认材质: 渲染顶点颜色;默认材质为PluginBundleDefault已注册的材质)
        let unlitmaterial = engine.create_default_material();
        
        // 创建一个立方体(标准立方体,尺寸为1,中心点在原点)(接口内部已自动初始化立方体为 TransformNode、Mesh)
        let quad = engine.new_cube(scene01);
        // 使用材质
        engine.use_material(quad, unlitmaterial);
        // 设置立方体的渲染层级
        engine.layer_mask(quad, LayerMask::default());
    }
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
    shell.add_plugin(PluginTest);
    shell.ready();
    shell.setup(&PluginTest::setup);
    shell.run();
}