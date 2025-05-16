
use pi_bevy_render_plugin::{GraphBuild, GraphRun};
use pi_scene_shell::prelude::*;


use crate::{
    bindgroup::*, cameras::prelude::*, object::*, pass::*, prelude::*, shadow::prelude::*, transforms::prelude::*
};

use self::{
    // render_item_info::{RendererItemsModifyByMaterialChange, RendererItemsReset, RendererItemsModifyByModelChange},
    // renderer_binds_sys::{SysSceneBindUpdate,},
    renderer::*,
    sys_renderer_pre::*,
    sys_renderer::*,
    command::*,
    command_sys::*,
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

pub fn sys_custom_render_target(
    device: Res<PiRenderDevice>,
    asset_samp: Res<ShareAssetMgr<SamplerRes>>,
    atlas_allocator: Res<PiSafeAtlasAllocator>,
    mut customrendertargets: ResMut<CustomRenderTargets>,
) {
    customrendertargets.update(&device, &asset_samp, &atlas_allocator);
}

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
        app.insert_resource(ActionListSubGraphCreate::default());
        app.insert_resource(ActionListRendererCreate::default());
        app.insert_resource(ActionListRendererConnect::default());
        app.insert_resource(ActionListRendererModify::default());
        app.insert_resource(ActionListRendererTarget::default());

#[cfg(feature = "use_bevy")]
        app.configure_sets(Update,
            (
                StageRenderer::RenderCreate.after(StageCamera::_CameraCreate).after(StageShadowGenerator::_ShadowCreate),
                StageRenderer::_RenderCreate.after(StageRenderer::RenderCreate),
                StageRenderer::RenderStateCommand.in_set(FrameDataPrepare).before(StageTransform::TransformCalcMatrix).after(StageRenderer::_RenderCreate),
                StageRenderer::RendererCommand.in_set(FrameDataPrepare).after(StageRenderer::_RenderCreate),
                StageRenderer::PassBindGroup.in_set(FrameDataPrepare).after(StageRenderer::RendererCommand).after(StageViewer::Culling).after(ERunStageChap::Collect),
                StageRenderer::PassBindGroups.in_set(FrameDataPrepare).after(StageRenderer::PassBindGroup),
                StageRenderer::PassShader.in_set(FrameDataPrepare).after(StageRenderer::PassBindGroups),
                StageRenderer::PassPipeline.in_set(FrameDataPrepare).after(StageRenderer::PassShader),
                StageRenderer::PassDraw.in_set(FrameDataPrepare).after(StageRenderer::PassPipeline),
                StageRenderer::DrawList.in_set(FrameDataPrepare).after(StageRenderer::PassDraw).before(ERunStageChap::Dispose),
            )
        );

#[cfg(feature = "use_bevy")]
        app.add_systems(
            Update, 
            (
                apply_deferred.in_set(StageRenderer::_RenderCreate),
                sys_create_renderer.in_set(StageRenderer::RenderCreate),
                (
                    // sys_act_model_blend,
                    // sys_act_mesh_primitive_state,
                    
                    // sys_act_depth_state,
                    // sys_act_stencil_state,
                    sys_act_renderer_connect,
                ).in_set(StageRenderer::RenderStateCommand),
                (
                    sys_act_renderer_modify,
                ).chain().in_set(StageRenderer::RendererCommand),
                sys_bind_buffer_apply.in_set(ERunStageChap::Collect),
                (
                    sys_sets_modify_by_viewer,
                    sys_sets_modify_by_model,
                    sys_passrendererid_pass_reset,
                    sys_sets_modify_by_scene_extend,
                ).chain().in_set(StageRenderer::PassBindGroup),
                (
                    // sys_set3_modify,
                    sys_pass_bind_groups,
                ).chain().in_set(StageRenderer::PassBindGroups),
                (
                    sys_pass_shader_request_by_model,
                    sys_pass_shader
                ).chain().in_set(StageRenderer::PassShader),
                (
                    sys_pass_pipeline_request_by_renderer,
                    sys_pass_pipeline
                ).chain().in_set(StageRenderer::PassPipeline),
                (
                    sys_pass_draw_modify_by_model,
                    sys_pass_draw_modify_by_pass
                ).chain().in_set(StageRenderer::PassDraw),
                (
                    sys_renderer_draws_modify,
                    sys_vertice_buffer_apply,
                ).chain().in_set(StageRenderer::DrawList),
                sys_dispose_renderer.after(sys_dispose_can).in_set(ERunStageChap::Dispose)
            )
        );

#[cfg(not(feature = "use_bevy"))]
        app
        .configure_set(StageD3, StageRenderer::RenderCreate      .in_set(ERunStageChap::Create).after(StageCamera::_CameraCreate).after(StageShadowGenerator::_ShadowCreate))
        .configure_set(StageD3, StageRenderer::_RenderCreate     .in_set(ERunStageChap::Create).after(StageRenderer::RenderCreate))
        .configure_set(StageD3, StageRenderer::RenderStateCommand.in_set(ERunStageChap::Modify).in_set(FrameDataPrepare).before(GraphBuild))
        .configure_set(StageD3, StageRenderer::RendererCommand   .in_set(ERunStageChap::Modify).in_set(FrameDataPrepare).before(GraphBuild))
        .configure_set(StageD3, StageRenderer::PassBindGroup     .in_set(ERunStageChap::Collect).in_set(FrameDataPrepare))
        .configure_set(StageD3, StageRenderer::PassBindGroups    .in_set(ERunStageChap::Collect).in_set(FrameDataPrepare).after(StageRenderer::PassBindGroup))
        .configure_set(StageD3, StageRenderer::PassShader        .in_set(ERunStageChap::Collect).in_set(FrameDataPrepare).after(StageRenderer::PassBindGroups))
        .configure_set(StageD3, StageRenderer::PassPipeline      .in_set(ERunStageChap::Collect).in_set(FrameDataPrepare).after(StageRenderer::PassShader))
        .configure_set(StageD3, StageRenderer::PassDraw          .in_set(ERunStageChap::Collect).in_set(FrameDataPrepare).after(StageRenderer::PassPipeline))
        .configure_set(StageD3, StageRenderer::DrawList          .in_set(ERunStageChap::Collect).in_set(FrameDataPrepare).after(StageRenderer::PassDraw).before(GraphRun))
        .configure_set(StageD3Final, StageRenderer::RendererDispose   .in_set(ERunStageChap::StateCheck))
        ;

#[cfg(not(feature = "use_bevy"))]
        app
            .add_system(StageD3, sys_create_subgraph.in_set(ERunStageChap::Create))
            .add_systems(StageD3, sys_custom_render_target               .in_set(ERunStageChap::Create))
            .add_systems(StageD3, sys_create_renderer
                .after(sys_create_subgraph)
                // .run_if(runif_acts::<OpsRendererCreate>)                 
                .in_set(StageRenderer::RenderCreate))
            // .add_systems(StageD3, sys_act_model_blend                 .in_set(StageRenderer::RenderStateCommand))
            // .add_systems(StageD3, sys_act_mesh_primitive_state        .in_set(StageRenderer::RenderStateCommand))
            
            // .add_systems(StageD3, sys_act_depth_state                 .in_set(StageRenderer::RenderStateCommand))

            .add_systems(StageD3, sys_act_renderer_connect
                // .run_if(runif_acts::<OpsRendererConnect>)           
                .in_set(StageRenderer::RenderStateCommand))
            .add_systems(StageD3, sys_act_renderer_modify
                // .run_if(runif_acts2::<OpsRendererTarget, OpsRendererCommand>)    
                .in_set(StageRenderer::RendererCommand))
            .add_systems(StageD3, sys_bind_buffer_apply                  .in_set(StageRenderer::PassBindGroups))
            .add_systems(StageD3, sys_sets_modify_by_viewer           .in_set(StageRenderer::PassBindGroup))
            .add_systems(StageD3, sys_sets_modify_by_model            .after(sys_sets_modify_by_viewer).in_set(StageRenderer::PassBindGroup))
            .add_systems(StageD3, sys_passrendererid_pass_reset
                // .run_if(runif_changes::<PassReset>)       
                .after(sys_sets_modify_by_model).in_set(StageRenderer::PassBindGroup))
            .add_systems(StageD3, sys_sets_modify_by_scene_extend     .after(sys_passrendererid_pass_reset).in_set(StageRenderer::PassBindGroup))

            .add_systems(StageD3, sys_pass_bind_groups
                // .run_if(runif_changes::<PassBindGroupsDirty>)        
                .in_set(StageRenderer::PassBindGroups))
            .add_systems(StageD3, sys_pass_shader_request_by_model    .in_set(StageRenderer::PassShader))
            .add_systems(StageD3, sys_pass_shader
                // .run_if(runif_changes::<PassFlagShader>)                     
                .after(sys_pass_shader_request_by_model).in_set(StageRenderer::PassShader))
            .add_systems(StageD3, sys_pass_pipeline_request_by_renderer   .in_set(StageRenderer::PassPipeline))
            .add_systems(StageD3, sys_pass_pipeline
                // .run_if(runif_changes::<PassPipelineStateDirty>)                       
                .after(sys_pass_pipeline_request_by_renderer).in_set(StageRenderer::PassPipeline))
            .add_systems(StageD3, sys_pass_draw_modify_by_model       .in_set(StageRenderer::PassDraw))
            .add_systems(StageD3, sys_pass_draw_modify_by_pass
                // .run_if(runif_changes::<PassDrawDirty>)   
                .after(sys_pass_draw_modify_by_model).in_set(StageRenderer::PassDraw))
            .add_systems(StageD3, sys_renderer_draws_modify           .in_set(StageRenderer::DrawList))
            .add_systems(StageD3, sys_vertice_buffer_apply       .after(sys_renderer_draws_modify).in_set(StageRenderer::DrawList))
            .add_systems(StageD3Final, sys_dispose_renderer                .before(sys_dispose).in_set(StageRenderer::RendererDispose))
            ;
    }
}
