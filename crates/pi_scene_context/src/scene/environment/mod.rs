
use pi_scene_shell::prelude::*;

pub mod fog;
pub mod ambient_light;
pub mod scene_time;
pub mod sys;
pub mod brdf;
pub mod environment_texture;

#[derive(Component, Default)]
pub struct BindSceneEffect(pub Option<ShaderBindSceneAboutEffect>);
impl BindSceneEffect {
    pub fn new(
        dynbuffer: &mut BindBufferAllocator,
    ) -> Self {
        Self(ShaderBindSceneAboutEffect::new(dynbuffer))
    }
}
