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
        app.world.insert_single_res(ActionListShadowGenerator::default());
        app.world.insert_single_res(ActionListShadowGeneratorParam::default());
        app.world.insert_single_res(StateShadow::default());

        app.configure_set(Update, StageShadowGenerator::Create.after(StageLighting::_LightCreate).after(StageCamera::CameraCreate));
        
        #[cfg(not(target_arch="wasm32"))]
        {
            app.configure_set(Update, StageShadowGenerator::_CreateApply.after(StageShadowGenerator::Create).before(StageRenderer::Create));
            app.configure_set(Update, StageShadowGenerator::Command.in_set(FrameDataPrepare).after(StageShadowGenerator::_CreateApply).after(StageLayerMask::Command).before(StageMaterial::Command));
            app.configure_set(Update, StageShadowGenerator::CalcMatrix.in_set(FrameDataPrepare).after(StageShadowGenerator::Command).after(StageTransform::TransformCalcMatrix));
            app.configure_set(Update, StageShadowGenerator::Culling.in_set(FrameDataPrepare).after(StageShadowGenerator::CalcMatrix).before(StageViewer::ForceInclude).before(ERunStageChap::Uniform));
            // app.add_system(Update, apply_deferred.in_set(StageShadowGenerator::_CreateApply));
    
            app.add_system(
            	Update,
                
                    sys_create_shadow_generator
                .in_set(StageShadowGenerator::Create)
            );
            
            // app.add_system(
            // 	Update,
            //     (
            //         sys_light_layermask_to_shadow,
            //         sys_act_shadow_generator,
            //         sys_shadow_param_update,
            //         // sys_shadow_direction_modify_by_directlight,
            //         sys_shadow_project_modify_by_direction_light,
            //         sys_shadow_project_modify_by_spot_light,
            //     ).chain().in_set(StageShadowGenerator::Command)
            // );
            app.add_system(Update,sys_light_layermask_to_shadow.in_set(StageShadowGenerator::Command));
            app.add_system(Update,sys_act_shadow_generator.after(sys_light_layermask_to_shadow).in_set(StageShadowGenerator::Command));
            app.add_system(Update,sys_shadow_param_update.after(sys_act_shadow_generator).in_set(StageShadowGenerator::Command));
            app.add_system(Update,sys_shadow_project_modify_by_direction_light.after(sys_shadow_param_update).in_set(StageShadowGenerator::Command));
            app.add_system(Update,sys_shadow_project_modify_by_spot_light.after(sys_shadow_project_modify_by_direction_light).in_set(StageShadowGenerator::Command));
    
            app.add_system(
            	Update,
                
                    sys_shadow_bind_modify
                .in_set(StageShadowGenerator::CalcMatrix)
            );
    
            app.add_system(
            	Update,
                
                    sys_shadow_param_update_while_mat_create
                .before(sys_shadow_bind_modify).in_set(StageShadowGenerator::CalcMatrix)
            );
            
            // app.add_system(
            // 	Update,
            //     (
            //         sys_shadow_enabled_modify,
            //         sys_calc_view_matrix_by_light,
            //     ).before(sys_shadow_bind_modify).in_set(StageShadowGenerator::CalcMatrix)
            // );
            app.add_system(Update,sys_shadow_enabled_modify.before(sys_shadow_bind_modify).in_set(StageShadowGenerator::CalcMatrix));
            app.add_system(Update,sys_calc_view_matrix_by_light.before(sys_shadow_bind_modify).in_set(StageShadowGenerator::CalcMatrix));
    
            // app.add_system(
            // 	Update,
            //     (
            //         sys_calc_proj_matrix::<DirectionalShadowProjection>,
            //         sys_calc_transform_matrix::<DirectionalShadowDirection, DirectionalShadowProjection>,
            //         sys_update_shadow_viewer_model_list_by_viewer::<DirectionalShadowDirection, DirectionalShadowProjection>,
            //         sys_update_shadow_viewer_model_list_by_model::<DirectionalShadowDirection, DirectionalShadowProjection>,
            //     ).chain().after(sys_calc_view_matrix_by_light).before(sys_shadow_bind_modify).in_set(StageShadowGenerator::CalcMatrix)
            // );
            app.add_system(Update,sys_calc_proj_matrix::<DirectionalShadowProjection>.after(sys_calc_view_matrix_by_light).before(sys_shadow_bind_modify).in_set(StageShadowGenerator::CalcMatrix));
            app.add_system(Update,sys_calc_transform_matrix::<DirectionalShadowDirection, DirectionalShadowProjection>.after(sys_calc_proj_matrix::<DirectionalShadowProjection>).after(sys_calc_view_matrix_by_light).before(sys_shadow_bind_modify).in_set(StageShadowGenerator::CalcMatrix));
            app.add_system(Update,sys_update_shadow_viewer_model_list_by_viewer::<DirectionalShadowDirection, DirectionalShadowProjection>.after(sys_calc_transform_matrix::<DirectionalShadowDirection, DirectionalShadowProjection>).after(sys_calc_view_matrix_by_light).before(sys_shadow_bind_modify).in_set(StageShadowGenerator::CalcMatrix));
            app.add_system(Update,sys_update_shadow_viewer_model_list_by_model::<DirectionalShadowDirection, DirectionalShadowProjection>.after(sys_update_shadow_viewer_model_list_by_viewer::<DirectionalShadowDirection, DirectionalShadowProjection>).after(sys_calc_view_matrix_by_light).before(sys_shadow_bind_modify).in_set(StageShadowGenerator::CalcMatrix));
    
    
            // app.add_system(
            // 	Update,
            //     (
            //         sys_calc_proj_matrix::<SpotShadowProjection>,
            //         sys_calc_transform_matrix::<DirectionalShadowDirection, SpotShadowProjection>,
            //         sys_update_shadow_viewer_model_list_by_viewer::<DirectionalShadowDirection, SpotShadowProjection>,
            //         sys_update_shadow_viewer_model_list_by_model::<DirectionalShadowDirection, SpotShadowProjection>,
            //     ).chain().after(sys_calc_view_matrix_by_light).before(sys_shadow_bind_modify).in_set(StageShadowGenerator::CalcMatrix)
            // );
            app.add_system(Update,sys_calc_proj_matrix::<SpotShadowProjection>.after(sys_calc_view_matrix_by_light).before(sys_shadow_bind_modify).in_set(StageShadowGenerator::CalcMatrix));
            app.add_system(Update,sys_calc_transform_matrix::<DirectionalShadowDirection, SpotShadowProjection>.after(sys_calc_proj_matrix::<SpotShadowProjection>).after(sys_calc_view_matrix_by_light).before(sys_shadow_bind_modify).in_set(StageShadowGenerator::CalcMatrix));
            app.add_system(Update,sys_update_shadow_viewer_model_list_by_viewer::<DirectionalShadowDirection, SpotShadowProjection>.after(sys_calc_transform_matrix::<DirectionalShadowDirection, SpotShadowProjection>).after(sys_calc_view_matrix_by_light).before(sys_shadow_bind_modify).in_set(StageShadowGenerator::CalcMatrix));
            app.add_system(Update,sys_update_shadow_viewer_model_list_by_model::<DirectionalShadowDirection, SpotShadowProjection>.after(sys_update_shadow_viewer_model_list_by_viewer::<DirectionalShadowDirection, SpotShadowProjection>).after(sys_calc_view_matrix_by_light).before(sys_shadow_bind_modify).in_set(StageShadowGenerator::CalcMatrix));
    
            // app.add_system(
            // 	Update,
            //     (
            //         sys_shadow_generator_apply_while_shadow_modify,
            //         sys_tick_viewer_culling::<DirectionalShadowDirection, DirectionalShadowProjection, StateShadow>     , //.run_if(should_run),
            //         sys_tick_viewer_culling::<DirectionalShadowDirection, SpotShadowProjection, StateShadow>            , // .run_if(should_run)
            //     ).chain().in_set(StageShadowGenerator::Culling)
            // );
            app.add_system(Update,sys_shadow_generator_apply_while_shadow_modify.in_set(StageShadowGenerator::Culling));
            app.add_system(Update,sys_tick_viewer_culling::<DirectionalShadowDirection, DirectionalShadowProjection, StateShadow>.after(sys_shadow_generator_apply_while_shadow_modify).in_set(StageShadowGenerator::Culling));
            app.add_system(Update,sys_tick_viewer_culling::<DirectionalShadowDirection, SpotShadowProjection, StateShadow>.after(sys_tick_viewer_culling::<DirectionalShadowDirection, DirectionalShadowProjection, StateShadow>).in_set(StageShadowGenerator::Culling));
    
            // app.add_system(
            // 	Update,
            //     (
            //         sys_update_viewer_uniform::<DirectionalShadowDirection, DirectionalShadowProjection>,
            //         sys_update_viewer_uniform::<DirectionalShadowDirection, SpotShadowProjection>,
            //     ).chain().in_set(ERunStageChap::Uniform)
            // );
            app.add_system(Update,sys_update_viewer_uniform::<DirectionalShadowDirection, DirectionalShadowProjection>.in_set(ERunStageChap::Uniform));
            app.add_system(Update,sys_update_viewer_uniform::<DirectionalShadowDirection, SpotShadowProjection>.after(sys_update_viewer_uniform::<DirectionalShadowDirection, DirectionalShadowProjection>).in_set(ERunStageChap::Uniform));
            
            app.add_system(
            	Update,
                (
                    sys_dispose_about_shadowcaster
                ).after(sys_dispose_ready).in_set(ERunStageChap::Dispose)
            );
    
            app.add_startup_system(Update, setup);
        }
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