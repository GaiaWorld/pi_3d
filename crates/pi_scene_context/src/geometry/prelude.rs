
use pi_engine_shell::prelude::*;

pub use super::instance::{
    instance_color::*,
    instance_tilloff::*,
    instance_world_matrix::*,
    sys_instance::*,
    InstanceSourceID,
    InstanceSourceRefs,
    DirtyInstanceSourceRefs
};
pub use super::vertex_buffer_useinfo::*;
pub use super::geometry::*;
pub use super::command::*;
pub use super::base::*;


#[derive(SystemParam)]
pub struct ActionSetGeometry<'w> {
    pub create: ResMut<'w, ActionListGeometryCreate>,
}

#[derive(SystemParam)]
pub struct ActionSetVertexIndexBuffer<'w> {
    pub asset_mgr: Res<'w, ShareAssetMgr<EVertexBufferRange>>,
    pub data_map: ResMut<'w, VertexBufferDataMap3D>,
}
