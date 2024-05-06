

use pi_scene_shell::prelude::*;
use pi_scene_math::Matrix;

use crate::{flags::{CullingFlag, Enable, GlobalEnable, RecordEnable}, prelude::{AbsoluteTransform, GlobalMatrix, LocalEulerAngles, LocalMatrix, LocalPosition, LocalRotation, LocalRotationQuaternion, LocalScaling, RecordLocalEulerAngles, RecordLocalPosition, RecordLocalRotationQuaternion, RecordLocalScaling, TransformNodeDirty}, transforms::command_sys::*};

use super::{
    command::*,
    skeleton::*,
    bone::*,
};

pub fn sys_create_skin(
    mut cmds: ResMut<ActionListSkinCreate>,
    mut commands: Alter<(), (), (SkeletonID,), ()>,
    mut commands1: Alter<
        (), 
        (), 
        (
            Skeleton,
            SkeletonInitBaseMatrix,
            SkeletonBonesDirty,
            SkeletonRefs,
            DirtySkeletonRefs
        ), 
        ()>,
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
                    if commands.get(id_bone.clone()).is_ok() {
                        ActionBone::modify_skin(*id_bone, &mut commands, id_skin);
                    }
                });

                if commands1.get(id_skin).is_ok() {
                    ActionSkeleton::init(id_skin, &mut commands1, skeleton);
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
    mut commands: Alter<(), (), (SkeletonID, )>,
) {
    cmds.drain().drain(..).for_each(|ops| {
        match ops {
            OpsSkinUse::Use(entity, skin) => {
                if let (Ok(mut bind), Ok((skeleton, mut skeletonrefs, mut flag))) = (meshes.get_mut(entity), skins.get_mut(skin)) {
                    *bind = BindSkinValue(Some(skeleton.bind.clone()));
                    commands.alter(entity,(SkeletonID(skin),));
                    
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
    // mut commands: Commands,
    mut alter1: Alter<(), (), (DisposeReady, DisposeCan)>,
    mut alter2: Alter<(), (), (SceneID,)>,
    mut alter3: Alter<(), (), (Down, Up, Layer, Enable, RecordEnable, GlobalEnable)>,
    mut alter4: Alter<(), (), ActionTransformNodeBundle>,
    mut alter5: Alter<(), (), ActionBoneBundle>,
    empty: Res<SingleEmptyEntity>,
) {
    cmds.drain().drain(..).for_each(|OpsBoneCreation(bone, parent, scene)| {
        let mut bonecmd = if alter1.get(bone).is_err() {
            return;
        };
        ActionBone::init(bone,  &mut alter1, &mut alter2, &mut alter3, &mut alter4, &mut alter5,  &empty, parent, scene);
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

pub struct ActionSkeleton;
impl ActionSkeleton {
    pub fn init(
        entity: Entity,
        commands: &mut Alter<
        (), 
        (), 
        (
            Skeleton,
            SkeletonInitBaseMatrix,
            SkeletonBonesDirty,
            SkeletonRefs,
            DirtySkeletonRefs
        ), 
        ()>,
        skeleton: Skeleton,
    ) {
        commands.alter(entity, 
            (
                skeleton,
                SkeletonInitBaseMatrix,
                SkeletonBonesDirty(true),
                SkeletonRefs::default(),
                DirtySkeletonRefs(false)
            )
        );
    }
}

pub type ActionBoneBundle  = (
    BoneParent,
    BoneAbsolute,
    BoneAbsoluteInv,
    BoneDifferenceMatrix,
    BoneMatrix,
    BoneBaseMatrix
);
pub struct ActionBone;
impl ActionBone {
    pub fn init(
        entity: Entity,
        alter1: &mut Alter<(), (), (DisposeReady, DisposeCan)>,
        alter2: &mut Alter<(), (), (SceneID,)>,
        alter3: &mut Alter<(), (), (Down, Up, Layer, Enable, RecordEnable, GlobalEnable)>,
        alter4: &mut Alter<(), (), ActionTransformNodeBundle>,
        alter5: &mut Alter<(), (), ActionBoneBundle>,
        _empty: &SingleEmptyEntity,
        parent: Entity,
        scene: Entity,
    ) {
        ActionTransformNode::init(entity, alter1, alter2, alter3, alter4, scene);

        alter5.alter(entity, 
            (BoneParent(parent),
            BoneAbsolute(Matrix::identity()),
            BoneAbsoluteInv(Matrix::identity()),
            BoneDifferenceMatrix(Matrix::identity()),
            BoneMatrix(Matrix::identity()),
            BoneBaseMatrix(Matrix::identity()))
            // .insert(SkeletonID(empty.id()))
        );
    }
    // pub(crate) fn modify_pose(
    //     commands: &mut EntityCommands,
    //     pose: Matrix,
    // ) {
    //     commands.insert(BoneBaseMatrix(pose));
    // }
    pub(crate) fn modify_skin(
        entity: Entity,
        commands: &mut Alter<(), (), (SkeletonID,), ()>,
        id_skin: Entity,
    ) {
        commands.alter(entity, (SkeletonID(id_skin),));
    }
}