use pi_scene_shell::prelude::*;

pub struct OpsSpriteCreate(pub(crate) Entity, pub(crate) Entity, pub(crate) u64);
impl OpsSpriteCreate {
    pub fn ops(mesh: Entity, sprite: Entity, atlas: u64) -> Self {
        Self(mesh, sprite, atlas)
    }
}
pub type ActionListSpriteCreate = ActionList<OpsSpriteCreate>;

pub enum SpriteModify{
    Idx(IdxTextureFrame),
    Data([u16;14]),
}
pub struct OpsSpriteModify(pub(crate) Entity, pub(crate) SpriteModify);
impl OpsSpriteModify {
    pub fn ops(sprite: Entity, frame: SpriteModify) -> Self {
        Self(sprite, frame)
    }
}
pub type ActionListSpriteModify = ActionList<OpsSpriteModify>;