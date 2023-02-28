#![feature(box_into_inner)]


use default_render::interface::InterfaceDefaultMaterial;
use pi_3d::PluginBundleDefault;
use pi_animation::{loop_mode::ELoopMode, amount::AnimationAmountCalc, curve_frame_event::CurveFrameEvent, animation_listener::{AnimationListener, EAnimationEventResult}};
use pi_atom::Atom;
use pi_curves::{curve::frame_curve::FrameCurve, easing::EEasingMode};
use pi_engine_shell::{engine_shell::AppShell, frame_time::InterfaceFrameTime, run_stage::{TSystemStageInfo, ERunStageChap}, assets::local_load::PluginLocalLoad, setup::TSetup, object::InterfaceObject};
use pi_render::{rhi::options::RenderOptions, renderer::vertex_buffer_desc::VertexBufferDesc};
use pi_scene_context::{plugin::Plugin, object::ObjectID,
    transforms::{command::{SingleTransformNodeModifyCommandList, ETransformNodeModifyCommand}, interface::InterfaceTransformNode, transform_node::{LocalPosition, LocalEulerAngles, LocalScaling}},
    scene::{interface::InterfaceScene},
    cameras::{interface::InterfaceCamera, camera::EFreeCameraMode},
    layer_mask::{interface::InterfaceLayerMask, LayerMask}, animation::interface::{InterfaceAnimeAsset, InterfaceAnimationGroup}, renderers::graphic::RendererGraphicDesc, pass::{EPassTag, PassTagOrders}, meshes::interface::InterfaceMesh, geometry::{TInterfaceGeomtery, indices::InterfaceBufferIndices}
};
use pi_ecs::{prelude::{ResMut, Setup}, storage::Local};
use pi_ecs_macros::setup;
use pi_scene_math::{Vector3, Vector4};
use pi_mesh_builder::{cube::{InterfaceCube, PluginCubeBuilder, CubeBuilder}, ball::PluginBallBuilder};
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

        let tes_size = 20;
        engine.frame_time(10);

        // Test Code
        let scene01 = engine.create_scene();

        let root = engine.create_transform_node(scene01);

        let camera01 = engine.create_free_camera(scene01);
        engine.free_camera_mode(camera01, EFreeCameraMode::Perspective);
        engine.active_camera(camera01, true);
        engine.layer_mask(camera01, LayerMask::default());
        engine.transform_position(camera01, Vector3::new(0., 10., -40.));
        engine.camera_renderer(camera01, RendererGraphicDesc { pre: Some(Atom::from("Clear")), curr: Atom::from("MainCamera"), next: None, passorders: PassTagOrders::new(vec![EPassTag::Opaque]) });
        // engine.transform_parent(camera01, root);
        engine.camera_target(camera01, Vector3::new(0., -1., 4.));

        let source = engine.create_mesh(scene01);
        let mut attrs = CubeBuilder::attrs_meta();
        attrs.push(VertexBufferDesc::instance_world_matrix());
        engine.use_geometry(source, attrs);
        engine.use_indices(source, CubeBuilder::indices_meta());
        engine.use_default_material(source);
        engine.layer_mask(source, LayerMask::default());

        let key_group = pi_atom::Atom::from("key_group");
        engine.create_animation_group(source, &key_group);

        let cell_col = 4.;
        let cell_row = 4.;
        for i in 0..tes_size {
            for j in 0..tes_size {
                for k in 0..1 {
                    let cube = engine.create_instanced_mesh(scene01, source.clone());
                    let pos = Vector3::new(i as f32 * 2. - (tes_size) as f32, 0., j as f32 * 2. - (tes_size) as f32);
                    engine.transform_position(cube, pos.clone());
                    
                    let key_curve0 = pi_atom::Atom::from((i * tes_size + j).to_string());
                    let curve = FrameCurve::<LocalScaling>::curve_easing(LocalScaling(Vector3::new(1., 1., 1.)), LocalScaling(Vector3::new(0., 2. * (1.1 + (i as f32).sin()), 0.)), (60. * (1.1 + ((i * j) as f32).cos())) as u16, 30, EEasingMode::None);
                    let asset_curve = if let Some(curve) = engine.check_anim_curve::<LocalScaling>(&key_curve0) {
                        curve
                    } else {
                        engine.creat_anim_curve::<LocalScaling>(&key_curve0, curve)
                    };
                    let animation = engine.create_animation::<LocalScaling>(asset_curve);


                    engine.create_target_animation(source, cube, &key_group, animation);
                }
            }
        }

        engine.start_animation_group(source, &key_group, 1.0, ELoopMode::OppositePly(None), 0., 1., 60, AnimationAmountCalc::default());
            
        // 创建帧事件
    }
}


pub fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

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