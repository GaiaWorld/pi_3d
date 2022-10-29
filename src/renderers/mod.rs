
use futures::FutureExt;
use pi_ecs::{world::World, query::QueryState};
use pi_futures::BoxFuture;
use pi_render::{components::view::{target_alloc::ShareTargetView, render_window::RenderWindow}, graph::{node::Node, param::{InParam, OutParam}, RenderContext}, rhi::texture::ScreenTexture};
use render_data_container::{TGeometryBufferID, GeometryBufferPool, TVertexBufferKindKey, EVertexDataFormat};
use render_derive::NodeParam;
use render_geometry::geometry::{};
use render_material::error::EMaterialError;

use crate::{meshes::Mesh, geometry::{VDK, GBID}, resources::{SingleRenderObjectPipelinePool, SingleGeometryBufferPool}, object::{ObjectID, GameObject}};

pub mod pipeline;
pub mod render_object;


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

