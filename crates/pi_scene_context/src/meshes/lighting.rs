use pi_scene_shell::prelude::*;

#[derive(Component)]
pub struct MeshCastShadow(pub bool);

#[derive(Component)]
pub struct MeshReceiveShadow(pub bool);

#[derive(Component, Default)]
pub enum MeshLightingMode {
    #[default]
    UnLit,
    Lambert,
    BSDF,
}