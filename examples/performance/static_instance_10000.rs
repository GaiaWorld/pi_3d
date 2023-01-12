#![feature(box_into_inner)]

use std::{any::TypeId, sync::Arc, time::{Instant, Duration}, ops::RangeBounds};

use default_render::interface::InterfaceDefaultMaterial;
use pi_3d::PluginBundleDefault;
use pi_engine_shell::{engine_shell::AppShell, frame_time::InterfaceFrameTime, setup::TSetup, run_stage::{TSystemStageInfo, ERunStageChap}};
use pi_render::rhi::options::RenderOptions;
use pi_scene_context::{plugin::Plugin, object::ObjectID,
    transforms::{command::{SingleTransformNodeCommandList, TransformNodeCommand}, interface::InterfaceTransformNode},
    scene::{interface::InterfaceScene},
    cameras::interface::InterfaceCamera,
    meshes::{interface::InterfaceMesh},
    main_camera_render::interface::InterfaceMainCamera,
    layer_mask::{interface::InterfaceLayerMask, LayerMask}, renderers::render_depth_and_stencil::{InterfaceRenderDepthAndStencil, RenderDepthAndStencil}, materials::material::{InterfaceMaterial, MaterialID}, geometry::{TInterfaceGeomtery, indices::InterfaceBufferIndices}
};
use pi_ecs::prelude::{ResMut, Setup};
use pi_ecs_macros::setup;
use pi_scene_math::{Vector3, Vector4};
use render_data_container::KeyVertexBuffer;
use render_geometry::vertex_data::VertexBufferDesc;
use pi_mesh_builder::cube::{InterfaceCube, CubeBuilder};

#[derive(Debug, Default)]
pub struct SingleTestData {
    pub transforms: Vec<(ObjectID, f32, f32, f32)>,
}

pub struct SysTest;
impl TSystemStageInfo for SysTest {}
#[setup]
impl SysTest {
    #[system]
    pub fn sys(
        mut list: ResMut<SingleTestData>,
        mut transform_commands: ResMut<SingleTransformNodeCommandList>,
    ) {
        list.transforms.iter_mut().for_each(|mut item| {
            item.1 = item.1 + 16.0;
            item.2 = item.2 + 16.0;
            item.3 = item.3 + 16.0;
            let x0 = item.1 % 4000.0 / 4000.0;
            let x = x0 * 3.1415926 * 2.;
            let y0 = item.2 % 4000.0 / 4000.0;
            let y = y0 * 3.1415926 * 2.;
            let z0 = item.3 % 4000.0 / 4000.0;
            let z = z0 * 3.1415926 * 2.;
            // transform_commands.list.push(TransformNodeCommand::ModifyPosition(item.0, Vector3::new(x.cos() * 3., 0., 0.)));
            // transform_commands.list.push(TransformNodeCommand::ModifyScaling(item.0, Vector3::new(x.cos() + 0.5, x.sin() + 0.5, x + 0.5)));
            transform_commands.list.push(TransformNodeCommand::ModifyRotation(item.0, Vector3::new(x, y, z)));
        });
    }
}

#[derive(Debug)]
pub struct PluginTest;
impl Plugin for PluginTest {
    fn init(
        &mut self,
        engine: &mut pi_scene_context::engine::Engine,
        stages: &mut pi_scene_context::run_stage::RunStage,
    ) -> Result<(), pi_scene_context::plugin::ErrorPlugin> {

        PluginBundleDefault.init(engine, stages);

        let world = engine.world_mut();

        SysTest::setup(world, stages.query_stage::<SysTest>(ERunStageChap::Command));

        let testdata = SingleTestData::default();
        world.insert_resource(testdata);

        Ok(())
    }
}

impl PluginTest {
    pub fn setup(
        engine: &pi_engine_shell::engine_shell::EnginShell,
    ) {

        let tes_size = 100;
        let testdata = engine.world().get_resource_mut::<SingleTestData>().unwrap();

        engine.frame_time(2);

        // Test Code
        let scene01 = engine.create_scene();
        let camera01 = engine.create_free_camera(scene01);
        engine.active_camera(camera01, true);
        engine.layer_mask(camera01, LayerMask::default());
        engine.transform_position(camera01, Vector3::new(0., 0., -10.));
        engine.free_camera_orth_size(camera01, tes_size as f32);

        // let matid = engine.create_default_material();
        // engine.emissive_intensity(entity, intensity);

        let source = engine.create_mesh(scene01);
        let mut attrs = CubeBuilder::attrs_meta();
        attrs.push(VertexBufferDesc::instance_world_matrix());
        attrs.push(VertexBufferDesc::instance_color());
        engine.use_geometry(source, attrs);
        engine.use_indices(source, CubeBuilder::indices_meta());
        engine.use_default_material(source);
        engine.layer_mask(source, LayerMask::default());

        for i in 0..tes_size {
            for j in 0..tes_size {
                for k in 0..1 {
                    let cube = engine.create_instanced_mesh(scene01, source.clone());
                    engine.set_instance_color(cube, Vector4::new((i as f32) / (tes_size as f32), (j as f32) / (tes_size as f32), 1., 1.));
                    engine.transform_position(cube, Vector3::new(i as f32 * 2. - (tes_size) as f32, j as f32 * 2. - (tes_size) as f32, k as f32 * 2. - (tes_size) as f32));
                    // engine.transform_rotation_euler(cube, Vector3::new(i as f32 * 0.2, j as f32 * 0.2, k as f32 * 0.2));
                    // testdata.transforms.push((cube, i as f32 * 100., j as f32 * 100., k as f32 * 100.));
                }
            }
        }
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