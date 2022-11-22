use default_render::PluginDefaultMaterial;
use skybox::{PluginSkybox, interface::PluginSkyboxMaterial};
use pi_scene_context::{renderers::PluginRenderer, meshes::{cube::PluginCubeBuilder, PluginMesh}, main_camera_render::PluginMainCameraRender, layer_mask::PluginLayerMask, materials::PluginMaterialID, vertex_data::{uv::PluginAttributeUV, indices::PluginAttributeIndices, position::PluginAttributePosition, normal::PluginAttributeNormal, color4::PluginAttributeColor4}, cullings::PluginCulling, cameras::PluginCamera, transforms::PluginTransformNode, scene::PluginScene, resources::PluginResource};

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

        PluginMesh.init(engine, stages);
        PluginMaterialID.init(engine, stages);
        PluginLayerMask.init(engine, stages);

        PluginMainCameraRender.init(engine, stages);

        PluginDefaultMaterial.init(engine, stages);
        PluginCubeBuilder.init(engine, stages);

        PluginRenderer.init(engine, stages);

        PluginSkyboxMaterial.init(engine, stages);
        PluginSkybox.init(engine, stages);

        Ok(())
    }
}