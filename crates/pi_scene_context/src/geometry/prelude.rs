
use pi_scene_shell::prelude::*;

pub use super::instance::{
    instance_color::*,
    instance_tilloff::*,
    instance_boneoffset::*,
    instance_world_matrix::*,
    instance_float::*,
    instance_vec4::*,
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
}

#[derive(SystemParam)]
pub struct ResourceGeometry<'w> {
    pub vb_mgr: Res<'w, ShareAssetMgr<EVertexBufferRange>>,
    pub vb_wait: ResMut<'w, VertexBufferDataMap3D>,
}

pub struct BundleGeometry(
    GeometryDesc,
    VertexBufferLayoutsComp,
    MeshID,
    GeometryRefs,
    RenderGeometryComp,
    IndicesBufferDesc,
    AssetResBufferIndices,
);