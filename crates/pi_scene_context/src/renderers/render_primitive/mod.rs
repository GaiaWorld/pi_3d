use std::mem::replace;

use pi_ecs::{query::Write, prelude::{Query, ResMut, Setup, Commands}, sys::system};
use pi_ecs_macros::setup;
use pi_engine_shell::run_stage::{TSystemStageInfo, ERunStageChap};

use crate::object::{ObjectID, GameObject};


#[derive(Debug, Clone, Copy)]
pub struct PrimitiveState {
    pub state: wgpu::PrimitiveState,
}
impl Default for PrimitiveState {
    fn default() -> Self {
        Self {
            state: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                front_face: wgpu::FrontFace::Ccw,
                polygon_mode: wgpu::PolygonMode::Fill,
                cull_mode: Some(wgpu::Face::Back),
                // 不设置可能渲染出来黑的
                unclipped_depth: true,
                ..Default::default()
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ERenderPrimitiveCommand {
    Default(ObjectID),
    Line(ObjectID),
    New(ObjectID, PrimitiveState),
}

#[derive(Debug, Default)]
pub struct SingleRenderPrimitiveCommandList {
    pub list: Vec<ERenderPrimitiveCommand>,
}

pub struct SysRenderPrimitiveCommand;
impl TSystemStageInfo for SysRenderPrimitiveCommand {

}
#[setup]
impl SysRenderPrimitiveCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingleRenderPrimitiveCommandList>,
        mut blends: Commands<GameObject, PrimitiveState>,
    ) {
        let mut list = replace(&mut cmds.list, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                ERenderPrimitiveCommand::Default(entity) => {
                    blends.insert(entity, PrimitiveState::default());
                },
                ERenderPrimitiveCommand::Line(entity) => {
                    let mut value = PrimitiveState::default();
                    value.state.topology = wgpu::PrimitiveTopology::LineStrip;
                    blends.insert(entity, value);
                },
                ERenderPrimitiveCommand::New(entity, value) => {
                    blends.insert(entity, value);
                },
            }
        });
    }
}

pub trait InterfaceRenderPrimitive {
    fn primitive(
        &self,
        entity: ObjectID,
        value: PrimitiveState,
    ) -> &Self;
}

impl InterfaceRenderPrimitive for crate::engine::Engine {
    fn primitive(
        &self,
        entity: ObjectID,
        value: PrimitiveState,
    ) -> &Self {
        let world = self.world();

        let commands = world.get_resource_mut::<SingleRenderPrimitiveCommandList>().unwrap();
        commands.list.push(ERenderPrimitiveCommand::New(entity, value));

        self
    }
}

pub struct PluginRenderPrimitive;
impl crate::Plugin for PluginRenderPrimitive {
    fn init(
        &mut self,
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        world.insert_resource(SingleRenderPrimitiveCommandList::default());

        SysRenderPrimitiveCommand::setup(world, stages.query_stage::<SysRenderPrimitiveCommand>(ERunStageChap::Command));

        Ok(())
    }
}