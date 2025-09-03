

use pi_scene_shell::prelude::*;
use serde::{Serialize, Deserialize};

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

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum EPrimitiveState {
    CCullMode   (CullMode),
    CPolygonMode(PolygonMode),
    CFrontFace  (FrontFace),
    CUnClipDepth(bool),
    Topology    (PrimitiveTopology),
}
