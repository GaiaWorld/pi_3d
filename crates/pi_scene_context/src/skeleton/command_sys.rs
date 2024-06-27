

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
    mut skinlinked: Query<&mut SkeletonID>,
    // mut alter: Alter<(), (), SkeletonBundle, ()>,
) {
    // log::error!("Skin Create");
    cmds.drain().drain(..).for_each(|OpsSkinCreation(id_skin, bonemode, (root, bones), cache_frames, cachedata)| {
        let bone_count = bones.len();
        let bonecount = EBoneCount::new(bone_count as u8 + 1);
        let mode = ESkinCode::UBO(bonemode, bonecount, cache_frames);

        match Skeleton::new(root, bones.clone(), mode, &device, &mut dynbuffer, cachedata) {
            Some(skeleton) => {
                bones.iter().for_each(|id_bone| {
                    if let Ok(mut skinlinked) = skinlinked.get_mut(id_bone.clone()) {
                        skinlinked.0 = Some(id_skin);
                    // } else {
                    //     log::error!("Bone No SkeletonID !!!");
                    }
                });

                if let Some(mut cmd) = commands.get_entity(id_skin) {
                    let bundle = ActionSkeleton::init(skeleton);
                    cmd.insert(bundle) ;
                    // alter.alter(id_skin, bundle);
                }
                // log::error!("Skeleton Create Success !!!");
            },
            None => {
                // log::error!("Skeleton Create Fail !!!");
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
    // mut bones: Query<&mut BoneParent>,
    mut skinlinks: Query<&mut SkeletonID>,
    mut bonelinks: Query<&mut BoneLinked>,
) {
    cmds.drain().drain(..).for_each(|ops| {
        match ops {
            OpsSkinUse::Use(entity, skin) => {
                if let (Ok(mut bind), Ok((skeleton, mut skeletonrefs, mut flag))) = (meshes.get_mut(entity), skins.get_mut(skin)) {
                    *bind = BindSkinValue(Some(skeleton.bind.as_ref().unwrap().clone()));
                    if let Ok(mut skinlinked) = skinlinks.get_mut(entity) {
                        skinlinked.0 = Some(skin);
                    }

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
            OpsSkinUse::Bone(bone, parent) => {
                // if let Ok(mut boneparent) = bones.get_mut(bone) {
                //     boneparent.0 = parent;
                // }
            },
            OpsSkinUse::BoneLink(bone, link) => {
                if let Ok(mut bonelink) = bonelinks.get_mut(bone) {
                    bonelink.0 = Some(link);
                }
            },
        }
    });
}

pub fn sys_create_bone(
    mut cmds: ResMut<ActionListBoneCreate>,
    mut commands: Commands,
    empty: Res<SingleEmptyEntity>,
    // mut alter: Alter<(), (), BoneBoundle, ()>,
) {
    // log::error!("Bone Create");
    cmds.drain().drain(..).for_each(|OpsBoneCreation(bone, scene)| {
        let mut bonecmd = if let Some(cmd) = commands.get_entity(bone) {
            cmd
        } else {
            return;
        };
        let bundle = ActionBone::init(&empty, scene);
        bonecmd.insert(bundle);
        // alter.alter(bone, bundle);
        // log::error!("Bone Create Success");
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
            if let Some(idskin) = skeleton.0 {
                if let Ok(mut flag) = skins.get_mut(idskin) {
                    *flag = SkeletonInitBaseMatrix;
                }
            }
        } else {
            cmds.push(OpsBonePose::ops(bone, matrix));
        }
    });
}

pub type SkeletonBundle = (
    Skeleton,
    SkeletonBoneWorldMatrixDirty,
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
            SkeletonBoneWorldMatrixDirty,
            SkeletonInitBaseMatrix,
            SkeletonBonesDirty(true),
            SkeletonRefs::default(),
            DirtySkeletonRefs(false),
        )
    }
}

pub type BoneBoundle = (
    (
        // BoneParent, 
        BoneLinked,
        BoneWorldMatrix, BoneAbsolute, BoneAbsoluteInv, BoneDifferenceMatrix, BoneMatrix, BoneBaseMatrix, SkeletonID
    ),
    // TransformNodeBundle
    BundleEntity,
    SceneID,
    BundleTreeNode,
);

pub struct ActionBone;
impl ActionBone {
    pub fn init(
        _empty: &SingleEmptyEntity,
        scene: Entity,
    ) -> BoneBoundle {
        (
            (
                // BoneParent(scene),
                BoneLinked(None),
                BoneWorldMatrix(Matrix::identity()),
                BoneAbsolute(Matrix::identity()),
                BoneAbsoluteInv(Matrix::identity()),
                BoneDifferenceMatrix(Matrix::identity()),
                BoneMatrix(Matrix::identity()),
                BoneBaseMatrix(Matrix::identity()),
                SkeletonID(None)
            ),
            // ActionTransformNode::init(scene)
            ActionEntity::init(),
            SceneID(scene),
            ActionTransformNode::init_for_tree(),
        )
    }
    // pub(crate) fn modify_pose(
    //     commands: &mut EntityCommands,
    //     pose: Matrix,
    // ) {
    //     commands.insert(BoneBaseMatrix(pose));
    // }
}