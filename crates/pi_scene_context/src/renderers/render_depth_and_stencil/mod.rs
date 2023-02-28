use std::mem::replace;

use pi_ecs::{prelude::{ResMut, Setup, Commands}};
use pi_ecs_macros::setup;
use pi_engine_shell::run_stage::{TSystemStageInfo, ERunStageChap};
use pi_render::renderer::pipeline::DepthStencilState;

use crate::object::{ObjectID, GameObject};

#[derive(Debug, Clone)]
pub struct RenderDepthAndStencil(pub Option<DepthStencilState>);
impl Default for RenderDepthAndStencil {
    fn default() -> Self {
        Self(None)
    }
}

#[derive(Debug, Clone)]
pub enum ERenderDepthAndStencilCommand {
    Disable(ObjectID),
    DepthStencil(ObjectID, RenderDepthAndStencil),
}

#[derive(Debug, Default)]
pub struct SingleRenderDepthAndStencilCommandList {
    pub list: Vec<ERenderDepthAndStencilCommand>,
}

pub struct SysRenderDepthAndStencilCommand;
impl TSystemStageInfo for SysRenderDepthAndStencilCommand {
    
}
#[setup]
impl SysRenderDepthAndStencilCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingleRenderDepthAndStencilCommandList>,
        mut item_cmd: Commands<GameObject, RenderDepthAndStencil>,
    ) {
        let mut list = replace(&mut cmds.list, vec![]);
        list.drain(..).for_each(|cmd| {
            match cmd {
                ERenderDepthAndStencilCommand::Disable(entity) => {
                    item_cmd.insert(entity, RenderDepthAndStencil::default());
                },
                ERenderDepthAndStencilCommand::DepthStencil(entity, value) => {
                    item_cmd.insert(entity, value);
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
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        world.insert_resource(SingleRenderDepthAndStencilCommandList::default());

        SysRenderDepthAndStencilCommand::setup(world, stages.query_stage::<SysRenderDepthAndStencilCommand>(ERunStageChap::Initial));

        Ok(())
    }
}