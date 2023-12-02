
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
pub mod geometry;
pub mod materials;
pub mod postprocess;
pub mod layer_mask;
pub mod plugin;
pub mod run_stage;
pub mod object;
pub mod viewer;
pub mod bindgroup;
pub mod light;
pub mod shadow;
pub mod skeleton;
pub mod animation;
pub mod pass;
pub mod state;
pub mod commands;
pub mod prelude;

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