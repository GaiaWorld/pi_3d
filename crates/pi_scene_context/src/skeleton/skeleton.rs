use std::sync::Arc;

use pi_scene_shell::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet, PartialOrd, Ord)]
pub enum StageSkeleton {
    SkinCreate,
    _SkinCreate,
    Command,
    Calc,
}

#[derive(Component, Default)]
pub struct SkeletonInitBaseMatrix;

#[derive(Component)]
pub struct Skeleton {
    pub root: Entity,
    pub bones: Vec<ObjectID>,
    pub mode: ESkinCode,
    pub meshes: Vec<ObjectID>,
    pub bind: Option<Arc<ShaderBindModelAboutSkinValue>>,
}
impl Default for Skeleton {
    fn default() -> Self {
        Self {
            root: Entity::from_bits(0),
            ..Default::default()
        }
    }
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
                bind: Some(Arc::new(bind)),
            })
        } else {
            None
        }
    }
}

#[derive(Clone, Component, Default)]
pub struct BindSkinValue(pub Option<Arc<ShaderBindModelAboutSkinValue>>);


#[derive(Component, Default)]
pub struct SkeletonID(pub Option<ObjectID>);
// impl TEntityRef for SkeletonID {
//     fn id(&self) -> Entity {
//         self.0
//     }
// }

#[derive(Clone, Component, Default)]
pub struct DirtySkeletonRefs(pub bool);

pub type SkeletonRefs = EntityRefInfo<DirtySkeletonRefs>;

#[derive(Component, Default)]
pub struct SkeletonBonesDirty(pub bool);
