use pi_render::rhi::{pipeline::RenderPipeline};
use pi_slotmap::{SlotMap, DefaultKey};

#[derive(Debug, Default)]
pub struct SingleRenderObjectPipelinePool {
    pub map: SlotMap<DefaultKey, RenderPipeline>,
}