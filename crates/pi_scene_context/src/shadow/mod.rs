use pi_engine_shell::prelude::*;

use crate::{
    viewer::prelude::*,
    transforms::prelude::*,
    object::sys_dispose_ready,
    materials::{command_sys::ActionMaterial, prelude::*},
    light::prelude::StageLighting, prelude::StageRenderer, layer_mask::StageLayerMask,
    // prelude::{StageTransform, ActionSetMaterial, ActionMaterial},
};

mod base;
mod system;
mod command;
mod command_sys;
mod direct_light;
mod shader;
pub mod prelude;

use self::{
    shader::ShaderShadowGenerator,
    system::*,
    base::*,
    command_sys::*,
    direct_light::*,
    command::*
};

pub struct PluginShadowGenerator;
impl Plugin for PluginShadowGenerator {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActionListShadowGenerator::default());
        app.insert_resource(ActionListShadowGeneratorParam::default());
        app.insert_resource(StateShadow::default());

        app.configure_set(Update, StageShadowGenerator::Create.after(StageLighting::LightingCommandApply).after(StageMaterial::MaterialCommand));
        app.configure_set(Update, StageShadowGenerator::CreateApply.after(StageShadowGenerator::Create).before(StageRenderer::Create));
        app.configure_set(Update, StageShadowGenerator::Command.after(StageShadowGenerator::CreateApply).after(StageLayerMask::Command));
        app.configure_set(Update, StageShadowGenerator::CommandApply.after(StageShadowGenerator::Command));
        app.configure_set(Update, StageShadowGenerator::CalcMatrix.after(StageShadowGenerator::CommandApply).after(StageTransform::TransformCalcMatrix));
        app.configure_set(Update, StageShadowGenerator::Culling.after(StageShadowGenerator::CalcMatrix).before(StageViewer::ForceInclude).before(ERunStageChap::Uniform));
        app.add_systems(Update, apply_deferred.in_set(StageShadowGenerator::CreateApply));
        app.add_systems(Update, apply_deferred.in_set(StageShadowGenerator::CommandApply));

        app.add_systems(
			Update,
            (
                sys_create_shadow_generator,
            ).in_set(StageShadowGenerator::Create)
        );
        
        app.add_systems(
			Update,
            (
                sys_light_layermask_to_shadow,
                sys_act_shadow_generator,
                sys_shadow_param_update,
                // sys_shadow_direction_modify_by_directlight,
                sys_shadow_project_modify_by_direction_light,
                sys_shadow_project_modify_by_spot_light,
            ).chain().in_set(StageShadowGenerator::Command)
        );

        app.add_systems(
			Update,
            (
                sys_shadow_bind_modify,
            ).in_set(StageShadowGenerator::CalcMatrix)
        );

        app.add_systems(
			Update,
            (
                sys_shadow_param_update_while_mat_create,
            ).chain().before(sys_shadow_bind_modify).in_set(StageShadowGenerator::CalcMatrix)
        );
        
        app.add_systems(
			Update,
            (
                sys_shadow_enabled_modify,
                sys_calc_view_matrix_by_light,
            ).before(sys_shadow_bind_modify).in_set(StageShadowGenerator::CalcMatrix)
        );

        app.add_systems(
			Update,
            (
                sys_calc_proj_matrix::<DirectionalShadowProjection>,
                sys_calc_transform_matrix::<DirectionalShadowDirection, DirectionalShadowProjection>,
                sys_update_shadow_viewer_model_list_by_viewer::<DirectionalShadowDirection, DirectionalShadowProjection, ShadowLayerMask>,
                sys_update_shadow_viewer_model_list_by_model::<DirectionalShadowDirection, DirectionalShadowProjection, ShadowLayerMask>,
            ).chain().after(sys_calc_view_matrix_by_light).before(sys_shadow_bind_modify).in_set(StageShadowGenerator::CalcMatrix)
        );

        app.add_systems(
			Update,
            (
                sys_calc_proj_matrix::<SpotShadowProjection>,
                sys_calc_transform_matrix::<DirectionalShadowDirection, SpotShadowProjection>,
                sys_update_shadow_viewer_model_list_by_viewer::<DirectionalShadowDirection, SpotShadowProjection, ShadowLayerMask>,
                sys_update_shadow_viewer_model_list_by_model::<DirectionalShadowDirection, SpotShadowProjection, ShadowLayerMask>,
            ).chain().after(sys_calc_view_matrix_by_light).before(sys_shadow_bind_modify).in_set(StageShadowGenerator::CalcMatrix)
        );

        app.add_systems(
			Update,
            (
                sys_shadow_generator_apply_while_shadow_modify,
                sys_tick_viewer_culling::<DirectionalShadowDirection, DirectionalShadowProjection, StateShadow>.run_if(should_run),
                sys_tick_viewer_culling::<DirectionalShadowDirection, SpotShadowProjection, StateShadow>.run_if(should_run)
            ).chain().in_set(StageShadowGenerator::Culling)
        );
        app.add_systems(
			Update,
            (
                sys_update_viewer_uniform::<DirectionalShadowDirection, DirectionalShadowProjection>,
                sys_update_viewer_uniform::<DirectionalShadowDirection, SpotShadowProjection>,
            ).chain().in_set(ERunStageChap::Uniform)
        );
        
        app.add_systems(
			Update,
            (
                sys_dispose_about_shadowcaster
            ).after(sys_dispose_ready).in_set(ERunStageChap::Dispose)
        );

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