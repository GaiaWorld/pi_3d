

use pi_scene_shell::{add_component, prelude::{pi_world::editor::{self, EntityEditor}, *}};
use pi_scene_math::Matrix;

use crate::{flags::{CullingFlag, Enable, GlobalEnable, RecordEnable}, prelude::{AbsoluteTransform, GlobalMatrix, LocalEulerAngles, LocalMatrix, LocalPosition, LocalRotation, LocalRotationQuaternion, LocalScaling, RecordLocalEulerAngles, RecordLocalPosition, RecordLocalRotationQuaternion, RecordLocalScaling, TransformNodeDirty}, transforms::command_sys::*};

use super::{
    command::*,
    skeleton::*,
    bone::*,
};

pub fn sys_create_skin(
    mut cmds: ResMut<ActionListSkinCreate>,
    mut editor: EntityEditor,
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
                    if editor.contains_entity(id_bone.clone()) {
                        ActionBone::modify_skin(*id_bone, &mut editor, id_skin);
                    }
                });

                if editor.contains_entity(id_skin) {
                    ActionSkeleton::init(id_skin, &mut editor, skeleton);
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
    mut editor: EntityEditor,
) {
    cmds.drain().drain(..).for_each(|ops| {
        match ops {
            OpsSkinUse::Use(entity, skin) => {
                if let (Ok(mut bind), Ok((skeleton, mut skeletonrefs, mut flag))) = (meshes.get_mut(entity), skins.get_mut(skin)) {
                    *bind = BindSkinValue(Some(skeleton.bind.as_ref().unwrap().clone()));
                    // commands.alter(entity,(SkeletonID(skin),));
                    add_component(&mut editor, entity, SkeletonID(skin)).unwrap();
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
    mut editor: EntityEditor,

    empty: Res<SingleEmptyEntity>,
) {
    cmds.drain().drain(..).for_each(|OpsBoneCreation(bone, parent, scene)| {
        let mut bonecmd = if !editor.contains_entity(bone) {
            return;
        };
        ActionBone::init(bone,  &mut editor, &empty, parent, scene);
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
        editor: &mut EntityEditor,
        skeleton: Skeleton,
    ) {
        let components = [
            editor.init_component::<Skeleton>(),
            editor.init_component::<SkeletonInitBaseMatrix>(),
            editor.init_component::<SkeletonBonesDirty>(),
            editor.init_component::<SkeletonRefs>(),
            editor.init_component::<DirtySkeletonRefs>(),
        ];
        editor.add_components(entity, &components);

        *editor.get_component_unchecked_mut_by_id(entity, components[0]) =skeleton;
        *editor.get_component_unchecked_mut_by_id(entity, components[1]) = SkeletonInitBaseMatrix;
        *editor.get_component_unchecked_mut_by_id(entity, components[2]) = SkeletonBonesDirty(true);
        *editor.get_component_unchecked_mut_by_id(entity, components[3]) = SkeletonRefs::default();
        *editor.get_component_unchecked_mut_by_id(entity, components[4]) = DirtySkeletonRefs(false);
        // commands.alter(entity, 
        //     (
        //         skeleton,
        //         SkeletonInitBaseMatrix,
        //         SkeletonBonesDirty(true),
        //         SkeletonRefs::default(),
        //         DirtySkeletonRefs(false)
        //     )
        // );
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
        editor: &mut EntityEditor,
        _empty: &SingleEmptyEntity,
        parent: Entity,
        scene: Entity,
    ) {
        ActionTransformNode::init(entity, editor, scene);
        let components = [
            editor.init_component::<BoneParent>(),
            editor.init_component::<BoneAbsolute>(),
            editor.init_component::<BoneAbsoluteInv>(),
            editor.init_component::<BoneDifferenceMatrix>(),
            editor.init_component::<BoneMatrix>(),
            editor.init_component::<BoneBaseMatrix>(),
        ];
        editor.add_components(entity, &components);

        *editor.get_component_unchecked_mut_by_id(entity, components[0]) = BoneParent(parent);
        *editor.get_component_unchecked_mut_by_id(entity, components[1]) = BoneAbsolute(Matrix::identity());
        *editor.get_component_unchecked_mut_by_id(entity, components[2]) = BoneAbsoluteInv(Matrix::identity());
        *editor.get_component_unchecked_mut_by_id(entity, components[3]) = BoneDifferenceMatrix(Matrix::identity());
        *editor.get_component_unchecked_mut_by_id(entity, components[4]) = BoneMatrix(Matrix::identity());
        *editor.get_component_unchecked_mut_by_id(entity, components[5]) = BoneBaseMatrix(Matrix::identity());
            // .insert(SkeletonID(empty.id()))
     
    }
    // pub(crate) fn modify_pose(
    //     commands: &mut EntityCommands,
    //     pose: Matrix,
    // ) {
    //     commands.insert(BoneBaseMatrix(pose));
    // }
    pub(crate) fn modify_skin(
        entity: Entity,
        editor: &mut EntityEditor,
        id_skin: Entity,
    ) {
        add_component(editor, entity, SkeletonID(id_skin)).unwrap();
        // commands.alter(entity, (SkeletonID(id_skin),));
    }
}