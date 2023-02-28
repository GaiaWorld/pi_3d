
use futures::FutureExt;
use pi_assets::{mgr::AssetMgr, asset::GarbageEmpty};
use pi_atom::Atom;
use pi_ecs::prelude::Setup;
use pi_engine_shell::{run_stage::ERunStageChap, assets::sync_load::PluginAssetSyncNotNeedLoad};
use pi_futures::BoxFuture;
use pi_hash::XHashMap;
use pi_render::{components::view::{target_alloc::{ShareTargetView}}, graph::{node::Node}, rhi::asset::RenderRes, };
use render_derive::NodeParam;

use crate::{renderers::sys_renderer::SysRendererDraws};

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
    sys_renderer_pre::{SysSet0ModifyByRendererID, SysSet0ModifyFromScene, SysSet1ModifyByRendererID, SysSet2ModifyByRendererID, SysSet1ModifyByModel, SysSet2ModifyByModel, SysBufferAllocatorUpdate},
    sys_renderer::*, pass::{KeyPipeline3D, Pipeline3D}
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

        PluginAssetSyncNotNeedLoad::<KeyPipeline3D, Pipeline3D>::new(false, 10 * 1024 * 1024, 60 * 1000).init(engine, stages);

        let world = engine.world_mut();
        // RendererItemsReset::setup(world, stages.query_stage::<RendererItemsReset>(ERunStageChap::Uniform));
        // RendererItemsModifyByModelChange::setup(world, stages.query_stage::<RendererItemsModifyByModelChange>(ERunStageChap::Uniform));
        // RendererItemsModifyByMaterialChange::setup(world, stages.query_stage::<RendererItemsModifyByMaterialChange>(ERunStageChap::Uniform));
        // SysSceneBindUpdate::setup(world, stages.query_stage::<SysSceneBindUpdate>(ERunStageChap::Command));
        // SysRendererCommandTick::setup(world, stages.query_stage::<SysDynBufferAllocatorUpdate>(ERunStageChap::Uniform));

        SysBufferAllocatorUpdate::setup(world, stages.query_stage::<SysBufferAllocatorUpdate>(ERunStageChap::Uniform));
        
        // Bindgroup
        SysSet0ModifyByRendererID::setup(world, stages.query_stage::<SysSet0ModifyByRendererID>(ERunStageChap::Uniform));
        SysSet0ModifyFromScene::setup(world, stages.query_stage::<SysSet0ModifyFromScene>(ERunStageChap::Uniform));
        SysSet1ModifyByRendererID::setup(world, stages.query_stage::<SysSet1ModifyByRendererID>(ERunStageChap::Uniform));
        SysSet1ModifyByModel::setup(world, stages.query_stage::<SysSet1ModifyByModel>(ERunStageChap::Uniform));
        SysSet2ModifyByRendererID::setup(world, stages.query_stage::<SysSet2ModifyByRendererID>(ERunStageChap::Uniform));
        SysSet2ModifyByModel::setup(world, stages.query_stage::<SysSet2ModifyByModel>(ERunStageChap::Uniform));

        // Shader
        SysPass01ShaderUpdate::setup(world, stages.query_stage::<SysPass01ShaderUpdate>(ERunStageChap::Uniform));
        SysPass02ShaderUpdate::setup(world, stages.query_stage::<SysPass02ShaderUpdate>(ERunStageChap::Uniform));
        SysPass03ShaderUpdate::setup(world, stages.query_stage::<SysPass03ShaderUpdate>(ERunStageChap::Uniform));
        SysPass04ShaderUpdate::setup(world, stages.query_stage::<SysPass04ShaderUpdate>(ERunStageChap::Uniform));
        SysPass05ShaderUpdate::setup(world, stages.query_stage::<SysPass05ShaderUpdate>(ERunStageChap::Uniform));
        SysPass06ShaderUpdate::setup(world, stages.query_stage::<SysPass06ShaderUpdate>(ERunStageChap::Uniform));
        SysPass07ShaderUpdate::setup(world, stages.query_stage::<SysPass07ShaderUpdate>(ERunStageChap::Uniform));
        SysPass08ShaderUpdate::setup(world, stages.query_stage::<SysPass08ShaderUpdate>(ERunStageChap::Uniform));

        SysPass01DrawUpdate::setup(world, stages.query_stage::<SysPass01DrawUpdate>(ERunStageChap::Uniform));
        SysPass02DrawUpdate::setup(world, stages.query_stage::<SysPass02DrawUpdate>(ERunStageChap::Uniform));
        SysPass03DrawUpdate::setup(world, stages.query_stage::<SysPass03DrawUpdate>(ERunStageChap::Uniform));
        SysPass04DrawUpdate::setup(world, stages.query_stage::<SysPass04DrawUpdate>(ERunStageChap::Uniform));
        SysPass05DrawUpdate::setup(world, stages.query_stage::<SysPass05DrawUpdate>(ERunStageChap::Uniform));
        SysPass06DrawUpdate::setup(world, stages.query_stage::<SysPass06DrawUpdate>(ERunStageChap::Uniform));
        SysPass07DrawUpdate::setup(world, stages.query_stage::<SysPass07DrawUpdate>(ERunStageChap::Uniform));
        SysPass08DrawUpdate::setup(world, stages.query_stage::<SysPass08DrawUpdate>(ERunStageChap::Uniform));

        SysRendererDraws::setup(world, stages.query_stage::<SysRendererDraws>(ERunStageChap::Uniform));

        Ok(())
    }
}
