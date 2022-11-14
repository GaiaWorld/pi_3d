use std::mem::replace;

use pi_ecs::{prelude::{ResMut, Query, Setup}, query::{Write, WithOut}, sys::system};
use pi_ecs_macros::setup;

use crate::object::{ObjectID, GameObject};

#[derive(Debug, Clone, Copy)]
pub struct RenderDepthAndStencil {
    pub depth: bool,
    pub stencil: bool,
    pub depth_compare: wgpu::CompareFunction,
}
impl Default for RenderDepthAndStencil {
    fn default() -> Self {
        Self {
            depth: false,
            stencil: false,
            depth_compare: wgpu::CompareFunction::LessEqual,
        }
    }
}
impl RenderDepthAndStencil {
    pub fn state(
        &self
    ) -> Option<wgpu::DepthStencilState> {
        match (self.depth, self.stencil) {
            (true, true) => {
                Some(
                    wgpu::DepthStencilState {
                        format: wgpu::TextureFormat::Depth24PlusStencil8,
                        depth_write_enabled: true,
                        depth_compare: self.depth_compare,
                        stencil: wgpu::StencilState::default(),
                        bias: wgpu::DepthBiasState::default(),
                    }
                )
            },
            (true, false) => {
                Some(
                    wgpu::DepthStencilState {
                        format: wgpu::TextureFormat::Depth24PlusStencil8,
                        depth_write_enabled: true,
                        depth_compare: self.depth_compare,
                        stencil: wgpu::StencilState::default(),
                        bias: wgpu::DepthBiasState::default(),
                    }
                )
            },
            (false, true) => {
                Some(
                    wgpu::DepthStencilState {
                        format: wgpu::TextureFormat::Depth24PlusStencil8,
                        depth_write_enabled: false,
                        depth_compare: self.depth_compare,
                        stencil: wgpu::StencilState::default(),
                        bias: wgpu::DepthBiasState::default(),
                    }
                )
            },
            (false, false) => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum ERenderDepthAndStencilCommand {
    Disable(ObjectID),
    DepthStencil(ObjectID, RenderDepthAndStencil),
}

#[derive(Debug, Default)]
struct SingleRenderDepthAndStencilCommandList {
    pub list: Vec<ERenderDepthAndStencilCommand>,
}

struct SysRenderDepthAndStencilCommand;
#[setup]
impl SysRenderDepthAndStencilCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingleRenderDepthAndStencilCommandList>,
        mut items: Query<GameObject, (Write<RenderDepthAndStencil>)>,
    ) {
        let mut list = replace(&mut cmds.list, vec![]);
        list.drain(..).for_each(|cmd| {
            match cmd {
                ERenderDepthAndStencilCommand::Disable(entity) => {
                    if let Some((mut item)) = items.get_mut(entity) {
                        match item.get_mut() {
                            Some(item) => {
                                item.depth = false;
                                item.stencil = false;
                            },
                            None => {
                                item.insert_no_notify(RenderDepthAndStencil::default());
                            },
                        }
                        item.notify_modify();
                    }
                },
                ERenderDepthAndStencilCommand::DepthStencil(entity, value) => {
                    if let Some((mut item)) = items.get_mut(entity) {
                        item.write(value);
                    }
                },
            }
        });
    }
}

pub trait InterfaceRenderDepthAndStencil {
    fn depth_stencil(
        &self,
        entity: ObjectID,
        value: RenderDepthAndStencil,
    ) -> &Self;

    fn disable_depth_stencil(
        &self,
        entity: ObjectID
    ) -> &Self;
}
impl InterfaceRenderDepthAndStencil for crate::engine::Engine {
    fn depth_stencil(
        &self,
        entity: ObjectID,
        value: RenderDepthAndStencil,
    ) -> &Self {
        let world = self.world();

        let commands = world.get_resource_mut::<SingleRenderDepthAndStencilCommandList>().unwrap();
        commands.list.push(ERenderDepthAndStencilCommand::DepthStencil(entity, value));

        self
    }

    fn disable_depth_stencil(
        &self,
        entity: ObjectID
    ) -> &Self {
        let world = self.world();

        let commands = world.get_resource_mut::<SingleRenderDepthAndStencilCommandList>().unwrap();
        commands.list.push(ERenderDepthAndStencilCommand::Disable(entity));

        self
    }
}

pub struct PluginRenderDepthAndStencil;
impl crate::Plugin for PluginRenderDepthAndStencil {
    fn init(
        &mut self,
        world: &mut pi_ecs::world::World,
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {

        world.insert_resource(SingleRenderDepthAndStencilCommandList::default());

        SysRenderDepthAndStencilCommand::setup(world, stages.command_stage());

        Ok(())
    }
}