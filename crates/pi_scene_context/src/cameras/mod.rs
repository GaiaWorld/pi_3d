
use pi_engine_shell::prelude::*;

use crate::{viewer::{PluginViewer, command::*, sys_culling::*, init_plugin_for_viewer}, transforms::transform_node_sys::*};

use self::{command::*, camera_sys::{sys_calc_target_camera_local_rot, sys_update_camera_param, sys_change_camera_render_size}, target_camera::TargetCameraParam, camera::CameraParam};

pub mod camera;
pub mod free_camera;
pub mod arc_rotate_camera;
pub mod target_camera;
pub mod camera_sys;
pub mod command;
pub mod interface;

// pub type SysViewerUpdatedForCamera = SysViewerTransformUpdated<TargetCameraParam, SysTargetCameraCommand, CameraParam, SysWorldMatrixCalc>;

pub struct PluginCamera;
impl Plugin for PluginCamera {
    // fn init(
    //     &mut self,
    //     engine: &mut crate::engine::Engine,
    //     stages: &mut crate::run_stage::RunStage,
    // ) -> Result<(), crate::plugin::ErrorPlugin> {
    //     let world = engine.world_mut();

    //     SysCameraCreate::setup(world, stages.query_stage::<SysCameraCreate>(ERunStageChap::Initial));

    //     if world.get_resource::<SingleRendererCommandList>().is_none() {
    //         world.insert_resource(SingleRendererCommandList::default());
    //         // 依赖的 ViewerRenderersInfo 初始化的 System 应该在 Initial 阶段
    //         SysViewerRendererCommandTick::setup(world, stages.query_stage::<SysViewerRendererCommandTick>(ERunStageChap::Command));
    //         SysModelListUpdateByViewer::setup(world, stages.query_stage::<SysModelListUpdateByViewer>(ERunStageChap::Command));
    //         SysModelListUpdateByModel::setup(world, stages.query_stage::<SysModelListUpdateByModel>(ERunStageChap::Command));
    //         SysModelListAfterCullingTick::setup(world, stages.query_stage::<SysModelListAfterCullingTick>(ERunStageChap::Command));
    //     }

    //     SysCameraParamCommand::setup(world, stages.query_stage::<SysCameraParamCommand>(ERunStageChap::Command));
    //     SysCameraParamUpdate::setup(world, stages.query_stage::<SysCameraParamUpdate>(ERunStageChap::Command));
    //     SysTargetCameraCommand::setup(world, stages.query_stage::<SysTargetCameraCommand>(ERunStageChap::Command));
    //     SysCameraRenderer::setup(world, stages.query_stage::<SysCameraRenderer>(ERunStageChap::Command));

    //     SyeCameraRenderSizeUpdate::setup(world, stages.query_stage::<SyeCameraRenderSizeUpdate>(ERunStageChap::Draw));

    //     world.insert_resource(SingleCameraCommandList::default());
    //     world.insert_resource(SingleTargetCameraCommandList::default());

    //     PluginViewer::<TargetCameraParam, SysTargetCameraCommand, CameraParam, SysWorldMatrixCalc>::default().init(engine, stages);

    //     Ok(())
    // }

    fn build(&self, app: &mut App) {
        // if app.world.get_resource::<SingleRendererCommandList>().is_none() {
        //     app.world.insert_resource(SingleRendererCommandList::default());
        //     // 依赖的 ViewerRenderersInfo 初始化的 System 应该在 Initial 阶段
        //     SysViewerRendererCommandTick::setup(world, stages.query_stage::<SysViewerRendererCommandTick>(ERunStageChap::Command));
        //     SysModelListUpdateByViewer::setup(world, stages.query_stage::<SysModelListUpdateByViewer>(ERunStageChap::Command));
        //     SysModelListUpdateByModel::setup(world, stages.query_stage::<SysModelListUpdateByModel>(ERunStageChap::Command));
        //     SysModelListAfterCullingTick::setup(world, stages.query_stage::<SysModelListAfterCullingTick>(ERunStageChap::Command));
        //     app.add_system(system)
        // }

        app.add_system(sys_cmds_camera_renderer_modify.in_set(ERunStageChap::Command));
        app.add_system(sys_update_camera_param.in_set(ERunStageChap::Command));
        app.add_system(sys_cmds_target_camera_modify.in_set(ERunStageChap::Command));
        app.add_system(sys_change_camera_render_size.in_set(ERunStageChap::Command));

        init_plugin_for_viewer::<TargetCameraParam, Fn, CameraParam, Fn>(app, sys_cmds_target_camera_modify, sys_world_matrix_calc)
    }
}
