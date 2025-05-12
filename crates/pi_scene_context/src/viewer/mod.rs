
use command::OpsViewerForceInclude;
use pi_scene_shell::prelude::*;
use prelude::sys_tick_viewer_culling;
use sys::{sys_calc_transform_matrix, sys_update_viewer_uniform};


use crate::cullings::StageCulling;

use self::{base::*, command::ActionListViewerForceInclude, command_sys::sys_act_viewer_force_include};

mod base;
mod command;
pub mod command_sys;
mod sys_culling;
mod sys;
mod interface;
pub mod prelude;

///
/// * 渲染视口功能
/// * T - 获取 ViewerViewMatrix 的 Component
/// * S - 获取 ViewerViewMatrix 需要依赖的 System
/// * T2 - 获取 ViewerProjectionMatrix 的 Component
/// * S2 - 获取 ViewerProjectionMatrix 需要依赖的 System
pub struct PluginViewerBase;
impl Plugin for PluginViewerBase {
    fn build(&self, app: &mut App) {
        app.configure_set(StageD3, StageViewer::TransformMatrixCalc  .in_set(ERunStageChap::Modify).in_set(FrameDataPrepare));
        app.configure_set(StageD3, StageViewer::ForceInclude         .in_set(ERunStageChap::Culling).in_set(FrameDataPrepare));
        app.configure_set(StageD3, StageViewer::Culling              .in_set(ERunStageChap::Culling).in_set(FrameDataPrepare).before(StageViewer::ForceInclude).after(StageCulling::CalcBounding));
        app.insert_resource(ActionListViewerForceInclude::default());
        app.add_systems(Update, sys_act_viewer_force_include
            // .run_if(runif_acts::<OpsViewerForceInclude>)
            .in_set(StageViewer::ForceInclude));
        app.add_systems(Update, sys_calc_transform_matrix.in_set(StageViewer::TransformMatrixCalc));
        app.add_systems(Update, sys_tick_viewer_culling.in_set(StageViewer::Culling));
        app.add_systems(Update, sys_update_viewer_uniform.in_set(ERunStageChap::Collect));
    }
}