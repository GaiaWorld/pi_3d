
use pi_scene_shell::prelude::*;

use crate::{
    cullings::StageCulling, flags::StageEnable, layer_mask::{prelude::LayerMask, StageLayerMask}, object::sys_dispose_ready, prelude::StageRenderer, scene::StageScene, transforms::prelude::*, viewer::prelude::*
};

use self::{
    command::*,
    command_sys::*,
    camera_sys::*,
    target_camera::TargetCameraParam, camera::*, system::*, prelude::*
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
mod system;
pub mod prelude;

#[derive(Resource, Default)]
pub struct AssetCapacityAnimeCamera(pub AssetCapacity);
impl AsRef<AssetCapacity> for AssetCapacityAnimeCamera {
    fn as_ref(&self) -> &AssetCapacity {
        &self.0
    }
}

pub struct PluginCamera;
impl Plugin for PluginCamera {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActionListCameraCreate::default());
        app.insert_resource(ActionListCameraModify::default());
        app.insert_resource(ActionListCameraTarget::default());
        app.insert_resource(StateCamera::default());

        app.configure_set(Update, StageCamera::CameraCreate.after(StageScene::_Create));
        app.configure_set(Update, StageCamera::_Create.after(StageCamera::CameraCreate).before(StageLayerMask::Command).before(StageTransform::TransformCommand).before(StageEnable::Command));
        app.configure_set(Update, StageCamera::CameraCommand.after(StageCamera::_Create).before(StageRenderer::Create));
        app.configure_set(Update, StageCamera::CameraCalcMatrix.in_set(FrameDataPrepare).after(StageCamera::CameraCommand).after(EStageAnimation::Running).after(StageTransform::TransformCalcMatrix).after(StageLayerMask::Command));
        app.configure_set(Update, StageCamera::CameraCulling.in_set(FrameDataPrepare).after(StageCamera::CameraCalcMatrix).before(StageViewer::ForceInclude).after(StageCulling::CalcBounding).before(ERunStageChap::Uniform));
        app.add_systems(Update, apply_deferred.in_set(StageCamera::_Create));

        app.add_systems(
			Update,
            (
                sys_create_camera,
                // sys_create_camera_renderer,
            ).in_set(StageCamera::CameraCreate)
        );

        app.add_systems(
			Update,
            (
                sys_act_camera_mode,
                sys_act_camera_aspect,
            ).in_set(StageCamera::CameraCommand)
        );

        app.add_systems(
			Update,
            (
                sys_update_camera_param,
                sys_update_target_camera_modify,
                // sys_update_camera_renderer,
            ).after(sys_act_camera_aspect).in_set(StageCamera::CameraCommand)
        );

        // init_plugin_for_viewer::<TargetCameraParam, Fn, CameraParam, Fn>(app, sys_cmds_target_camera_modify, sys_world_matrix_calc)
        app.add_systems(
			Update,
            (
                sys_calc_view_matrix_by_viewer::<TargetCameraParam>,
                sys_calc_proj_matrix::<CameraParam>,
                sys_calc_transform_matrix::<TargetCameraParam, CameraParam>,
            ).chain().in_set(StageCamera::CameraCalcMatrix)
        );
        app.add_systems(
			Update,
            (
                sys_update_viewer_model_list_by_viewer::<TargetCameraParam, CameraParam>,
                sys_update_viewer_model_list_by_model::<TargetCameraParam, CameraParam>,
            ).chain().in_set(StageCamera::CameraCalcMatrix)
        );
        app.add_systems(
			Update,
            (
                sys_tick_viewer_culling::<TargetCameraParam, CameraParam, StateCamera>       // .run_if(should_run)
            ).chain().in_set(StageCamera::CameraCulling)
        );

        app.add_systems(
			Update,
            (
                sys_update_viewer_uniform::<TargetCameraParam, CameraParam>,
            ).in_set(ERunStageChap::Uniform)
        );

        app.add_systems(Update, sys_dispose_about_camera.after(sys_dispose_ready).in_set(ERunStageChap::Dispose));
    }
}