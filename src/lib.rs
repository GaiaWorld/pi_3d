use default_render::PluginDefaultMaterial;
use pi_scene_context::{texture::{texture2d::PluginTexture2D, texture_sampler::PluginTextureSampler}, renderers::PluginRenderer, meshes::{cube::PluginCubeBuilder, PluginMesh, ball::PluginBallBuilder}, main_camera_render::PluginMainCameraRender, layer_mask::PluginLayerMask, materials::PluginMaterialID, vertex_data::{uv::PluginAttributeUV, indices::PluginAttributeIndices, position::PluginAttributePosition, normal::PluginAttributeNormal, color4::PluginAttributeColor4}, cullings::{PluginCulling, oct_tree::PluginBoundingOctTree}, cameras::PluginCamera, transforms::PluginTransformNode, scene::PluginScene, resources::PluginResource};

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

        PluginTextureSampler.init(engine, stages);
        PluginTexture2D.init(engine, stages);

        skybox::interface::PluginSkyboxMaterial.init(engine, stages);
        skybox::PluginSkybox.init(engine, stages);

        procedural_texture::perlin_noise::interface::PluginPerlinNoiseMaterial.init(engine, stages);
        procedural_texture::PluginTestPerlinNoise.init(engine, stages);

        procedural_texture::cloud::interface::PluginCloudMaterial.init(engine, stages);
        // procedural_texture::PluginTestPerlinNoise.init(engine, stages);

        PluginBoundingOctTree.init(engine, stages);
        PluginBallBuilder.init(engine, stages);
        Ok(())
    }
}