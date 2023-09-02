use std::sync::Arc;

use pi_engine_shell::prelude::*;

use crate::{
    pass::*, renderers::prelude::*, object::ActionEntity, prelude::{ActionListDisposeReady, ActionListDisposeCan, OpsDisposeReady},
};

use super::{
    material::{MaterialID, MaterialRefs, DirtyMaterialRefs},
    shader_effect::*,
    uniforms::{
        texture::*,
        uniform::*,
    },
    command::*,
};

pub fn sys_create_material(
    mut cmds: ResMut<ActionListMaterialCreate>,
    asset_shader: Res<ShareAssetMgr<ShaderEffectMeta>>,
    mut allocator: ResMut<ResBindBufferAllocator>,
    device: Res<PiRenderDevice>,
    mut commands: Commands,
    mut disposereadylist: ResMut<ActionListDisposeReady>,
    mut _disposecanlist: ResMut<ActionListDisposeCan>,
) {
    cmds.drain().drain(..).for_each(|OpsMaterialCreate(entity, key_shader, passtag)| {
        // log::error!("Material: ");
        let mut matcmds = if let Some(cmd) = commands.get_entity(entity) { 
            cmd
        } else {
            disposereadylist.push(OpsDisposeReady::ops(entity));
            return;
        };

        if let Some(meta) = asset_shader.get(&key_shader) {
            if let Some(effect_val_bind) = BindEffectValues::new(&device, key_shader.clone(), meta.clone(), &mut allocator) {
                // log::error!("Material: {:?} {:?}", key_shader, true);
                matcmds.insert(BindEffect(effect_val_bind));
            } else {
                // log::error!("Material: {:?} {:?} Bind", key_shader, false);
            }
            matcmds.insert(AssetResShaderEffectMeta::from(meta));
        } else {
            // log::error!("Material: {:?} {:?}", key_shader, false);
        }

        ActionEntity::init(&mut matcmds);

        matcmds
            .insert(AssetKeyShaderEffect(key_shader))
            .insert(MaterialRefs::default())
            .insert(BindEffectValueDirty)
            .insert(BindEffectReset)
            .insert(passtag)
            .insert(UniformTextureWithSamplerParams::default())
            .insert(UniformTextureWithSamplerParamsDirty)
            .insert(FlagAnimationStartResetComp)
            .insert(DirtyMaterialRefs::default())
            .insert(EffectBindTexture2D01Comp::default())
            .insert(EffectBindTexture2D02Comp::default())
            .insert(EffectBindTexture2D03Comp::default())
            .insert(EffectBindTexture2D04Comp::default())
            .insert(EffectBindTexture2D05Comp::default())
            .insert(EffectBindTexture2D06Comp::default())
            .insert(EffectBindTexture2D07Comp::default())
            .insert(EffectBindTexture2D08Comp::default())
            .insert(EffectBindSampler2D01Comp::default())
            .insert(EffectBindSampler2D02Comp::default())
            .insert(EffectBindSampler2D03Comp::default())
            .insert(EffectBindSampler2D04Comp::default())
            .insert(EffectBindSampler2D05Comp::default())
            .insert(EffectBindSampler2D06Comp::default())
            .insert(EffectBindSampler2D07Comp::default())
            .insert(EffectBindSampler2D08Comp::default())
            .insert(EffectBindSampler2D08Comp::default())
            .insert(EffectTextureSamplersComp::default())
            ;
    });
}

pub fn sys_act_material_use(
    mut cmds: ResMut<ActionListMaterialUse>,
    mut materials: Query<(&mut MaterialRefs, &mut DirtyMaterialRefs, &EPassTag)>,
    meshes: Query<(&mut PassID01, &mut PassID02, &mut PassID03, &mut PassID04, &mut PassID05, &mut PassID06, &mut PassID07, &mut PassID08)>,
    mut targets: Query<&mut MaterialID>,
    empty: Res<SingleEmptyEntity>,
    mut commands: Commands,
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
                            if let Ok((mut materialrefs, mut flag, _pass)) = materials.get_mut(old) {
                                if materialrefs.remove(&id_mesh) {
                                    *flag = DirtyMaterialRefs::default();
                                }
                            }
                        }
                    } else if let Ok(passid) = meshes.get(id_mesh) {
                        let pass = pass.as_pass();
                        let id_pass = if pass == EPassTag::PASS_TAG_01 { passid.0.0 }
                        else if pass == EPassTag::PASS_TAG_02 { passid.1.0 }
                        else if pass == EPassTag::PASS_TAG_03 { passid.2.0 }
                        else if pass == EPassTag::PASS_TAG_04 { passid.3.0 }
                        else if pass == EPassTag::PASS_TAG_05 { passid.4.0 }
                        else if pass == EPassTag::PASS_TAG_06 { passid.5.0 }
                        else if pass == EPassTag::PASS_TAG_07 { passid.6.0 }
                        else { passid.7.0 };

                        if let Ok(mut matid) = targets.get_mut(id_pass) {
                            if matid.0 != id_mat {
                                let old = matid.0;
                                // use
                                if materialrefs.insert(id_pass) {
                                    *flag = DirtyMaterialRefs::default();
                                }
                                
                                // unuse
                                if let Ok((mut materialrefs, mut flag, _pass)) = materials.get_mut(old) {
                                    if materialrefs.remove(&id_pass) {
                                        *flag = DirtyMaterialRefs::default();
                                    }
                                }
                                
                                *matid = MaterialID(id_mat);
                                reset_passobj(id_pass, id_mat, &mut commands);
                            }
                        } else {
                            cmds.push(OpsMaterialUse::Use(id_pass, id_mat));
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
                    if let Ok((mut materialrefs, mut flag, _pass)) = materials.get_mut(old) {
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

fn reset_passobj(
    idpass: Entity,
    _idmat: Entity,
    commands: &mut Commands,
) {
    if let Some(mut cmd) = commands.get_entity(idpass) { 
        cmd
        .insert(PassBindEffectValue(None))
        .insert(PassBindEffectTextures(None))
        .insert(PassBindGroupScene(None))
        .insert(PassBindGroupModel(None))
        .insert(PassBindGroupTextureSamplers(None))
        .insert(PassBindGroups(None))
        .insert(PassEffectReady(None))
        .insert(PassShader(None))
        .insert(PassPipeline(None))
        .insert(PassDraw(None))
        ;
    }
}

pub fn sys_act_material_mat4(
    mut cmds: ResMut<ActionListUniformMat4>,
    mut bindvalues: Query<(&mut BindEffect, &mut BindEffectValueDirty)>,
) {
    cmds.drain().drain(..).for_each(|OpsUniformMat4(entity, slot, value, count)| {
        if let Ok((mut bindvalues, mut flag)) = bindvalues.get_mut(entity) {
            if let Some(slot) = bindvalues.slot(&slot) {
                bindvalues.mat4(slot, &value);
                *flag = BindEffectValueDirty;
            }
            return;
        }

        if count < MATERIAL_UNIFORM_OPS_WAIT_FRAME {
            cmds.push(OpsUniformMat4(entity, slot, value, count + 1));
        }
    });
}

// pub fn sys_act_material_mat2(
//     mut cmds: ResMut<ActionListUniformMat2>,
//     mut bindvalues: Query<(&mut BindEffect, &mut BindEffectValueDirty)>,
// ) {
//     cmds.drain().drain(..).for_each(|OpsUniformMat2(entity, slot, value, count)| {
//         if let Ok((mut bindvalues, mut flag)) = bindvalues.get_mut(entity) {
//             if let Some(slot) = bindvalues.slot(&slot) {
//                 bindvalues.mat2(slot, &value);
//                 *flag = BindEffectValueDirty;
//             }
//             return;
//         }

//         if count < MATERIAL_UNIFORM_OPS_WAIT_FRAME {
//             OpsUniformMat2(entity, slot, value, count + 1);
//         }
//     });
// }

pub fn sys_act_material_vec4(
    mut cmds: ResMut<ActionListUniformVec4>,
    mut bindvalues: Query<(&mut BindEffect, &mut BindEffectValueDirty)>,
) {
    cmds.drain().drain(..).for_each(|OpsUniformVec4(entity, slot, x, y, z, w, count)| {
        if let Ok((mut bindvalues, mut flag)) = bindvalues.get_mut(entity) {
            if let Some(slot) = bindvalues.slot(&slot) {
                bindvalues.vec4(slot, &[x, y, z, w]);
                *flag = BindEffectValueDirty;
            }
            return;
        }

        if count < MATERIAL_UNIFORM_OPS_WAIT_FRAME {
            cmds.push(OpsUniformVec4(entity, slot, x, y, z, w, count + 1));
        }
    });
}

pub fn sys_act_material_vec2(
    mut cmds: ResMut<ActionListUniformVec2>,
    mut bindvalues: Query<(&mut BindEffect, &mut BindEffectValueDirty)>,
) {
    cmds.drain().drain(..).for_each(|OpsUniformVec2(entity, slot, x, y, count)| {
        if let Ok((mut bindvalues, mut flag)) = bindvalues.get_mut(entity) {
            if let Some(slot) = bindvalues.slot(&slot) {
                bindvalues.vec2(slot, &[x, y]);
                *flag = BindEffectValueDirty;
            }
            return;
        }

        if count < MATERIAL_UNIFORM_OPS_WAIT_FRAME {
            cmds.push(OpsUniformVec2(entity, slot, x, y, count + 1));
        }
    });
}

pub fn sys_act_material_float(
    mut cmds: ResMut<ActionListUniformFloat>,
    mut bindvalues: Query<(&mut BindEffect, &mut BindEffectValueDirty)>,
) {
    cmds.drain().drain(..).for_each(|OpsUniformFloat(entity, slot, value, count)| {
        if let Ok((mut bindvalues, mut flag)) = bindvalues.get_mut(entity) {
            if let Some(slot) = bindvalues.slot(&slot) {
                bindvalues.float(slot, value);
                *flag = BindEffectValueDirty;
            }
            return;
        }

        if count < MATERIAL_UNIFORM_OPS_WAIT_FRAME {
            cmds.push(OpsUniformFloat(entity, slot, value, count + 1));
        }
    });
}

// pub fn sys_act_material_int(
//     mut cmds: ResMut<ActionListUniformInt>,
//     mut bindvalues: Query<(&mut BindEffect, &mut BindEffectValueDirty)>,
// ) {
//     cmds.drain().drain(..).for_each(|OpsUniformInt(entity, slot, value, count)| {
//         if let Ok((mut bindvalues, mut flag)) = bindvalues.get_mut(entity) {
//             if let Some(slot) = bindvalues.slot(&slot) {
//                 bindvalues.int(slot, value);
//                 *flag = BindEffectValueDirty;
//             }
//             return;
//         }

//         if count < MATERIAL_UNIFORM_OPS_WAIT_FRAME {
//             OpsUniformInt(entity, slot, value, count + 1);
//         }
//     });
// }

pub fn sys_act_material_uint(
    mut cmds: ResMut<ActionListUniformUint>,
    mut bindvalues: Query<(&mut BindEffect, &mut BindEffectValueDirty)>,
) {
    cmds.drain().drain(..).for_each(|OpsUniformUint(entity, slot, value, count)| {
        if let Ok((mut bindvalues, mut flag)) = bindvalues.get_mut(entity) {
            if let Some(slot) = bindvalues.slot(&slot) {
                bindvalues.uint(slot, value);
                *flag = BindEffectValueDirty;
            }
            return;
        }

        if count < MATERIAL_UNIFORM_OPS_WAIT_FRAME {
            cmds.push(OpsUniformUint(entity, slot, value, count + 1));
        }
    });
}

pub fn sys_act_material_texture(
    mut cmds: ResMut<ActionListUniformTexture>,
    mut textureparams: Query<(&mut UniformTextureWithSamplerParams, &mut UniformTextureWithSamplerParamsDirty)>,
) {
    cmds.drain().drain(..).for_each(|OpsUniformTexture(entity, param, count)| {
        if let Ok((mut textureparams, mut flag)) = textureparams.get_mut(entity) {
            // log::warn!("EUniformCommand::Texture");
            textureparams.0.insert(param.slotname.clone(), Arc::new(param));
            *flag = UniformTextureWithSamplerParamsDirty;
            return;
        }

        if count < MATERIAL_UNIFORM_OPS_WAIT_FRAME {
            cmds.push(OpsUniformTexture(entity, param, count + 1));
        }
    });
}

pub struct ActionMaterial;
impl ActionMaterial {
    pub fn regist_material_meta(
        asset_mgr: &ShareAssetMgr<ShaderEffectMeta>,
        // wait_list: &mut AssetSyncWait<KeyShaderMeta, AssetKeyShaderEffect, ShaderEffectMeta, AssetResShaderEffectMeta>,
        key: KeyShaderMeta,
        meta: ShaderEffectMeta,
    ) {
        // log::warn!("Regist ShaderName: {:?}", key);
        if !asset_mgr.contains_key(&key) {
            if let Ok(_meta) = asset_mgr.insert(key.clone(), meta) {
                // wait_list.1.push((key.clone(), meta));
                // log::warn!("Regist ShaderName Success: {:?}", key);
            } else {
                // log::warn!("Regist ShaderName Insert Fail: {:?}", key);
            }
            // let meta = asset_mgr.insert(key.clone(), meta);
            // wait_list.1.push((key.clone(), meta));
        } else {
            // log::warn!("Regist ShaderName contains_key ??: {:?}", key);
        }
    }
    // pub fn init(
    //     app: &mut App,
    //     mat: Entity,
    //     key: KeyShaderMeta,
    //     passtag: EPassTag,
    // ) {
    //     let mut cmds = app.world.get_resource_mut::<ActionListMaterialCreate>().unwrap();
    //     cmds.push(OpsMaterialCreate(mat, key, passtag));
    // }
    
    pub fn use_material(
        app: &mut App,
        cmd: OpsMaterialUse,
    ) {
        let mut cmds = app.world.get_resource_mut::<ActionListMaterialUse>().unwrap();
        cmds.push(cmd);
    }
}

