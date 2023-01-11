use std::time::Instant;

use pi_3d::PluginBundleDefault;
use pi_ecs::prelude::{Res, ResMut, Setup};
use pi_ecs_macros::setup;
use pi_engine_shell::{
    engine_shell::{AppShell, EnginShell},
    frame_time::InterfaceFrameTime,
    object::{GameObject, ObjectID},
    plugin::Plugin, run_stage::{SysCommonUserCommand, ERunStageChap},
};
use pi_render::rhi::options::RenderOptions;
use pi_scene_context::{
    cameras::interface::InterfaceCamera,
    layer_mask::{interface::InterfaceLayerMask, LayerMask},
    main_camera_render::interface::InterfaceMainCamera,
    materials::{material::{ECommand, InterfaceMaterial, MaterialID, SingleValueUniformCommands}, uniforms::uint::UintUniform},
    scene::interface::InterfaceScene,
    transforms::interface::InterfaceTransformNode,
};
use pi_scene_math::{Matrix, Vector3};
use skeletons::{
    interface::InterfaceSkinMaterial,
    skin::{InterfaceSkin, PluginSkinBuilder},
    PluginBones,
};

#[derive(Debug, Default)]
pub struct SingleTestData {
    pub transforms: Vec<(ObjectID, bool, f32)>,
}

pub struct SysTest;
#[setup]
impl SysTest {
    #[system]
    pub fn sys(
        mut list: ResMut<SingleTestData>,
        mut transform_commands: ResMut<SingleValueUniformCommands>,
    ) {
        list.transforms.iter_mut().for_each(|mut item| {
            if  item.2 > 3000.{
                item.1 = false;
            }

            if item.2 < -3000.{
                item.1 = true;
            }


            if item.1 {
                item.2 = item.2 - 16.0;
            } else {
                item.2 = item.2 + 16.0;
            }

         
            let z = item.2 % 4000.0 / 4000.0 * 3.1415926 * 2.;
            let m = Matrix::from_euler_angles(0., 0., z);
            
            log::debug!("====== m: {:?}", m);
            transform_commands.0.push(ECommand::Mat4(
                item.0,
                1,
                m,
                false,
            ));
        });
    }
}

pub struct PluginTest;
impl Plugin for PluginTest {
    fn init(
        &mut self,
        engine: &mut pi_scene_context::engine::Engine,
        stages: &mut pi_scene_context::run_stage::RunStage,
    ) -> Result<(), pi_scene_context::plugin::ErrorPlugin> {
        // PluginQuadBuilder.init(engine, stages);
        PluginSkinBuilder.init(engine, stages);

        let mut world = engine.world_mut().clone();

        SysTest::setup(&mut world, stages.query_stage::<SysCommonUserCommand>(ERunStageChap::Command));

        let testdata = SingleTestData::default();
        world.insert_resource(testdata);

        Ok(())
    }
}

impl PluginTest {
    fn setup(engine: &EnginShell) {
        engine.frame_time(16);
        // Test Code
        let scene01 = engine.create_scene();
        let camera01 = engine.create_free_camera(scene01);
        let node01 = engine.create_transform_node(scene01);
        // engine.set_parent(camera01, scene01, Some(node01));
        engine.active_camera(camera01, true);
        engine.transform_position(camera01, Vector3::new(0., 0., -5.));
        engine.free_camera_orth_size(camera01, 1 as f32);

        let skin_box = engine.new_skin(scene01);
        let material = engine.create_skin_material();
        engine.use_material(skin_box, MaterialID(material));

        engine.layer_mask(camera01, LayerMask::default());
        engine.layer_mask(skin_box, LayerMask::default());

        let mut testdata = engine.world().get_resource_mut::<SingleTestData>().unwrap();
        testdata.transforms.push((material, true, 0.));
    }
}

pub fn main() {
    let mut shell = AppShell::new(RenderOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        ..Default::default()
    });
    shell.add_plugin(PluginBundleDefault);
    // shell.add_plugin(PluginSkinBuilder);
    shell.add_plugin(PluginBones);
    shell.add_plugin(PluginTest);
    shell.ready();
    shell.setup(&PluginTest::setup);
    shell.run();
}
