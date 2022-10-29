use pi_render::rhi::{device::RenderDevice};

use crate::{plugin::Plugin};

mod bind_group_pool;
mod dyn_uniform_buffer;
mod vertex_buffer_pool;
mod pipeline;

pub use bind_group_pool::*;
pub use dyn_uniform_buffer::*;
pub use vertex_buffer_pool::*;
pub use pipeline::*;

pub struct PluginResource;
impl Plugin for PluginResource {
    fn init(
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        world.insert_resource(SingleRenderBindGroupPool::default());
        world.insert_resource(SingleRenderObjectPipelinePool::default());
        world.insert_resource(SingleGeometryBufferPool::default());

        let device = world.get_resource::<RenderDevice>().unwrap().clone();
        world.insert_resource(RenderDynUniformBuffer::new(&device));

        Ok(())
    }
}

