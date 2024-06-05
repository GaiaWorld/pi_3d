

use pi_scene_shell::prelude::*;

use super::*;

#[derive(Clone, Copy)]
pub struct PrimitiveState {
    pub cull: CullMode,
    pub polygon: PolygonMode,
    pub topology: PrimitiveTopology,
    pub unclip_depth: bool,
    pub frontface: FrontFace,
}
impl Default for PrimitiveState {
    fn default() -> Self {
        Self {
            cull: CullMode::Back,
            polygon: PolygonMode::Fill,
            topology: PrimitiveTopology::TriangleList,
            unclip_depth: false,
            frontface: FrontFace::Ccw,
        }
    }
}
impl PrimitiveState {
    pub fn state(&self) -> wgpu::PrimitiveState {
        wgpu::PrimitiveState {
            topology: self.topology.val(),
            front_face: self.frontface.val(),
            polygon_mode: self.polygon.val(),
            cull_mode: self.cull.val(),
            // 不设置可能渲染出来黑的
            unclipped_depth: self.unclip_depth,
            ..Default::default()
        }
    }
}

pub enum EPrimitiveState {
    CCullMode   (CullMode),
    CPolygonMode(PolygonMode),
    CFrontFace  (FrontFace),
    CUnClipDepth(bool),
    Topology    (PrimitiveTopology),
}

pub struct OpsPrimitiveState(pub(crate) Entity, pub(crate) PassTag, pub(crate)EPrimitiveState);
impl OpsPrimitiveState {
    pub fn ops(model: Entity, passtag: PassTag, cmd: EPrimitiveState) -> Self {
        Self(model, passtag, cmd)
    }
}
pub type ActionListPrimitiveState = ActionList<OpsPrimitiveState>;