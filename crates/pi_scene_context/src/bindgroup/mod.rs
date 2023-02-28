
use pi_assets::{mgr::AssetMgr, asset::GarbageEmpty};
use pi_engine_shell::{plugin::Plugin};
use pi_render::{rhi::{device::RenderDevice, bind_group::BindGroup}, renderer::{bind_buffer::{BindBufferAllocator}, bind_group::BindGroupLayout}};

use crate::object::{ObjectID};


pub struct PluginRenderBindGroup;
impl Plugin for PluginRenderBindGroup {
    fn init(
        &mut self,
        engine: &mut pi_engine_shell::engine_shell::EnginShell,
        stages: &mut pi_engine_shell::run_stage::RunStage,
    ) -> Result<(), pi_engine_shell::plugin::ErrorPlugin> {
        let world = engine.world_mut();
        let device = world.get_resource::<RenderDevice>().unwrap();
        let allocator = BindBufferAllocator::new(device);
        
        let world = engine.world_mut();
        world.insert_resource(allocator);
        
        // log::info!("{:?}", device.limits());
        world.insert_resource(AssetMgr::<BindGroup>::new(GarbageEmpty(), false, 2 * 1024 * 1024, 60 * 1000));
        world.insert_resource(AssetMgr::<BindGroupLayout>::new(GarbageEmpty(), false, 2 * 1024 * 1024, 60 * 1000));

        Ok(())
    }
}
