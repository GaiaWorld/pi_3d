#![feature(box_into_inner)]


use default_render::interface::InterfaceDefaultMaterial;
use pi_3d::PluginBundleDefault;
use pi_animation::{loop_mode::ELoopMode, amount::AnimationAmountCalc, curve_frame_event::CurveFrameEvent, animation_listener::{AnimationListener, EAnimationEventResult}};
use pi_atom::Atom;
use pi_curves::{curve::frame_curve::FrameCurve, easing::EEasingMode};
use pi_engine_shell::{engine_shell::AppShell, frame_time::InterfaceFrameTime, run_stage::{TSystemStageInfo, ERunStageChap}, assets::local_load::PluginLocalLoad, setup::TSetup, object::InterfaceObject};
use pi_render::rhi::options::RenderOptions;
use pi_scene_context::{plugin::Plugin, object::ObjectID,
    transforms::{command::{SingleTransformNodeModifyCommandList, ETransformNodeModifyCommand}, interface::InterfaceTransformNode, transform_node::LocalPosition},
    scene::{interface::InterfaceScene},
    cameras::interface::InterfaceCamera,
    main_camera_render::interface::InterfaceMainCamera,
    layer_mask::{interface::InterfaceLayerMask, LayerMask}, animation::interface::{InterfaceAnimeAsset, InterfaceAnimationGroup}
};
use pi_ecs::{prelude::{ResMut, Setup}, storage::Local};
use pi_ecs_macros::setup;
use pi_scene_math::Vector3;
use pi_mesh_builder::{cube::{InterfaceCube, PluginCubeBuilder}, ball::PluginBallBuilder};
use unlit_material::PluginUnlitMaterial;



#[derive(Debug)]
pub struct PluginTest;
impl Plugin for PluginTest {
    fn init(
        &mut self,
        engine: &mut pi_scene_context::engine::Engine,
        stages: &mut pi_scene_context::run_stage::RunStage,
    ) -> Result<(), pi_scene_context::plugin::ErrorPlugin> {
        PluginLocalLoad.init(engine, stages);
        PluginBundleDefault.init(engine, stages);
        PluginUnlitMaterial.init(engine, stages);

        PluginCubeBuilder.init(engine, stages);

        Ok(())
    }
}

impl PluginTest {
    fn setup(
        engine: &pi_engine_shell::engine_shell::EnginShell,
    ) {

        let tes_size = 5;
        engine.frame_time(4);

        // Test Code
        let scene01 = engine.create_scene();
        let camera01 = engine.create_free_camera(scene01);
        engine.active_camera(camera01, true);
        engine.layer_mask(camera01, LayerMask::default());
        engine.transform_position(camera01, Vector3::new(0., 0., -10.));
        engine.free_camera_orth_size(camera01, tes_size as f32);

        let cube = engine.new_cube(scene01);
        engine.use_default_material(cube);

        let key_curve0 = pi_atom::Atom::from("key_curve0");
        let curve = FrameCurve::<LocalPosition>::curve_easing(LocalPosition(Vector3::new(-3., 0., 0.)), LocalPosition(Vector3::new(6., 0., 0.)), 30, 30, EEasingMode::None);
        let asset_curve = if let Some(curve) = engine.check_anim_curve::<LocalPosition>(&key_curve0) {
            curve
        } else {
            engine.creat_anim_curve::<LocalPosition>(&key_curve0, curve)
        };

        let animation = engine.create_animation(asset_curve);

        let key_group = pi_atom::Atom::from("key_group");
        engine.create_animation_group(cube, &key_group)
            .create_target_animation(cube, cube, &key_group, animation)
            .start_animation_group(cube, &key_group, 1.0, ELoopMode::Positive(None), 0., 1., 200, AnimationAmountCalc::default());
            
        // =========================================
        let cube = engine.new_cube(scene01);
        engine.use_default_material(cube).transform_position(cube, Vector3::new(0., 2., 0.));

        let key_curve0 = pi_atom::Atom::from("key_curve1");
        let curve = FrameCurve::<LocalPosition>::curve_easing(LocalPosition(Vector3::new(-3., 2., 0.)), LocalPosition(Vector3::new(6., 0., 0.)), 30, 30, EEasingMode::ElasticInOut);
        let asset_curve = if let Some(curve) = engine.check_anim_curve::<LocalPosition>(&key_curve0) {
            curve
        } else {
            engine.creat_anim_curve::<LocalPosition>(&key_curve0, curve)
        };
        let animation = engine.create_animation(asset_curve);
        let key_group = pi_atom::Atom::from("key_group2");
        engine.create_animation_group(cube, &key_group)
            .create_target_animation(cube, cube, &key_group, animation)
            .start_animation_group(cube, &key_group, 1.0, ELoopMode::Positive(None), 0., 1., 200, AnimationAmountCalc::default());
        
            
        // 创建帧事件
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