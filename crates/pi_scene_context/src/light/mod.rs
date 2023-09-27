
use pi_engine_shell::prelude::*;

use crate::{
    viewer::prelude::*,
    transforms::transform_node_sys::*,
    pass::*,
    object::sys_dispose_ready, prelude::{StageTransform, ActionSetMaterial, ActionMaterial},
};

use self::{
    base::LightDirection,
    directional::{DirectionalShadowProjection, system::*},
    shadow_generator::*,
    command::*,
    command_sys::*, system::*, prelude::*,
};

mod base;
mod directional;
mod point;
mod vertex;
mod command;
mod command_sys;
mod shadow_generator;
mod interface;
mod system;
pub mod prelude;

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
        app.insert_resource(StateLight::default());
        
        app.configure_set(Update, StageLighting::LightingCommand.after(ERunStageChap::_InitialApply));
        app.configure_set(Update, StageLighting::LightingCommandApply.after(StageLighting::LightingCommand));
        app.configure_set(Update, StageLighting::LightingCalcMatrix.after(StageLighting::LightingCommandApply).after(StageTransform::TransformCalcMatrix));
        app.configure_set(Update, StageLighting::LightingCulling.after(StageLighting::LightingCalcMatrix).before(ERunStageChap::Uniform));
        app.add_systems(Update, apply_deferred.in_set(StageLighting::LightingCommandApply));



        // app.add_systems(Update, sys_cmd_light_create.in_set(ERunStageChap::Initial));
        // app.add_systems(Update, sys_cmd_light_modify.in_set(ERunStageChap::Command));
        // app.add_systems(Update, sys_light_render_modify.in_set(ERunStageChap::Command));
        app.add_systems(
			Update,
            (
                sys_create_light,
            ).chain().in_set(ERunStageChap::Initial)
        );
        app.add_systems(
			Update,
            (
                sys_act_light_param,
                sys_light_render_modify,
                sys_directional_light_shadow_modify,
            ).chain().in_set(StageLighting::LightingCommand)
        );
        
        app.add_systems(
			Update,
            (
                sys_shadow_generator_apply_while_shadow_modify::<PassID01>,
                sys_shadow_generator_apply_while_shadow_modify::<PassID02>,
                sys_shadow_generator_apply_while_shadow_modify::<PassID03>,
                sys_shadow_generator_apply_while_shadow_modify::<PassID04>,
                sys_shadow_generator_apply_while_shadow_modify::<PassID05>,
                sys_shadow_generator_apply_while_shadow_modify::<PassID06>,
                sys_shadow_generator_apply_while_shadow_modify::<PassID07>,
                sys_shadow_generator_apply_while_shadow_modify::<PassID08>,
            ).in_set(StageLighting::LightingCommand).after(sys_act_light_param)
        );
        
        app.add_systems(
			Update,
            (
                sys_shadow_param_update,
                sys_shadow_param_update_while_mat_create,
            ).chain().in_set(StageLighting::LightingCalcMatrix)
        );

        // SysLightCreateCommand::setup(world, stages.query_stage::<SysLightCreateCommand>(ERunStageChap::Initial));
        // SysLightModifyCommand::setup(world, stages.query_stage::<SysLightModifyCommand>(ERunStageChap::Command));
        // SysLightModifyEffectRender::setup(world, stages.query_stage::<SysLightModifyEffectRender>(ERunStageChap::Command));

        // SysDirectionalShadowModify::setup(world, stages.query_stage::<SysDirectionalShadowModify>(ERunStageChap::Command));

        // app.add_plugins(PluginShadowGenerator);
        // PluginShadowGenerator.init(engine, stages);

        // init_plugin_for_viewer::<LightDirection, Fn, DirectionalShadowProjection, Fn>(app, sys_world_matrix_calc, sys_directional_light_shadow_modify)
        // PluginViewer::<LightDirection, SysLightModifyCommand, DirectionalShadowProjection, SysDirectionalShadowModify>::default().init(engine, stages);
        app.add_systems(
			Update,
            (
                sys_calc_view_matrix_by_viewer::<LightDirection>.after(sys_world_matrix_calc),
                sys_calc_proj_matrix::<DirectionalShadowProjection>,
                sys_calc_transform_matrix::<LightDirection, DirectionalShadowProjection>,
            ).chain().in_set(StageLighting::LightingCalcMatrix)
        );
        app.add_systems(
			Update,
            (
                sys_update_viewer_model_list_by_viewer::<LightDirection, DirectionalShadowProjection>,
                sys_update_viewer_model_list_by_model::<LightDirection, DirectionalShadowProjection>,
            ).chain().in_set(StageLighting::LightingCalcMatrix)
        );

        app.add_systems(
			Update,
            (
                sys_tick_viewer_culling::<LightDirection, DirectionalShadowProjection, StateLight>.run_if(should_run)
            ).chain().in_set(StageLighting::LightingCulling)
        );
        app.add_systems(
			Update,
            (
                sys_update_viewer_uniform::<LightDirection, DirectionalShadowProjection>,
            ).chain().in_set(ERunStageChap::Uniform)
        );

        app.add_systems(Update, sys_dispose_about_light.after(sys_dispose_ready).in_set(ERunStageChap::Dispose));

        app.add_systems(Startup, setup);
    }
}

fn setup(
    metas: Res<ShareAssetMgr<ShaderEffectMeta>>
) {
    ActionMaterial::regist_material_meta(
        &metas,
        KeyShaderMeta::from(ShaderShadowGenerator::KEY),
        ShaderShadowGenerator::res(),
    );
}