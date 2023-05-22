use std::{mem::replace};

use pi_engine_shell::prelude::*;
use pi_scene_math::Matrix;

use crate::{
    scene::command_sys::*,
    animation::command_sys::*,
    transforms::{prelude::*, command_sys::*},
};

use super::{
    command::*,
    skeleton::*,
    bone::*,
};

pub fn sys_act_skin_create(
    mut cmds: ResMut<ActionListSkinCreate>,
    mut commands: Commands,
    device: Res<PiRenderDevice>,
    mut dynbuffer: ResMut<ResBindBufferAllocator>,
) {
    cmds.drain().drain(..).for_each(|OpsSkinCreation(id_skin, bonemode, (root, bones))| {
        let bone_count = bones.len();
        let bonecount = EBoneCount::new(bone_count as u8 + 1);
        let mode = ESkinCode::UBO(bonemode, bonecount);
                
        bones.iter().for_each(|id_bone| {
            ActionBone::modify_skin(&mut commands.entity(id_bone.clone()), id_skin);
        });

        match Skeleton::new(root, bones, mode, &device, &mut dynbuffer ) {
            Some(skeleton) => {
                commands.entity(id_skin)
                    .insert(skeleton)
                    .insert(SkeletonInitBaseMatrix)
                    .insert(SkeletonBonesDirty(true))
                    .insert(SkeletonRefs::default())
                    .insert(DirtySkeletonRefs(false))
                    ;
            },
            None => {

            },
        }
    });
}

pub fn sys_act_skin_use(
    mut cmds: ResMut<ActionListSkinUse>,
    mut skins: Query<(&mut Skeleton, &mut SkeletonRefs, &mut DirtySkeletonRefs)>,
    mut meshes: Query<&mut BindSkinValue>,
) {
    cmds.drain().drain(..).for_each(|ops| {
        match ops {
            OpsSkinUse::Use(entity, skin) => {
                if let (Ok(mut bind), Ok((mut skeleton, mut skeletonrefs, mut flag))) = (meshes.get_mut(entity), skins.get_mut(skin)) {
                    *bind = BindSkinValue(Some(skeleton.bind.clone()));
                    if skeletonrefs.insert(entity) {
                        *flag = DirtySkeletonRefs::default();
                    }
                } else {
                    cmds.push(OpsSkinUse::Use(entity, skin));
                }
            },
            OpsSkinUse::UnUse(entity, skin) => {
                if let Ok((mut skeleton, mut skeletonrefs, mut flag)) = skins.get_mut(skin) {
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

pub fn sys_act_bone_create(
    mut cmds: ResMut<ActionListBoneCreate>,
    mut tree: ResMut<ActionListTransformNodeParent>,
    mut commands: Commands,
    empty: Res<SingleEmptyEntity>,
) {
    cmds.drain().drain(..).for_each(|OpsBoneCreation(bone, parent, scene, name)| {
        let mut bonecmd = commands.entity(bone);
        ActionScene::add_to_scene(&mut bonecmd, &mut tree, scene);
        ActionTransformNode::as_transform_node(&mut bonecmd, name);
        ActionTransformNode::init_for_tree(&mut bonecmd);
        ActionAnime::as_anime_group_target(&mut bonecmd);
        ActionBone::init(&mut bonecmd, &empty, parent);
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
