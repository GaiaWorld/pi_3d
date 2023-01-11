use pi_ecs::prelude::{Setup};
use pi_engine_shell::run_stage::ERunStageChap;

use crate::{plugin::Plugin};

use self::{command::{SysTransformNodeCommand, SingleTransformNodeCommandList, SysTreeCommand, SingleTreeCommandList}, transform_node_sys::{SysLocalMatrixCalc, SysWorldMatrixCalc}};

pub mod transform_node;
pub mod transform_node_sys;
pub mod command;
pub mod interface;

pub struct PluginTransformNode;
impl Plugin for PluginTransformNode {
    fn init(
        &mut self,
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        SysTreeCommand::setup(world, stages.query_stage::<SysTreeCommand>(ERunStageChap::Command));
        SysTransformNodeCommand::setup(world, stages.query_stage::<SysTransformNodeCommand>(ERunStageChap::Command));
        SysLocalMatrixCalc::setup(world, stages.query_stage::<SysLocalMatrixCalc>(ERunStageChap::Command));
        SysWorldMatrixCalc::setup(world, stages.query_stage::<SysWorldMatrixCalc>(ERunStageChap::Command));

        world.insert_resource(SingleTreeCommandList{ list: vec![] });
        world.insert_resource(SingleTransformNodeCommandList{ list: vec![] });

        Ok(())
    }
}