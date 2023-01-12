use default_render::PluginDefaultMaterial;
use pi_mesh_builder::{cube::PluginCubeBuilder, ball::PluginBallBuilder};
use pi_scene_context::{
    renderers::PluginRenderer,
    meshes::{PluginMesh,},
    main_camera_render::PluginMainCameraRender, layer_mask::PluginLayerMask, materials::PluginMaterial,
    cullings::{PluginCulling, oct_tree::PluginBoundingOctTree},
    cameras::PluginCamera,
    transforms::PluginTransformNode,
    scene::PluginScene, geometry::{PluginBuildinGeometry, indices::PluginBufferIndices}, bindgroup::PluginRenderBindGroup
};

pub struct PluginBundleDefault;
impl pi_engine_shell::plugin::Plugin for PluginBundleDefault {
    fn init(
        &mut self,
        engine: &mut pi_engine_shell::engine_shell::EnginShell,
        stages: &mut pi_engine_shell::run_stage::RunStage,
    ) -> Result<(), pi_engine_shell::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        PluginRenderBindGroup.init(engine, stages);
        PluginScene.init(engine, stages);
        PluginTransformNode.init(engine, stages);
        PluginCamera.init(engine, stages);
        PluginCulling.init(engine, stages);

        PluginBufferIndices.init(engine, stages);
        PluginBuildinGeometry.init(engine, stages);
        // PluginBufferPosition.init(engine, stages);
        // PluginAttributeIndices.init(engine, stages);
        // PluginBufferUV.init(engine, stages);
        PluginMesh.init(engine, stages);
        PluginMaterial.init(engine, stages);
        PluginLayerMask.init(engine, stages);

        PluginMainCameraRender.init(engine, stages);

        PluginDefaultMaterial.init(engine, stages);
        PluginCubeBuilder.init(engine, stages);

        PluginRenderer.init(engine, stages);

        PluginBoundingOctTree.init(engine, stages);
        PluginBallBuilder.init(engine, stages);
        Ok(())
    }
}