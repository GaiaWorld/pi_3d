use pi_engine_shell::prelude::*;

#[derive(Component)]
pub struct MeshCastShadow(pub bool);

#[derive(Component)]
pub struct MeshReceiveShadow(pub bool);