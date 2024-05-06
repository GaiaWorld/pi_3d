use std::sync::Arc;

use pi_scene_shell::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash,  PartialOrd, Ord)]
pub enum StageSkeleton {
    SkinCreate,
    _SkinCreate,
    Command,
    Calc,
}


pub struct SkeletonInitBaseMatrix;


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
        cache: Option<BindBufferRange>,
    ) -> Option<Self> {
        if let Some(bind) = ShaderBindModelAboutSkinValue::new(&mode, device, dynbuffer, cache) {
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

#[derive(Clone, )]
pub struct BindSkinValue(pub Option<Arc<ShaderBindModelAboutSkinValue>>);



pub struct SkeletonID(pub ObjectID);
impl TEntityRef for SkeletonID {
    fn id(&self) -> Entity {
        self.0
    }
}

#[derive(Default, Clone, )]
pub struct DirtySkeletonRefs(pub bool);

pub type SkeletonRefs = EntityRefInfo<DirtySkeletonRefs>;


pub struct SkeletonBonesDirty(pub bool);
