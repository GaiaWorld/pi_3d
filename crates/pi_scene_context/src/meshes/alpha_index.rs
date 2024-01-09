use std::mem::replace;

use pi_ecs::{prelude::{ResMut, Setup, Commands}};
use pi_ecs_macros::setup;
use pi_engine_shell::run_stage::{TSystemStageInfo, ERunStageChap};

use crate::object::{ObjectID, GameObject};

#[derive(Debug, Clone, Copy)]
pub struct RenderQueue(pub usize);
impl Default for RenderQueue {
    fn default() -> Self {
        Self(2000)
    }
}

#[derive(Debug, Default)]
struct SingleAlphaIndexCommandList {
    pub list: Vec<(ObjectID, RenderQueue)>,
}
struct SysAlphaIndexCommand;
impl TSystemStageInfo for SysAlphaIndexCommand {}
#[setup]
impl SysAlphaIndexCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingleAlphaIndexCommandList>,
        mut meshes: Commands<GameObject, RenderQueue>,
    ) {
        let mut list = replace(&mut cmds.list, vec![]);

        list.drain(..).for_each(|cmd| {
            meshes.insert(cmd.0, cmd.1);
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
        commands.list.push((entity, RenderQueue(alpha_index)));

        self
    }
}