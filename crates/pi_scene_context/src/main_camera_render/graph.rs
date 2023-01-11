use std::time::Instant;

use futures::FutureExt;
use pi_assets::mgr::AssetMgr;
use pi_ecs::query::QueryState;
use pi_futures::BoxFuture;
use pi_render::{graph::{node::Node, RenderContext}, rhi::{texture::ScreenTexture}};
use render_data_container::VertexBufferPool;

use crate::{
    renderers::{render_object_list::DrawList, renderer::Renderer},
    object::{ObjectID, GameObject},
    main_camera_render::MainCameraRenderer,
    resources::{SingleRenderObjectPipelinePool},
    bindgroup::{RenderBindGroupKey, RenderBindGroupPool}
};

pub struct SingleMainCameraOpaqueRenderNode {
    pub renderer_id: ObjectID,
}
impl SingleMainCameraOpaqueRenderNode {
    pub fn new(renderer_id: ObjectID) -> Self {
        Self {
            renderer_id,
        }
    }
}
impl Node for SingleMainCameraOpaqueRenderNode {
    type Input = ();

    type Output = ();

    fn run<'a>(
        &'a mut self,
        context: pi_render::graph::RenderContext,
        mut commands: pi_share::ShareRefCell<wgpu::CommandEncoder>,
        input: &'a Self::Input,
        usage: &'a pi_render::graph::node::ParamUsage,
    ) -> BoxFuture<'a, Result<Self::Output, String>> {
        let time = Instant::now();

        let RenderContext {
            mut world, ..
        } = context;

        // let window = world.get_resource::<RenderWindow>().unwrap();

        let query = QueryState::<GameObject, & Renderer>::new(&mut world);

        //  log::debug!("SingleMainCameraOpaqueRenderNode ............. {:?}", self.renderer_id);
        match query.get(&world, self.renderer_id) {
            Some(renderer) => {
                let surface = world.get_resource::<ScreenTexture>().unwrap();
                let bindgrouppool = world.get_resource::<RenderBindGroupPool>().unwrap();
                let vbpool = world.get_resource::<VertexBufferPool>().unwrap();
                renderer.opaque_draws.render(&mut commands, surface.view.as_ref().unwrap(), None, bindgrouppool, vbpool);
                renderer.transparent_draws.render(&mut commands, surface.view.as_ref().unwrap(), None, bindgrouppool, vbpool);
            },
            None => {
                
            },
        }
 
        let mut query = QueryState::<GameObject, &mut Renderer>::new(&mut world);
        if let Some(mut renderer) = query.get_mut(&mut world, self.renderer_id) {
            renderer.clear();
        }

        let time1 = Instant::now();
        log::debug!("MainCameraRenderNode: {:?}", time1 - time);

        async move {
            Ok(())
        }.boxed()
    }
}
