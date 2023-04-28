use std::sync::Arc;

use pi_engine_shell::prelude::*;

#[derive(Component)]
pub struct SkeletonInitBaseMatrix;

#[derive(Component)]
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

#[derive(Debug, Clone, Component)]
pub struct BindSkinValue(pub Option<Arc<ShaderBindModelAboutSkinValue>>);