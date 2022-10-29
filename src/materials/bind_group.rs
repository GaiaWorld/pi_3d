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
    fn new(device: &RenderDevice, layout: BindGroupLayout, set: u32) -> Self {
        Self {
            set,
            layout,
            bind_group: None,
            offsets: vec![],
        }
    }
}

#[derive(Debug)]
pub enum RenderBindGroupCommand {
    Create(ObjectID, BindGroupLayout, u32),
}
#[derive(Debug, Default)]
pub struct SingleRenderBindGroupCommandList {
    pub list: Vec<RenderBindGroupCommand>,
}

pub struct SysRenderBindGroupCommand;
#[setup]
impl SysRenderBindGroupCommand {
    #[system]
    pub fn sys(
        mut cmds: ResMut<SingleRenderBindGroupCommandList>,
        mut groups: Query<GameObject, Write<RenderBindGroup>>,
        device: Res<RenderDevice>,
    ) {
        cmds.list.drain(..).for_each(|cmd| {
            match cmd {
                RenderBindGroupCommand::Create(entity, layout, set) => {
                    match groups.get_mut(entity) {
                        Some(mut group) => {
                            group.insert_no_notify(
                                RenderBindGroup::new(&device, layout, set)
                            );
                        },
                        None => todo!(),
                    }
                },
            }
        });
    }
}