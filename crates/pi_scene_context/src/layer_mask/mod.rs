///
/// Layer Mask
/// * 通过 layer mask 数据标识目标的层级信息
/// * 提供用户操作接口, 对应实现操作命令, 实现操作命令队列, 命令队列的执行System

use pi_engine_shell::prelude::*;

use self::{
    command::*,
    command_sys::*,
};

mod base;
pub mod command_sys;
mod command;
mod interface;
pub mod prelude;


pub struct PluginLayerMask;
impl Plugin for PluginLayerMask {
    // fn init(
    //     &mut self,
    //     engine: &mut crate::engine::Engine,
    //     stages: &mut crate::run_stage::RunStage,
    // ) -> Result<(), crate::plugin::ErrorPlugin> {
    //     let world = engine.world_mut();

    //     SysLayerMaskCommand::setup(world, stages.query_stage::<SysLayerMaskCommand>(ERunStageChap::Initial));

    //     world.insert_resource(SingleLayerMaskCommandList::default());

    //     Ok(())
    // }

    fn build(&self, app: &mut pi_engine_shell::prelude::App) {
        app.insert_resource(ActionListLayerMask::default());
        app.add_systems(Update, sys_act_layer_mask.run_if(should_run).in_set(ERunStageChap::Command));

        // app.world.insert_resource(SingleLayerMaskCommandList::default());
    }
}
