use pi_ecs::prelude::{Query, ResMut};
use pi_ecs_macros::setup;
use pi_render::rhi::dyn_uniform_buffer::DynUniformBuffer;

use crate::{object::GameObject, cameras::camera::{ProjectionMatrix, ViewMatrix}, shaders::buildin_uniforms::BuildinCameraBind};



pub struct CameraUniformTickUpdate;
#[setup]
impl CameraUniformTickUpdate {
    #[system]
    pub fn tick(
        query_cameras: Query<GameObject, (&ViewMatrix, &ProjectionMatrix, &BuildinCameraBind)>,
        dynbuffer: ResMut<DynUniformBuffer>,
    ) {
        query_cameras.iter().for_each(|(view, projection, bind)| {
            dynbuffer.set_uniform::<ViewMatrix>(&bind.bind_offset, view);
            dynbuffer.set_uniform::<ProjectionMatrix>(&bind.bind_offset, projection);
        });
    }
}