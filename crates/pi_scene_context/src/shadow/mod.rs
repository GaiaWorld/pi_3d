
use pi_scene_shell::{prelude::*, run_stage::EngineCustomPlugins};

use crate::{
    cameras::prelude::StageCamera, layer_mask::StageLayerMask, light::prelude::StageLighting, materials::{command_sys::ActionMaterial, prelude::*}, object::sys_dispose_ready, prelude::StageRenderer, renderers::command_sys::sys_create_subgraph, transforms::prelude::*, viewer::prelude::*
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

#[cfg(feature = "use_bevy")]
        app.configure_sets(Update, 
            (
                StageShadowGenerator::ShadowCreate.after(StageLighting::_LightCreate).after(StageCamera::_CameraCreate),
                StageShadowGenerator::_ShadowCreate.after(StageShadowGenerator::ShadowCreate).before(StageRenderer::RenderCreate),
                StageShadowGenerator::ShadowCommand.in_set(FrameDataPrepare).after(StageShadowGenerator::_ShadowCreate).after(StageLayerMask::Command).before(StageMaterial::MatCommand),
                StageShadowGenerator::ShadowCalcMatrix.in_set(FrameDataPrepare).after(StageShadowGenerator::ShadowCommand).after(StageTransform::TransformCalcMatrix),
                StageShadowGenerator::ShadowViewerUpdate.in_set(FrameDataPrepare).after(StageShadowGenerator::ShadowCalcMatrix),
                StageShadowGenerator::ShadowBindUpdate.in_set(FrameDataPrepare).after(StageShadowGenerator::ShadowViewerUpdate),
            )
        );

#[cfg(not(feature = "use_bevy"))]
{
    app
    .configure_set(StageD3, StageShadowGenerator::ShadowCreate         .in_set(ERunStageChap::Create).after(StageLighting::_LightCreate).after(StageCamera::_CameraCreate))
    .configure_set(StageD3, StageShadowGenerator::_ShadowCreate        .in_set(ERunStageChap::Create).after(StageShadowGenerator::ShadowCreate).before(StageRenderer::RenderCreate))
    .configure_set(StageD3, StageShadowGenerator::ShadowCommand        .in_set(ERunStageChap::Modify).after(StageShadowGenerator::_ShadowCreate).after(StageLayerMask::Command).before(StageMaterial::MatCommand))
    .configure_set(StageD3, StageShadowGenerator::ShadowCalcMatrix     .in_set(ERunStageChap::Modify).in_set(FrameDataPrepare).after(StageShadowGenerator::ShadowCommand).after(StageTransform::TransformCalcMatrix))
    .configure_set(StageD3, StageShadowGenerator::ShadowViewerUpdate   .in_set(ERunStageChap::Modify).in_set(FrameDataPrepare).after(StageShadowGenerator::ShadowCalcMatrix).before(StageViewer::TransformMatrixCalc))
    .configure_set(StageD3, StageShadowGenerator::ShadowBindUpdate     .in_set(ERunStageChap::Collect).in_set(FrameDataPrepare).after(StageShadowGenerator::ShadowViewerUpdate))
    .configure_set(StageD3, StageShadowGenerator::ShadowDispose     .in_set(ERunStageChap::Dispose))
    ;
}

#[cfg(feature = "use_bevy")]
        app.add_systems(Startup, setup);
#[cfg(not(feature = "use_bevy"))]
        app.add_startup_system(Update, setup);

        let enginepugins = app.world.get_resource::<EngineCustomPlugins>().unwrap();
        if enginepugins.shadowmapping {
            #[cfg(feature = "use_bevy")]
            {
                use pi_scene_shell::schedule::IntoSystemConfigs;
                app.add_systems(StageD3, 
                    (
                        apply_deferred.in_set(StageShadowGenerator::_ShadowCreate),
                        (
                            sys_create_shadow_generator,
                        ).in_set(StageShadowGenerator::ShadowCreate),
                        (
                            sys_light_layermask_to_shadow,
                            sys_act_shadow_generator,
                            sys_shadow_param_update,
                            // sys_shadow_direction_modify_by_directlight,
                            sys_shadow_project_modify,
                            sys_shadow_project_modify_by_spot_light,
                        ).chain().in_set(StageShadowGenerator::ShadowCommand),
                        (
                            sys_shadow_enabled_modify,
                            sys_calc_view_matrix_by_light,
                        ).in_set(StageShadowGenerator::ShadowCalcMatrix),
                        (
                            sys_calc_proj_matrix::<DirectionalShadowProjection>,
                            sys_calc_transform_matrix::<DirectionalShadowDirection, DirectionalShadowProjection>,
                            sys_update_shadow_viewer_model_list_by_viewer::<DirectionalShadowDirection, DirectionalShadowProjection>,
                            sys_update_shadow_viewer_model_list_by_model::<DirectionalShadowDirection, DirectionalShadowProjection>,
                        ).chain().in_set(StageShadowGenerator::ShadowViewerUpdate),
                        (
                            sys_calc_proj_matrix::<SpotShadowProjection>,
                            sys_calc_transform_matrix::<DirectionalShadowDirection, SpotShadowProjection>,
                            sys_update_shadow_viewer_model_list_by_viewer::<DirectionalShadowDirection, SpotShadowProjection>,
                            sys_update_shadow_viewer_model_list_by_model::<DirectionalShadowDirection, SpotShadowProjection>,
                        ).chain().in_set(StageShadowGenerator::ShadowViewerUpdate),
                        sys_shadow_bind_modify.in_set(StageShadowGenerator::ShadowBindUpdate),
                        sys_shadow_generator_apply_while_shadow_modify.before(sys_tick_viewer_culling).in_set(StageViewer::Culling),
                        (
                            sys_dispose_about_shadowcaster
                        ).after(sys_dispose_ready).in_set(ERunStageChap::Dispose),
                    )
                );
            }

            #[cfg(not(feature = "use_bevy"))]
            {
                app
                .add_systems(StageD3, sys_create_shadow_generator.after(sys_create_subgraph).in_set(StageShadowGenerator::ShadowCreate))
                .add_systems(StageD3, sys_light_layermask_to_shadow                                                                           .in_set(StageShadowGenerator::ShadowCommand),)
                .add_systems(StageD3, sys_act_shadow_generator                        .after(sys_light_layermask_to_shadow)                   .in_set(StageShadowGenerator::ShadowCommand),)
                .add_systems(StageD3, sys_shadow_param_update                         .after(sys_act_shadow_generator)                        .in_set(StageShadowGenerator::ShadowCommand),)
                .add_systems(StageD3, sys_shadow_project_modify    .after(sys_shadow_param_update)                         .in_set(StageShadowGenerator::ShadowCommand),)
                .add_systems(StageD3, sys_shadow_enabled_modify               .in_set(StageShadowGenerator::ShadowCalcMatrix))
                .add_systems(StageD3, sys_calc_view_matrix_by_light           .in_set(StageShadowGenerator::ShadowCalcMatrix))
                .add_systems(StageD3, sys_calc_proj_matrix::<DirectionalShadowProjection>                                                                                                                                                                     .in_set(StageShadowGenerator::ShadowViewerUpdate))
                .add_systems(StageD3, sys_update_shadow_viewer_model_list_by_viewer::<DirectionalShadowDirection, DirectionalShadowProjection>.in_set(StageShadowGenerator::ShadowViewerUpdate))
                .add_systems(StageD3, sys_update_shadow_viewer_model_list_by_model::<DirectionalShadowDirection, DirectionalShadowProjection> .after(sys_update_shadow_viewer_model_list_by_viewer::<DirectionalShadowDirection, DirectionalShadowProjection>).in_set(StageShadowGenerator::ShadowViewerUpdate))
                .add_systems(StageD3, sys_calc_proj_matrix::<SpotShadowProjection>                                                                                                                                                                            .in_set(StageShadowGenerator::ShadowViewerUpdate))
                .add_systems(StageD3, sys_update_shadow_viewer_model_list_by_viewer::<DirectionalShadowDirection, SpotShadowProjection>       .in_set(StageShadowGenerator::ShadowViewerUpdate))
                .add_systems(StageD3, sys_update_shadow_viewer_model_list_by_model::<DirectionalShadowDirection, SpotShadowProjection>        .after(sys_update_shadow_viewer_model_list_by_viewer::<DirectionalShadowDirection, SpotShadowProjection>)       .in_set(StageShadowGenerator::ShadowViewerUpdate))
                .add_systems(StageD3, sys_shadow_bind_modify                  .in_set(StageShadowGenerator::ShadowBindUpdate))
                .add_systems(StageD3, sys_shadow_generator_apply_while_shadow_modify.before(sys_tick_viewer_culling).in_set(StageViewer::Culling))
                .add_systems(StageD3, sys_dispose_about_shadowcaster.after(sys_dispose_ready).in_set(StageShadowGenerator::ShadowDispose))
                ;
            }
        }
    }
}

fn setup(
    metas: Res<ShareAssetMgr<ShaderEffectMeta>>,
    engineopt: Res<EngineCustomPlugins>,
) {
    ActionMaterial::regist_material_meta(
        &metas,
        KeyShaderMeta::from(ShaderShadowGenerator::KEY),
        ShaderShadowGenerator::res(&engineopt),
    );
}