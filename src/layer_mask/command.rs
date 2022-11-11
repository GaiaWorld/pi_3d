
use pi_ecs::{prelude::{ResMut, Query, Setup}, query::Write, entity};
use pi_ecs_macros::setup;

use crate::{object::{ObjectID, GameObject}, plugin::Plugin};

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
#[setup]
impl SysLayerMaskCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingleLayerMaskCommandList>,
        mut objects: Query<GameObject, Write<LayerMask>>,
    ) {
        cmds.list.drain(..).for_each(|cmd| {
            match cmd {
                LayerMaskCommand::Set(entity, layer) => {
                    match objects.get_mut(entity) {
                        Some(mut object) => {
                            object.insert_no_notify(layer);
                        },
                        None => todo!(),
                    }
                },
            }
        });
    }
}
