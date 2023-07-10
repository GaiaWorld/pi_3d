use pi_engine_shell::{prelude::*, run_stage::PluginRunstage};
use default_render::PluginDefaultMaterial;
use pi_scene_context::{prelude::*, scene::PluginScene, animation::PluginSceneAnimation, transforms::PluginGroupTransformNode, cameras::PluginCamera, meshes::PluginMesh, geometry::PluginGeometry, light::PluginLighting, layer_mask::PluginLayerMask, materials::PluginGroupMaterial, renderers::PluginRenderer, skeleton::PluginSkeleton};

pub struct Limit(pub wgpu::Limits);
// impl TMemoryAllocatorLimit for Limit {
//     fn max_size(&self) -> u64 {
//         500 * 1024 * 1024
//     }
// }

pub fn sys_scene_time_from_frame(
    mut scenes: Query<&mut SceneTime>,
    frame: Res<SingleFrameTimeCommand>,
) {
    scenes.iter_mut().for_each(|mut comp| {
        let time = comp.last_time_ms + frame.frame_ms;
        comp.reset(time);
    });
}

pub struct PluginSceneTimeFromPluginFrame;
impl Plugin for PluginSceneTimeFromPluginFrame {
    fn build(&self, app: &mut App) {
        app.add_system(
            sys_scene_time_from_frame.in_set(ERunStageChap::Command)
        );
    }
}

pub struct PluginBundleDefault;
impl PluginGroup for PluginBundleDefault {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();
        
        group = group.add(PluginRunstage);
        group = group.add(PluginGlobalAnimation);
        group = group.add(PluginRenderBindGroup);
        group = group.add(PluginScene);
        group = group.add(PluginSceneAnimation);
        group = group.add(PluginFlags);
        group = group.add(PluginAnimeNodeEnable::new());
        group = PluginGroupTransformNode::add(group);
        group = group.add(PluginCamera)
            .add(PluginAnimeCameraFOV::new())
            .add(PluginAnimeCameraSize::new())
            .add(PluginMesh)
            .add(PluginGeometry)
            .add(PluginLighting)
            .add(PluginLayerMask);
        group = PluginGroupMaterial::add(group);
        group = group.add(PluginRenderer)
            .add(PluginSkeleton)
            .add(PluginDefaultMaterial)
            .add(PluginDispose)
            ;

        group
    }
    // fn init(
    //     &mut self,
    //     engine: &mut pi_engine_shell::engine_shell::EnginShell,
    //     stages: &mut pi_engine_shell::run_stage::RunStage,
    // ) -> Result<(), pi_engine_shell::plugin::ErrorPlugin> {
    //     let world = engine.world_mut();

    //     let device = world.get_resource::<RenderDevice>().unwrap();
    //     let limit = Limit(device.limits());
    //     // world.insert_resource(DynMergyBufferAllocator::new(&limit, 4 * 1024 * 1024));

    //     PluginFlags.init(engine, stages);
    //     PluginRenderBindGroup.init(engine, stages);
    //     PluginScene.init(engine, stages);
    //     PluginTransformNode.init(engine, stages);
    //     PluginMesh.init(engine, stages);
    //     PluginCamera.init(engine, stages);

    //     PluginCulling.init(engine, stages);
    //     PluginGeometry.init(engine, stages);

    //     PluginMaterial.init(engine, stages);
    //     PluginLayerMask.init(engine, stages);

    //     PluginDefaultMaterial.init(engine, stages);

    //     PluginRenderer.init(engine, stages);
    //     PluginBoundingOctTree.init(engine, stages);

    //     // PluginCubeBuilder.init(engine, stages);
    //     // PluginBallBuilder.init(engine, stages);
    //     Ok(())
    // }
}