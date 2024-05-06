use pi_scene_shell::prelude::*;


pub struct MeshCastShadow(pub bool);


pub struct MeshReceiveShadow(pub bool);

#[derive( Default)]
pub enum MeshLightingMode {
    #[default]
    UnLit,
    Lambert,
    BSDF,
}