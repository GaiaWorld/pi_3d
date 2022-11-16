use std::mem::replace;

use pi_ecs::{prelude::{ResMut, Query, Setup}, query::Write, sys::system};
use pi_ecs_macros::setup;

use crate::object::{ObjectID, GameObject};

#[derive(Debug, Clone, Copy)]
pub struct AlphaIndex(pub usize);
impl Default for AlphaIndex {
    fn default() -> Self {
        Self(2000)
    }
}

#[derive(Debug, Default)]
struct SingleAlphaIndexCommandList {
    pub list: Vec<(ObjectID, AlphaIndex)>,
}
struct SysAlphaIndexCommand;
#[setup]
impl SysAlphaIndexCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingleAlphaIndexCommandList>,
        mut meshes: Query<GameObject, Write<AlphaIndex>>,
    ) {
        let mut list = replace(&mut cmds.list, vec![]);

        list.drain(..).for_each(|cmd| {
            if let Some(mut mesh) = meshes.get_mut(cmd.0) {
                mesh.insert_no_notify(cmd.1);
            }
        });
    }
}

pub trait InterfaceAlphaIndex {
    fn alpha_index(
        &self,
        entity: ObjectID,
        alpha_index: usize,
    ) -> &Self;
}

impl InterfaceAlphaIndex for crate::engine::Engine {
    fn alpha_index(
        &self,
        entity: ObjectID,
        alpha_index: usize,
    ) -> &Self {
        let commands = self.world().get_resource_mut::<SingleAlphaIndexCommandList>().unwrap();
        commands.list.push((entity, AlphaIndex(alpha_index)));

        self
    }
}

pub struct PluginAlphaIndex;
impl crate::Plugin for PluginAlphaIndex {
    fn init(
        &mut self,
        world: &mut pi_ecs::world::World,
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {

        world.insert_resource(SingleAlphaIndexCommandList::default());
        SysAlphaIndexCommand::setup(world, stages.command_stage());
        
        Ok(())
    }
}