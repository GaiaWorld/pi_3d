
use futures::FutureExt;
use pi_ecs::prelude::Setup;
use pi_engine_shell::{object::ObjectID, run_stage::ERunStageChap};
use pi_futures::BoxFuture;
use pi_render::{components::view::{target_alloc::ShareTargetView}, graph::{node::Node}, };
use render_derive::NodeParam;

use self::{render_blend::PluginRenderBlend, render_depth_and_stencil::PluginRenderDepthAndStencil, render_primitive::PluginRenderPrimitive, render_mode::PluginRenderMode, render_sort::PluginRenderSort, render_item_info::{RendererItemsModifyByMaterialChange, RendererItemsReset, RendererItemsModifyByModelChange}, renderer_binds_sys::{SysSceneBindUpdate,}};

pub mod pipeline;
pub mod render_object;
pub mod opaque;
pub mod renderer;
pub mod render_mode;
pub mod render_blend;
pub mod render_depth_and_stencil;
pub mod render_primitive;
pub mod render_sort;
pub mod render_target_state;
pub mod render_object_list;
pub mod render_item_info;
pub mod renderer_binds_sys;


pub struct SingleScreenClearGraphicNodeKey(pub String);

pub struct SingleResultToScreenGraphicNodeKey(pub String);

pub struct ModelList(pub Vec<ObjectID>);

pub struct ModelListAfterCulling(pub Vec<ObjectID>);

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
        RendererItemsReset::setup(world, stages.query_stage::<RendererItemsReset>(ERunStageChap::Uniform));
        RendererItemsModifyByModelChange::setup(world, stages.query_stage::<RendererItemsModifyByModelChange>(ERunStageChap::Uniform));
        RendererItemsModifyByMaterialChange::setup(world, stages.query_stage::<RendererItemsModifyByMaterialChange>(ERunStageChap::Uniform));
        SysSceneBindUpdate::setup(world, stages.query_stage::<SysSceneBindUpdate>(ERunStageChap::Command));

        Ok(())
    }
}
