use std::{marker::PhantomData, sync::Arc};

use pi_engine_shell::prelude::*;
use pi_hash::XHashMap;
use pi_scene_math::{Vector3, Matrix, coordiante_system::CoordinateSytem3};

use crate::{transforms::{transform_node::{GlobalTransform, LocalPosition}, transform_node_sys::*}, meshes::prelude::*};

use self::{
    sys::*, sys_culling::*,
    base::*,
};

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
#[derive(Debug)]
pub struct PluginViewer<
    T: TViewerViewMatrix + Component,
    // S: TSystemStageInfo + 'static,
    T2: TViewerProjectMatrix + Component,
    // S2: TSystemStageInfo + 'static,
>(PhantomData<(T, T2)>);
impl<
    T: TViewerViewMatrix + Component,
    // S: TSystemStageInfo + 'static,
    T2: TViewerProjectMatrix + Component,
    // S2: TSystemStageInfo + 'static,
> Plugin for PluginViewer<T, T2> {
    // fn init(
    //     &mut self,
    //     engine: &mut pi_engine_shell::engine_shell::EnginShell,
    //     stages: &mut pi_engine_shell::run_stage::RunStage,
    // ) -> Result<(), pi_engine_shell::plugin::ErrorPlugin> {
    //     let world = engine.world_mut();

    //     // if world.get_resource::<SingleRendererCommandList>().is_none() {
    //     //     world.insert_resource(SingleRendererCommandList::default());
    //     //     // 依赖的 ViewerRenderersInfo 初始化的 System 应该在 Initial 阶段
    //     //     log::warn!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> ");
    //     //     SysViewerRendererCommandTick::setup(world, stages.query_stage::<SysViewerRendererCommandTick>(ERunStageChap::Command));
    //     //     SysModelListUpdateByViewer::setup(world, stages.query_stage::<SysModelListUpdateByViewer>(ERunStageChap::Command));
    //     //     SysModelListUpdateByModel::setup(world, stages.query_stage::<SysModelListUpdateByModel>(ERunStageChap::Command));
    //     //     SysModelListAfterCullingTick::setup(world, stages.query_stage::<SysModelListAfterCullingTick>(ERunStageChap::Command));
    //     // } else {
    //     //     log::warn!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> 2");
    //     // }

    //     SysViewerViewMatrixByViewCalc::<T, S>::setup(world, stages.query_stage::<SysViewerViewMatrixByViewCalc::<T, S>>(ERunStageChap::Command));
    //     // SysViewerViewMatrixUpdateByLocalPos::<T>::setup(world, stages.query_stage::<SysViewerViewMatrixUpdateByLocalPos::<T>>(ERunStageChap::Command));
    //     SysViewerViewMatrixUpdateByParentModify::<T>::setup(world, stages.query_stage::<SysViewerViewMatrixUpdateByParentModify::<T>>(ERunStageChap::Command));
    //     SysViewerProjectionCalc::<T2, S2>::setup(world, stages.query_stage::<SysViewerProjectionCalc::<T2, S2>>(ERunStageChap::Command));
    //     SysViewerTransformUpdated::<T, S, T2, S2>::setup(world, stages.query_stage::<SysViewerTransformUpdated::<T, S, T2, S2>>(ERunStageChap::Command));
    //     SysViewerUpdated::<T, S, T2, S2>::setup(world, stages.query_stage::<SysViewerUpdated::<T, S, T2, S2>>(ERunStageChap::Command));

    //     Ok(())
    // }

    fn build(&self, app: &mut App) {
        let world = &mut app.world;

        // if world.get_resource::<SingleRendererCommandList>().is_none() {
        //     world.insert_resource(SingleRendererCommandList::default());
        //     // 依赖的 ViewerRenderersInfo 初始化的 System 应该在 Initial 阶段
        //     log::warn!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> ");
        //     SysViewerRendererCommandTick::setup(world, stages.query_stage::<SysViewerRendererCommandTick>(ERunStageChap::Command));
        //     SysModelListUpdateByViewer::setup(world, stages.query_stage::<SysModelListUpdateByViewer>(ERunStageChap::Command));
        //     SysModelListUpdateByModel::setup(world, stages.query_stage::<SysModelListUpdateByModel>(ERunStageChap::Command));
        //     SysModelListAfterCullingTick::setup(world, stages.query_stage::<SysModelListAfterCullingTick>(ERunStageChap::Command));
        // } else {
        //     log::warn!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> 2");
        // }


        // SysViewerViewMatrixByViewCalc::<T, S>::setup(world, stages.query_stage::<SysViewerViewMatrixByViewCalc::<T, S>>(ERunStageChap::Command));
        // // SysViewerViewMatrixUpdateByLocalPos::<T>::setup(world, stages.query_stage::<SysViewerViewMatrixUpdateByLocalPos::<T>>(ERunStageChap::Command));
        // SysViewerViewMatrixUpdateByParentModify::<T>::setup(world, stages.query_stage::<SysViewerViewMatrixUpdateByParentModify::<T>>(ERunStageChap::Command));
        // SysViewerProjectionCalc::<T2, S2>::setup(world, stages.query_stage::<SysViewerProjectionCalc::<T2, S2>>(ERunStageChap::Command));
        // SysViewerTransformUpdated::<T, S, T2, S2>::setup(world, stages.query_stage::<SysViewerTransformUpdated::<T, S, T2, S2>>(ERunStageChap::Command));
        // SysViewerUpdated::<T, S, T2, S2>::setup(world, stages.query_stage::<SysViewerUpdated::<T, S, T2, S2>>(ERunStageChap::Command));

    }
}
// impl<
//     T: TViewerViewMatrix + Component,
//     S: TSystemStageInfo,
//     T2: TViewerProjectMatrix + Component,
//     S2: TSystemStageInfo,
// > Default for PluginViewer<T, S, T2, S2> {
//     fn default() -> Self {
//         Self(PhantomData)
//     }
// }

// pub fn init_plugin_for_viewer<
//     T: TViewerViewMatrix + Component,
//     M,
//     T2: TViewerProjectMatrix + Component,
//     M2
// >(
//     app: &mut App,
//     sys_before_view_matrix_calc: impl IntoSystemAppConfig<M>, 
//     sys_before_project_matrix_calc: impl IntoSystemAppConfig<M2>, 
// ) {
//     app.add_systems(
//         (
//             sys_calc_view_matrix_by_viewer::<T>.after(sys_before_view_matrix_calc),
//             sys_calc_view_matrix_by_tree::<T>.after(sys_world_matrix_calc),
//             sys_calc_proj_matrix::<T2>.after(sys_before_project_matrix_calc),
//             sys_calc_transform_matrix::<T, T2>,
//             sys_update_viewer_uniform::<T, T2>,
//         ).chain()
//     );
//     app.add_systems(
//         (
//             sys_update_viewer_model_list_by_viewer,
//             sys_update_viewer_model_list_by_model,
//             sys_tick_viewer_culling.after(sys_calc_render_matrix)
//         ).chain()
//     );
// }
