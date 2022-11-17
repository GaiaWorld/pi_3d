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
                            group.write(
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

pub trait InterfaceRenderBindGroup {
    fn as_render_bind_group(
        &self,
        entity: ObjectID,
        layout: BindGroupLayout,
        set: u32,
    ) -> &Self;
}

impl InterfaceRenderBindGroup for crate::engine::Engine {
    fn as_render_bind_group(
            &self,
            entity: ObjectID,
            layout: BindGroupLayout,
            set: u32,
        ) -> &Self {
        let commands = self.world().get_resource_mut::<SingleRenderBindGroupCommandList>().unwrap();

        commands.list.push(RenderBindGroupCommand::Create(entity, layout, set));
        
        self
    }
}