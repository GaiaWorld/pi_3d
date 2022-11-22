use pi_ecs::{prelude::{ResMut, Query, Res}, query::Write};
use pi_ecs_macros::setup;
use pi_render::rhi::{bind_group_layout::BindGroupLayout, bind_group::BindGroup, device::RenderDevice};
use pi_slotmap::{SlotMap, DefaultKey};

use crate::object::{ObjectID, GameObject};

#[derive(Debug)]
pub struct RenderBindGroup {
    pub set: u32,
    pub layout: BindGroupLayout,
    pub bind_group: Option<BindGroup>,
    pub offsets: Vec<wgpu::BufferAddress>,
}

impl RenderBindGroup {
    pub fn new(device: &RenderDevice, layout: BindGroupLayout, set: u32) -> Self {
        Self {
            set,
            layout,
            bind_group: None,
            offsets: vec![],
        }
    }
}

pub type RenderBindGroupKey = DefaultKey;

#[derive(Debug, Default)]
pub struct RenderBindGroupPool {
    map: SlotMap<RenderBindGroupKey, RenderBindGroup>,
}
impl RenderBindGroupPool {
    pub fn creat(
        &mut self,
        device: &RenderDevice,
        layout: BindGroupLayout,
        set: u32
    ) -> RenderBindGroupKey {
        self.map.insert(RenderBindGroup::new(device, layout, set))
    }
    pub fn get(
        & self,
        key: RenderBindGroupKey
    ) -> Option<&RenderBindGroup> {
        self.map.get(key)
    }
    pub fn get_mut(&mut self,
        key: RenderBindGroupKey
    ) -> Option<&mut RenderBindGroup> {
        self.map.get_mut(key)
    }
}