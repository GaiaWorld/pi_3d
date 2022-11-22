
use plugin::Plugin;

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
pub mod vertex_data;
pub mod main_camera_render;
pub mod layer_mask;
pub mod plugin;
pub mod run_stage;
pub mod object;


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