use std::{mem::replace};

use pi_assets::mgr::AssetMgr;
use pi_ecs::prelude::{ResMut, Commands, Query, Res, Event};
use pi_ecs_macros::{setup, listen};
use pi_engine_shell::{object::{ObjectID, GameObject}, run_stage::TSystemStageInfo};
use pi_render::{rhi::device::RenderDevice, renderer::{bind_buffer::{BindBufferAllocator}, sampler::SamplerRes}, render_3d::{shader::skin_code::{ESkinBonesPerVertex, EBoneCount, ESkinCode}, binds::model::skin::BindUseSkinValue}};
use pi_share::Share;

use super::{
    skeleton::{Skeleton, BindSkinValue},
    SkeletonID,
    SkeletonBonesDirty
};



pub enum ESkinCreateCommand {
    UBO(ObjectID, ESkinBonesPerVertex, (ObjectID, Vec<ObjectID>)),
    Row(ObjectID, ESkinBonesPerVertex, (ObjectID, Vec<ObjectID>)),
    RowCache(ObjectID, ESkinBonesPerVertex, (ObjectID, Vec<ObjectID>, Vec<Vec<u8>>)),
    Frames(ObjectID, ESkinBonesPerVertex, (ObjectID, Vec<ObjectID>, Vec<Vec<u8>>)),
}

#[derive(Default)]
pub struct SingleSkinCreateCommands(pub Vec<ESkinCreateCommand>);

pub struct SysSkinCreateCommand;
impl TSystemStageInfo for SysSkinCreateCommand {
    
}
#[setup]
impl SysSkinCreateCommand {
    #[system]
    fn cmds(
        mut cmds: ResMut<SingleSkinCreateCommands>,
        mut skeleton_cmd: Commands<GameObject, Skeleton>,
        mut bonedirty_cmd: Commands<GameObject, SkeletonBonesDirty>,
        // mut skeltex_cmd: Commands<GameObject, SkinTexture>,
        mut bone_cmd: Commands<GameObject, SkeletonID>,
        mut dynbuffer: ResMut<BindBufferAllocator>,
        samplerpool: Res<Share<AssetMgr<SamplerRes>>>,
        device: Res<RenderDevice>,
    ) {
        let mut list = replace(&mut cmds.0, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                ESkinCreateCommand::UBO(id_skin, bonemode, (root, bones)) => {
                    let bone_count = bones.len();

                    bones.iter().for_each(|id_bone| {
                        bone_cmd.insert(id_bone.clone(), SkeletonID(id_skin.clone()));
                    });

                    let bonecount = EBoneCount::new(bone_count as u8 + 1);

                    let mode = ESkinCode::UBO(bonemode, bonecount);
                    if let Some(skeleton) = Skeleton::new(
                        root,
                        bones,
                        mode,
                        &device,
                        &mut dynbuffer,
                    ) {
                        skeleton_cmd.insert(id_skin, skeleton);
                        bonedirty_cmd.insert(id_skin, SkeletonBonesDirty(true));
                    }
                },
                ESkinCreateCommand::Row(_, _, _) => todo!(),
                ESkinCreateCommand::RowCache(_, _, _) => todo!(),
                ESkinCreateCommand::Frames(_, _, _) => todo!(),
            }
        });
    }
}


pub enum ESkinModifyCommand {
    Use(ObjectID, ObjectID),
}

#[derive(Default)]
pub struct SingleSkinModifyCommands(pub Vec<ESkinModifyCommand>);

pub struct SysSkinModifyCommand;
impl TSystemStageInfo for SysSkinModifyCommand {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysSkinCreateCommand::key()
        ]
    }
}
#[setup]
impl SysSkinModifyCommand {
    #[listen(entity=(GameObject, Delete))]
    fn listen(
        e: Event,
        meshes: Query<GameObject, (ObjectID, &SkeletonID)>,
        mut skeletons: Query<GameObject, &mut Skeleton>,
    ) {
        if let Some((obj, id_skl)) = meshes.get_by_entity(e.id) {
            if let Some(mut skl) = skeletons.get_mut(id_skl.0) {
                match skl.meshes.binary_search(&obj) {
                    Ok(index) => {
                        let len = skl.meshes.len() - 1;
                        for i in index..len {
                            skl.meshes[i] = skl.meshes[i + 1];
                        }
                        skl.meshes.pop();
                    },
                    Err(_) => todo!(),
                }
            }
        }
    }
    #[system]
    fn cmds(
        mut cmds: ResMut<SingleSkinModifyCommands>,
        mut skeletons: Query<GameObject, &mut Skeleton>,
        mut useskin_cmd: Commands<GameObject, SkeletonID>,
        mut model_cmd: Commands<GameObject, BindSkinValue>,
    ) {
        let mut list = replace(&mut cmds.0, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                ESkinModifyCommand::Use(id_obj, id_skin) => {
                    if let Some(mut skeleton) = skeletons.get_mut(id_skin) {
                        match skeleton.meshes.binary_search(&id_obj) {
                            Ok(_) => {
                                
                            },
                            Err(index) => {
                                skeleton.meshes.insert(index, id_obj);
                            },
                        }
                        useskin_cmd.insert(id_obj, SkeletonID(id_skin));
                        model_cmd.insert(id_obj, BindSkinValue(skeleton.bind.clone()));
                    }
                },
            }
        });
    }
}