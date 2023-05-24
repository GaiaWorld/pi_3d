use pi_engine_shell::prelude::*;

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