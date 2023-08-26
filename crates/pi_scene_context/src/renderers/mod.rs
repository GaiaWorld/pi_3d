
use pi_assets::{mgr::AssetMgr, asset::GarbageEmpty, homogeneous::HomogeneousMgr};
use pi_engine_shell::{prelude::*, engine_shell::asset_capacity};


use crate::pass::*;

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
    render_sort::*
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
            let cfg = asset_capacity::<AssetCfgShader3D>(app);
            app.insert_resource(AssetDataCenterShader3D::new(cfg.flag, cfg.min, cfg.timeout));
        }
        if app.world.get_resource::<AssetDataCenterPipeline3D>().is_none() {
            let cfg = asset_capacity::<AssetCfgRenderPipeline>(app);
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
        app.add_systems(Update, 
            sys_create_renderer.in_set(ERunStageChap::Initial)
        );

        app.add_systems(
			Update,
            (
                sys_act_model_blend.run_if(should_run),
                sys_act_mesh_cull_mode.run_if(should_run),
                sys_act_mesh_polygon_mode.run_if(should_run),
                sys_act_mesh_frontface.run_if(should_run),
                sys_act_mesh_topolygon.run_if(should_run),
                sys_act_mesh_unclip_depth.run_if(should_run),
                
                sys_act_depth_write.run_if(should_run),
                sys_act_depth_compare.run_if(should_run),
                sys_act_depth_bias.run_if(should_run),
                sys_act_stencil_front.run_if(should_run),
                sys_act_stencil_back.run_if(should_run),
                sys_act_stencil_read.run_if(should_run),
                sys_act_stencil_write.run_if(should_run),
                
                sys_act_render_queue.run_if(should_run),
            ).in_set(ERunStageChap::SecondInitial)
        );
        // app.add_systems(Update, 
        //     sys_render_primitive_modify.in_set(ERunStageChap::Command)
        // );
        app.add_systems(Update, 
            sys_act_renderer_connect.run_if(should_run).in_set(ERunStageChap::Command)
        );
        app.add_systems(Update, 
            sys_renderer_modify.run_if(should_run).in_set(ERunStageChap::Command)
        );


        app.add_systems(
			Update,
            (
                sys_bind_buffer_apply.run_if(should_run).in_set(ERunStageChap::DrawUniformToGPU),
                sys_bind_group_loaded.run_if(should_run).in_set(ERunStageChap::DrawBindGroupsLoaded),
                sys_pass_shader_loaded.run_if(should_run).in_set(ERunStageChap::DrawShaderLoaded),
                sys_pass_pipeline_loaded.run_if(should_run).in_set(ERunStageChap::DrawPipelineLoaded),
                sys_renderer_draws_modify.run_if(should_run).in_set(ERunStageChap::Draw)
            )
        );

        app.add_systems(
			Update,
            (
                sys_set0_modify_by_scene::<Pass01, PassID01>.run_if(should_run),
                sys_set0_modify_by_scene::<Pass02, PassID02>.run_if(should_run),
                sys_set0_modify_by_scene::<Pass03, PassID03>.run_if(should_run),
                sys_set0_modify_by_scene::<Pass04, PassID04>.run_if(should_run),
                sys_set0_modify_by_scene::<Pass05, PassID05>.run_if(should_run),
                sys_set0_modify_by_scene::<Pass06, PassID06>.run_if(should_run),
                sys_set0_modify_by_scene::<Pass07, PassID07>.run_if(should_run),
                sys_set0_modify_by_scene::<Pass08, PassID08>.run_if(should_run),
            ).in_set(ERunStageChap::DrawBindsAndCulling)
        );
        app.add_systems(
			Update,
            (
                sys_set0_modify_by_renderer::<Pass01, PassID01>.run_if(should_run),
                sys_set0_modify_by_renderer::<Pass02, PassID02>.run_if(should_run),
                sys_set0_modify_by_renderer::<Pass03, PassID03>.run_if(should_run),
                sys_set0_modify_by_renderer::<Pass04, PassID04>.run_if(should_run),
                sys_set0_modify_by_renderer::<Pass05, PassID05>.run_if(should_run),
                sys_set0_modify_by_renderer::<Pass06, PassID06>.run_if(should_run),
                sys_set0_modify_by_renderer::<Pass07, PassID07>.run_if(should_run),
                sys_set0_modify_by_renderer::<Pass08, PassID08>.run_if(should_run),
            ).after(sys_set0_modify_by_scene::<Pass08, PassID08>).in_set(ERunStageChap::DrawBindsAndCulling)
        );
        app.add_systems(
			Update,
            (
                sys_set0_modify_by_pass::<Pass01, PassID01>.run_if(should_run),
                sys_set0_modify_by_pass::<Pass02, PassID02>.run_if(should_run),
                sys_set0_modify_by_pass::<Pass03, PassID03>.run_if(should_run),
                sys_set0_modify_by_pass::<Pass04, PassID04>.run_if(should_run),
                sys_set0_modify_by_pass::<Pass05, PassID05>.run_if(should_run),
                sys_set0_modify_by_pass::<Pass06, PassID06>.run_if(should_run),
                sys_set0_modify_by_pass::<Pass07, PassID07>.run_if(should_run),
                sys_set0_modify_by_pass::<Pass08, PassID08>.run_if(should_run),
            ).after(sys_set0_modify_by_renderer::<Pass08, PassID08>).in_set(ERunStageChap::DrawBindsAndCulling)
        );

        app.add_systems(
			Update,
            (
                sys_set1_modify_by_renderer::<Pass01, PassID01>.run_if(should_run),
                sys_set1_modify_by_renderer::<Pass02, PassID02>.run_if(should_run),
                sys_set1_modify_by_renderer::<Pass03, PassID03>.run_if(should_run),
                sys_set1_modify_by_renderer::<Pass04, PassID04>.run_if(should_run),
                sys_set1_modify_by_renderer::<Pass05, PassID05>.run_if(should_run),
                sys_set1_modify_by_renderer::<Pass06, PassID06>.run_if(should_run),
                sys_set1_modify_by_renderer::<Pass07, PassID07>.run_if(should_run),
                sys_set1_modify_by_renderer::<Pass08, PassID08>.run_if(should_run),
            ).in_set(ERunStageChap::DrawBindsAndCulling)
        );
        app.add_systems(
			Update,
            (
                sys_set1_modify_by_model::<Pass01, PassID01>.run_if(should_run),
                sys_set1_modify_by_model::<Pass02, PassID02>.run_if(should_run),
                sys_set1_modify_by_model::<Pass03, PassID03>.run_if(should_run),
                sys_set1_modify_by_model::<Pass04, PassID04>.run_if(should_run),
                sys_set1_modify_by_model::<Pass05, PassID05>.run_if(should_run),
                sys_set1_modify_by_model::<Pass06, PassID06>.run_if(should_run),
                sys_set1_modify_by_model::<Pass07, PassID07>.run_if(should_run),
                sys_set1_modify_by_model::<Pass08, PassID08>.run_if(should_run),
            ).after(sys_set1_modify_by_renderer::<Pass08, PassID08>).in_set(ERunStageChap::DrawBindsAndCulling)
        );
        app.add_systems(
			Update,
            (
                sys_set1_modify_by_pass::<Pass01, PassID01>.run_if(should_run),
                sys_set1_modify_by_pass::<Pass02, PassID02>.run_if(should_run),
                sys_set1_modify_by_pass::<Pass03, PassID03>.run_if(should_run),
                sys_set1_modify_by_pass::<Pass04, PassID04>.run_if(should_run),
                sys_set1_modify_by_pass::<Pass05, PassID05>.run_if(should_run),
                sys_set1_modify_by_pass::<Pass06, PassID06>.run_if(should_run),
                sys_set1_modify_by_pass::<Pass07, PassID07>.run_if(should_run),
                sys_set1_modify_by_pass::<Pass08, PassID08>.run_if(should_run),
            ).after(sys_set1_modify_by_model::<Pass08, PassID08>).in_set(ERunStageChap::DrawBindsAndCulling)
        );

        app.add_systems(
			Update,
            (
                sys_set2_modify_by_renderer::<Pass01, PassID01>.run_if(should_run),
                sys_set2_modify_by_renderer::<Pass02, PassID02>.run_if(should_run),
                sys_set2_modify_by_renderer::<Pass03, PassID03>.run_if(should_run),
                sys_set2_modify_by_renderer::<Pass04, PassID04>.run_if(should_run),
                sys_set2_modify_by_renderer::<Pass05, PassID05>.run_if(should_run),
                sys_set2_modify_by_renderer::<Pass06, PassID06>.run_if(should_run),
                sys_set2_modify_by_renderer::<Pass07, PassID07>.run_if(should_run),
                sys_set2_modify_by_renderer::<Pass08, PassID08>.run_if(should_run),
            ).in_set(ERunStageChap::DrawBindsAndCulling)
        );
        app.add_systems(
			Update,
            (
                sys_set2_modify_by_pass::<Pass01, PassID01>.run_if(should_run),
                sys_set2_modify_by_pass::<Pass02, PassID02>.run_if(should_run),
                sys_set2_modify_by_pass::<Pass03, PassID03>.run_if(should_run),
                sys_set2_modify_by_pass::<Pass04, PassID04>.run_if(should_run),
                sys_set2_modify_by_pass::<Pass05, PassID05>.run_if(should_run),
                sys_set2_modify_by_pass::<Pass06, PassID06>.run_if(should_run),
                sys_set2_modify_by_pass::<Pass07, PassID07>.run_if(should_run),
                sys_set2_modify_by_pass::<Pass08, PassID08>.run_if(should_run),
            ).in_set(ERunStageChap::DrawBindsAndCulling)
        );

        app.add_systems(
			Update,
            (
                sys_pass_bind_groups.run_if(should_run),
            ).in_set(ERunStageChap::DrawBindGroups)
        );
        
        app.add_systems(
			Update,
            (
                sys_pass_shader_request_by_model::<Pass01, PassID01>.run_if(should_run),
                sys_pass_shader_request_by_model::<Pass02, PassID02>.run_if(should_run),
                sys_pass_shader_request_by_model::<Pass03, PassID03>.run_if(should_run),
                sys_pass_shader_request_by_model::<Pass04, PassID04>.run_if(should_run),
                sys_pass_shader_request_by_model::<Pass05, PassID05>.run_if(should_run),
                sys_pass_shader_request_by_model::<Pass06, PassID06>.run_if(should_run),
                sys_pass_shader_request_by_model::<Pass07, PassID07>.run_if(should_run),
                sys_pass_shader_request_by_model::<Pass08, PassID08>.run_if(should_run),
            ).in_set(ERunStageChap::DrawShader)
        );
        
        app.add_systems(
			Update,
            (
                sys_pass_shader_request_by_pass::<Pass01, PassID01>.run_if(should_run),
                sys_pass_shader_request_by_pass::<Pass02, PassID02>.run_if(should_run),
                sys_pass_shader_request_by_pass::<Pass03, PassID03>.run_if(should_run),
                sys_pass_shader_request_by_pass::<Pass04, PassID04>.run_if(should_run),
                sys_pass_shader_request_by_pass::<Pass05, PassID05>.run_if(should_run),
                sys_pass_shader_request_by_pass::<Pass06, PassID06>.run_if(should_run),
                sys_pass_shader_request_by_pass::<Pass07, PassID07>.run_if(should_run),
                sys_pass_shader_request_by_pass::<Pass08, PassID08>.run_if(should_run),
            ).in_set(ERunStageChap::DrawShader)
        );
        
        app.add_systems(
			Update,
            (
                sys_pass_pipeline_request_by_model::<Pass01, PassID01>.run_if(should_run),
                sys_pass_pipeline_request_by_model::<Pass02, PassID02>.run_if(should_run),
                sys_pass_pipeline_request_by_model::<Pass03, PassID03>.run_if(should_run),
                sys_pass_pipeline_request_by_model::<Pass04, PassID04>.run_if(should_run),
                sys_pass_pipeline_request_by_model::<Pass05, PassID05>.run_if(should_run),
                sys_pass_pipeline_request_by_model::<Pass06, PassID06>.run_if(should_run),
                sys_pass_pipeline_request_by_model::<Pass07, PassID07>.run_if(should_run),
                sys_pass_pipeline_request_by_model::<Pass08, PassID08>.run_if(should_run),
            ).in_set(ERunStageChap::DrawPipeline)
        );
        app.add_systems(
			Update,
            (
                sys_pass_pipeline_request_by_pass::<Pass01, PassID01>.run_if(should_run),
                sys_pass_pipeline_request_by_pass::<Pass02, PassID02>.run_if(should_run),
                sys_pass_pipeline_request_by_pass::<Pass03, PassID03>.run_if(should_run),
                sys_pass_pipeline_request_by_pass::<Pass04, PassID04>.run_if(should_run),
                sys_pass_pipeline_request_by_pass::<Pass05, PassID05>.run_if(should_run),
                sys_pass_pipeline_request_by_pass::<Pass06, PassID06>.run_if(should_run),
                sys_pass_pipeline_request_by_pass::<Pass07, PassID07>.run_if(should_run),
                sys_pass_pipeline_request_by_pass::<Pass08, PassID08>.run_if(should_run),
            ).in_set(ERunStageChap::DrawPipeline)
        );
        app.add_systems(
			Update,
            (
                sys_pass_draw_modify_by_model::<Pass01, PassID01>.run_if(should_run),
                sys_pass_draw_modify_by_model::<Pass02, PassID02>.run_if(should_run),
                sys_pass_draw_modify_by_model::<Pass03, PassID03>.run_if(should_run),
                sys_pass_draw_modify_by_model::<Pass04, PassID04>.run_if(should_run),
                sys_pass_draw_modify_by_model::<Pass05, PassID05>.run_if(should_run),
                sys_pass_draw_modify_by_model::<Pass06, PassID06>.run_if(should_run),
                sys_pass_draw_modify_by_model::<Pass07, PassID07>.run_if(should_run),
                sys_pass_draw_modify_by_model::<Pass08, PassID08>.run_if(should_run),
            ).in_set(ERunStageChap::DrawCall)
        );
        app.add_systems(
			Update,
            (
                sys_pass_draw_modify_by_pass::<Pass01, PassID01>.run_if(should_run),
                sys_pass_draw_modify_by_pass::<Pass02, PassID02>.run_if(should_run),
                sys_pass_draw_modify_by_pass::<Pass03, PassID03>.run_if(should_run),
                sys_pass_draw_modify_by_pass::<Pass04, PassID04>.run_if(should_run),
                sys_pass_draw_modify_by_pass::<Pass05, PassID05>.run_if(should_run),
                sys_pass_draw_modify_by_pass::<Pass06, PassID06>.run_if(should_run),
                sys_pass_draw_modify_by_pass::<Pass07, PassID07>.run_if(should_run),
                sys_pass_draw_modify_by_pass::<Pass08, PassID08>.run_if(should_run),
            ).in_set(ERunStageChap::DrawCall).after(sys_pass_draw_modify_by_model::<Pass01, PassID01>)
        );

        // app.add_systems(
        //     (
        //         sys_bind_buffer_apply,
        //         (
        //             sys_set0_modify_by_renderer::<Pass01, PassID01>,
        //             sys_set0_modify_by_renderer::<Pass02, PassID02>,
        //             sys_set0_modify_by_renderer::<Pass03, PassID03>,
        //             sys_set0_modify_by_renderer::<Pass04, PassID04>,
        //             sys_set0_modify_by_renderer::<Pass05, PassID05>,
        //             sys_set0_modify_by_renderer::<Pass06, PassID06>,
        //             sys_set0_modify_by_renderer::<Pass07, PassID07>,
        //             sys_set0_modify_by_renderer::<Pass08, PassID08>,

        //             sys_set1_modify_by_renderer::<Pass01, PassID01>,
        //             sys_set1_modify_by_renderer::<Pass02, PassID02>,
        //             sys_set1_modify_by_renderer::<Pass03, PassID03>,
        //             sys_set1_modify_by_renderer::<Pass04, PassID04>,
        //             sys_set1_modify_by_renderer::<Pass05, PassID05>,
        //             sys_set1_modify_by_renderer::<Pass06, PassID06>,
        //             sys_set1_modify_by_renderer::<Pass07, PassID07>,
        //             sys_set1_modify_by_renderer::<Pass08, PassID08>,

        //             sys_set2_modify_by_renderer::<Pass01, PassID01>,
        //             sys_set2_modify_by_renderer::<Pass02, PassID02>,
        //             sys_set2_modify_by_renderer::<Pass03, PassID03>,
        //             sys_set2_modify_by_renderer::<Pass04, PassID04>,
        //             sys_set2_modify_by_renderer::<Pass05, PassID05>,
        //             sys_set2_modify_by_renderer::<Pass06, PassID06>,
        //             sys_set2_modify_by_renderer::<Pass07, PassID07>,
        //             sys_set2_modify_by_renderer::<Pass08, PassID08>,
        //         ),
        //         (
        //             sys_set1_modify_by_pass::<Pass01, PassID01>,
        //             sys_set1_modify_by_pass::<Pass02, PassID02>,
        //             sys_set1_modify_by_pass::<Pass03, PassID03>,
        //             sys_set1_modify_by_pass::<Pass04, PassID04>,
        //             sys_set1_modify_by_pass::<Pass05, PassID05>,
        //             sys_set1_modify_by_pass::<Pass06, PassID06>,
        //             sys_set1_modify_by_pass::<Pass07, PassID07>,
        //             sys_set1_modify_by_pass::<Pass08, PassID08>,
        //         )
        //         (

        //             sys_set0_modify_by_scene::<Pass01, PassID01>,
        //             sys_set0_modify_by_scene::<Pass02, PassID02>,
        //             sys_set0_modify_by_scene::<Pass03, PassID03>,
        //             sys_set0_modify_by_scene::<Pass04, PassID04>,
        //             sys_set0_modify_by_scene::<Pass05, PassID05>,
        //             sys_set0_modify_by_scene::<Pass06, PassID06>,
        //             sys_set0_modify_by_scene::<Pass07, PassID07>,
        //             sys_set0_modify_by_scene::<Pass08, PassID08>,

        //             sys_set1_modify_by_model::<Pass01, PassID01>,
        //             sys_set1_modify_by_model::<Pass02, PassID02>,
        //             sys_set1_modify_by_model::<Pass03, PassID03>,
        //             sys_set1_modify_by_model::<Pass04, PassID04>,
        //             sys_set1_modify_by_model::<Pass05, PassID05>,
        //             sys_set1_modify_by_model::<Pass06, PassID06>,
        //             sys_set1_modify_by_model::<Pass07, PassID07>,
        //             sys_set1_modify_by_model::<Pass08, PassID08>,

        //             sys_set2_modify_by_model::<Pass01, PassID01>,
        //             sys_set2_modify_by_model::<Pass02, PassID02>,
        //             sys_set2_modify_by_model::<Pass03, PassID03>,
        //             sys_set2_modify_by_model::<Pass04, PassID04>,
        //             sys_set2_modify_by_model::<Pass05, PassID05>,
        //             sys_set2_modify_by_model::<Pass06, PassID06>,
        //             sys_set2_modify_by_model::<Pass07, PassID07>,
        //             sys_set2_modify_by_model::<Pass08, PassID08>,
        //         ),
        //         sys_bind_group_loaded
        //         ,
        //         (
        //             sys_pass_bind_groups::<Pass01, PassID01>,
        //             sys_pass_bind_groups::<Pass02, PassID02>,
        //             sys_pass_bind_groups::<Pass03, PassID03>,
        //             sys_pass_bind_groups::<Pass04, PassID04>,
        //             sys_pass_bind_groups::<Pass05, PassID05>,
        //             sys_pass_bind_groups::<Pass06, PassID06>,
        //             sys_pass_bind_groups::<Pass07, PassID07>,
        //             sys_pass_bind_groups::<Pass08, PassID08>,
        //         ),
        //         (
        //             sys_pass_shader_request_by_model::<Pass01, PassID01>,
        //             sys_pass_shader_request_by_model::<Pass02, PassID02>,
        //             sys_pass_shader_request_by_model::<Pass03, PassID03>,
        //             sys_pass_shader_request_by_model::<Pass04, PassID04>,
        //             sys_pass_shader_request_by_model::<Pass05, PassID05>,
        //             sys_pass_shader_request_by_model::<Pass06, PassID06>,
        //             sys_pass_shader_request_by_model::<Pass07, PassID07>,
        //             sys_pass_shader_request_by_model::<Pass08, PassID08>,
        //         ),
        //         (
        //             sys_pass_shader_request_by_pass::<Pass01, PassID01>,
        //             sys_pass_shader_request_by_pass::<Pass02, PassID02>,
        //             sys_pass_shader_request_by_pass::<Pass03, PassID03>,
        //             sys_pass_shader_request_by_pass::<Pass04, PassID04>,
        //             sys_pass_shader_request_by_pass::<Pass05, PassID05>,
        //             sys_pass_shader_request_by_pass::<Pass06, PassID06>,
        //             sys_pass_shader_request_by_pass::<Pass07, PassID07>,
        //             sys_pass_shader_request_by_pass::<Pass08, PassID08>,
        //         ),
        //         sys_pass_shader_loaded
        //         ,
        //         (
        //             sys_pass_pipeline_request_by_model::<Pass01, PassID01>,
        //             sys_pass_pipeline_request_by_model::<Pass02, PassID02>,
        //             sys_pass_pipeline_request_by_model::<Pass03, PassID03>,
        //             sys_pass_pipeline_request_by_model::<Pass04, PassID04>,
        //             sys_pass_pipeline_request_by_model::<Pass05, PassID05>,
        //             sys_pass_pipeline_request_by_model::<Pass06, PassID06>,
        //             sys_pass_pipeline_request_by_model::<Pass07, PassID07>,
        //             sys_pass_pipeline_request_by_model::<Pass08, PassID08>,
        //         ),
        //         (
        //             sys_pass_pipeline_request_by_pass::<Pass01, PassID01>,
        //             sys_pass_pipeline_request_by_pass::<Pass02, PassID02>,
        //             sys_pass_pipeline_request_by_pass::<Pass03, PassID03>,
        //             sys_pass_pipeline_request_by_pass::<Pass04, PassID04>,
        //             sys_pass_pipeline_request_by_pass::<Pass05, PassID05>,
        //             sys_pass_pipeline_request_by_pass::<Pass06, PassID06>,
        //             sys_pass_pipeline_request_by_pass::<Pass07, PassID07>,
        //             sys_pass_pipeline_request_by_pass::<Pass08, PassID08>,
        //         ),
        //         sys_pass_pipeline_loaded
        //         ,
        //         (
        //             sys_pass_draw_modify_by_model::<Pass01, PassID01>,
        //             sys_pass_draw_modify_by_model::<Pass02, PassID02>,
        //             sys_pass_draw_modify_by_model::<Pass03, PassID03>,
        //             sys_pass_draw_modify_by_model::<Pass04, PassID04>,
        //             sys_pass_draw_modify_by_model::<Pass05, PassID05>,
        //             sys_pass_draw_modify_by_model::<Pass06, PassID06>,
        //             sys_pass_draw_modify_by_model::<Pass07, PassID07>,
        //             sys_pass_draw_modify_by_model::<Pass08, PassID08>,
        //         ),
        //         (
        //             sys_pass_draw_modify_by_pass::<Pass01, PassID01>,
        //             sys_pass_draw_modify_by_pass::<Pass02, PassID02>,
        //             sys_pass_draw_modify_by_pass::<Pass03, PassID03>,
        //             sys_pass_draw_modify_by_pass::<Pass04, PassID04>,
        //             sys_pass_draw_modify_by_pass::<Pass05, PassID05>,
        //             sys_pass_draw_modify_by_pass::<Pass06, PassID06>,
        //             sys_pass_draw_modify_by_pass::<Pass07, PassID07>,
        //             sys_pass_draw_modify_by_pass::<Pass08, PassID08>,
        //         ),
        //         sys_renderer_draws_modify
        //     ).in_set(ERunStageChap::Draw)
        // )
    }
}
