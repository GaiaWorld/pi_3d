use std::sync::Arc;
use pi_render::{rhi::device::RenderDevice, renderer::bind_buffer::{BindBufferAllocator}, render_3d::binds::scene::effect::ShaderBindSceneAboutEffect};

pub mod fog;
pub mod ambient_light;
pub mod scene_time;

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
