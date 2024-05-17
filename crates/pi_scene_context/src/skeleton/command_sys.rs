

use pi_scene_shell::prelude::*;
use pi_scene_math::Matrix;

use crate::transforms::command_sys::*;

use super::{
    command::*,
    skeleton::*,
    bone::*,
};

pub fn sys_create_skin(
    mut cmds: ResMut<ActionListSkinCreate>,
    mut commands: Commands,
    device: Res<PiRenderDevice>,
    mut dynbuffer: ResMut<ResBindBufferAllocator>,
    mut _disposereadylist: ResMut<ActionListDisposeReadyForRef>,
    mut disposecanlist: ResMut<ActionListDisposeCan>,
) {
    cmds.drain().drain(..).for_each(|OpsSkinCreation(id_skin, bonemode, (root, bones), cache_frames, cachedata)| {
        let bone_count = bones.len();
        let bonecount = EBoneCount::new(bone_count as u8 + 1);
        let mode = ESkinCode::UBO(bonemode, bonecount, cache_frames);

        match Skeleton::new(root, bones.clone(), mode, &device, &mut dynbuffer, cachedata) {
            Some(skeleton) => {
                bones.iter().for_each(|id_bone| {
                    if let Some(mut cmd) = commands.get_entity(id_bone.clone()) {
                        cmd.insert(SkeletonID(id_skin));
                    }
                });

                if let Some(mut cmd) = commands.get_entity(id_skin) {
                    cmd.insert(ActionSkeleton::init(skeleton)) ;
                }
            },
            None => {
                bones.iter().for_each(|entity| {
                    disposecanlist.push(OpsDisposeCan::ops(*entity));
                });
                disposecanlist.push(OpsDisposeCan::ops(id_skin));
            },
        }
    });
}

pub fn sys_act_skin_use(
    mut cmds: ResMut<ActionListSkinUse>,
    mut skins: Query<(&mut Skeleton, &mut SkeletonRefs, &mut DirtySkeletonRefs)>,
    mut meshes: Query<&mut BindSkinValue>,
    mut commands: Commands,
) {
    cmds.drain().drain(..).for_each(|ops| {
        match ops {
            OpsSkinUse::Use(entity, skin) => {
                if let (Ok(mut bind), Ok((skeleton, mut skeletonrefs, mut flag))) = (meshes.get_mut(entity), skins.get_mut(skin)) {
                    *bind = BindSkinValue(Some(skeleton.bind.clone()));
                    commands.entity(entity).insert(SkeletonID(skin));
                    // log::warn!("Skinn OKKKKKKKKKKKK");
                    if skeletonrefs.insert(entity) {
                        *flag = DirtySkeletonRefs::default();
                    }
                } else {
                    cmds.push(OpsSkinUse::Use(entity, skin));
                }
            },
            OpsSkinUse::UnUse(entity, skin) => {
                if let Ok((_, mut skeletonrefs, mut flag)) = skins.get_mut(skin) {
                    if skeletonrefs.remove(&entity) && skeletonrefs.is_empty() {
                        *flag = DirtySkeletonRefs::default();
                    }
                } else {
                    cmds.push(OpsSkinUse::UnUse(entity, skin));
                }
            },
        }
    });
}

pub fn sys_create_bone(
    mut cmds: ResMut<ActionListBoneCreate>,
    mut commands: Commands,
    empty: Res<SingleEmptyEntity>,
) {
    cmds.drain().drain(..).for_each(|OpsBoneCreation(bone, parent, scene)| {
        let mut bonecmd = if let Some(cmd) = commands.get_entity(bone) {
            cmd
        } else {
            return;
        };
        bonecmd.insert(ActionBone::init(&empty, parent, scene));
    });
}

pub fn sys_act_bone_pose(
    mut cmds: ResMut<ActionListBonePose>,
    mut skins: Query<&mut SkeletonInitBaseMatrix>,
    mut bones: Query<(&SkeletonID, &mut BoneBaseMatrix)>,
) {
    cmds.drain().drain(..).for_each(|OpsBonePose(bone, matrix)| {
        if let Ok((skeleton, mut basematrix)) = bones.get_mut(bone) {
            *basematrix = BoneBaseMatrix(matrix);
            if let Ok(mut flag) = skins.get_mut(skeleton.0) {
                *flag = SkeletonInitBaseMatrix;
            }
        } else {
            cmds.push(OpsBonePose::ops(bone, matrix));
        }
    });
}

pub type SkeletonBundle = (
    Skeleton,
    SkeletonInitBaseMatrix,
    SkeletonBonesDirty,
    SkeletonRefs,
    DirtySkeletonRefs,
);

pub struct ActionSkeleton;
impl ActionSkeleton {
    pub fn init(
        skeleton: Skeleton,
    ) -> SkeletonBundle {
        (
            skeleton,
            SkeletonInitBaseMatrix,
            SkeletonBonesDirty(true),
            SkeletonRefs::default(),
            DirtySkeletonRefs(false),
        )
    }
}

pub type BoneBoundle = (
    (
        BoneParent, BoneAbsolute, BoneAbsoluteInv, BoneDifferenceMatrix, BoneMatrix, BoneBaseMatrix
    ),
    TransformNodeBundle
);

pub struct ActionBone;
impl ActionBone {
    pub fn init(
        _empty: &SingleEmptyEntity,
        parent: Entity,
        scene: Entity,
    ) -> BoneBoundle {
        (
            (
                BoneParent(parent),
                BoneAbsolute(Matrix::identity()),
                BoneAbsoluteInv(Matrix::identity()),
                BoneDifferenceMatrix(Matrix::identity()),
                BoneMatrix(Matrix::identity()),
                BoneBaseMatrix(Matrix::identity()),
                // .insert(SkeletonID(empty.id()))
            ),
            ActionTransformNode::init(scene)
        )
    }
    // pub(crate) fn modify_pose(
    //     commands: &mut EntityCommands,
    //     pose: Matrix,
    // ) {
    //     commands.insert(BoneBaseMatrix(pose));
    // }
}