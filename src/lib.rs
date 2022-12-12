use default_render::PluginDefaultMaterial;
use pi_scene_context::{
    cameras::PluginCamera,
    cullings::{oct_tree::PluginBoundingOctTree, PluginCulling},
    layer_mask::PluginLayerMask,
    main_camera_render::PluginMainCameraRender,
    materials::PluginMaterialID,
    meshes::{ball::PluginBallBuilder, cube::PluginCubeBuilder, PluginMesh},
    renderers::PluginRenderer,
    resources::PluginResource,
    scene::PluginScene,
    transforms::PluginTransformNode,
    vertex_data::{
        color4::PluginAttributeColor4, indices::PluginAttributeIndices,
        normal::PluginAttributeNormal, position::PluginAttributePosition, uv::PluginAttributeUV,
    },
};
use skeletons::{matrices_indices::PluginAttributeMatricesIndices, matrices_weights::PluginAttributeMatricesWeights};

pub struct PluginBundleDefault;
impl pi_engine_shell::plugin::Plugin for PluginBundleDefault {
    fn init(
        &mut self,
        engine: &mut pi_engine_shell::engine_shell::EnginShell,
        stages: &mut pi_engine_shell::run_stage::RunStage,
    ) -> Result<(), pi_engine_shell::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        PluginResource.init(engine, stages);
        PluginScene.init(engine, stages);
        PluginTransformNode.init(engine, stages);
        PluginCamera.init(engine, stages);
        PluginCulling.init(engine, stages);

        PluginAttributeColor4.init(engine, stages);
        PluginAttributeNormal.init(engine, stages);
        PluginAttributePosition.init(engine, stages);
        PluginAttributeIndices.init(engine, stages);
        PluginAttributeUV.init(engine, stages);
        PluginAttributeMatricesIndices.init(engine, stages);
        PluginAttributeMatricesWeights.init(engine, stages);

        PluginMesh.init(engine, stages);
        PluginMaterialID.init(engine, stages);
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
