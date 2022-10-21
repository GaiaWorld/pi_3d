
pub mod object;
pub mod scene;
pub mod transforms;
pub mod cameras;
pub mod cullings;
pub mod renderers;
pub mod systems;
pub mod meshes;
pub mod tree;
pub mod flags;
pub mod shaders;
pub mod resources;
pub mod engine;
pub mod environment;
pub mod geometry;
pub mod materials;
pub mod default_render;

pub fn bytes_write_to_memory(
    bytes: &[u8],
    offset: usize,
    memory: &mut [u8],
) {
    let mut index = 0;
    for v in bytes.iter() {
        memory[offset + index] = *v;
    }
}