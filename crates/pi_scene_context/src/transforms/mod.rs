use pi_ecs::prelude::{Setup};
use pi_engine_shell::run_stage::ERunStageChap;

use crate::{plugin::Plugin};

use self::{
    command::{SysTransformNodeCreateCommand, SingleTransformNodeCreateCommandList, SysTransformNodeModifyCommand, SingleTransformNodeModifyCommandList, SysTreeCommand, SingleTreeCommandList},
    transform_node_sys::{SysLocalMatrixCalc, SysWorldMatrixCalc, SysLocalEulerModifyCalc, SysLocalQuaternionModifyCalc, SysWorldMatrixCalc2}, animation::{PluginAnimeLocalPosition, PluginAnimeLocalEuler, PluginAnimeLocalQuaternion, PluginAnimeLocalScaling}
};

pub mod transform_node;
pub mod transform_node_sys;
pub mod command;
pub mod interface;
pub mod animation;
pub mod tree_left_right;
pub mod object;

pub struct PluginTransformNode;
impl Plugin for PluginTransformNode {
    fn init(
        &mut self,
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {

        PluginAnimeLocalPosition::new(false, 2 * 1024 * 1024, 1000).init(engine, stages);
        PluginAnimeLocalEuler::new(false, 2 * 1024 * 1024, 1000).init(engine, stages);
        PluginAnimeLocalQuaternion::new(false, 2 * 1024 * 1024, 1000).init(engine, stages);
        PluginAnimeLocalScaling::new(false, 2 * 1024 * 1024, 1000).init(engine, stages);

        let world = engine.world_mut();

        SysTransformNodeCreateCommand::setup(world, stages.query_stage::<SysTransformNodeCreateCommand>(ERunStageChap::Initial));
        SysTreeCommand::setup(world, stages.query_stage::<SysTreeCommand>(ERunStageChap::Initial));
        SysTransformNodeModifyCommand::setup(world, stages.query_stage::<SysTransformNodeModifyCommand>(ERunStageChap::Initial));

        SysLocalEulerModifyCalc::setup(world, stages.query_stage::<SysLocalEulerModifyCalc>(ERunStageChap::Command));
        SysLocalQuaternionModifyCalc::setup(world, stages.query_stage::<SysLocalQuaternionModifyCalc>(ERunStageChap::Command));
        SysLocalMatrixCalc::setup(world, stages.query_stage::<SysLocalMatrixCalc>(ERunStageChap::Command));
        SysWorldMatrixCalc::setup(world, stages.query_stage::<SysWorldMatrixCalc>(ERunStageChap::Command));
        SysWorldMatrixCalc2::setup(world, stages.query_stage::<SysWorldMatrixCalc2>(ERunStageChap::Command));

        world.insert_resource(SingleTreeCommandList{ list: vec![] });
        world.insert_resource(SingleTransformNodeCreateCommandList{ list: vec![] });
        world.insert_resource(SingleTransformNodeModifyCommandList{ list: vec![] });

        Ok(())
    }
}