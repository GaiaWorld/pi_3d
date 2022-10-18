use pi_ecs::prelude::{Query, ResMut};
use pi_ecs_macros::setup;
use pi_render::rhi::dyn_uniform_buffer::DynUniformBuffer;

use crate::{object::GameObject, cameras::camera::{CameraRenderData}};



pub struct CameraUniformTickUpdate;
#[setup]
impl CameraUniformTickUpdate {
    #[system]
    pub fn tick(
        query_cameras: Query<GameObject, &CameraRenderData>,
        mut dynbuffer: ResMut<DynUniformBuffer>,
    ) {
        println!("Camera Uniform Tick Update");
        query_cameras.iter().for_each(|camera| {
            dynbuffer.set_uniform::<CameraRenderData>(&camera.bind_offset, camera);
        });
    }
}