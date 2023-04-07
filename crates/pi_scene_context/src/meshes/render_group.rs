use std::mem::replace;

use pi_ecs::{prelude::{ResMut, Setup, Commands}};
use pi_ecs_macros::setup;
use pi_engine_shell::run_stage::{TSystemStageInfo, ERunStageChap};

use crate::object::{ObjectID, GameObject};

#[derive(Debug, Clone, Copy)]
pub struct RenderGroup(pub usize);
impl Default for RenderGroup {
    fn default() -> Self {
        Self(2000)
    }
}

#[derive(Debug, Default)]
struct SingleRenderGroupCommandList {
    pub list: Vec<(ObjectID, RenderGroup)>,
}
struct SysRenderGroupCommand;
impl TSystemStageInfo for SysRenderGroupCommand {

}
#[setup]
impl SysRenderGroupCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingleRenderGroupCommandList>,
        mut rendergroup_cmd: Commands<GameObject, RenderGroup>,
    ) {
        let mut list = replace(&mut cmds.list, vec![]);

        list.drain(..).for_each(|(obj, value)| {
            rendergroup_cmd.insert(obj, value);
        });
    }
}

pub trait InterfaceRenderGroup {
    fn render_group(
        &self,
        entity: ObjectID,
        render_group: usize,
    ) -> &Self;
}

impl InterfaceRenderGroup for crate::engine::Engine {
    fn render_group(
        &self,
        entity: ObjectID,
        render_group: usize,
    ) -> &Self {
        let commands = self.world().get_resource_mut::<SingleRenderGroupCommandList>().unwrap();
        commands.list.push((entity, RenderGroup(render_group)));

        self
    }
}

// pub struct PluginRenderGroup;
// impl crate::Plugin for PluginRenderGroup {
//     fn init(
//         &mut self,
//         engine: &mut crate::engine::Engine,
//         stages: &mut crate::run_stage::RunStage,
//     ) -> Result<(), crate::plugin::ErrorPlugin> {
//         let world = engine.world_mut();

//         world.insert_resource(SingleRenderGroupCommandList::default());
//         SysRenderGroupCommand::setup(world, stages.query_stage::<SysRenderGroupCommand>(ERunStageChap::Command));
        
//         Ok(())
//     }
// }