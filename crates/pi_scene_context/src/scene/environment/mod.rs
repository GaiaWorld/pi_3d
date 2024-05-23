use std::sync::Arc;
use pi_scene_shell::prelude::*;

pub mod fog;
pub mod ambient_light;
pub mod scene_time;
pub mod sys;
pub mod brdf;
pub mod environment_texture;

#[derive(Component)]
pub struct BindSceneEffect(pub Arc<ShaderBindSceneAboutEffect>);
impl BindSceneEffect {
    pub fn new(
        dynbuffer: &mut BindBufferAllocator,
    ) -> Option<Self> {
        if let Some(data) = ShaderBindSceneAboutEffect::new(dynbuffer) {
            Some(
                Self(
                    Arc::new(data)
                )
            )
        } else {
            None
        }
    }
}
