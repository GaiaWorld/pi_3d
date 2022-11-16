use std::mem::replace;

use pi_ecs::{prelude::{ResMut, Query, Setup}, query::Write, sys::system};
use pi_ecs_macros::setup;

use crate::object::{ObjectID, GameObject};


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ERenderMode {
    Opaque = 1,
    Skybox = 2,
    AlphaTest = 3,
    Transparent = 4,
}

#[derive(Debug, Default)]
struct SingleRenderModeCommandList {
    pub list: Vec<(ObjectID, ERenderMode)>,
}

struct SysRenderModeCommand;
#[setup]
impl SysRenderModeCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingleRenderModeCommandList>,
        mut items: Query<GameObject, (Write<ERenderMode>)>,
    ) {
        let mut list = replace(&mut cmds.list, vec![]);

        list.drain(..).for_each(|cmd| {
            if let Some((mut mat)) = items.get_mut(cmd.0) {
                mat.write(cmd.1);
            }
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
        world: &mut pi_ecs::world::World,
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {

        world.insert_resource(SingleRenderModeCommandList::default());
        SysRenderModeCommand::setup(world, stages.command_stage());

        Ok(())
    }
}