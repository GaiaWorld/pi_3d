use pi_engine_shell::{prelude::*, run_stage::PluginRunstage};
use default_render::PluginDefaultMaterial;
use pi_scene_context::{
    renderers::PluginRenderer,
    meshes::{PluginMesh,},
    layer_mask::PluginLayerMask, materials::PluginGroupMaterial,
    cullings::{PluginCulling},
    cameras::PluginCamera,
    transforms::PluginGroupTransformNode,
    scene::PluginScene, geometry::{PluginGeometry}, bindgroup::PluginRenderBindGroup, flags::PluginFlags, light::PluginLighting, skeleton::PluginSkeleton, animation::PluginAnimation
};

pub struct Limit(pub wgpu::Limits);
// impl TMemoryAllocatorLimit for Limit {
//     fn max_size(&self) -> u64 {
//         500 * 1024 * 1024
//     }
// }

pub struct PluginBundleDefault;
impl PluginGroup for PluginBundleDefault {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();
        
        group = group.add(PluginRunstage);
        group = group.add(PluginRenderBindGroup);
        group = group.add(PluginScene);
        group = group.add(PluginAnimation);
        group = PluginGroupTransformNode::add(group);
        group = group.add(PluginCamera)
            .add(PluginMesh)
            .add(PluginGeometry)
            .add(PluginLighting)
            .add(PluginLayerMask);
        group = PluginGroupMaterial::add(group);
        group = group.add(PluginRenderer)
            .add(PluginSkeleton)
            .add(PluginDefaultMaterial);

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