

use pi_engine_shell::prelude::*;

use super::LayerMask;

#[derive(Debug)]
pub enum LayerMaskCommand {
    Set(ObjectID, LayerMask),
}
#[derive(Debug, Default, Resource)]
pub struct SingleLayerMaskCommandList {
    pub list: Vec<LayerMaskCommand>,
}

// pub struct SysLayerMaskCommand;
// impl TSystemStageInfo for SysLayerMaskCommand {
    
// }
// #[setup]
// impl SysLayerMaskCommand {
//     #[system]
    pub fn sys_cmd_layer_mask(
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
// }
