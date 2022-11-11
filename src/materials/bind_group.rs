use pi_ecs::{prelude::{ResMut, Query, Res}, query::Write};
use pi_ecs_macros::setup;
use pi_render::rhi::{bind_group_layout::BindGroupLayout, bind_group::BindGroup, device::RenderDevice};

use crate::object::{ObjectID, GameObject};

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