use std::sync::Arc;

use pi_engine_shell::object::ObjectID;
use pi_render::{rhi::device::RenderDevice, renderer::bind_buffer::{BindBufferAllocator}, render_3d::{binds::model::skin::ShaderBindModelAboutSkinValue, shader::skin_code::ESkinCode}};


pub struct Skeleton {
    pub root: ObjectID,
    pub bones: Vec<ObjectID>,
    pub mode: ESkinCode,
    pub meshes: Vec<ObjectID>,
    pub bind: Arc<ShaderBindModelAboutSkinValue>,
}
impl Skeleton {
    pub fn new(
        root: ObjectID,
        bones: Vec<ObjectID>,
        mode: ESkinCode,
        device: &RenderDevice,
        dynbuffer: &mut BindBufferAllocator,
    ) -> Option<Self> {
        if let Some(bind) = ShaderBindModelAboutSkinValue::new(&mode, device, dynbuffer) {
            Some(Self {
                root,
                bones,
                mode,
                meshes: vec![],
                bind: Arc::new(bind),
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct BindSkinValue(pub Arc<ShaderBindModelAboutSkinValue>);