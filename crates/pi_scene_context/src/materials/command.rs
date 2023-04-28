use std::{mem::replace};

use pi_engine_shell::prelude::*;

use crate::{pass::EPassTag, animation::command, prelude::*};

use super::{
    material::{MaterialID, MaterialRefs, DirtyMaterialRefs},
    shader_effect::{AssetKeyShaderEffect, AssetResShaderEffectMeta},
    uniforms::{
        texture::{UniformTextureWithSamplerParams}, uniform::*,
    }
};

pub struct OpsMaterialCreate(pub Entity, pub KeyShaderMeta, pub EPassTag);
impl OpsMaterialCreate {
    pub fn ops(mat: Entity, shader_meta: &str, pass: EPassTag) -> Self {
        Self(mat, Atom::from(shader_meta), pass)
    }
}
pub type ActionListMaterialCreate = ActionList<OpsMaterialCreate>;
pub fn sys_act_material_create(
    mut cmds: ResMut<ActionListMaterialCreate>,
    mut commands: Commands,
) {
    cmds.drain().drain(..).for_each(|OpsMaterialCreate(entity, key_shader, passtag)| {
        let mut matcmds = commands.entity(entity);

        matcmds
            .insert(AssetKeyShaderEffect(key_shader))
            .insert(MaterialRefs::default())
            .insert(BindEffect(None))
            .insert(BindEffectValueDirty(false))
            .insert(passtag)
            .insert(UniformTextureWithSamplerParams::default())
            .insert(DirtyMaterialRefs::default());
    });
}

#[derive(Debug)]
pub enum OpsMaterialUse {
    Use(Entity, Entity),
    UnUse(Entity, Entity),
}
impl OpsMaterialUse {
    pub fn ops(id_mesh: Entity, id_mat: Entity) -> Self {
        Self::Use(id_mesh, id_mat)
    }
}

pub type ActionListMaterialUse = ActionList<OpsMaterialUse>;
pub fn sys_act_material_use(
    mut cmds: ResMut<ActionListMaterialUse>,
    mut materials: Query<(&mut MaterialRefs, &mut DirtyMaterialRefs, &EPassTag)>,
    meshes: Query<(&PassID01, &PassID02, &PassID03, &PassID04, &PassID05, &PassID06, &PassID07, &PassID08)>,
    mut targets: Query<&mut MaterialID>,
    empty: Res<SingleEmptyEntity>,
) {
    cmds.drain().drain(..).for_each(|cmd| {
        match cmd {
            OpsMaterialUse::Use(id_mesh, id_mat) => {
                if let Ok((mut materialrefs, mut flag, pass)) = materials.get_mut(id_mat) {
                    if let Ok(mut matid) = targets.get_mut(id_mesh) {
                        if matid.0 != id_mat {
                            let old = matid.0;
                            // use
                            if materialrefs.insert(id_mesh) {
                                *flag = DirtyMaterialRefs::default();
                            }
                            *matid = MaterialID(id_mat);
                            
                            // unuse
                            if let Ok((mut materialrefs, mut flag, pass)) = materials.get_mut(old) {
                                if materialrefs.remove(&id_mesh) {
                                    *flag = DirtyMaterialRefs::default();
                                }
                            }
                        }
                    } else if let Ok(passid) = meshes.get(id_mesh) {
                        let pass = pass.as_pass();
                        let id_mesh = if pass == EPassTag::PASS_TAG_01 { passid.0.0 }
                        else if pass == EPassTag::PASS_TAG_02 { passid.1.0 }
                        else if pass == EPassTag::PASS_TAG_03 { passid.2.0 }
                        else if pass == EPassTag::PASS_TAG_04 { passid.3.0 }
                        else if pass == EPassTag::PASS_TAG_05 { passid.4.0 }
                        else if pass == EPassTag::PASS_TAG_06 { passid.5.0 }
                        else if pass == EPassTag::PASS_TAG_07 { passid.6.0 }
                        else { passid.7.0 };
                        if let Ok(mut matid) = targets.get_mut(id_mesh) {
                            if matid.0 != id_mat {
                                let old = matid.0;
                                // use
                                if materialrefs.insert(id_mesh) {
                                    *flag = DirtyMaterialRefs::default();
                                }
                                *matid = MaterialID(id_mat);
                                
                                // unuse
                                if let Ok((mut materialrefs, mut flag, pass)) = materials.get_mut(old) {
                                    if materialrefs.remove(&id_mesh) {
                                        *flag = DirtyMaterialRefs::default();
                                    }
                                }
                            }
                        } else {
                            cmds.push(OpsMaterialUse::Use(id_mesh, id_mat));
                        }
                    } else {
                        cmds.push(OpsMaterialUse::Use(id_mesh, id_mat));
                    }
                } else {
                    cmds.push(OpsMaterialUse::Use(id_mesh, id_mat));
                }
            },
            OpsMaterialUse::UnUse(id_mesh, id_mat) => {
                if let Ok(mut matid) = targets.get_mut(id_mesh) {
                    let old = matid.0;
                    *matid = MaterialID(empty.id());
                    // unuse
                    if let Ok((mut materialrefs, mut flag, pass)) = materials.get_mut(old) {
                        if materialrefs.remove(&id_mesh) {
                            *flag = DirtyMaterialRefs::default();
                        }
                    } else {
                        cmds.push(OpsMaterialUse::UnUse(id_mesh, id_mat));
                    }
                // } else if let Ok(passid) = meshes.get(id_mesh) {
                //     let pass = pass.as_pass();
                //     let id_mesh = if pass == EPassTag::PASS_TAG_01 { passid.0.0 }
                //     else if pass == EPassTag::PASS_TAG_02 { passid.1.0 }
                //     else if pass == EPassTag::PASS_TAG_03 { passid.2.0 }
                //     else if pass == EPassTag::PASS_TAG_04 { passid.3.0 }
                //     else if pass == EPassTag::PASS_TAG_05 { passid.4.0 }
                //     else if pass == EPassTag::PASS_TAG_06 { passid.5.0 }
                //     else if pass == EPassTag::PASS_TAG_07 { passid.6.0 }
                //     else { passid.7.0 };
                //     if let Ok(mut matid) = targets.get_mut(id_mesh) {
                //         let old = matid.0;
                //         *matid = MaterialID(empty.id());
                //         // unuse
                //         if let Ok((mut materialrefs, mut flag, pass)) = materials.get_mut(old) {
                //             if materialrefs.remove(&id_mesh) {
                //                 *flag = DirtyMaterialRefs::default();
                //             }
                //         }
                //     }
                } else {
                    cmds.push(OpsMaterialUse::UnUse(id_mesh, id_mat));
                }
            },
        }
    });
}

pub struct ActionMaterial;
impl ActionMaterial {
    pub fn regist_material_meta(
        asset_mgr: &ShareAssetMgr<ShaderEffectMeta>,
        wait_list: &mut AssetSyncWait<KeyShaderMeta, AssetKeyShaderEffect, ShaderEffectMeta, AssetResShaderEffectMeta>,
        key: KeyShaderMeta,
        meta: ShaderEffectMeta,
    ) {
        if !asset_mgr.contains_key(&key) {
            if let Ok(meta) = asset_mgr.insert(key.clone(), meta) {
                wait_list.1.push((key.clone(), meta));
            }
            // let meta = asset_mgr.insert(key.clone(), meta);
            // wait_list.1.push((key.clone(), meta));
        }
    }
    pub fn init(
        app: &mut App,
        mat: Entity,
        key: KeyShaderMeta,
        passtag: EPassTag,
    ) {
        let mut cmds = app.world.get_resource_mut::<ActionListMaterialCreate>().unwrap();
        cmds.push(OpsMaterialCreate(mat, key, passtag));
    }
    
    pub fn use_material(
        app: &mut App,
        cmd: OpsMaterialUse,
    ) {
        let mut cmds = app.world.get_resource_mut::<ActionListMaterialUse>().unwrap();
        cmds.push(cmd);
    }
}



// #[derive(Debug)]
// pub enum EMatCreateCommand {
//     Use(ObjectID, KeyShaderMeta, EPassTag),
// }
// #[derive(Debug, Default)]
// pub struct SingleMatCreateCommands(pub Vec<EMatCreateCommand>);

// pub struct SysMaterailCreateCommands;
// impl TSystemStageInfo for SysMaterailCreateCommands {
    
// }
// #[setup]
// impl SysMaterailCreateCommands {
//     #[system]
//     pub fn cmds(
//         mut cmds: ResMut<SingleMatCreateCommands>,
//         mut items: Commands<GameObject, AssetKeyShaderEffect>,
//         mut usedlist_cmd: Commands<GameObject, MaterialUsedList>,
//         mut dirty_cmd: Commands<GameObject, DirtyMaterialUsedList>,
//         mut passtag_cmd: Commands<GameObject, EPassTag>,
//         mut texparams_cmd: Commands<GameObject, UniformTextureWithSamplerParams>,
//     ) {
//         let mut list = replace(&mut cmds.0, vec![]);

//         list.drain(..).for_each(|cmd| {
//             match cmd {
//                 EMatCreateCommand::Use(entity, key, passtag) => {
//                     items.insert(entity, AssetKeyShaderEffect(key));
//                     usedlist_cmd.insert(entity, MaterialUsedList::default());
//                     passtag_cmd.insert(entity, passtag);
//                     texparams_cmd.insert(entity, UniformTextureWithSamplerParams::default());
//                     dirty_cmd.insert(entity, DirtyMaterialUsedList);
//                 },
//             }
//         });
//     }
// }


// #[derive(Debug, Default)]
// pub struct SingleMaterialIDCommandList {
//     pub list: Vec<EMaterialIDCommand>,
// }

// pub struct SysMaterialIDCommand;
// impl TSystemStageInfo for SysMaterialIDCommand {
//     fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
//         vec![
//             SysMaterailCreateCommands::key()
//         ]
//     }
// }
// #[setup]
// impl SysMaterialIDCommand {
//     #[listen(entity=(GameObject, Delete))]
//     fn listen(
//         e: Event,
//         items: Query<GameObject, (ObjectID, &MaterialID)>,
//         mut material: Query<GameObject, &mut MaterialUsedList>,
//     ) {
//         if let Some((obj, id_mat)) = items.get_by_entity(e.id) {
//             material.iter_mut().for_each(|mut list| {
//                 list.0.remove(&obj);
//             });
//         }
//     }
//     #[system]
//     pub fn cmds(
//         mut cmds: ResMut<SingleMaterialIDCommandList>,
//         mut material: Query<GameObject, &mut MaterialUsedList>,
//         mut dirty_cmd: Commands<GameObject, DirtyMaterialUsedList>,
//     ) {
//         cmds.list.drain(..).for_each(|cmd| {
//             match cmd {
//                 EMaterialIDCommand::Use(obj, id_mat) => {
//                     if let Some(mut list) = material.get_mut(id_mat.0) {
//                         list.0.insert(obj.clone(), obj.clone());
//                         dirty_cmd.insert(id_mat.0.clone(), DirtyMaterialUsedList);
//                     }
//                 },
//                 EMaterialIDCommand::UnUse(obj, id_mat) => {
//                     if let Some(mut list) = material.get_mut(id_mat.0) {
//                         list.0.remove(&obj);
//                     }
//                 },
//             }
//         });
//     }
// }

// pub type SysAssetShaderEffectLoad = AssetSyncLoad::<KeyShaderMeta, AssetKeyShaderEffect, ShaderEffectMeta, AssetResShaderEffectMeta, SysMaterailCreateCommands>;
