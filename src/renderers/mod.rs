
use futures::FutureExt;
use pi_futures::BoxFuture;
use pi_render::{components::view::{target_alloc::ShareTargetView, render_window::RenderWindow}, graph::{node::Node, param::{InParam, OutParam}, RenderContext}, rhi::texture::ScreenTexture};
use render_data_container::{TGeometryBufferID, GeometryBufferPool, TVertexBufferKindKey, EVertexDataFormat};
use render_derive::NodeParam;
use render_geometry::geometry::{};
use render_material::error::EMaterialError;

use crate::{meshes::Mesh, geometry::{VDK, GBID}, resources::{SingleRenderObjectPipelinePool, SingleGeometryBufferPool}, object::{ObjectID, GameObject}, plugin::Plugin};

use self::{render_blend::PluginRenderBlend, render_depth_and_stencil::PluginRenderDepthAndStencil, render_primitive::PluginRenderPrimitive, render_mode::PluginRenderMode};

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
pub mod dirty;


pub struct SingleScreenClearGraphicNodeKey(pub String);

pub struct SingleResultToScreenGraphicNodeKey(pub String);

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
        world: &mut pi_ecs::world::World,
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {

        PluginRenderBlend.init(world, engine, stages);
        PluginRenderDepthAndStencil.init(world, engine, stages);
        PluginRenderPrimitive.init(world, engine, stages);
        PluginRenderMode.init(world, engine, stages);

        Ok(())
    }
}
