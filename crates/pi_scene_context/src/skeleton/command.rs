use std::{mem::replace};

use pi_engine_shell::prelude::*;
use pi_scene_math::Matrix;

use crate::prelude::{ActionScene, ActionTransformNode, ActionAnime, ActionListTransformNodeParent};

use super::{
    skeleton::{Skeleton, BindSkinValue, SkeletonInitBaseMatrix},
    SkeletonID,
    SkeletonBonesDirty, bone::{ActionBone, BoneBaseMatrix}, SkeletonRefs, DirtySkeletonRefs
};


pub enum ESkinCreateCommand {
    UBO(ObjectID, ESkinBonesPerVertex, (ObjectID, Vec<ObjectID>)),
    Row(ObjectID, ESkinBonesPerVertex, (ObjectID, Vec<ObjectID>)),
    RowCache(ObjectID, ESkinBonesPerVertex, (ObjectID, Vec<ObjectID>, Vec<Vec<u8>>)),
    Frames(ObjectID, ESkinBonesPerVertex, (ObjectID, Vec<ObjectID>, Vec<Vec<u8>>)),
}

pub struct OpsSkinCreation(Entity, ESkinBonesPerVertex, (ObjectID, Vec<ObjectID>));
impl OpsSkinCreation {
    pub fn ops(skin: Entity, state: ESkinBonesPerVertex, rootbone: Entity, bones: &[Entity]) -> Self {
        Self(skin, state, (rootbone, bones.to_vec()))
    }
}
pub type ActionListSkinCreate = ActionList<OpsSkinCreation>;
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

pub struct OpsBoneCreation(Entity, Entity, Entity, String);
impl OpsBoneCreation {
    pub fn ops(bone: Entity, parent: Entity, scene: Entity, name: String) -> Self {
        Self(bone, parent, scene, name)
    }
}
pub type ActionListBoneCreate = ActionList<OpsBoneCreation>;
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

pub struct OpsBonePose(Entity, Matrix);
impl OpsBonePose {
    pub fn ops(bone: Entity, basematrix: Matrix) -> Self {
        Self(bone, basematrix)
    }
}
pub type ActionListBonePose = ActionList<OpsBonePose>;
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
