use pi_ecs::prelude::{ResMut, Query, Setup};
use pi_ecs_macros::setup;
use pi_ecs_utils::prelude::EntityTreeMut;
use pi_scene_math::Vector3;

use crate::{object::{ObjectID, GameObject}, plugin::Plugin, scene::{interface::InterfaceScene}};

use self::{command::{SysTransformNodeCommand, SingleTransformNodeCommandList, TransformNodeCommand, SysTreeCommand, SingleTreeCommandList}, transform_node_sys::{LocalRotationMatrixCalc, LocalMatrixCalc, WorldMatrixCalc}, dirty::SysDirtyTransformNodeTick};

pub mod transform_node;
pub mod transform_node_sys;
pub mod dirty;
pub mod command;
pub mod interface;

pub struct PluginTransformNode;
impl Plugin for PluginTransformNode {
    fn init(
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        SysDirtyTransformNodeTick::setup(world, stages.dirty_state_stage());
        SysTreeCommand::setup(world, stages.command_stage());
        SysTransformNodeCommand::setup(world, stages.command_stage());
        LocalRotationMatrixCalc::setup(world, stages.local_matrix_stage());
        LocalMatrixCalc::setup(world, stages.local_matrix_stage());
        WorldMatrixCalc::setup(world, stages.world_matrix());

        world.insert_resource(SingleTreeCommandList{ list: vec![] });
        world.insert_resource(SingleTransformNodeCommandList{ list: vec![] });

        Ok(())
    }
}