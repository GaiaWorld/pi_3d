
use pi_assets::{mgr::AssetMgr, asset::GarbageEmpty, homogeneous::HomogeneousMgr};
use pi_engine_shell::{prelude::*, engine_shell::asset_capacity};


use crate::{pass::*, transforms::prelude::*, cameras::prelude::StageCamera};

use self::{
    // render_item_info::{RendererItemsModifyByMaterialChange, RendererItemsReset, RendererItemsModifyByModelChange},
    // renderer_binds_sys::{SysSceneBindUpdate,},
    renderer::*,
    sys_renderer_pre::*,
    sys_renderer::*,
    pass::*,
    render_primitive::*,
    render_blend::{ActionListBlend, sys_act_model_blend},
    render_depth_and_stencil::*,
    command::*,
    command_sys::*,
    render_sort::*, base::{StageRenderer, Pipeline3D}
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
        
        if app.world.get_resource::<AssetDataCenterShader3D>().is_none() {
            // let cfg = asset_capacity::<AssetCfgShader3D>(app);
            let cfg = app.world.get_resource_mut::<AssetMgrConfigs>().unwrap().query::<Shader3D>();
            app.insert_resource(AssetDataCenterShader3D::new(cfg.flag, cfg.min, cfg.timeout));
        }
        if app.world.get_resource::<AssetDataCenterPipeline3D>().is_none() {
            // let cfg = asset_capacity::<AssetCfgRenderPipeline>(app);
            let cfg = app.world.get_resource_mut::<AssetMgrConfigs>().unwrap().query::<Pipeline3D>();
            app.insert_resource(AssetDataCenterPipeline3D::new(cfg.flag, cfg.min, cfg.timeout));
        }
        if app.world.get_resource::<AssetLoaderShader3D>().is_none() {
            app.insert_resource(AssetLoaderShader3D::default());
        }
        if app.world.get_resource::<AssetLoaderPipeline3D>().is_none() {
            app.insert_resource(AssetLoaderPipeline3D::default());
        }

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
        app.insert_resource(RendererDrawCallRecord::default());

        app.configure_set(Update, StageRenderer::RenderStateCommand.before(StageTransform::TransformCalcMatrix).after(ERunStageChap::_InitialApply));
        app.configure_set(Update, StageRenderer::RendererCommand.after(ERunStageChap::_InitialApply));
        app.configure_set(Update, StageRenderer::PassBindGroup.after(StageRenderer::RendererCommand).after(ERunStageChap::Uniform));
        app.configure_set(Update, StageRenderer::PassBindGroups.after(StageRenderer::PassBindGroup));
        app.configure_set(Update, StageRenderer::PassShader.after(StageRenderer::PassBindGroups));
        app.configure_set(Update, StageRenderer::PassPipeline.after(StageRenderer::PassShader));
        app.configure_set(Update, StageRenderer::PassDraw.after(StageRenderer::PassPipeline));
        app.configure_set(Update, StageRenderer::DrawList.after(StageRenderer::PassDraw).after(StageCamera::CameraCulling).before(ERunStageChap::Dispose));

        app.add_systems(Update, 
            sys_create_renderer.in_set(ERunStageChap::Initial)
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
                sys_set0_modify_by_scene::<Pass01, PassID01>,
                sys_set0_modify_by_scene::<Pass02, PassID02>,
                sys_set0_modify_by_scene::<Pass03, PassID03>,
                sys_set0_modify_by_scene::<Pass04, PassID04>,
                sys_set0_modify_by_scene::<Pass05, PassID05>,
                sys_set0_modify_by_scene::<Pass06, PassID06>,
                sys_set0_modify_by_scene::<Pass07, PassID07>,
                sys_set0_modify_by_scene::<Pass08, PassID08>,
            ).in_set(StageRenderer::PassBindGroup)
        );
        app.add_systems(
			Update,
            (
                sys_set0_modify_by_renderer::<Pass01, PassID01>,
                sys_set0_modify_by_renderer::<Pass02, PassID02>,
                sys_set0_modify_by_renderer::<Pass03, PassID03>,
                sys_set0_modify_by_renderer::<Pass04, PassID04>,
                sys_set0_modify_by_renderer::<Pass05, PassID05>,
                sys_set0_modify_by_renderer::<Pass06, PassID06>,
                sys_set0_modify_by_renderer::<Pass07, PassID07>,
                sys_set0_modify_by_renderer::<Pass08, PassID08>,
            ).after(sys_set0_modify_by_scene::<Pass08, PassID08>).in_set(StageRenderer::PassBindGroup)
        );
        app.add_systems(
			Update,
            (
                sys_set0_modify_by_pass::<Pass01, PassID01>,
                sys_set0_modify_by_pass::<Pass02, PassID02>,
                sys_set0_modify_by_pass::<Pass03, PassID03>,
                sys_set0_modify_by_pass::<Pass04, PassID04>,
                sys_set0_modify_by_pass::<Pass05, PassID05>,
                sys_set0_modify_by_pass::<Pass06, PassID06>,
                sys_set0_modify_by_pass::<Pass07, PassID07>,
                sys_set0_modify_by_pass::<Pass08, PassID08>,
            ).after(sys_set0_modify_by_renderer::<Pass08, PassID08>).in_set(StageRenderer::PassBindGroup)
        );
        app.add_systems(
			Update,
            (
                sys_set1_modify_by_renderer::<Pass01, PassID01>,
                sys_set1_modify_by_renderer::<Pass02, PassID02>,
                sys_set1_modify_by_renderer::<Pass03, PassID03>,
                sys_set1_modify_by_renderer::<Pass04, PassID04>,
                sys_set1_modify_by_renderer::<Pass05, PassID05>,
                sys_set1_modify_by_renderer::<Pass06, PassID06>,
                sys_set1_modify_by_renderer::<Pass07, PassID07>,
                sys_set1_modify_by_renderer::<Pass08, PassID08>,
            ).in_set(StageRenderer::PassBindGroup)
        );
        app.add_systems(
			Update,
            (
                sys_set1_modify_by_model::<Pass01, PassID01>,
                sys_set1_modify_by_model::<Pass02, PassID02>,
                sys_set1_modify_by_model::<Pass03, PassID03>,
                sys_set1_modify_by_model::<Pass04, PassID04>,
                sys_set1_modify_by_model::<Pass05, PassID05>,
                sys_set1_modify_by_model::<Pass06, PassID06>,
                sys_set1_modify_by_model::<Pass07, PassID07>,
                sys_set1_modify_by_model::<Pass08, PassID08>,
            ).after(sys_set1_modify_by_renderer::<Pass08, PassID08>).in_set(StageRenderer::PassBindGroup)
        );
        app.add_systems(
			Update,
            (
                sys_set1_modify_by_pass::<Pass01, PassID01>,
                sys_set1_modify_by_pass::<Pass02, PassID02>,
                sys_set1_modify_by_pass::<Pass03, PassID03>,
                sys_set1_modify_by_pass::<Pass04, PassID04>,
                sys_set1_modify_by_pass::<Pass05, PassID05>,
                sys_set1_modify_by_pass::<Pass06, PassID06>,
                sys_set1_modify_by_pass::<Pass07, PassID07>,
                sys_set1_modify_by_pass::<Pass08, PassID08>,
            ).after(sys_set1_modify_by_model::<Pass08, PassID08>).in_set(StageRenderer::PassBindGroup)
        );
        app.add_systems(
			Update,
            (
                sys_set2_modify_by_renderer::<Pass01, PassID01>,
                sys_set2_modify_by_renderer::<Pass02, PassID02>,
                sys_set2_modify_by_renderer::<Pass03, PassID03>,
                sys_set2_modify_by_renderer::<Pass04, PassID04>,
                sys_set2_modify_by_renderer::<Pass05, PassID05>,
                sys_set2_modify_by_renderer::<Pass06, PassID06>,
                sys_set2_modify_by_renderer::<Pass07, PassID07>,
                sys_set2_modify_by_renderer::<Pass08, PassID08>,
            ).in_set(StageRenderer::PassBindGroup)
        );
        app.add_systems(
			Update,
            (
                sys_set2_modify_by_pass::<Pass01, PassID01>,
                sys_set2_modify_by_pass::<Pass02, PassID02>,
                sys_set2_modify_by_pass::<Pass03, PassID03>,
                sys_set2_modify_by_pass::<Pass04, PassID04>,
                sys_set2_modify_by_pass::<Pass05, PassID05>,
                sys_set2_modify_by_pass::<Pass06, PassID06>,
                sys_set2_modify_by_pass::<Pass07, PassID07>,
                sys_set2_modify_by_pass::<Pass08, PassID08>,
            ).after(sys_set2_modify_by_renderer::<Pass08, PassID08>).in_set(StageRenderer::PassBindGroup)
        );
        // app.add_systems(
		// 	Update,
        //     sys_bind_group_loaded.in_set(StageRenderer::PassBindGroups),
        // );
        app.add_systems(
			Update,
            (
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
            ).in_set(StageRenderer::PassShader)
        );
        app.add_systems(
			Update,
            (
                sys_pass_shader_request_by_pass::<Pass01, PassID01>,
                sys_pass_shader_request_by_pass::<Pass02, PassID02>,
                sys_pass_shader_request_by_pass::<Pass03, PassID03>,
                sys_pass_shader_request_by_pass::<Pass04, PassID04>,
                sys_pass_shader_request_by_pass::<Pass05, PassID05>,
                sys_pass_shader_request_by_pass::<Pass06, PassID06>,
                sys_pass_shader_request_by_pass::<Pass07, PassID07>,
                sys_pass_shader_request_by_pass::<Pass08, PassID08>,
            ).after(sys_pass_shader_request_by_model::<Pass08, PassID08>).in_set(StageRenderer::PassShader)
        );
        app.add_systems(
			Update,
            sys_pass_shader_loaded.in_set(StageRenderer::PassPipeline),
        );
        app.add_systems(
			Update,
            (
                sys_pass_pipeline_request_by_model::<Pass01, PassID01>.after(sys_pass_shader_loaded),
                sys_pass_pipeline_request_by_model::<Pass02, PassID02>.after(sys_pass_shader_loaded),
                sys_pass_pipeline_request_by_model::<Pass03, PassID03>.after(sys_pass_shader_loaded),
                sys_pass_pipeline_request_by_model::<Pass04, PassID04>.after(sys_pass_shader_loaded),
                sys_pass_pipeline_request_by_model::<Pass05, PassID05>.after(sys_pass_shader_loaded),
                sys_pass_pipeline_request_by_model::<Pass06, PassID06>.after(sys_pass_shader_loaded),
                sys_pass_pipeline_request_by_model::<Pass07, PassID07>.after(sys_pass_shader_loaded),
                sys_pass_pipeline_request_by_model::<Pass08, PassID08>.after(sys_pass_shader_loaded),
            ).in_set(StageRenderer::PassPipeline)
        );
        app.add_systems(
			Update,
            (
                sys_pass_pipeline_request_by_pass::<Pass01, PassID01>,
                sys_pass_pipeline_request_by_pass::<Pass02, PassID02>,
                sys_pass_pipeline_request_by_pass::<Pass03, PassID03>,
                sys_pass_pipeline_request_by_pass::<Pass04, PassID04>,
                sys_pass_pipeline_request_by_pass::<Pass05, PassID05>,
                sys_pass_pipeline_request_by_pass::<Pass06, PassID06>,
                sys_pass_pipeline_request_by_pass::<Pass07, PassID07>,
                sys_pass_pipeline_request_by_pass::<Pass08, PassID08>,
            ).after(sys_pass_pipeline_request_by_model::<Pass08, PassID08>).in_set(StageRenderer::PassPipeline)
        );
        app.add_systems(
			Update,
            sys_pass_pipeline_loaded.in_set(StageRenderer::PassDraw),
        );
        app.add_systems(
			Update,
            (
                sys_pass_draw_modify_by_model::<Pass01, PassID01>.after(sys_pass_pipeline_loaded),
                sys_pass_draw_modify_by_model::<Pass02, PassID02>.after(sys_pass_pipeline_loaded),
                sys_pass_draw_modify_by_model::<Pass03, PassID03>.after(sys_pass_pipeline_loaded),
                sys_pass_draw_modify_by_model::<Pass04, PassID04>.after(sys_pass_pipeline_loaded),
                sys_pass_draw_modify_by_model::<Pass05, PassID05>.after(sys_pass_pipeline_loaded),
                sys_pass_draw_modify_by_model::<Pass06, PassID06>.after(sys_pass_pipeline_loaded),
                sys_pass_draw_modify_by_model::<Pass07, PassID07>.after(sys_pass_pipeline_loaded),
                sys_pass_draw_modify_by_model::<Pass08, PassID08>.after(sys_pass_pipeline_loaded),
            ).in_set(StageRenderer::PassDraw)
        );
        app.add_systems(
			Update,
            (
                sys_pass_draw_modify_by_pass::<Pass01, PassID01>,
                sys_pass_draw_modify_by_pass::<Pass02, PassID02>,
                sys_pass_draw_modify_by_pass::<Pass03, PassID03>,
                sys_pass_draw_modify_by_pass::<Pass04, PassID04>,
                sys_pass_draw_modify_by_pass::<Pass05, PassID05>,
                sys_pass_draw_modify_by_pass::<Pass06, PassID06>,
                sys_pass_draw_modify_by_pass::<Pass07, PassID07>,
                sys_pass_draw_modify_by_pass::<Pass08, PassID08>,
            ).after(sys_pass_draw_modify_by_model::<Pass01, PassID01>).in_set(StageRenderer::PassDraw)
        );

        app.add_systems(
            Update,
            sys_renderer_draws_modify.in_set(StageRenderer::DrawList)
        );
    }
}
