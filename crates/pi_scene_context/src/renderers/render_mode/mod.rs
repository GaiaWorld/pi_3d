use std::mem::replace;

use pi_ecs::{prelude::{ResMut, Setup, Commands}};
use pi_ecs_macros::setup;
use pi_engine_shell::run_stage::{TSystemStageInfo, ERunStageChap};

use crate::object::{ObjectID, GameObject};

pub struct RenderMode(pub ERenderMode);


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ERenderMode {
    Opaque = 1,
    Skybox = 2,
    AlphaTest = 3,
    Transparent = 4,
}

#[derive(Debug, Default)]
pub struct SingleRenderModeCommandList {
    pub list: Vec<(ObjectID, ERenderMode)>,
}

pub struct SysRenderModeCommand;
impl TSystemStageInfo for SysRenderModeCommand {

}
#[setup]
impl SysRenderModeCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingleRenderModeCommandList>,
        mut items: Commands<GameObject, RenderMode>,
    ) {
        let mut list = replace(&mut cmds.list, vec![]);

        list.drain(..).for_each(|(obj, value)| {
            items.insert(obj, RenderMode(value))
        });
    }
}

pub trait InterfaceRenderMode {
    fn render_mode(
        &self,
        entity: ObjectID,
        mode: ERenderMode,
    ) -> &Self;
}

impl InterfaceRenderMode for crate::engine::Engine {
    fn render_mode(
        &self,
        entity: ObjectID,
        mode: ERenderMode,
    ) -> &Self {
        let commands = self.world().get_resource_mut::<SingleRenderModeCommandList>().unwrap();
        commands.list.push((entity, mode));

        self
    }
}

pub struct PluginRenderMode;
impl crate::Plugin for PluginRenderMode {
    fn init(
        &mut self,
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        world.insert_resource(SingleRenderModeCommandList::default());
        SysRenderModeCommand::setup(world, stages.query_stage::<SysRenderModeCommand>(ERunStageChap::Initial));

        Ok(())
    }
}