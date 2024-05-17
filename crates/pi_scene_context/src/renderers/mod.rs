
use pi_scene_shell::{prelude::*, engine_shell::asset_capacity};


use crate::{
    bindgroup::*, cameras::prelude::*, object::sys_dispose_can, pass::*, scene::StageScene, shadow::prelude::StageShadowGenerator, transforms::prelude::*
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
        // if app.world.get_resource::<PiSafeAtlasAllocator>().is_none() {
        //     let cfg = asset_capacity::<AssetCfgRenderResTextureView>(app);
        //     let texture_assets_mgr = if let Some(texture_assets_mgr) = app.world.get_resource::<ShareAssetMgr<RenderRes<wgpu::TextureView>>>() {
        //         texture_assets_mgr.0.clone()
        //     } else {
        //         let texture_assets_mgr = AssetMgr::<RenderRes<wgpu::TextureView>>::new(GarbageEmpty(),  cfg.flag, cfg.min, cfg.timeout);
        //         app.insert_resource(ShareAssetMgr(texture_assets_mgr.clone()));
        //         texture_assets_mgr
        //     };
        //     let cfg = asset_capacity::<AssetCfgRenderResUnuseTexture>(app);
        //     let unusetexture_assets_mgr = HomogeneousMgr::<RenderRes<UnuseTexture>>::new(HomoGarbageEmpty(), cfg.min, cfg.timeout);
        //     let atlas = SafeAtlasAllocator::new(device, texture_assets_mgr, unusetexture_assets_mgr);
        //     app.insert_resource(PiSafeAtlasAllocator(atlas));
        // }
        
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

        app.insert_resource(CustomRenderTargets::default());
        app.insert_resource(ActionListBlend::default());

        app.insert_resource(ActionListPrimitiveState::default());

        app.insert_resource(ActionListDepthState::default());
        app.insert_resource(ActionListStencilState::default());

        app.insert_resource(ActionListRenderQueue::default());
        app.insert_resource(ActionListRendererCreate::default());
        app.insert_resource(ActionListRendererConnect::default());
        app.insert_resource(ActionListRendererModify::default());
        app.insert_resource(ActionListRendererTarget::default());
        app.insert_resource(RendererDrawCallRecord::default());

        
        app.configure_set(Update, StageRenderer::Create.after(StageCamera::_Create).after(StageShadowGenerator::_Create));
        app.configure_set(Update, StageRenderer::_CreateApply.after(StageRenderer::Create));
        app.configure_set(Update, StageRenderer::RenderStateCommand.in_set(FrameDataPrepare).before(StageTransform::TransformCalcMatrix).after(StageRenderer::_CreateApply));
        app.configure_set(Update, StageRenderer::RendererCommand.in_set(FrameDataPrepare).after(StageRenderer::_CreateApply));
        app.configure_set(Update, StageRenderer::PassBindGroup.in_set(FrameDataPrepare).after(StageRenderer::RendererCommand).after(StageCamera::CameraCulling).after(ERunStageChap::Uniform));
        app.configure_set(Update, StageRenderer::PassBindGroups.in_set(FrameDataPrepare).after(StageRenderer::PassBindGroup));
        app.configure_set(Update, StageRenderer::PassShader.in_set(FrameDataPrepare).after(StageRenderer::PassBindGroups));
        app.configure_set(Update, StageRenderer::PassPipeline.in_set(FrameDataPrepare).after(StageRenderer::PassShader));
        app.configure_set(Update, StageRenderer::PassDraw.in_set(FrameDataPrepare).after(StageRenderer::PassPipeline));
        app.configure_set(Update, StageRenderer::DrawList.in_set(FrameDataPrepare).after(StageRenderer::PassDraw).before(ERunStageChap::Dispose));
        
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
                sys_act_mesh_primitive_state,
                
                sys_act_depth_state,
                sys_act_stencil_state,

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
                sys_sets_modify_by_viewer,
                sys_sets_modify_by_model,
                sys_passrendererid_pass_reset,
                sys_sets_modify_by_scene_extend,
            ).chain().in_set(StageRenderer::PassBindGroup)
        );
        app.add_systems(
			Update,
            (
                sys_set0_modify,
                sys_set1_modify,
                sys_set2_modify,
                // sys_set3_modify,
                sys_bind_group_loaded,
                sys_pass_bind_groups,
            ).chain().in_set(StageRenderer::PassBindGroups)
        );
        app.add_systems(
			Update,
            (
                (
                    sys_pass_shader_request_by_model,
                    sys_pass_shader_request_by_geometry
                ),
                sys_pass_shader
            ).chain().in_set(StageRenderer::PassShader)
        );
        app.add_systems(
			Update,
            (
                (
                    sys_pass_pipeline_request_by_model,
                sys_pass_pipeline_request_by_renderer
                ),
                sys_pass_pipeline
            ).chain().in_set(StageRenderer::PassPipeline)
        );
        app.add_systems(
			Update,
            (
                sys_pass_draw_modify_by_model,
                sys_pass_draw_modify_by_pass
            ).chain().in_set(StageRenderer::PassDraw)
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
