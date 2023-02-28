use default_render::PluginDefaultMaterial;
use pi_render::rhi::device::RenderDevice;
use pi_scene_context::{
    renderers::PluginRenderer,
    meshes::{PluginMesh,},
    layer_mask::PluginLayerMask, materials::PluginMaterial,
    cullings::{PluginCulling, oct_tree::PluginBoundingOctTree},
    cameras::PluginCamera,
    transforms::PluginTransformNode,
    scene::PluginScene, geometry::{PluginGeometry, indices::PluginBufferIndices}, bindgroup::PluginRenderBindGroup
};

pub struct Limit(pub wgpu::Limits);
// impl TMemoryAllocatorLimit for Limit {
//     fn max_size(&self) -> u64 {
//         500 * 1024 * 1024
//     }
// }

pub struct PluginBundleDefault;
impl pi_engine_shell::plugin::Plugin for PluginBundleDefault {
    fn init(
        &mut self,
        engine: &mut pi_engine_shell::engine_shell::EnginShell,
        stages: &mut pi_engine_shell::run_stage::RunStage,
    ) -> Result<(), pi_engine_shell::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        let device = world.get_resource::<RenderDevice>().unwrap();
        let limit = Limit(device.limits());
        // world.insert_resource(DynMergyBufferAllocator::new(&limit, 4 * 1024 * 1024));

        PluginRenderBindGroup.init(engine, stages);
        PluginScene.init(engine, stages);
        PluginTransformNode.init(engine, stages);
        PluginMesh.init(engine, stages);
        PluginCamera.init(engine, stages);

        PluginCulling.init(engine, stages);
        PluginBufferIndices.init(engine, stages);
        PluginGeometry.init(engine, stages);

        PluginMaterial.init(engine, stages);
        PluginLayerMask.init(engine, stages);

        PluginDefaultMaterial.init(engine, stages);

        PluginRenderer.init(engine, stages);
        PluginBoundingOctTree.init(engine, stages);

        // PluginCubeBuilder.init(engine, stages);
        // PluginBallBuilder.init(engine, stages);
        Ok(())
    }
}