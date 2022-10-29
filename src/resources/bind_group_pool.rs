use pi_render::rhi::{bind_group::BindGroup};
use pi_slotmap::{SlotMap, DefaultKey};

#[derive(Debug, Default)]
pub struct SingleRenderBindGroupPool {
    pub map: SlotMap<DefaultKey, BindGroup>,
}
