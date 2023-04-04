#![feature(box_into_inner)]


use default_render::interface::InterfaceDefaultMaterial;
use pi_3d::PluginBundleDefault;
use pi_animation::{loop_mode::ELoopMode, amount::AnimationAmountCalc};
use pi_atom::Atom;
use pi_curves::{curve::frame_curve::FrameCurve, easing::EEasingMode};
use pi_engine_shell::{engine_shell::AppShell, frame_time::InterfaceFrameTime, run_stage::{TSystemStageInfo, ERunStageChap}, assets::local_load::PluginLocalLoad, setup::TSetup};
use pi_render::{rhi::options::RenderOptions, renderer::{texture::KeyTexture, sampler::KeySampler, vertex_buffer_desc::VertexBufferDesc}, render_3d::shader::uniform_texture::UniformTextureWithSamplerParam};
use pi_scene_context::{plugin::Plugin, object::ObjectID,
    transforms::{command::{SingleTransformNodeModifyCommandList, ETransformNodeModifyCommand}, interface::InterfaceTransformNode, transform_node::{LocalPosition, LocalEulerAngles}},
    scene::{interface::InterfaceScene},
    cameras::interface::InterfaceCamera,
    layer_mask::{interface::InterfaceLayerMask, LayerMask},
    animation::interface::{InterfaceAnimeAsset, InterfaceAnimationGroup},
    meshes::interface::InterfaceMesh,
    geometry::{TInterfaceGeomtery},
    materials::{interface::InterfaceMaterial}, pass::{EPassTag, PassTagOrders}, renderers::graphic::RendererGraphicDesc
};
use pi_ecs::{prelude::{ResMut, Setup}, storage::Local};
use pi_ecs_macros::setup;
use pi_scene_math::{Vector3, Vector4};
use pi_mesh_builder::{cube::{InterfaceCube, PluginCubeBuilder, CubeBuilder}, ball::PluginBallBuilder};
use unlit_material::{PluginUnlitMaterial, interface::InterfaceUnlitMaterial};



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

        let tes_size = 100;
        engine.frame_time(4);

        // Test Code
        let scene01 = engine.create_scene();
        let camera01 = engine.create_free_camera(scene01);
        engine.active_camera(camera01, true);
        engine.layer_mask(camera01, LayerMask::default());
        engine.transform_position(camera01, Vector3::new(0., 0., -10.));
        engine.free_camera_orth_size(camera01, tes_size as f32);
        engine.camera_renderer(camera01, RendererGraphicDesc { pre: Some(Atom::from("Clear")), curr: Atom::from("MainCamera"), next: None, passorders: PassTagOrders::new(vec![EPassTag::Opaque]) });

        let unlitmaterial = engine.create_unlit_material(EPassTag::Opaque);
        engine.set_texture(
            unlitmaterial,
            UniformTextureWithSamplerParam {
                slotname: Atom::from("_MainTex"),
                filter: true,
                sample: KeySampler::default(),
                url: KeyTexture::from("E:/Rust/PI/pi_3d/assets/images/bubbles.png"),
            },
            false
        );

        let source = engine.create_mesh(scene01);
        let mut attrs = CubeBuilder::attrs_meta();
        attrs.push(VertexBufferDesc::instance_world_matrix());
        attrs.push(VertexBufferDesc::instance_tilloff());
        engine.use_geometry(source, attrs, Some(CubeBuilder::indices_meta()));
        engine.use_material(source, unlitmaterial);
        engine.layer_mask(source, LayerMask::default());
        
        let key_group = pi_atom::Atom::from("key_group");
        engine.create_animation_group(source, &key_group);

        let cell_col = 4.;
        let cell_row = 4.;
        for i in 0..tes_size {
            for j in 0..tes_size {
                for k in 0..1 {
                    let cube = engine.create_instanced_mesh(scene01, source.clone());
                    let pos = Vector3::new(i as f32 * 2. - (tes_size) as f32, j as f32 * 2. - (tes_size) as f32, k as f32 * 2. - (tes_size) as f32);
                    engine.transform_position(cube, pos.clone());
                    engine.set_instance_tilloff(cube, Vector4::new(1.0 / cell_col, 1.0 / cell_row, (i % 4) as f32 / cell_col, (j % 4) as f32 / cell_row));
                    
                    let key_curve0 = pi_atom::Atom::from((i * tes_size + j).to_string());
                    let curve = FrameCurve::<LocalEulerAngles>::curve_easing(LocalEulerAngles(pos), LocalEulerAngles(Vector3::new(10., 10., 10.)), 30, 30, EEasingMode::None);
                    let asset_curve = if let Some(curve) = engine.check_anim_curve::<LocalEulerAngles>(&key_curve0) {
                        curve
                    } else {
                        engine.creat_anim_curve::<LocalEulerAngles>(&key_curve0, curve)
                    };
                    let animation = engine.create_animation::<LocalEulerAngles>(asset_curve);


                    engine.create_target_animation(source, cube, &key_group, animation);
                }
            }
        }

        engine.start_animation_group(source, &key_group, 1.0, ELoopMode::OppositePly(None), 0., 1., 60, AnimationAmountCalc::default());
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