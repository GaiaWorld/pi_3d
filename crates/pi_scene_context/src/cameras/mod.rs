

use pi_ecs::prelude::Setup;

use self::{command::{SysCameraCommand, SingleCameraCommandList, SysTargetCameraCommand, SingleTargetCameraCommandList, SysFreeCameraCommand, SingleFreeCameraCommandList}, camera_sys::{TargetCameraViewMatrixCalc, SysCameraTransformMatrix, SysCameraProjectionCalc}};

pub mod camera;
pub mod free_camera;
pub mod arc_rotate_camera;
pub mod target_camera;
pub mod camera_sys;
pub mod command;
pub mod interface;

pub struct PluginCamera;
impl crate::Plugin for PluginCamera {
    fn init(
        &mut self,
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        SysCameraCommand::setup(world, stages.command_stage());
        SysTargetCameraCommand::setup(world, stages.command_stage());
        SysFreeCameraCommand::setup(world, stages.command_stage());
        SysCameraProjectionCalc::setup(world, stages.after_world_matrix());
        TargetCameraViewMatrixCalc::setup(world, stages.after_world_matrix());
        SysCameraTransformMatrix::setup(world, stages.after_world_matrix());

        world.insert_resource(SingleCameraCommandList::default());
        world.insert_resource(SingleTargetCameraCommandList::default());
        world.insert_resource(SingleFreeCameraCommandList::default());

        Ok(())
    }
}
