
use pi_engine_shell::prelude::*;

use super::Enable;

// #[derive(Default, Resource)]
// pub struct SingleEnableCommands(pub Vec<(ObjectID, bool)>);
// impl TCommandList<(ObjectID, bool)> for SingleEnableCommands {
//     fn list(&mut self) -> &mut Vec<(ObjectID, bool)> {
//         &mut self.0
//     }
// }

// pub struct SysEnableCommand;
// #[setup]
// impl SysEnableCommand {
//     #[system]
    // pub fn sys_cmds_enable(
    //     mut cmds: ResMut<SingleEnableCommands>,
    //     mut enable_cmd: Commands<GameObject, Enable>,
    // ) {
    //     let mut list = cmds.reset();
    //     list.drain(..).for_each(|(entity, val)| {
    //         enable_cmd.insert(entity, Enable(val));
    //     })
    // }
// }
