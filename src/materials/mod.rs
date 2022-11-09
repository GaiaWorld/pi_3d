use pi_ecs::prelude::{Setup, ResMut, Res};
use pi_ecs_macros::setup;
use pi_render::rhi::{device::RenderDevice, RenderQueue};

use crate::{plugin::Plugin, resources::RenderDynUniformBuffer};

use self::{material::{SingleMaterialIDCommandList, SysMaterialIDCommand}, bind_group::{SysRenderBindGroupCommand, SingleRenderBindGroupCommandList}};

pub mod material;
pub mod bind_group;

pub type MBKK = usize;

#[derive(Debug, Default)]
pub struct SingleDynUnifromBufferReBindFlag(pub bool);

pub struct SysDynUnifromBufferUpdate;
#[setup]
impl SysDynUnifromBufferUpdate {
    #[system]
    pub fn tick(
        device: Res<RenderDevice>,
        queue: Res<RenderQueue>,
        mut dynbuffer: ResMut<RenderDynUniformBuffer>,
        mut flag: ResMut<SingleDynUnifromBufferReBindFlag>,
    ) {
        //  println!("SysDynUnifromBuffer Update");
        flag.0 = dynbuffer.write_buffer(&device, &queue);
    }
}
// pub struct SysDynUnifromBufferReBindFlag;
// #[setup]
// impl SysDynUnifromBufferReBindFlag {
//     #[system]
//     pub fn tick(
//         dynbuffer: Res<RenderDynUniformBuffer>,
//         mut flag: ResMut<SingleDynUnifromBufferReBindFlag>,
//     ) {
//         flag.0 = false;
//     }
// }

pub struct PluginMaterialID;
impl Plugin for PluginMaterialID {
    fn init(
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        SysMaterialIDCommand::setup(world, stages.command_stage());
        SysRenderBindGroupCommand::setup(world, stages.command_stage());
        SysDynUnifromBufferUpdate::setup(world, stages.between_uniform_update_and_filter_culling());
        // SysDynUnifromBufferReBindFlag::setup(world, stages.between_uniform_update_and_filter_culling());

        world.insert_resource(SingleMaterialIDCommandList::default());
        world.insert_resource(SingleDynUnifromBufferReBindFlag::default());
        world.insert_resource(SingleRenderBindGroupCommandList::default());

        Ok(())
    }
}