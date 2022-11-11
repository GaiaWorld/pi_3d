use pi_ecs::{prelude::{ResMut, Query, Res}, query::Write};
use pi_ecs_macros::setup;
use pi_render::rhi::{bind_group_layout::BindGroupLayout, device::RenderDevice};

use crate::object::{ObjectID, GameObject};

use super::bind_group::RenderBindGroup;



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