use std::mem::replace;

use pi_ecs::{query::Write, prelude::{Query, ResMut, Setup}, sys::system};
use pi_ecs_macros::setup;

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
enum ERenderPrimitiveCommand {
    Default(ObjectID),
    Line(ObjectID),
    New(ObjectID, PrimitiveState),
}

#[derive(Debug, Default)]
struct SingleRenderPrimitiveCommandList {
    pub list: Vec<ERenderPrimitiveCommand>,
}

struct SysRenderPrimitiveCommand;
#[setup]
impl SysRenderPrimitiveCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingleRenderPrimitiveCommandList>,
        mut blends: Query<GameObject, (Write<PrimitiveState>)>,
    ) {
        let mut list = replace(&mut cmds.list, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                ERenderPrimitiveCommand::Default(entity) => {
                    if let Some((mut blend)) = blends.get_mut(entity) {
                        
                        blend.write(PrimitiveState::default());
                    }
                },
                ERenderPrimitiveCommand::Line(entity) => {
                    if let Some((mut blend)) = blends.get_mut(entity) {
                        let mut value = PrimitiveState::default();
                        value.state.topology = wgpu::PrimitiveTopology::LineStrip;
                        blend.write(value);
                    }
                },
                ERenderPrimitiveCommand::New(entity, value) => {
                    if let Some((mut blend)) = blends.get_mut(entity) {
                        blend.write(value);
                    }
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

        SysRenderPrimitiveCommand::setup(world, stages.command_stage());

        Ok(())
    }
}