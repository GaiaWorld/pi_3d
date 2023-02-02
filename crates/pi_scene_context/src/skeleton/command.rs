use std::{mem::replace, sync::Arc};

use pi_ecs::prelude::{ResMut, Commands, Query, EntityDelete, Res};
use pi_ecs_macros::setup;
use pi_engine_shell::{object::{ObjectID, GameObject}, run_stage::TSystemStageInfo};
use pi_render::rhi::device::RenderDevice;
use render_resource::{sampler::SamplerPool, uniform_buffer::RenderDynUniformBuffer};
use render_shader::{skin_code::{ESkinBonesPerVertex, ESkinCode, EBoneCount}, shader_bind::ShaderBindModelAboutSkin};

use super::{skeleton::Skeleton, SkeletonID, skin_texture::SkinTexture, bone::BoneMatrix, SkeletonBonesDirty};



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
        mut skeltex_cmd: Commands<GameObject, SkinTexture>,
        mut bone_cmd: Commands<GameObject, SkeletonID>,
        device: Res<RenderDevice>,
        mut samplerpool: ResMut<SamplerPool>,
        mut dynbuffer: ResMut<RenderDynUniformBuffer>,
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
                    let bind = ShaderBindModelAboutSkin::new_ubo(bonecount, &mut dynbuffer);
                    let skin = Skeleton {
                        root,
                        bones,
                        mode,
                        meshes: vec![],
                        bind: Arc::new(bind)
                    };
                    skeleton_cmd.insert(id_skin, skin);
                    bonedirty_cmd.insert(id_skin, SkeletonBonesDirty(true));
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
    Destroy(ObjectID),
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
    #[system]
    fn cmds(
        mut cmds: ResMut<SingleSkinModifyCommands>,
        mut skeletons: Query<GameObject, &mut Skeleton>,
        mut useskin_cmd: Commands<GameObject, SkeletonID>,
        mut delete: EntityDelete<GameObject>,
    ) {
        let mut list = replace(&mut cmds.0, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                ESkinModifyCommand::Use(id_obj, id_skin) => {
                    if let Some(mut skeleton) = skeletons.get_mut(id_skin) {
                        if skeleton.meshes.contains(&id_obj) == false {
                            skeleton.meshes.push(id_obj);
                        }
                        useskin_cmd.insert(id_obj, SkeletonID(id_skin));
                    }
                },
                ESkinModifyCommand::Destroy(id_skin) => {
                    delete.despawn(id_skin);
                },
            }
        });
    }
}