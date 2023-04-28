use std::sync::Arc;
use pi_engine_shell::prelude::*;

pub mod fog;
pub mod ambient_light;
pub mod scene_time;

#[derive(Component)]
pub struct BindSceneEffect(pub Arc<ShaderBindSceneAboutEffect>);
impl BindSceneEffect {
    pub fn new(
        device: &RenderDevice,
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
