use pi_scene_shell::prelude::*;

/// 标识 Mesh 是否产生阴影
#[derive(Component, Default)]
pub struct MeshCastShadow(pub bool);

/// 标识 Mesh 是否接受阴影
#[derive(Component, Default)]
pub struct MeshReceiveShadow(pub bool);

/// 标识 Mesh 光照模式
#[derive(Component, Default)]
pub enum MeshLightingMode {
    #[default]
    UnLit,
    Lambert,
    BSDF,
}