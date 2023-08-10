
use pi_engine_shell::prelude::*;

pub use super::instance::{
    instance_color::*,
    instance_tilloff::*,
    instance_boneoffset::*,
    instance_world_matrix::*,
    sys_instance::*,
    InstanceMesh,
    InstanceSourceRefs,
    DirtyInstanceSourceRefs
};
pub use super::vertex_buffer_useinfo::*;
pub use super::geometry::*;
pub use super::command::*;
pub use super::command_sys::*;
pub use super::base::*;


#[derive(SystemParam)]
pub struct ActionSetGeometry<'w> {
    pub create: ResMut<'w, ActionListGeometryCreate>,
    pub vb_mgr: Res<'w, ShareAssetMgr<EVertexBufferRange>>,
    pub vb_wait: ResMut<'w, VertexBufferDataMap3D>,
}
