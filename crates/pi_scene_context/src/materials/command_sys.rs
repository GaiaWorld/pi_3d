use std::{mem::replace};

use pi_engine_shell::prelude::*;

use crate::{
    pass::*,
    animation::prelude::*
};

use super::{
    material::{MaterialID, MaterialRefs, DirtyMaterialRefs},
    shader_effect::{AssetKeyShaderEffect, AssetResShaderEffectMeta},
    uniforms::{
        texture::{UniformTextureWithSamplerParams}, uniform::*,
    },
    command::*,
};

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

