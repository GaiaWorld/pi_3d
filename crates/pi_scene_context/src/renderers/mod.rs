
use pi_assets::{mgr::AssetMgr, asset::GarbageEmpty, homogeneous::HomogeneousMgr};
use pi_scene_shell::{prelude::*, engine_shell::asset_capacity};


use crate::{
    bindgroup::*,
    pass::*,
    transforms::prelude::*,
    cameras::prelude::*,
    object::sys_dispose_can, scene::StageScene,
};

use self::{
    // render_item_info::{RendererItemsModifyByMaterialChange, RendererItemsReset, RendererItemsModifyByModelChange},
    // renderer_binds_sys::{SysSceneBindUpdate,},
    renderer::*,
    sys_renderer_pre::*,
    sys_renderer::*,
    render_primitive::*,
    render_blend::{ActionListBlend, sys_act_model_blend},
    render_depth_and_stencil::*,
    command::*,
    command_sys::*,
    render_sort::*,
    base::StageRenderer
};

mod render_object;
mod opaque;
mod renderer;
mod render_mode;
mod render_blend;
mod render_depth_and_stencil;
mod render_primitive;
mod render_sort;
mod render_target_state;
mod graphic;
mod sys_bindgroup_0;
mod sys_bindgroup_1;
mod sys_bindgroup_2;
mod sys_bindgroup_3;
mod sys_renderer_pre;
mod sys_renderer;
mod pass;
mod command;
pub mod command_sys;
mod base;
pub mod prelude;

pub struct PluginRenderer;
impl Plugin for PluginRenderer {
    fn build(&self, app: &mut App) {
        app.insert_resource(RendererHasher::default());

        let device = app.world.get_resource::<PiRenderDevice>().unwrap().0.clone();
        if app.world.get_resource::<PiSafeAtlasAllocator>().is_none() {
            let cfg = asset_capacity::<AssetCfgRenderResTextureView>(app);
            let texture_assets_mgr = if let Some(texture_assets_mgr) = app.world.get_resource::<ShareAssetMgr<RenderRes<wgpu::TextureView>>>() {
                texture_assets_mgr.0.clone()
            } else {
                let texture_assets_mgr = AssetMgr::<RenderRes<wgpu::TextureView>>::new(GarbageEmpty(),  cfg.flag, cfg.min, cfg.timeout);
                app.insert_resource(ShareAssetMgr(texture_assets_mgr.clone()));
                texture_assets_mgr
            };
            let cfg = asset_capacity::<AssetCfgRenderResUnuseTexture>(app);
            let unusetexture_assets_mgr = HomogeneousMgr::<RenderRes<UnuseTexture>>::new(pi_assets::homogeneous::GarbageEmpty(), cfg.min, cfg.timeout);
            let atlas = SafeAtlasAllocator::new(device, texture_assets_mgr, unusetexture_assets_mgr);
            app.insert_resource(PiSafeAtlasAllocator(atlas));
        }
        
        if app.world.get_resource::<ShareAssetMgr<Shader3D>>().is_none() {
            // let cfg = asset_capacity::<AssetCfgShader3D>(app);
            let cfg = app.world.get_resource_mut::<AssetMgrConfigs>().unwrap().query::<Shader3D>();
            app.insert_resource(ShareAssetMgr(AssetMgr::<Shader3D>::new(GarbageEmpty(), cfg.flag, cfg.min, cfg.timeout)));
        }
        if app.world.get_resource::<ShareAssetMgr<Shader3D>>().is_none() {
            // let cfg = asset_capacity::<AssetCfgRenderPipeline>(app);
            let cfg = app.world.get_resource_mut::<AssetMgrConfigs>().unwrap().query::<Pipeline3D>();
            app.insert_resource(ShareAssetMgr(AssetMgr::<Pipeline3D>::new(GarbageEmpty(), cfg.flag, cfg.min, cfg.timeout)));
        }
        // if app.world.get_resource::<AssetLoaderShader3D>().is_none() {
        //     app.insert_resource(AssetLoaderShader3D::default());
        // }
        // if app.world.get_resource::<AssetLoaderPipeline3D>().is_none() {
        //     app.insert_resource(AssetLoaderPipeline3D::default());
        // }

        app.insert_resource(CustomRenderTargets::default());
        app.insert_resource(ActionListBlend::default());
        app.insert_resource(ActionListCullMode::default());
        app.insert_resource(ActionListPolyginMode::default());
        app.insert_resource(ActionListFrontFace::default());
        app.insert_resource(ActionListTopology::default());
        app.insert_resource(ActionListUnClipDepth::default());
        app.insert_resource(ActionListDepthWrite::default());
        app.insert_resource(ActionListDepthBias::default());
        app.insert_resource(ActionListDepthCompare::default());
        app.insert_resource(ActionListStencilBack::default());
        app.insert_resource(ActionListStencilFront::default());
        app.insert_resource(ActionListStencilRead::default());
        app.insert_resource(ActionListStencilWrite::default());
        app.insert_resource(ActionListRenderQueue::default());
        app.insert_resource(ActionListRendererCreate::default());
        app.insert_resource(ActionListRendererConnect::default());
        app.insert_resource(ActionListRendererModify::default());
        app.insert_resource(ActionListRendererTarget::default());
        app.insert_resource(RendererDrawCallRecord::default());

        
        app.configure_set(Update, StageRenderer::Create.after(StageScene::Create));
        app.configure_set(Update, StageRenderer::_CreateApply.after(StageRenderer::Create));
        app.configure_set(Update, StageRenderer::RenderStateCommand.before(StageTransform::TransformCalcMatrix).after(StageRenderer::_CreateApply));
        app.configure_set(Update, StageRenderer::RendererCommand.after(StageRenderer::_CreateApply));
        app.configure_set(Update, StageRenderer::PassBindGroup.after(StageRenderer::RendererCommand).after(StageCamera::CameraCulling).after(ERunStageChap::Uniform));
        app.configure_set(Update, StageRenderer::PassBindGroups.after(StageRenderer::PassBindGroup));
        app.configure_set(Update, StageRenderer::PassShader.after(StageRenderer::PassBindGroups));
        app.configure_set(Update, StageRenderer::PassPipeline.after(StageRenderer::PassShader));
        app.configure_set(Update, StageRenderer::PassDraw.after(StageRenderer::PassPipeline));
        app.configure_set(Update, StageRenderer::DrawList.after(StageRenderer::PassDraw).before(ERunStageChap::Dispose));
        
        app.add_systems(Update, 
            apply_deferred.in_set(StageRenderer::_CreateApply)
        );

        app.add_systems(Update, 
            sys_create_renderer.in_set(StageRenderer::Create)
        );

        app.add_systems(
			Update,
            (
                sys_act_model_blend,
                sys_act_mesh_cull_mode,
                sys_act_mesh_polygon_mode,
                sys_act_mesh_frontface,
                sys_act_mesh_topolygon,
                sys_act_mesh_unclip_depth,
                
                sys_act_depth_write,
                sys_act_depth_compare,
                sys_act_depth_bias,
                sys_act_stencil_front,
                sys_act_stencil_back,
                sys_act_stencil_read,
                sys_act_stencil_write,

                sys_act_render_queue,
                sys_act_renderer_connect,
                sys_act_renderer_target,
            ).in_set(StageRenderer::RenderStateCommand)
        );

        app.add_systems(Update, 
            sys_renderer_modify.in_set(StageRenderer::RendererCommand)
        );
        app.add_systems(
			Update,
            sys_bind_buffer_apply.in_set(ERunStageChap::Uniform),
        );

        app.add_systems(
			Update,
            (
                sys_sets_modify_by_viewer::<Pass01, PassID01>,
                sys_sets_modify_by_viewer::<Pass02, PassID02>,
                sys_sets_modify_by_viewer::<Pass03, PassID03>,
                sys_sets_modify_by_viewer::<Pass04, PassID04>,
                sys_sets_modify_by_viewer::<Pass05, PassID05>,
                sys_sets_modify_by_viewer::<Pass06, PassID06>,
                sys_sets_modify_by_viewer::<Pass07, PassID07>,
                sys_sets_modify_by_viewer::<Pass08, PassID08>,
                // sys_sets_modify_by_viewer::<Pass09, PassID09>,
                // sys_sets_modify_by_viewer::<Pass10, PassID10>,
                // sys_sets_modify_by_viewer::<Pass11, PassID11>,
                // sys_sets_modify_by_viewer::<Pass12, PassID12>,
            ).in_set(StageRenderer::PassBindGroup)
        );
        app.add_systems(
			Update,
            (
                sys_sets_modify_by_model::<Pass01, PassID01>,
                sys_sets_modify_by_model::<Pass02, PassID02>,
                sys_sets_modify_by_model::<Pass03, PassID03>,
                sys_sets_modify_by_model::<Pass04, PassID04>,
                sys_sets_modify_by_model::<Pass05, PassID05>,
                sys_sets_modify_by_model::<Pass06, PassID06>,
                sys_sets_modify_by_model::<Pass07, PassID07>,
                sys_sets_modify_by_model::<Pass08, PassID08>,
                // sys_sets_modify_by_model::<Pass09, PassID09>,
                // sys_sets_modify_by_model::<Pass10, PassID10>,
                // sys_sets_modify_by_model::<Pass11, PassID11>,
                // sys_sets_modify_by_model::<Pass12, PassID12>,
            ).after(sys_sets_modify_by_viewer::<Pass01, PassID01>).in_set(StageRenderer::PassBindGroup)
        );

        app.add_systems(
			Update,
            (
                sys_passrendererid_pass_reset::<Pass01, PassID01>,
                sys_passrendererid_pass_reset::<Pass02, PassID02>,
                sys_passrendererid_pass_reset::<Pass03, PassID03>,
                sys_passrendererid_pass_reset::<Pass04, PassID04>,
                sys_passrendererid_pass_reset::<Pass05, PassID05>,
                sys_passrendererid_pass_reset::<Pass06, PassID06>,
                sys_passrendererid_pass_reset::<Pass07, PassID07>,
                sys_passrendererid_pass_reset::<Pass08, PassID08>,
                // sys_sets_modify_by_scene::<Pass09, PassID09>,
                // sys_sets_modify_by_scene::<Pass10, PassID10>,
                // sys_sets_modify_by_scene::<Pass11, PassID11>,
                // sys_sets_modify_by_scene::<Pass12, PassID12>,
            ).after(sys_sets_modify_by_model::<Pass01, PassID01>).in_set(StageRenderer::PassBindGroup)
        );
        app.add_systems(
			Update,
            (
                sys_sets_modify_by_scene_extend::<Pass01, PassID01>,
                sys_sets_modify_by_scene_extend::<Pass02, PassID02>,
                sys_sets_modify_by_scene_extend::<Pass03, PassID03>,
                sys_sets_modify_by_scene_extend::<Pass04, PassID04>,
                sys_sets_modify_by_scene_extend::<Pass05, PassID05>,
                sys_sets_modify_by_scene_extend::<Pass06, PassID06>,
                sys_sets_modify_by_scene_extend::<Pass07, PassID07>,
                sys_sets_modify_by_scene_extend::<Pass08, PassID08>,
                // sys_sets_modify_by_scene_extend::<Pass09, PassID09>,
                // sys_sets_modify_by_scene_extend::<Pass10, PassID10>,
                // sys_sets_modify_by_scene_extend::<Pass11, PassID11>,
                // sys_sets_modify_by_scene_extend::<Pass12, PassID12>,
            ).after(sys_passrendererid_pass_reset::<Pass01, PassID01>).in_set(StageRenderer::PassBindGroup)
        );

        // app.add_systems(
		// 	Update,
        //     sys_bind_group_loaded.in_set(StageRenderer::PassBindGroups),
        // );
        app.add_systems(
			Update,
            (
                sys_set0_modify,
                sys_set1_modify,
                sys_set2_modify,
                sys_set3_modify,
                sys_bind_group_loaded,
                sys_pass_bind_groups,
            ).chain().in_set(StageRenderer::PassBindGroups)
        );
        app.add_systems(
			Update,
            (
                sys_pass_shader_request_by_model::<Pass01, PassID01>,
                sys_pass_shader_request_by_model::<Pass02, PassID02>,
                sys_pass_shader_request_by_model::<Pass03, PassID03>,
                sys_pass_shader_request_by_model::<Pass04, PassID04>,
                sys_pass_shader_request_by_model::<Pass05, PassID05>,
                sys_pass_shader_request_by_model::<Pass06, PassID06>,
                sys_pass_shader_request_by_model::<Pass07, PassID07>,
                sys_pass_shader_request_by_model::<Pass08, PassID08>,
                // sys_pass_shader_request_by_model::<Pass09, PassID09>,
                // sys_pass_shader_request_by_model::<Pass10, PassID10>,
                // sys_pass_shader_request_by_model::<Pass11, PassID11>,
                // sys_pass_shader_request_by_model::<Pass12, PassID12>,
                sys_pass_shader_request_by_geometry::<Pass01, PassID01>,
                sys_pass_shader_request_by_geometry::<Pass02, PassID02>,
                sys_pass_shader_request_by_geometry::<Pass03, PassID03>,
                sys_pass_shader_request_by_geometry::<Pass04, PassID04>,
                sys_pass_shader_request_by_geometry::<Pass05, PassID05>,
                sys_pass_shader_request_by_geometry::<Pass06, PassID06>,
                sys_pass_shader_request_by_geometry::<Pass07, PassID07>,
                sys_pass_shader_request_by_geometry::<Pass08, PassID08>,
                // sys_pass_shader_request_by_geometry::<Pass09, PassID09>,
                // sys_pass_shader_request_by_geometry::<Pass10, PassID10>,
                // sys_pass_shader_request_by_geometry::<Pass11, PassID11>,
                // sys_pass_shader_request_by_geometry::<Pass12, PassID12>,
            ).in_set(StageRenderer::PassShader)
        );
        app.add_systems(
			Update,
            (
                sys_pass_shader,
            ).after(sys_pass_shader_request_by_model::<Pass08, PassID08>).in_set(StageRenderer::PassShader)
        );
        app.add_systems(
			Update,
            (
                sys_pass_pipeline_request_by_model::<Pass01, PassID01>,
                sys_pass_pipeline_request_by_model::<Pass02, PassID02>,
                sys_pass_pipeline_request_by_model::<Pass03, PassID03>,
                sys_pass_pipeline_request_by_model::<Pass04, PassID04>,
                sys_pass_pipeline_request_by_model::<Pass05, PassID05>,
                sys_pass_pipeline_request_by_model::<Pass06, PassID06>,
                sys_pass_pipeline_request_by_model::<Pass07, PassID07>,
                sys_pass_pipeline_request_by_model::<Pass08, PassID08>,
                // sys_pass_pipeline_request_by_model::<Pass09, PassID09>,
                // sys_pass_pipeline_request_by_model::<Pass10, PassID10>,
                // sys_pass_pipeline_request_by_model::<Pass11, PassID11>,
                // sys_pass_pipeline_request_by_model::<Pass12, PassID12>,
                sys_pass_pipeline_request_by_renderer::<Pass01, PassID01>,
                sys_pass_pipeline_request_by_renderer::<Pass02, PassID02>,
                sys_pass_pipeline_request_by_renderer::<Pass03, PassID03>,
                sys_pass_pipeline_request_by_renderer::<Pass04, PassID04>,
                sys_pass_pipeline_request_by_renderer::<Pass05, PassID05>,
                sys_pass_pipeline_request_by_renderer::<Pass06, PassID06>,
                sys_pass_pipeline_request_by_renderer::<Pass07, PassID07>,
                sys_pass_pipeline_request_by_renderer::<Pass08, PassID08>,
                // sys_pass_pipeline_request_by_renderer::<Pass09, PassID09>,
                // sys_pass_pipeline_request_by_renderer::<Pass10, PassID10>,
                // sys_pass_pipeline_request_by_renderer::<Pass11, PassID11>,
                // sys_pass_pipeline_request_by_renderer::<Pass12, PassID12>,
            ).in_set(StageRenderer::PassPipeline)
        );
        app.add_systems(
			Update,
            (
                sys_pass_pipeline
            ).after(sys_pass_pipeline_request_by_model::<Pass08, PassID08>).in_set(StageRenderer::PassPipeline)
        );
        app.add_systems(
			Update,
            (
                sys_pass_draw_modify_by_model::<Pass01, PassID01>,
                sys_pass_draw_modify_by_model::<Pass02, PassID02>,
                sys_pass_draw_modify_by_model::<Pass03, PassID03>,
                sys_pass_draw_modify_by_model::<Pass04, PassID04>,
                sys_pass_draw_modify_by_model::<Pass05, PassID05>,
                sys_pass_draw_modify_by_model::<Pass06, PassID06>,
                sys_pass_draw_modify_by_model::<Pass07, PassID07>,
                sys_pass_draw_modify_by_model::<Pass08, PassID08>,
                // sys_pass_draw_modify_by_model::<Pass09, PassID09>,
                // sys_pass_draw_modify_by_model::<Pass10, PassID10>,
                // sys_pass_draw_modify_by_model::<Pass11, PassID11>,
                // sys_pass_draw_modify_by_model::<Pass12, PassID12>,
            ).in_set(StageRenderer::PassDraw)
        );
        app.add_systems(
			Update,
            (
                sys_pass_draw_modify_by_pass
            ).after(sys_pass_draw_modify_by_model::<Pass01, PassID01>).in_set(StageRenderer::PassDraw)
        );

        app.add_systems(
            Update,
            sys_renderer_draws_modify.in_set(StageRenderer::DrawList)
        );

        app.add_systems(
            Update,
            sys_dispose_renderer.after(sys_dispose_can).in_set(ERunStageChap::Dispose)
        );
    }
}
