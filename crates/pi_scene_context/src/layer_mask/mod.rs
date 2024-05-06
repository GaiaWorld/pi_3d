///
/// Layer Mask
/// * 通过 layer mask 数据标识目标的层级信息
/// * 提供用户操作接口, 对应实现操作命令, 实现操作命令队列, 命令队列的执行System

use pi_scene_shell::prelude::*;

use self::{
    command::*,
    command_sys::*,
};

mod base;
pub mod command_sys;
mod command;
mod interface;
pub mod prelude;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash,  PartialOrd, Ord)]
pub enum StageLayerMask {
    Command
}

pub struct PluginLayerMask;
impl Plugin for PluginLayerMask {
    fn build(&self, app: &mut pi_scene_shell::prelude::App) {
        app.world.insert_single_res(ActionListLayerMask::default());
        // app.configure_set(Update, StageLayerMask::Command);
        app.add_system(
            Update,
            // (
                sys_act_layer_mask      // .run_if(should_run)
            // )
            /* .in_set(StageLayerMask::Command) */);
    }
}
