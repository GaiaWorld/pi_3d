use cameras::PluginCamera;
use cullings::PluginCulling;
use default_render::PluginDefaultMaterial;
use layer_mask::PluginLayerMask;
use materials::PluginMaterialID;
use meshes::{cube::PluginCubeBuilder, PluginMesh};
use plugin::Plugin;
use main_camera_render::PluginMainCameraRender;
use resources::PluginResource;
use scene::PluginScene;
use skybox::{PluginSkybox, interface::PluginSkyboxMaterial};
use transforms::PluginTransformNode;
use vertex_data::{color4::PluginAttributeColor4, normal::PluginAttributeNormal, position::PluginAttributePosition, indices::PluginAttributeIndices, uv::PluginAttributeUV};

pub mod object;
pub mod scene;
pub mod transforms;
pub mod cameras;
pub mod cullings;
pub mod renderers;
pub mod meshes;
pub mod tree;
pub mod flags;
pub mod shaders;
pub mod resources;
pub mod engine;
pub mod environment;
pub mod geometry;
pub mod materials;
pub mod postprocess;
pub mod default_render;
pub mod plugin;
pub mod vertex_data;
pub mod run_stage;
pub mod main_camera_render;
pub mod layer_mask;
pub mod skybox;
pub mod texture2d;
pub mod context;


pub fn bytes_write_to_memory(
    bytes: &[u8],
    offset: usize,
    memory: &mut [u8],
) {
    let mut index = 0;
    for v in bytes.iter() {
        memory[offset + index] = *v;
        index += 1;
    }
}

pub struct PluginBundleDefault;
impl Plugin for PluginBundleDefault {
    fn init(
        &mut self,
        world: &mut pi_ecs::world::World,
        engine: &mut engine::Engine,
        stages: &mut run_stage::RunStage,
    ) -> Result<(), plugin::ErrorPlugin> {
        PluginResource.init(world, engine, stages);
        PluginScene.init(world, engine, stages);
        PluginTransformNode.init(world, engine, stages);
        PluginCamera.init(world, engine, stages);
        PluginCulling.init(world, engine, stages);

        PluginAttributeColor4.init(world, engine, stages);
        PluginAttributeNormal.init(world, engine, stages);
        PluginAttributePosition.init(world, engine, stages);
        PluginAttributeIndices.init(world, engine, stages);
        PluginAttributeUV.init(world, engine, stages);

        PluginMesh.init(world, engine, stages);
        PluginMaterialID.init(world, engine, stages);
        PluginLayerMask.init(world, engine, stages);

        PluginMainCameraRender.init(world, engine, stages);

        PluginDefaultMaterial.init(world, engine, stages);
        PluginCubeBuilder.init(world, engine, stages);

        PluginSkyboxMaterial.init(world, engine, stages);
        PluginSkybox.init(world, engine, stages);

        Ok(())
    }
}