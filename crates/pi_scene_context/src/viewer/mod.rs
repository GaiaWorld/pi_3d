
use pi_scene_shell::prelude::*;


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
#[derive(Debug)]
pub struct PluginViewerBase;
impl Plugin for PluginViewerBase {
    fn build(&self, app: &mut App) {
        app.configure_set(Update, StageViewer::ForceInclude.before(ERunStageChap::Uniform));
        app.insert_resource(ActionListViewerForceInclude::default());
        app.add_systems(Update, sys_act_viewer_force_include.in_set(StageViewer::ForceInclude));
    }
}