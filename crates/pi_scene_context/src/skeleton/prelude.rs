use pi_scene_shell::prelude::*;

pub use super::{
    command::*,
    skeleton::*,
    bone::*,
};

#[derive(SystemParam)]
pub struct ActionSetSkeleton<'w> {
    pub bone_create: ResMut<'w, ActionListBoneCreate>,
    pub bone_pose: ResMut<'w, ActionListBonePose>,
    pub skin_create: ResMut<'w, ActionListSkinCreate>,
    pub skin_use: ResMut<'w, ActionListSkinUse>,
}
impl<'w> MemSize for ActionSetSkeleton<'w> {
    fn memsize(&self) -> usize {
        self.bone_create.memsize()
        + self.bone_pose.memsize()
        + self.skin_create.memsize()
        + self.skin_use.memsize()
    }
}