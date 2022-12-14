use default_render::PluginDefaultMaterial;
use pi_scene_context::{
    renderers::PluginRenderer,
    meshes::{cube::PluginCubeBuilder, PluginMesh, ball::PluginBallBuilder},
    main_camera_render::PluginMainCameraRender, layer_mask::PluginLayerMask, materials::PluginMaterial,
    vertex_data::{normal::PluginBufferNormal, color4::PluginBufferColor4, indices::PluginBufferIndices},
    cullings::{PluginCulling, oct_tree::PluginBoundingOctTree},
    cameras::PluginCamera,
    transforms::PluginTransformNode,
    scene::PluginScene, resources::PluginResource, geometry::PluginBuildinGeometry
};

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

        PluginBufferIndices.init(engine, stages);
        PluginBuildinGeometry.init(engine, stages);
        PluginBufferColor4.init(engine, stages);
        PluginBufferNormal.init(engine, stages);
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

        // skybox::interface::PluginSkyboxMaterial.init(engine, stages);
        // skybox::PluginSkybox.init(engine, stages);

        // procedural_texture::perlin_noise::interface::PluginPerlinNoiseMaterial.init(engine, stages);
        // procedural_texture::PluginTestPerlinNoise.init(engine, stages);

        // procedural_texture::cloud::interface::PluginCloudMaterial.init(engine, stages);
        // procedural_texture::PluginTestPerlinNoise.init(engine, stages);

        PluginBoundingOctTree.init(engine, stages);
        PluginBallBuilder.init(engine, stages);
        Ok(())
    }
}
