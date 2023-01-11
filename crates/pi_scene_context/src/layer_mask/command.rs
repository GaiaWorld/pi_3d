
use pi_ecs::{prelude::{ResMut, Query, Commands}, query::Write};
use pi_ecs_macros::setup;
use pi_engine_shell::run_stage::{TSystemStageInfo, SysCommonUserCommand};

use crate::{object::{ObjectID, GameObject}};

use super::LayerMask;

#[derive(Debug)]
pub enum LayerMaskCommand {
    Set(ObjectID, LayerMask),
}
#[derive(Debug, Default)]
pub struct SingleLayerMaskCommandList {
    pub list: Vec<LayerMaskCommand>,
}

pub struct SysLayerMaskCommand;
impl TSystemStageInfo for SysLayerMaskCommand {
    
}
#[setup]
impl SysLayerMaskCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingleLayerMaskCommandList>,
        mut layer_cmd: Commands<GameObject, LayerMask>,
    ) {
        cmds.list.drain(..).for_each(|cmd| {
            match cmd {
                LayerMaskCommand::Set(entity, layer) => {
                    layer_cmd.insert(entity, layer);
                },
            }
        });
    }
}
