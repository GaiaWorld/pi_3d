

use pi_ecs::prelude::Setup;
use pi_engine_shell::run_stage::ERunStageChap;

use crate::{viewer::{PluginViewer, sys::SysViewerUpdated}, transforms::transform_node_sys::SysWorldMatrixCalc};

use self::{command::{SysCameraCommand, SingleCameraCommandList, SysTargetCameraCommand, SingleTargetCameraCommandList, SysFreeCameraCommand, SingleFreeCameraCommandList, SysCameraCreate}, target_camera::TargetCameraParam, camera::CameraParam};

pub mod camera;
pub mod free_camera;
pub mod arc_rotate_camera;
pub mod target_camera;
pub mod camera_sys;
pub mod command;
pub mod interface;

pub type SysViewerUpdatedForCamera = SysViewerUpdated<TargetCameraParam, SysTargetCameraCommand, CameraParam, SysWorldMatrixCalc>;

pub struct PluginCamera;
impl crate::Plugin for PluginCamera {
    fn init(
        &mut self,
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        SysCameraCreate::setup(world, stages.query_stage::<SysCameraCreate>(ERunStageChap::Command));
        SysCameraCommand::setup(world, stages.query_stage::<SysCameraCommand>(ERunStageChap::Command));
        SysTargetCameraCommand::setup(world, stages.query_stage::<SysTargetCameraCommand>(ERunStageChap::Command));
        SysFreeCameraCommand::setup(world, stages.query_stage::<SysFreeCameraCommand>(ERunStageChap::Command));

        world.insert_resource(SingleCameraCommandList::default());
        world.insert_resource(SingleTargetCameraCommandList::default());
        world.insert_resource(SingleFreeCameraCommandList::default());

        PluginViewer::<TargetCameraParam, SysTargetCameraCommand, CameraParam, SysWorldMatrixCalc>::default().init(engine, stages);

        Ok(())
    }
}
