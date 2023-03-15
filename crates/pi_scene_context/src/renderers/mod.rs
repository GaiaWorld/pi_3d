
use std::mem::size_of;

use futures::FutureExt;
use pi_assets::{mgr::AssetMgr, asset::GarbageEmpty, homogeneous::HomogeneousMgr};
use pi_atom::Atom;
use pi_ecs::prelude::Setup;
use pi_engine_shell::{run_stage::ERunStageChap};
use pi_futures::BoxFuture;
use pi_hash::XHashMap;
use pi_render::{components::view::{target_alloc::{ShareTargetView, SafeAtlasAllocator, UnuseTexture}}, graph::{node::Node}, rhi::{asset::RenderRes, device::RenderDevice} };
use render_derive::NodeParam;

use crate::{renderers::sys_renderer::SysRendererDraws, pass::*};

use self::{
    render_blend::PluginRenderBlend,
    render_depth_and_stencil::PluginRenderDepthAndStencil,
    render_primitive::PluginRenderPrimitive,
    render_mode::PluginRenderMode,
    render_sort::PluginRenderSort,
    // render_item_info::{RendererItemsModifyByMaterialChange, RendererItemsReset, RendererItemsModifyByModelChange},
    // renderer_binds_sys::{SysSceneBindUpdate,},
    renderer::RendererHasher,
    graphic::RendererGraphicDesc,
    render_object::RendererID,
    sys_renderer_pre::{SysSet0ModifyByRendererID, SysSet0ModifyFromScene, SysSet1ModifyByRendererID, SysSet2ModifyByRendererID, SysSet1ModifyByModel, SysSet2ModifyByModel, SysBufferAllocatorUpdate, SysBindGroupLoad},
    sys_renderer::*,
    pass::{AssetDataCenterShader3D, AssetDataCenterPipeline3D, AssetLoaderShader3D, AssetLoaderPipeline3D},
};

pub mod render_object;
pub mod opaque;
pub mod renderer;
pub mod render_mode;
pub mod render_blend;
pub mod render_depth_and_stencil;
pub mod render_primitive;
pub mod render_sort;
pub mod render_target_state;
pub mod graphic;
pub mod sys_renderer_pre;
pub mod sys_renderer;
pub mod pass;
pub mod command;
pub mod base;


#[derive(Debug, Clone, Default)]
pub struct ViewerRenderersInfo {
    pub map: XHashMap<Atom, (RendererGraphicDesc, RendererID)>,
}


#[derive(NodeParam, Clone, Default)]
pub struct RenderTarget {
    pub target: Option<ShareTargetView>,
}

pub struct ResultToScreenGraphicNode {

}
impl Node for ResultToScreenGraphicNode {
    type Input = RenderTarget;

    type Output = ();

    fn run<'a>(
        &'a mut self,
        context: pi_render::graph::RenderContext,
        commands: pi_share::ShareRefCell<wgpu::CommandEncoder>,
        input: &'a Self::Input,
        usage: &'a pi_render::graph::node::ParamUsage,
    ) -> BoxFuture<'a, Result<Self::Output, String>> {
        async move {
            Ok(())
        }.boxed()
    }
}

pub struct PluginRenderer;
impl crate::Plugin for PluginRenderer {
    fn init(
        &mut self,
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {

        PluginRenderBlend.init(engine, stages);
        PluginRenderDepthAndStencil.init(engine, stages);
        PluginRenderPrimitive.init(engine, stages);
        PluginRenderMode.init(engine, stages);
        PluginRenderSort.init(engine, stages);

        let world = engine.world_mut();
        world.insert_resource(RendererHasher::default());
        world.insert_resource(AssetMgr::<RenderRes<wgpu::TextureView>>::new(
            GarbageEmpty(), 
            false,
            60 * 1024 * 1024, 
            3 * 60 * 1000
        ));

        let device = world.get_resource::<RenderDevice>().unwrap().clone();
        
        if world.get_resource::<SafeAtlasAllocator>().is_none() {
            let texture_assets_mgr = AssetMgr::<RenderRes<wgpu::TextureView>>::new(
                GarbageEmpty(), 
                false,
                60 * 1024 * 1024, 
                3 * 60 * 1000
            );
            let unusetexture_assets_mgr = HomogeneousMgr::<RenderRes<UnuseTexture>>::new(
                pi_assets::homogeneous::GarbageEmpty(), 
                10 * size_of::<UnuseTexture>(),
                size_of::<UnuseTexture>(),
                3 * 60 * 1000,
            );
            let atlas = SafeAtlasAllocator::new(device, texture_assets_mgr, unusetexture_assets_mgr);
            world.insert_resource(atlas);
        }

        if world.get_resource::<AssetDataCenterShader3D>().is_none() {
            world.insert_resource(AssetDataCenterShader3D::new(false, 10 * 1024 * 1024, 60 * 1000));
        }
        if world.get_resource::<AssetDataCenterPipeline3D>().is_none() {
            world.insert_resource(AssetDataCenterPipeline3D::new(false, 10 * 1024 * 1024, 60 * 1000));
        }
        if world.get_resource::<AssetLoaderShader3D>().is_none() {
            world.insert_resource(AssetLoaderShader3D::default());
        }
        if world.get_resource::<AssetLoaderPipeline3D>().is_none() {
            world.insert_resource(AssetLoaderPipeline3D::default());
        }

        let world = engine.world_mut();
        // RendererItemsReset::setup(world, stages.query_stage::<RendererItemsReset>(ERunStageChap::Uniform));
        // RendererItemsModifyByModelChange::setup(world, stages.query_stage::<RendererItemsModifyByModelChange>(ERunStageChap::Uniform));
        // RendererItemsModifyByMaterialChange::setup(world, stages.query_stage::<RendererItemsModifyByMaterialChange>(ERunStageChap::Uniform));
        // SysSceneBindUpdate::setup(world, stages.query_stage::<SysSceneBindUpdate>(ERunStageChap::Command));
        // SysRendererCommandTick::setup(world, stages.query_stage::<SysDynBufferAllocatorUpdate>(ERunStageChap::Uniform));

        SysBufferAllocatorUpdate::setup(world, stages.query_stage::<SysBufferAllocatorUpdate>(ERunStageChap::Uniform));
        
        // Bindgroup
        SysSet0ModifyByRendererID::<Pass01, PassID01>::setup(world, stages.query_stage::<SysSet0ModifyByRendererID::<Pass01, PassID01>>(ERunStageChap::Uniform));
        SysSet0ModifyByRendererID::<Pass02, PassID02>::setup(world, stages.query_stage::<SysSet0ModifyByRendererID::<Pass02, PassID02>>(ERunStageChap::Uniform));
        SysSet0ModifyByRendererID::<Pass03, PassID03>::setup(world, stages.query_stage::<SysSet0ModifyByRendererID::<Pass03, PassID03>>(ERunStageChap::Uniform));
        SysSet0ModifyByRendererID::<Pass04, PassID04>::setup(world, stages.query_stage::<SysSet0ModifyByRendererID::<Pass04, PassID04>>(ERunStageChap::Uniform));
        SysSet0ModifyByRendererID::<Pass05, PassID05>::setup(world, stages.query_stage::<SysSet0ModifyByRendererID::<Pass05, PassID05>>(ERunStageChap::Uniform));
        SysSet0ModifyByRendererID::<Pass06, PassID06>::setup(world, stages.query_stage::<SysSet0ModifyByRendererID::<Pass06, PassID06>>(ERunStageChap::Uniform));
        SysSet0ModifyByRendererID::<Pass07, PassID07>::setup(world, stages.query_stage::<SysSet0ModifyByRendererID::<Pass07, PassID07>>(ERunStageChap::Uniform));
        SysSet0ModifyByRendererID::<Pass08, PassID08>::setup(world, stages.query_stage::<SysSet0ModifyByRendererID::<Pass08, PassID08>>(ERunStageChap::Uniform));

        SysSet0ModifyFromScene::<Pass01, PassID01>::setup(world, stages.query_stage::<SysSet0ModifyFromScene::<Pass01, PassID01>>(ERunStageChap::Uniform));
        SysSet0ModifyFromScene::<Pass02, PassID02>::setup(world, stages.query_stage::<SysSet0ModifyFromScene::<Pass02, PassID02>>(ERunStageChap::Uniform));
        SysSet0ModifyFromScene::<Pass03, PassID03>::setup(world, stages.query_stage::<SysSet0ModifyFromScene::<Pass03, PassID03>>(ERunStageChap::Uniform));
        SysSet0ModifyFromScene::<Pass04, PassID04>::setup(world, stages.query_stage::<SysSet0ModifyFromScene::<Pass04, PassID04>>(ERunStageChap::Uniform));
        SysSet0ModifyFromScene::<Pass05, PassID05>::setup(world, stages.query_stage::<SysSet0ModifyFromScene::<Pass05, PassID05>>(ERunStageChap::Uniform));
        SysSet0ModifyFromScene::<Pass06, PassID06>::setup(world, stages.query_stage::<SysSet0ModifyFromScene::<Pass06, PassID06>>(ERunStageChap::Uniform));
        SysSet0ModifyFromScene::<Pass07, PassID07>::setup(world, stages.query_stage::<SysSet0ModifyFromScene::<Pass07, PassID07>>(ERunStageChap::Uniform));
        SysSet0ModifyFromScene::<Pass08, PassID08>::setup(world, stages.query_stage::<SysSet0ModifyFromScene::<Pass08, PassID08>>(ERunStageChap::Uniform));

        SysSet1ModifyByRendererID::<Pass01, PassID01>::setup(world, stages.query_stage::<SysSet1ModifyByRendererID::<Pass01, PassID01>>(ERunStageChap::Uniform));
        SysSet1ModifyByRendererID::<Pass02, PassID02>::setup(world, stages.query_stage::<SysSet1ModifyByRendererID::<Pass02, PassID02>>(ERunStageChap::Uniform));
        SysSet1ModifyByRendererID::<Pass03, PassID03>::setup(world, stages.query_stage::<SysSet1ModifyByRendererID::<Pass03, PassID03>>(ERunStageChap::Uniform));
        SysSet1ModifyByRendererID::<Pass04, PassID04>::setup(world, stages.query_stage::<SysSet1ModifyByRendererID::<Pass04, PassID04>>(ERunStageChap::Uniform));
        SysSet1ModifyByRendererID::<Pass05, PassID05>::setup(world, stages.query_stage::<SysSet1ModifyByRendererID::<Pass05, PassID05>>(ERunStageChap::Uniform));
        SysSet1ModifyByRendererID::<Pass06, PassID06>::setup(world, stages.query_stage::<SysSet1ModifyByRendererID::<Pass06, PassID06>>(ERunStageChap::Uniform));
        SysSet1ModifyByRendererID::<Pass07, PassID07>::setup(world, stages.query_stage::<SysSet1ModifyByRendererID::<Pass07, PassID07>>(ERunStageChap::Uniform));
        SysSet1ModifyByRendererID::<Pass08, PassID08>::setup(world, stages.query_stage::<SysSet1ModifyByRendererID::<Pass08, PassID08>>(ERunStageChap::Uniform));

        SysSet1ModifyByModel::<Pass01, PassID01>::setup(world, stages.query_stage::<SysSet1ModifyByModel::<Pass01, PassID01>>(ERunStageChap::Uniform));
        SysSet1ModifyByModel::<Pass02, PassID02>::setup(world, stages.query_stage::<SysSet1ModifyByModel::<Pass02, PassID02>>(ERunStageChap::Uniform));
        SysSet1ModifyByModel::<Pass03, PassID03>::setup(world, stages.query_stage::<SysSet1ModifyByModel::<Pass03, PassID03>>(ERunStageChap::Uniform));
        SysSet1ModifyByModel::<Pass04, PassID04>::setup(world, stages.query_stage::<SysSet1ModifyByModel::<Pass04, PassID04>>(ERunStageChap::Uniform));
        SysSet1ModifyByModel::<Pass05, PassID05>::setup(world, stages.query_stage::<SysSet1ModifyByModel::<Pass05, PassID05>>(ERunStageChap::Uniform));
        SysSet1ModifyByModel::<Pass06, PassID06>::setup(world, stages.query_stage::<SysSet1ModifyByModel::<Pass06, PassID06>>(ERunStageChap::Uniform));
        SysSet1ModifyByModel::<Pass07, PassID07>::setup(world, stages.query_stage::<SysSet1ModifyByModel::<Pass07, PassID07>>(ERunStageChap::Uniform));
        SysSet1ModifyByModel::<Pass08, PassID08>::setup(world, stages.query_stage::<SysSet1ModifyByModel::<Pass08, PassID08>>(ERunStageChap::Uniform));

        SysSet2ModifyByRendererID::<Pass01, PassID01>::setup(world, stages.query_stage::<SysSet2ModifyByRendererID::<Pass01, PassID01>>(ERunStageChap::Uniform));
        SysSet2ModifyByRendererID::<Pass02, PassID02>::setup(world, stages.query_stage::<SysSet2ModifyByRendererID::<Pass02, PassID02>>(ERunStageChap::Uniform));
        SysSet2ModifyByRendererID::<Pass03, PassID03>::setup(world, stages.query_stage::<SysSet2ModifyByRendererID::<Pass03, PassID03>>(ERunStageChap::Uniform));
        SysSet2ModifyByRendererID::<Pass04, PassID04>::setup(world, stages.query_stage::<SysSet2ModifyByRendererID::<Pass04, PassID04>>(ERunStageChap::Uniform));
        SysSet2ModifyByRendererID::<Pass05, PassID05>::setup(world, stages.query_stage::<SysSet2ModifyByRendererID::<Pass05, PassID05>>(ERunStageChap::Uniform));
        SysSet2ModifyByRendererID::<Pass06, PassID06>::setup(world, stages.query_stage::<SysSet2ModifyByRendererID::<Pass06, PassID06>>(ERunStageChap::Uniform));
        SysSet2ModifyByRendererID::<Pass07, PassID07>::setup(world, stages.query_stage::<SysSet2ModifyByRendererID::<Pass07, PassID07>>(ERunStageChap::Uniform));
        SysSet2ModifyByRendererID::<Pass08, PassID08>::setup(world, stages.query_stage::<SysSet2ModifyByRendererID::<Pass08, PassID08>>(ERunStageChap::Uniform));

        SysSet2ModifyByModel::<Pass01, PassID01>::setup(world, stages.query_stage::<SysSet2ModifyByModel::<Pass01, PassID01>>(ERunStageChap::Uniform));
        SysSet2ModifyByModel::<Pass02, PassID02>::setup(world, stages.query_stage::<SysSet2ModifyByModel::<Pass02, PassID02>>(ERunStageChap::Uniform));
        SysSet2ModifyByModel::<Pass03, PassID03>::setup(world, stages.query_stage::<SysSet2ModifyByModel::<Pass03, PassID03>>(ERunStageChap::Uniform));
        SysSet2ModifyByModel::<Pass04, PassID04>::setup(world, stages.query_stage::<SysSet2ModifyByModel::<Pass04, PassID04>>(ERunStageChap::Uniform));
        SysSet2ModifyByModel::<Pass05, PassID05>::setup(world, stages.query_stage::<SysSet2ModifyByModel::<Pass05, PassID05>>(ERunStageChap::Uniform));
        SysSet2ModifyByModel::<Pass06, PassID06>::setup(world, stages.query_stage::<SysSet2ModifyByModel::<Pass06, PassID06>>(ERunStageChap::Uniform));
        SysSet2ModifyByModel::<Pass07, PassID07>::setup(world, stages.query_stage::<SysSet2ModifyByModel::<Pass07, PassID07>>(ERunStageChap::Uniform));
        SysSet2ModifyByModel::<Pass08, PassID08>::setup(world, stages.query_stage::<SysSet2ModifyByModel::<Pass08, PassID08>>(ERunStageChap::Uniform));

        SysBindGroupLoad::setup(world, stages.query_stage::<SysBindGroupLoad>(ERunStageChap::Uniform));

        SysPassBindGroups::<Pass01, PassID01>::setup(world, stages.query_stage::<SysPassBindGroups::<Pass01, PassID01>>(ERunStageChap::Uniform));
        SysPassBindGroups::<Pass02, PassID02>::setup(world, stages.query_stage::<SysPassBindGroups::<Pass02, PassID02>>(ERunStageChap::Uniform));
        SysPassBindGroups::<Pass03, PassID03>::setup(world, stages.query_stage::<SysPassBindGroups::<Pass03, PassID03>>(ERunStageChap::Uniform));
        SysPassBindGroups::<Pass04, PassID04>::setup(world, stages.query_stage::<SysPassBindGroups::<Pass04, PassID04>>(ERunStageChap::Uniform));
        SysPassBindGroups::<Pass05, PassID05>::setup(world, stages.query_stage::<SysPassBindGroups::<Pass05, PassID05>>(ERunStageChap::Uniform));
        SysPassBindGroups::<Pass06, PassID06>::setup(world, stages.query_stage::<SysPassBindGroups::<Pass06, PassID06>>(ERunStageChap::Uniform));
        SysPassBindGroups::<Pass07, PassID07>::setup(world, stages.query_stage::<SysPassBindGroups::<Pass07, PassID07>>(ERunStageChap::Uniform));
        SysPassBindGroups::<Pass08, PassID08>::setup(world, stages.query_stage::<SysPassBindGroups::<Pass08, PassID08>>(ERunStageChap::Uniform));

        // Shader
        SysPassShaderRequestByModel::<Pass01, PassID01>::setup(world, stages.query_stage::<SysPassShaderRequestByModel::<Pass01, PassID01>>(ERunStageChap::Uniform));
        SysPassShaderRequestByModel::<Pass02, PassID02>::setup(world, stages.query_stage::<SysPassShaderRequestByModel::<Pass02, PassID02>>(ERunStageChap::Uniform));
        SysPassShaderRequestByModel::<Pass03, PassID03>::setup(world, stages.query_stage::<SysPassShaderRequestByModel::<Pass03, PassID03>>(ERunStageChap::Uniform));
        SysPassShaderRequestByModel::<Pass04, PassID04>::setup(world, stages.query_stage::<SysPassShaderRequestByModel::<Pass04, PassID04>>(ERunStageChap::Uniform));
        SysPassShaderRequestByModel::<Pass05, PassID05>::setup(world, stages.query_stage::<SysPassShaderRequestByModel::<Pass05, PassID05>>(ERunStageChap::Uniform));
        SysPassShaderRequestByModel::<Pass06, PassID06>::setup(world, stages.query_stage::<SysPassShaderRequestByModel::<Pass06, PassID06>>(ERunStageChap::Uniform));
        SysPassShaderRequestByModel::<Pass07, PassID07>::setup(world, stages.query_stage::<SysPassShaderRequestByModel::<Pass07, PassID07>>(ERunStageChap::Uniform));
        SysPassShaderRequestByModel::<Pass08, PassID08>::setup(world, stages.query_stage::<SysPassShaderRequestByModel::<Pass08, PassID08>>(ERunStageChap::Uniform));

        SysPassShaderRequestByPass::<Pass01, PassID01>::setup(world, stages.query_stage::<SysPassShaderRequestByPass::<Pass01, PassID01>>(ERunStageChap::Uniform));
        SysPassShaderRequestByPass::<Pass02, PassID02>::setup(world, stages.query_stage::<SysPassShaderRequestByPass::<Pass02, PassID02>>(ERunStageChap::Uniform));
        SysPassShaderRequestByPass::<Pass03, PassID03>::setup(world, stages.query_stage::<SysPassShaderRequestByPass::<Pass03, PassID03>>(ERunStageChap::Uniform));
        SysPassShaderRequestByPass::<Pass04, PassID04>::setup(world, stages.query_stage::<SysPassShaderRequestByPass::<Pass04, PassID04>>(ERunStageChap::Uniform));
        SysPassShaderRequestByPass::<Pass05, PassID05>::setup(world, stages.query_stage::<SysPassShaderRequestByPass::<Pass05, PassID05>>(ERunStageChap::Uniform));
        SysPassShaderRequestByPass::<Pass06, PassID06>::setup(world, stages.query_stage::<SysPassShaderRequestByPass::<Pass06, PassID06>>(ERunStageChap::Uniform));
        SysPassShaderRequestByPass::<Pass07, PassID07>::setup(world, stages.query_stage::<SysPassShaderRequestByPass::<Pass07, PassID07>>(ERunStageChap::Uniform));
        SysPassShaderRequestByPass::<Pass08, PassID08>::setup(world, stages.query_stage::<SysPassShaderRequestByPass::<Pass08, PassID08>>(ERunStageChap::Uniform));

        SysPassShaderLoad::setup(world, stages.query_stage::<SysPassShaderLoad>(ERunStageChap::Uniform));

        SysPassPipelineRequestByModel::<Pass01, PassID01>::setup(world, stages.query_stage::<SysPassPipelineRequestByModel::<Pass01, PassID01>>(ERunStageChap::Uniform));
        SysPassPipelineRequestByModel::<Pass02, PassID02>::setup(world, stages.query_stage::<SysPassPipelineRequestByModel::<Pass02, PassID02>>(ERunStageChap::Uniform));
        SysPassPipelineRequestByModel::<Pass03, PassID03>::setup(world, stages.query_stage::<SysPassPipelineRequestByModel::<Pass03, PassID03>>(ERunStageChap::Uniform));
        SysPassPipelineRequestByModel::<Pass04, PassID04>::setup(world, stages.query_stage::<SysPassPipelineRequestByModel::<Pass04, PassID04>>(ERunStageChap::Uniform));
        SysPassPipelineRequestByModel::<Pass05, PassID05>::setup(world, stages.query_stage::<SysPassPipelineRequestByModel::<Pass05, PassID05>>(ERunStageChap::Uniform));
        SysPassPipelineRequestByModel::<Pass06, PassID06>::setup(world, stages.query_stage::<SysPassPipelineRequestByModel::<Pass06, PassID06>>(ERunStageChap::Uniform));
        SysPassPipelineRequestByModel::<Pass07, PassID07>::setup(world, stages.query_stage::<SysPassPipelineRequestByModel::<Pass07, PassID07>>(ERunStageChap::Uniform));
        SysPassPipelineRequestByModel::<Pass08, PassID08>::setup(world, stages.query_stage::<SysPassPipelineRequestByModel::<Pass08, PassID08>>(ERunStageChap::Uniform));

        SysPassPipelineRequestByPass::<Pass01, PassID01>::setup(world, stages.query_stage::<SysPassPipelineRequestByPass::<Pass01, PassID01>>(ERunStageChap::Uniform));
        SysPassPipelineRequestByPass::<Pass02, PassID02>::setup(world, stages.query_stage::<SysPassPipelineRequestByPass::<Pass02, PassID02>>(ERunStageChap::Uniform));
        SysPassPipelineRequestByPass::<Pass03, PassID03>::setup(world, stages.query_stage::<SysPassPipelineRequestByPass::<Pass03, PassID03>>(ERunStageChap::Uniform));
        SysPassPipelineRequestByPass::<Pass04, PassID04>::setup(world, stages.query_stage::<SysPassPipelineRequestByPass::<Pass04, PassID04>>(ERunStageChap::Uniform));
        SysPassPipelineRequestByPass::<Pass05, PassID05>::setup(world, stages.query_stage::<SysPassPipelineRequestByPass::<Pass05, PassID05>>(ERunStageChap::Uniform));
        SysPassPipelineRequestByPass::<Pass06, PassID06>::setup(world, stages.query_stage::<SysPassPipelineRequestByPass::<Pass06, PassID06>>(ERunStageChap::Uniform));
        SysPassPipelineRequestByPass::<Pass07, PassID07>::setup(world, stages.query_stage::<SysPassPipelineRequestByPass::<Pass07, PassID07>>(ERunStageChap::Uniform));
        SysPassPipelineRequestByPass::<Pass08, PassID08>::setup(world, stages.query_stage::<SysPassPipelineRequestByPass::<Pass08, PassID08>>(ERunStageChap::Uniform));

        SysPassPipeline3DLoad::setup(world, stages.query_stage::<SysPassPipeline3DLoad>(ERunStageChap::Uniform));
        
        SysPassDraw::<Pass01, PassID01>::setup(world, stages.query_stage::<SysPassDraw::<Pass01, PassID01>>(ERunStageChap::Uniform));
        SysPassDraw::<Pass02, PassID02>::setup(world, stages.query_stage::<SysPassDraw::<Pass02, PassID02>>(ERunStageChap::Uniform));
        SysPassDraw::<Pass03, PassID03>::setup(world, stages.query_stage::<SysPassDraw::<Pass03, PassID03>>(ERunStageChap::Uniform));
        SysPassDraw::<Pass04, PassID04>::setup(world, stages.query_stage::<SysPassDraw::<Pass04, PassID04>>(ERunStageChap::Uniform));
        SysPassDraw::<Pass05, PassID05>::setup(world, stages.query_stage::<SysPassDraw::<Pass05, PassID05>>(ERunStageChap::Uniform));
        SysPassDraw::<Pass06, PassID06>::setup(world, stages.query_stage::<SysPassDraw::<Pass06, PassID06>>(ERunStageChap::Uniform));
        SysPassDraw::<Pass07, PassID07>::setup(world, stages.query_stage::<SysPassDraw::<Pass07, PassID07>>(ERunStageChap::Uniform));
        SysPassDraw::<Pass08, PassID08>::setup(world, stages.query_stage::<SysPassDraw::<Pass08, PassID08>>(ERunStageChap::Uniform));
        
        SysPassDrawByModel::<Pass01, PassID01>::setup(world, stages.query_stage::<SysPassDrawByModel::<Pass01, PassID01>>(ERunStageChap::Uniform));
        SysPassDrawByModel::<Pass02, PassID02>::setup(world, stages.query_stage::<SysPassDrawByModel::<Pass02, PassID02>>(ERunStageChap::Uniform));
        SysPassDrawByModel::<Pass03, PassID03>::setup(world, stages.query_stage::<SysPassDrawByModel::<Pass03, PassID03>>(ERunStageChap::Uniform));
        SysPassDrawByModel::<Pass04, PassID04>::setup(world, stages.query_stage::<SysPassDrawByModel::<Pass04, PassID04>>(ERunStageChap::Uniform));
        SysPassDrawByModel::<Pass05, PassID05>::setup(world, stages.query_stage::<SysPassDrawByModel::<Pass05, PassID05>>(ERunStageChap::Uniform));
        SysPassDrawByModel::<Pass06, PassID06>::setup(world, stages.query_stage::<SysPassDrawByModel::<Pass06, PassID06>>(ERunStageChap::Uniform));
        SysPassDrawByModel::<Pass07, PassID07>::setup(world, stages.query_stage::<SysPassDrawByModel::<Pass07, PassID07>>(ERunStageChap::Uniform));
        SysPassDrawByModel::<Pass08, PassID08>::setup(world, stages.query_stage::<SysPassDrawByModel::<Pass08, PassID08>>(ERunStageChap::Uniform));

        SysRendererDraws::setup(world, stages.query_stage::<SysRendererDraws>(ERunStageChap::Uniform));

        Ok(())
    }
}
