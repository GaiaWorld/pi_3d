use pi_scene_shell::prelude::*;

#[derive(Component, Default)]
pub struct MeshCastShadow(pub bool);

#[derive(Component, Default)]
pub struct MeshReceiveShadow(pub bool);

#[derive(Component, Default)]
pub enum MeshLightingMode {
    #[default]
    UnLit,
    Lambert,
    BSDF,
}