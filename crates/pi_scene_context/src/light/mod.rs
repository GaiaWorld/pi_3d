
use pi_engine_shell::prelude::*;

use crate::{
    viewer::prelude::*,
    transforms::transform_node_sys::*,
    meshes::command_sys::*,
    pass::*,
};

use self::{
    base::LightDirection,
    directional::{DirectionalShadowProjection, system::*},
    shadow_generator::{system::*},
    command::*,
};

pub mod base;
pub mod directional;
pub mod point;
pub mod vertex;
pub mod command;
pub mod shadow_generator;
pub mod interface;

pub struct PluginLighting;
impl Plugin for PluginLighting {
    // fn init(
    //     &mut self,
    //     engine: &mut pi_engine_shell::engine_shell::EnginShell,
    //     stages: &mut pi_engine_shell::run_stage::RunStage,
    // ) -> Result<(), pi_engine_shell::plugin::ErrorPlugin> {
    //     let world = engine.world_mut();
    //     world.insert_resource(SingleLightCreateCommands::default());
    //     world.insert_resource(SingleModifyCommands::default());

    //     SysLightCreateCommand::setup(world, stages.query_stage::<SysLightCreateCommand>(ERunStageChap::Initial));
    //     SysLightModifyCommand::setup(world, stages.query_stage::<SysLightModifyCommand>(ERunStageChap::Command));
    //     SysLightModifyEffectRender::setup(world, stages.query_stage::<SysLightModifyEffectRender>(ERunStageChap::Command));

    //     SysDirectionalShadowModify::setup(world, stages.query_stage::<SysDirectionalShadowModify>(ERunStageChap::Command));

    //     PluginShadowGenerator.init(engine, stages);

    //     PluginViewer::<LightDirection, SysLightModifyCommand, DirectionalShadowProjection, SysDirectionalShadowModify>::default().init(engine, stages);

    //     Ok(())
    // }

    fn build(&self, app: &mut App) {
        // app.world.insert_resource(SingleLightCreateCommands::default());
        app.insert_resource(ActionListLightCreate::default());
        app.insert_resource(ActionListLightParam::default());

        // app.add_system(sys_cmd_light_create.in_set(ERunStageChap::Initial));
        // app.add_system(sys_cmd_light_modify.in_set(ERunStageChap::Command));
        // app.add_system(sys_light_render_modify.in_set(ERunStageChap::Command));
        app.add_systems(
            (
                sys_act_light_create,
                sys_act_light_param,
            ).chain().in_set(ERunStageChap::Initial)
        );

        app.add_systems(
            (
                sys_light_render_modify,
                sys_shadow_param_update,
                sys_shadow_param_update_while_mat_create,
            ).in_set(ERunStageChap::Command)
        );
        
        app.add_systems(
            (
                sys_shadow_generator_apply_while_shadow_modify::<PassID01>,
                sys_shadow_generator_apply_while_shadow_modify::<PassID02>,
                sys_shadow_generator_apply_while_shadow_modify::<PassID03>,
                sys_shadow_generator_apply_while_shadow_modify::<PassID04>,
                sys_shadow_generator_apply_while_shadow_modify::<PassID05>,
                sys_shadow_generator_apply_while_shadow_modify::<PassID06>,
                sys_shadow_generator_apply_while_shadow_modify::<PassID07>,
                sys_shadow_generator_apply_while_shadow_modify::<PassID08>,
            ).in_set(ERunStageChap::Command)
        );

        app.add_system(
            sys_directional_light_shadow_modify.in_set(ERunStageChap::Command)
        );

        // SysLightCreateCommand::setup(world, stages.query_stage::<SysLightCreateCommand>(ERunStageChap::Initial));
        // SysLightModifyCommand::setup(world, stages.query_stage::<SysLightModifyCommand>(ERunStageChap::Command));
        // SysLightModifyEffectRender::setup(world, stages.query_stage::<SysLightModifyEffectRender>(ERunStageChap::Command));

        // SysDirectionalShadowModify::setup(world, stages.query_stage::<SysDirectionalShadowModify>(ERunStageChap::Command));

        // app.add_plugin(PluginShadowGenerator);
        // PluginShadowGenerator.init(engine, stages);

        // init_plugin_for_viewer::<LightDirection, Fn, DirectionalShadowProjection, Fn>(app, sys_world_matrix_calc, sys_directional_light_shadow_modify)
        // PluginViewer::<LightDirection, SysLightModifyCommand, DirectionalShadowProjection, SysDirectionalShadowModify>::default().init(engine, stages);
        app.add_systems(
            (
                sys_calc_view_matrix_by_viewer::<LightDirection>.after(sys_world_matrix_calc),
                sys_calc_proj_matrix::<DirectionalShadowProjection>,
                sys_calc_transform_matrix::<LightDirection, DirectionalShadowProjection>,
            ).chain().in_set(ERunStageChap::CalcWorldMatrix)
        );
        app.add_systems(
            (
                sys_update_viewer_uniform::<LightDirection, DirectionalShadowProjection>.run_if(should_run),
                sys_update_viewer_model_list_by_viewer::<LightDirection, DirectionalShadowProjection>.run_if(should_run),
                sys_update_viewer_model_list_by_model::<LightDirection, DirectionalShadowProjection>.run_if(should_run),
                sys_tick_viewer_culling::<LightDirection, DirectionalShadowProjection>.run_if(should_run).after(sys_calc_render_matrix)
            ).chain().in_set(ERunStageChap::Uniform)
        );
    }
}