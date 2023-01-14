use pi_render::{components::view::target_alloc::ShareTargetView, graph::param::OutParam};


#[derive(Clone)]
pub struct RendererGraphicParam {
    pub srt: Option<ShareTargetView>,
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub depth: bool,
}
impl Default for RendererGraphicParam {
    fn default() -> Self {
        Self {
            srt: None,
            x: 0,
            y: 0,
            w: 0,
            h: 0,
            depth: false,
        }
    }
}
impl OutParam for RendererGraphicParam {
    fn can_fill(&self, set: &mut Option<&mut pi_hash::XHashSet<std::any::TypeId>>, ty: std::any::TypeId) -> bool {
        if set.is_none() {
            true
        } else {
            std::any::TypeId::of::<Self>() == ty
        }
    }

    fn fill_to(&self, this_id: pi_render::graph::NodeId, to: &mut dyn pi_render::graph::param::Assign, ty: std::any::TypeId) -> bool {
        if std::any::TypeId::of::<Self>() == ty {
            true
        } else {
            false
        }
    }
}