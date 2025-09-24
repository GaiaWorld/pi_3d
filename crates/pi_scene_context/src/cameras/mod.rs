
use pi_scene_shell::prelude::*;

use crate::{
    flags::StageEnable, layer_mask::*, object::sys_dispose_ready, renderers::command_sys::sys_create_subgraph, scene::StageScene, transforms::prelude::*, viewer::prelude::*
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

#[cfg(feature = "use_bevy")]
        app.configure_sets(
            Update,
            (
                StageCamera::CameraCreate.after(StageScene::_SceneCreate),
                StageCamera::_CameraCreate.after(StageCamera::CameraCreate).before(StageLayerMask::Command).before(StageTransform::TransformCommand).before(StageEnable::Command),
                StageCamera::CameraCommand.after(StageCamera::_CameraCreate).before(StageRenderer::RenderCreate),
                StageCamera::CameraCalcMatrix.in_set(FrameDataPrepare).after(StageCamera::CameraCommand).after(EStageAnimation::Running).after(StageTransform::TransformCalcMatrix).after(StageLayerMask::Command).before(StageViewer::TransformMatrixCalc),
            )
        );
#[cfg(feature = "use_bevy")]
        app.add_systems(
            Update, (
                apply_deferred.in_set(StageCamera::_CameraCreate),
                sys_create_camera.in_set(StageCamera::CameraCreate),
                (
                    sys_act_camera_mode,
                ).in_set(StageCamera::CameraCommand),
                (
                    sys_update_camera_param,
                    sys_update_target_camera_modify,
                ).after(sys_act_camera_mode).in_set(StageCamera::CameraCommand),
                (
                    sys_calc_view_matrix_by_viewer::<TargetCameraParam>,
                    sys_calc_proj_matrix::<CameraParam>,
                ).chain().in_set(StageCamera::CameraCalcMatrix),
                (
                    sys_update_viewer_model_list_by_viewer::<TargetCameraParam, CameraParam>,
                    sys_update_viewer_model_list_by_model::<TargetCameraParam, CameraParam>,
                ).chain().in_set(StageCamera::CameraCalcMatrix),
                (
                    sys_tick_viewer_culling::<TargetCameraParam, CameraParam, StateCamera>       // .run_if(should_run)
                ).chain().in_set(StageCamera::CameraCulling),
                (
                    sys_update_viewer_uniform::<TargetCameraParam, CameraParam>,
                ).in_set(ERunStageChap::Collect),
                sys_dispose_about_camera.after(sys_dispose_ready).in_set(ERunStageChap::Dispose)
            )
        );

#[cfg(not(feature = "use_bevy"))]
        app
        .configure_set(StageD3, StageCamera::CameraCreate        .in_set(ERunStageChap::Create).after(StageScene::_SceneCreate))
        .configure_set(StageD3, StageCamera::_CameraCreate             .in_set(ERunStageChap::Create).after(StageCamera::CameraCreate).before(StageLayerMask::Command).before(StageTransform::TransformCommand).before(StageEnable::Command))
        .configure_set(StageD3, StageCamera::CameraCommand       .in_set(ERunStageChap::Modify))
        .configure_set(StageD3, StageCamera::CameraCalcMatrix    .in_set(ERunStageChap::Modify).in_set(FrameDataPrepare).after(StageCamera::CameraCommand).after(EStageAnimation::Running).after(StageTransform::TransformCalcMatrix).after(StageLayerMask::Command).before(StageViewer::TransformMatrixCalc))
        .configure_set(StageD3, StageCamera::CameraDispose       .in_set(ERunStageChap::Dispose).before(StageScene::SceneDispose))
        ;

#[cfg(not(feature = "use_bevy"))]
        app
        .add_systems(StageD3, sys_create_camera
            .after(sys_create_subgraph)
            // .run_if(runif_acts::<OpsCameraCreation>)                            
            .in_set(StageCamera::CameraCreate))
        .add_systems(StageD3, sys_act_camera_mode
            // .run_if(runif_acts2::<OpsCameraModify, OpsCameraTarget>)      
            .in_set(StageCamera::CameraCommand))
        .add_systems(StageD3, sys_update_camera_param                                                 .after(sys_act_camera_mode).in_set(StageCamera::CameraCommand))
        .add_systems(StageD3, sys_update_target_camera_modify                                         .after(sys_act_camera_mode).in_set(StageCamera::CameraCommand))
        .add_systems(StageD3, sys_camera_link_transform                                               .in_set(StageCamera::CameraCalcMatrix))
        .add_systems(StageD3, sys_calc_view_matrix_by_viewer::<TargetCameraParam>                     .in_set(StageCamera::CameraCalcMatrix).after(sys_camera_link_transform))
        .add_systems(StageD3, sys_calc_proj_matrix::<CameraParam>                                     .after(sys_calc_view_matrix_by_viewer::<TargetCameraParam>).in_set(StageCamera::CameraCalcMatrix))
        .add_systems(StageD3, sys_update_viewer_model_list_by_viewer::<TargetCameraParam, CameraParam>.in_set(StageViewer::Culling))
        .add_systems(StageD3, sys_update_viewer_model_list_by_model::<TargetCameraParam, CameraParam> .after(sys_update_viewer_model_list_by_viewer::<TargetCameraParam, CameraParam>).before(sys_tick_viewer_culling).in_set(StageViewer::Culling))
        .add_systems(StageD3, sys_dispose_about_camera                                                .after(sys_dispose_ready).in_set(StageCamera::CameraDispose))
        ;
    }
}