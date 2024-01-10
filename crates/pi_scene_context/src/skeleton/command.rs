

use pi_scene_shell::prelude::*;
use pi_scene_math::Matrix;

pub enum ESkinCreateCommand {
    UBO(ObjectID, ESkinBonesPerVertex, (ObjectID, Vec<ObjectID>)),
    Row(ObjectID, ESkinBonesPerVertex, (ObjectID, Vec<ObjectID>)),
    RowCache(ObjectID, ESkinBonesPerVertex, (ObjectID, Vec<ObjectID>, Vec<Vec<u8>>)),
    Frames(ObjectID, ESkinBonesPerVertex, (ObjectID, Vec<ObjectID>, Vec<Vec<u8>>)),
}

pub struct OpsSkinCreation(pub(crate) Entity, pub(crate) ESkinBonesPerVertex, pub(crate) (ObjectID, Vec<ObjectID>), pub(crate) u16, pub(crate) Option<BindBufferRange>);
impl OpsSkinCreation {
    pub fn ops(skin: Entity, state: ESkinBonesPerVertex, rootbone: Entity, bones: &[Entity], cache_frames: u16, cachedata: Option<BindBufferRange>) -> Self {
        Self(skin, state, (rootbone, bones.to_vec()), cache_frames, cachedata)
    }
}
pub type ActionListSkinCreate = ActionList<OpsSkinCreation>;

pub enum OpsSkinUse {
    Use(Entity, Entity),
    UnUse(Entity, Entity),
}
impl OpsSkinUse {
    pub fn ops(id_mesh: Entity, skin: Entity) -> Self {
        Self::Use(id_mesh, skin)
    }
    pub fn ops_unuse(id_mesh: Entity, skin: Entity) -> Self {
        Self::UnUse(id_mesh, skin)
    }
}
pub type ActionListSkinUse = ActionList<OpsSkinUse>;

pub struct OpsBoneCreation(pub(crate) Entity, pub(crate) Entity, pub(crate) Entity);
impl OpsBoneCreation {
    pub fn ops(bone: Entity, parent: Entity, scene: Entity) -> Self {
        Self(bone, parent, scene)
    }
}
pub type ActionListBoneCreate = ActionList<OpsBoneCreation>;

pub struct OpsBonePose(pub(crate) Entity, pub(crate) Matrix);
impl OpsBonePose {
    pub fn ops(bone: Entity, basematrix: Matrix) -> Self {
        Self(bone, basematrix)
    }
}
pub type ActionListBonePose = ActionList<OpsBonePose>;
