use std::time::Instant;

use futures::FutureExt;
use pi_ecs::query::QueryState;
use pi_futures::BoxFuture;
use pi_render::{graph::{node::Node, RenderContext}, rhi::{texture::ScreenTexture}};

use crate::{renderers::{render_object_list::DrawList}, object::{ObjectID, GameObject}, main_camera_render::MainCameraRenderer, materials::bind_group::{RenderBindGroupPool}, resources::{SingleRenderObjectPipelinePool, SingleGeometryBufferPool}};

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

        let query = QueryState::<GameObject, & MainCameraRenderer>::new(&mut world);

        //  println!("SingleMainCameraOpaqueRenderNode ............. {:?}", self.renderer_id);
        match query.get(&world, self.renderer_id) {
            Some(renderer) => {
                let surface = world.get_resource::<ScreenTexture>().unwrap();
                let bindgrouppool = world.get_resource::<RenderBindGroupPool>().unwrap();
                let pipelines = world.get_resource::<SingleRenderObjectPipelinePool>().unwrap();
                let gbp = world.get_resource::<SingleGeometryBufferPool>().unwrap();
                renderer.opaque_draws.render(&mut commands, surface.view.as_ref().unwrap(), bindgrouppool, pipelines, gbp);
                renderer.transparent_draws.render(&mut commands, surface.view.as_ref().unwrap(), bindgrouppool, pipelines, gbp);
            },
            None => {
                
            },
        }
 
        let mut query = QueryState::<GameObject, &mut MainCameraRenderer>::new(&mut world);
        if let Some(mut renderer) = query.get_mut(&mut world, self.renderer_id) {
            renderer.clear();
        }

        let time1 = Instant::now();
        println!("MainCameraRenderNode: {:?}", time1 - time);

        async move {
            Ok(())
        }.boxed()
    }
}
