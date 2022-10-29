use cameras::PluginCamera;
use cullings::PluginCulling;
use default_render::PluginDefaultMaterial;
use layer_mask::PluginLayerMask;
use materials::PluginMaterialID;
use meshes::{cube::PluginCubeBuilder, PluginMesh};
use plugin::Plugin;
use main_camera_render::PluginMainCameraRender;
use scene::PluginScene;
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
        engine: &mut engine::Engine,
        stages: &mut run_stage::RunStage,
    ) -> Result<(), plugin::ErrorPlugin> {
        PluginScene::init(engine, stages);
        PluginTransformNode::init(engine, stages);
        PluginCamera::init(engine, stages);
        PluginCulling::init(engine, stages);

        PluginAttributeColor4::init(engine, stages);
        PluginAttributeNormal::init(engine, stages);
        PluginAttributePosition::init(engine, stages);
        PluginAttributeIndices::init(engine, stages);
        PluginAttributeUV::init(engine, stages);

        PluginMesh::init(engine, stages);
        PluginMaterialID::init(engine, stages);
        PluginLayerMask::init(engine, stages);

        PluginDefaultMaterial::init(engine, stages);

        PluginMainCameraRender::init(engine, stages);
        PluginCubeBuilder::init(engine, stages);

        Ok(())
    }
}