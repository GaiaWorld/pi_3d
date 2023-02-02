///
/// Layer Mask
/// * 通过 layer mask 数据标识目标的层级信息
/// * 提供用户操作接口, 对应实现操作命令, 实现操作命令队列, 命令队列的执行System
use pi_ecs::{prelude::{Setup}};
use pi_engine_shell::run_stage::{ERunStageChap};

use crate::{plugin::Plugin};

use self::command::{SysLayerMaskCommand, SingleLayerMaskCommandList};

pub mod command;
pub mod interface;

#[derive(Debug, Clone, Copy)]
pub struct LayerMask(pub u32);
impl Default for LayerMask {
    fn default() -> Self {
        Self(0xFFFFFFFF)
    }
}
impl LayerMask {
    pub fn include(&self, other: &Self) -> bool {
        return self.0 & other.0 > 0;
    }
}

pub struct PluginLayerMask;
impl Plugin for PluginLayerMask {
    fn init(
        &mut self,
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        SysLayerMaskCommand::setup(world, stages.query_stage::<SysLayerMaskCommand>(ERunStageChap::Initial));

        world.insert_resource(SingleLayerMaskCommandList::default());

        Ok(())
    }
}
