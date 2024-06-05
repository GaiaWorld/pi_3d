use pi_scene_shell::prelude::*;

use crate::{
    viewer::prelude::*,
    transforms::prelude::*,
    object::sys_dispose_ready,
    materials::{command_sys::ActionMaterial, prelude::*},
    light::prelude::StageLighting, prelude::StageRenderer, layer_mask::StageLayerMask, cameras::prelude::StageCamera,
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

        app.configure_sets(Update, 
            (
                StageShadowGenerator::Create.after(StageLighting::_LightCreate).after(StageCamera::_Create),
                StageShadowGenerator::_Create.after(StageShadowGenerator::Create).before(StageRenderer::Create),
                StageShadowGenerator::Command.in_set(FrameDataPrepare).after(StageShadowGenerator::_Create).after(StageLayerMask::Command).before(StageMaterial::Command),
                StageShadowGenerator::CalcMatrix.in_set(FrameDataPrepare).after(StageShadowGenerator::Command).after(StageTransform::TransformCalcMatrix),
                StageShadowGenerator::ViewerUpdate.in_set(FrameDataPrepare).after(StageShadowGenerator::CalcMatrix),
                StageShadowGenerator::BindUpdate.in_set(FrameDataPrepare).after(StageShadowGenerator::ViewerUpdate),
                StageShadowGenerator::Culling.in_set(FrameDataPrepare).after(StageShadowGenerator::BindUpdate).before(StageViewer::ForceInclude).before(ERunStageChap::Uniform),
            )
        );

        app.add_systems(Startup, setup);

        app.add_systems(Update, 
            (
                apply_deferred.in_set(StageShadowGenerator::_Create),
                (
                    sys_create_shadow_generator,
                ).in_set(StageShadowGenerator::Create),
                (
                    sys_light_layermask_to_shadow,
                    sys_act_shadow_generator,
                    sys_shadow_param_update,
                    // sys_shadow_direction_modify_by_directlight,
                    sys_shadow_project_modify_by_direction_light,
                    sys_shadow_project_modify_by_spot_light,
                ).chain().in_set(StageShadowGenerator::Command),
                (
                    sys_shadow_enabled_modify,
                    sys_shadow_param_update_while_mat_create,
                    sys_calc_view_matrix_by_light,
                ).in_set(StageShadowGenerator::CalcMatrix),
                (
                    sys_calc_proj_matrix::<DirectionalShadowProjection>,
                    sys_calc_transform_matrix::<DirectionalShadowDirection, DirectionalShadowProjection>,
                    sys_update_shadow_viewer_model_list_by_viewer::<DirectionalShadowDirection, DirectionalShadowProjection>,
                    sys_update_shadow_viewer_model_list_by_model::<DirectionalShadowDirection, DirectionalShadowProjection>,
                ).chain().in_set(StageShadowGenerator::ViewerUpdate),
                (
                    sys_calc_proj_matrix::<SpotShadowProjection>,
                    sys_calc_transform_matrix::<DirectionalShadowDirection, SpotShadowProjection>,
                    sys_update_shadow_viewer_model_list_by_viewer::<DirectionalShadowDirection, SpotShadowProjection>,
                    sys_update_shadow_viewer_model_list_by_model::<DirectionalShadowDirection, SpotShadowProjection>,
                ).chain().in_set(StageShadowGenerator::ViewerUpdate),
                sys_shadow_bind_modify.in_set(StageShadowGenerator::BindUpdate),
                (
                    sys_shadow_generator_apply_while_shadow_modify,
                    sys_tick_viewer_culling::<DirectionalShadowDirection, DirectionalShadowProjection, StateShadow>     , //.run_if(should_run),
                    sys_tick_viewer_culling::<DirectionalShadowDirection, SpotShadowProjection, StateShadow>            , // .run_if(should_run)
                ).chain().in_set(StageShadowGenerator::Culling),
                (
                    sys_update_viewer_uniform::<DirectionalShadowDirection, DirectionalShadowProjection>,
                    sys_update_viewer_uniform::<DirectionalShadowDirection, SpotShadowProjection>,
                ).chain().in_set(ERunStageChap::Uniform),
                (
                    sys_dispose_about_shadowcaster
                ).after(sys_dispose_ready).in_set(ERunStageChap::Dispose),
            )
        );
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