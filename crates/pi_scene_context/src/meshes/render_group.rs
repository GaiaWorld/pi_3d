use std::mem::replace;

use pi_ecs::{prelude::{ResMut, Query, Setup}, query::Write};
use pi_ecs_macros::setup;

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
#[setup]
impl SysRenderGroupCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingleRenderGroupCommandList>,
        mut meshes: Query<GameObject, Write<RenderGroup>>,
    ) {
        let mut list = replace(&mut cmds.list, vec![]);

        list.drain(..).for_each(|cmd| {
            if let Some(mut mesh) = meshes.get_mut(cmd.0) {
                mesh.insert_no_notify(cmd.1);
            }
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

pub struct PluginRenderGroup;
impl crate::Plugin for PluginRenderGroup {
    fn init(
        &mut self,
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        world.insert_resource(SingleRenderGroupCommandList::default());
        SysRenderGroupCommand::setup(world, stages.command_stage());
        
        Ok(())
    }
}