use pi_ecs::prelude::{Setup, ResMut, Res};
use pi_ecs_macros::setup;
use pi_render::rhi::{device::RenderDevice, RenderQueue};

use crate::resources::RenderDynUniformBuffer;


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