use pi_scene_shell::prelude::*;

///
/// 标识目标类型 AbstructMesh
#[derive(Component, Default, Clone)]
pub struct AbstructMesh;

///
/// 标识Mesh需要重新检查被哪些Viewer包含
#[derive(Component, Default, Clone)]
pub struct FlagMeshNeedRecheckForView;