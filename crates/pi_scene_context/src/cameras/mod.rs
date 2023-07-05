
use pi_engine_shell::prelude::*;

use crate::{
    viewer::prelude::*,
};

use self::{
    command::*,
    command_sys::*,
    camera_sys::*,
    target_camera::TargetCameraParam, camera::CameraParam
};

mod animation;
mod camera;
mod free_camera;
mod arc_rotate_camera;
mod target_camera;
pub mod camera_sys;
mod command;
pub mod command_sys;
mod interface;
pub mod prelude;

#[derive(Resource, Default)]
pub struct AssetCapacityAnimeCamera(pub AssetCapacity);
impl AsRef<AssetCapacity> for AssetCapacityAnimeCamera {
    fn as_ref(&self) -> &AssetCapacity {
        &self.0
    }
}

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
        app.insert_resource(ActionListCameraCreate::default());
        app.insert_resource(ActionListCameraMode::default());
        app.insert_resource(ActionListCameraTarget::default());
        app.insert_resource(ActionListCameraActive::default());
        app.insert_resource(ActionListCameraFixedMode::default());
        app.insert_resource(ActionListCameraFov::default());
        app.insert_resource(ActionListCameraOrthSize::default());
        app.insert_resource(ActionListCameraAspect::default());
        app.insert_resource(ActionListCameraPixelSize::default());
        app.insert_resource(ActionListCameraNearFar::default());
        app.insert_resource(ActionListCameraRenderer::default());

        app.add_systems(
            (
                sys_camera_create,
                sys_camera_renderer_action,
            ).chain().in_set(ERunStageChap::Initial)
        );

        app.add_systems(
            (
                sys_camera_mode,
                sys_camera_fixed_mode,
                sys_camera_nearfar,
                sys_camera_fov,
                sys_camera_orth_size,
                sys_camera_active,
                sys_camera_aspect,
                sys_camera_pixel_size,
                sys_camera_target,
            ).in_set(ERunStageChap::SecondInitial)
        );

        app.add_systems(
            (
                sys_update_camera_param,
                sys_cmds_target_camera_modify,
                sys_change_camera_render_size,
                sys_camera_renderer_modify,
            ).chain().in_set(ERunStageChap::Command)
        );

        // init_plugin_for_viewer::<TargetCameraParam, Fn, CameraParam, Fn>(app, sys_cmds_target_camera_modify, sys_world_matrix_calc)
        app.add_systems(
            (
                sys_calc_view_matrix_by_viewer::<TargetCameraParam>.run_if(should_run),
                sys_calc_proj_matrix::<CameraParam>.run_if(should_run),
                sys_calc_transform_matrix::<TargetCameraParam, CameraParam>.run_if(should_run),
                sys_update_viewer_uniform::<TargetCameraParam, CameraParam>.run_if(should_run),
            ).chain().in_set(ERunStageChap::DrawUniformToGPU)
        );
        app.add_systems(
            (
                sys_update_viewer_model_list_by_viewer::<TargetCameraParam, CameraParam>.run_if(should_run),
                sys_update_viewer_model_list_by_model::<TargetCameraParam, CameraParam>.run_if(should_run),
                sys_tick_viewer_culling::<TargetCameraParam, CameraParam>.run_if(should_run)
            ).chain().in_set(ERunStageChap::DrawBinds)
        );
    }
}