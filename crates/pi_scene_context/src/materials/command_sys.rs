use std::sync::Arc;

use pi_engine_shell::prelude::*;

use crate::{
    pass::*,
    object::ActionEntity, prelude::{TypeAnimeAssetMgrs, TypeAnimeContexts},
};

use super::{
    material::{LinkedMaterialID, MaterialRefs, DirtyMaterialRefs},
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
    mut disposereadylist: ResMut<ActionListDisposeReadyForRef>,
    mut _disposecanlist: ResMut<ActionListDisposeCan>,
    mut errors: ResMut<ErrorRecord>,
) {
    cmds.drain().drain(..).for_each(|OpsMaterialCreate(entity, key_shader)| {
        // log::warn!("MaterialInit: {:?}", entity);
        if commands.get_entity(entity).is_none() { 
            // log::error!("Material: Not Found!!");
            disposereadylist.push(OpsDisposeReadyForRef::ops(entity));
            return;
        };

        if let Some(meta) = asset_shader.get(&key_shader) {
            let effect_val_bind = BindEffectValues::new(&device, key_shader.clone(), meta.clone(), &mut allocator);
            let mut matcmds = commands.entity(entity);
            matcmds.insert(BindEffect(effect_val_bind));
            matcmds.insert(AssetResShaderEffectMeta::from(meta));
        } else {
            errors.record(entity, ErrorRecord::ERROR_MATERIAL_SHADER_NOTFOUND);
        }

        let mut matcmds = commands.entity(entity);
        ActionEntity::init(&mut matcmds);
        let keytex = Arc::new(UniformTextureWithSamplerParam::default());

        matcmds
            .insert(TargetAnimatorableIsRunning)
            .insert(UniformAnimated::default())
            .insert(AssetKeyShaderEffect(key_shader))
            .insert(MaterialRefs::default())
            .insert(BindEffectReset)
            .insert(UniformTextureWithSamplerParams::default())
            .insert(UniformTextureWithSamplerParamsDirty)
            .insert(FlagAnimationStartResetComp)
            .insert(DirtyMaterialRefs::default())
            .insert(TextureSlot01(keytex.clone()))
            .insert(TextureSlot02(keytex.clone()))
            .insert(TextureSlot03(keytex.clone()))
            .insert(TextureSlot04(keytex.clone()))
            .insert(TextureSlot05(keytex.clone()))
            .insert(TextureSlot06(keytex.clone()))
            .insert(TextureSlot07(keytex.clone()))
            .insert(TextureSlot08(keytex.clone()))
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
    mut renderobjectcmds: ResMut<ActionListPassObject>,
    mut materials: Query<(&mut MaterialRefs, &mut DirtyMaterialRefs)>,
    meshes: Query<(
        & PassID01, & PassID02, & PassID03, & PassID04, & PassID05, & PassID06, & PassID07, & PassID08,
        // & PassID09, & PassID10, & PassID11, & PassID12
    )>,
    mut linkedtargets: Query<&mut LinkedMaterialID>,
    passes: Query<&PassMaterialID>,
    empty: Res<SingleEmptyEntity>,
    mut errors: ResMut<ErrorRecord>,
) {
    cmds.drain().drain(..).for_each(|cmd| {
        match cmd {
            OpsMaterialUse::Use(id_mesh, id_mat, pass) => {
                if let Ok((mut materialrefs, mut flag)) = materials.get_mut(id_mat) {
                    if let Ok(mut matid) = linkedtargets.get_mut(id_mesh) {
                        let oldmat = matid.0;
                        if matid.0 != id_mat {
                            // use
                            if materialrefs.insert(id_mesh) { *flag = DirtyMaterialRefs::default(); }
                            *matid = LinkedMaterialID(id_mat);

                            // unuse
                            if let Ok((mut materialrefs, mut flag)) = materials.get_mut(oldmat) {
                                if materialrefs.remove(&id_mesh) { *flag = DirtyMaterialRefs::default(); }
                            }
                        }
                    } else if let Ok(passid) = meshes.get(id_mesh) {
                        let id_pass = match pass {
                            PassTag::PASS_TAG_01 => { passid.0.0 },
                            PassTag::PASS_TAG_02 => { passid.1.0 },
                            PassTag::PASS_TAG_03 => { passid.2.0 },
                            PassTag::PASS_TAG_04 => { passid.3.0 },
                            PassTag::PASS_TAG_05 => { passid.4.0 },
                            PassTag::PASS_TAG_06 => { passid.5.0 },
                            PassTag::PASS_TAG_07 => { passid.6.0 },
                            PassTag::PASS_TAG_08 => { passid.7.0 },
                            // PassTag::PASS_TAG_09 => { passid.8.0 },
                            // PassTag::PASS_TAG_10 => { passid.9.0 },
                            // PassTag::PASS_TAG_11 => { passid.10.0 },
                            // PassTag::PASS_TAG_12 => { passid.11.0 },
                            _ => { passid.7.0 }
                        };

                        renderobjectcmds.push(OpsPassObject::ops(id_mesh, id_mat, pass));

                        if let Ok(matid) = passes.get(id_pass) {
                            // log::error!("Material Use Pass {:?}", pass);
                            let oldmat = matid.0;
                            if oldmat != id_mat {
                                // use
                                // *matid = PassMaterialID(id_mat);
                                if materialrefs.insert(id_pass) { *flag = DirtyMaterialRefs::default(); }
                                
                                // unuse
                                if let Ok((mut materialrefs, mut _flag)) = materials.get_mut(oldmat) {
                                    if materialrefs.remove(&id_pass) {
                                        // *flag = DirtyMaterialRefs::default();
                                    }
                                }
                            }
                        } else {
                            errors.record(id_mesh, ErrorRecord::ERROR_USE_MATERIAL_NULL_TARGET);
                        }
                    } else {
                        errors.record(id_mesh, ErrorRecord::ERROR_USE_MATERIAL_NULL_TARGET);
                    }
                } else {
                    errors.record(id_mesh, ErrorRecord::ERROR_USE_MATERIAL_NULL_MAT);
                }
            },
            OpsMaterialUse::UnUse(id_mesh, id_mat) => {
                if let Ok(mut matid) = linkedtargets.get_mut(id_mesh) {
                    let old = matid.0;
                    *matid = LinkedMaterialID(empty.id());
                    // unuse
                    if let Ok((mut materialrefs, mut flag)) = materials.get_mut(old) {
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

pub fn sys_act_material_value(
    mut cmdsmat4: ResMut<ActionListUniformMat4>,
    mut cmdsvec4: ResMut<ActionListUniformVec4>,
    mut cmdsvec3: ResMut<ActionListUniformVec3>,
    mut cmdsvec2: ResMut<ActionListUniformVec2>,
    mut cmdsfloat: ResMut<ActionListUniformFloat>,
    mut cmdsuint: ResMut<ActionListUniformUint>,

    mut animator_vec4: ResMut<ActionListAnimatorableVec4>,
    mut animator_vec3: ResMut<ActionListAnimatorableVec3>,
    mut animator_vec2: ResMut<ActionListAnimatorableVec2>,
    mut animator_float: ResMut<ActionListAnimatorableFloat>,
    mut animator_uint: ResMut<ActionListAnimatorableUint>,

    mut bindvalues: Query<&mut BindEffect>,
) {
    cmdsmat4.drain().drain(..).for_each(|OpsUniformMat4(entity, slot, val)| {
        if let Ok(mut bindvalue) = bindvalues.get_mut(entity) {
            if let Some(bindvalue) = &mut bindvalue.0 {
                let value = bytemuck::cast_slice(&val);
                match bindvalue.offset(&slot) {
                    Some(offset) => {
                        let (strip, offset, entity) = offset.strip_offset();
                        if strip <= value.len() {
                            bindvalue.update(offset, &value[0..strip]);
                            bindvalue.bind().data().write_data(offset, &value[0..strip]);
                        }
                    },
                    None => {},
                }
            }
        }
    });
    cmdsvec4.drain().drain(..).for_each(|OpsUniformVec4(linked, slot, x, y, z, w)| {
        if let Ok(mut bindvalue) = bindvalues.get_mut(linked) {
            if let Some(bindvalue) = &mut bindvalue.0 {
                let val = [x, y, z, w];
                let value = bytemuck::cast_slice(&val);
                match bindvalue.offset(&slot) {
                    Some(offset) => {
                        let (strip, offset, entity) = offset.strip_offset();
                        if strip <= value.len() {
                            bindvalue.bind().data().write_data(offset, &value[0..strip]);
                            bindvalue.update(offset, &value[0..strip]);
                        }
                        if let Some(target) = entity {
                            animator_vec4.push(OpsAnimatorableVec4::ops(target, linked, AnimatorableVec4::from(val.as_slice()), EAnimatorableEntityType::Uniform));
                        }
                    },
                    None => {},
                }
            }
        }
    });
    cmdsvec3.drain().drain(..).for_each(|OpsUniformVec3(linked, slot, x, y, z)| {
        if let Ok(mut bindvalue) = bindvalues.get_mut(linked) {
            if let Some(bindvalue) = &mut bindvalue.0 {
                let val = [x, y, z];
                let value = bytemuck::cast_slice(&val);
                match bindvalue.offset(&slot) {
                    Some(offset) => {
                        let (strip, offset, entity) = offset.strip_offset();
                        if strip <= value.len() {
                            bindvalue.bind().data().write_data(offset, &value[0..strip]);
                            bindvalue.update(offset, &value[0..strip]);
                        }
                        if let Some(target) = entity {
                            animator_vec3.push(OpsAnimatorableVec3::ops(target, linked, AnimatorableVec3::from(val.as_slice()), EAnimatorableEntityType::Uniform));
                        }
                    },
                    None => {},
                }
            }
        }
    });
    cmdsvec2.drain().drain(..).for_each(|OpsUniformVec2(linked, slot, x, y)| {
        if let Ok(mut bindvalue) = bindvalues.get_mut(linked) {
            if let Some(bindvalue) = &mut bindvalue.0 {
                let val = [x, y];
                let value = bytemuck::cast_slice(&val);
                match bindvalue.offset(&slot) {
                    Some(offset) => {
                        let (strip, offset, entity) = offset.strip_offset();
                        if strip <= value.len() {
                            bindvalue.bind().data().write_data(offset, &value[0..strip]);
                            bindvalue.update(offset, &value[0..strip]);
                        }
                        if let Some(target) = entity {
                            animator_vec2.push(OpsAnimatorableVec2::ops(target, linked, AnimatorableVec2::from(val.as_slice()), EAnimatorableEntityType::Uniform));
                        }
                    },
                    None => {},
                }
            }
        }
    });
    cmdsfloat.drain().drain(..).for_each(|OpsUniformFloat(linked, slot, val)| {
        if let Ok(mut bindvalue) = bindvalues.get_mut(linked) {
            if let Some(bindvalue) = &mut bindvalue.0 {
                let vv = [val];
                let value = bytemuck::cast_slice(&vv);
                match bindvalue.offset(&slot) {
                    Some(offset) => {
                        let (strip, offset, entity) = offset.strip_offset();
                        if strip <= value.len() {
                            bindvalue.bind().data().write_data(offset, &value[0..strip]);
                            bindvalue.update(offset, &value[0..strip]);
                        }
                        if let Some(target) = entity {
                            animator_float.push(OpsAnimatorableFloat::ops(target, linked, AnimatorableFloat(val), EAnimatorableEntityType::Uniform));
                        }
                    },
                    None => {},
                }
            }
        }
    });
    cmdsuint.drain().drain(..).for_each(|OpsUniformUint(linked, slot, val)| {
        if let Ok(mut bindvalue) = bindvalues.get_mut(linked) {
            if let Some(bindvalue) = &mut bindvalue.0 {
                let vv = [val];
                let value = bytemuck::cast_slice(&vv);
                match bindvalue.offset(&slot) {
                    Some(offset) => {
                        let (strip, offset, entity) = offset.strip_offset();
                        if strip <= value.len() {
                            bindvalue.bind().data().write_data(offset, &value[0..strip]);
                            bindvalue.update(offset, &value[0..strip]);
                        }
                        if let Some(target) = entity {
                            animator_uint.push(OpsAnimatorableUint::ops(target, linked, AnimatorableUint(val), EAnimatorableEntityType::Uniform));
                        }
                    },
                    None => {},
                }
            }
        }
    });
}

pub fn sys_act_material_texture(
    mut cmds: ResMut<ActionListUniformTexture>,
    mut textureparams: Query<(&mut UniformTextureWithSamplerParams, &mut UniformTextureWithSamplerParamsDirty)>,
) {
    cmds.drain().drain(..).for_each(|OpsUniformTexture(entity, param)| {
        if let Ok((mut textureparams, mut flag)) = textureparams.get_mut(entity) {
            // log::warn!("EUniformCommand::Texture");
            textureparams.0.insert(param.slotname.clone(), Arc::new(param));
            *flag = UniformTextureWithSamplerParamsDirty;
            return;
        }
    });
}

pub fn sys_act_material_texture_from_target(
    mut cmds: ResMut<ActionListUniformTextureFromRenderTarget>,
    mut tilloffcmds: ResMut<ActionListUniformVec4>,
    mut textureparams: Query<(
        &AssetResShaderEffectMeta, &mut UniformTextureWithSamplerParams,
        (&mut EffectBindTexture2D01Comp, &mut EffectBindTexture2D02Comp, &mut EffectBindTexture2D03Comp, &mut EffectBindTexture2D04Comp, 
        &mut EffectBindTexture2D05Comp, &mut EffectBindTexture2D06Comp, &mut EffectBindTexture2D07Comp, &mut EffectBindTexture2D08Comp)
    )>,
    targets: Res<CustomRenderTargets>,
    mut errors: ResMut<ErrorRecord>,
) {
    cmds.drain().drain(..).for_each(|OpsUniformTextureFromRenderTarget(entity, mut param, key, tilloffslot)| {
        if let Ok((meta, mut textureparams, mut slots)) = textureparams.get_mut(entity) {
            // log::warn!("EUniformCommand::Texture");
            if let Some(target) = targets.get(key) {
                let tilloff = target.tilloff((0., 0., 1., 1.));
                tilloffcmds.push(OpsUniformVec4::ops(entity, tilloffslot, tilloff.0, tilloff.1, tilloff.2, tilloff.3));
                match meta.query_tex_slot(&param.slotname) {
                    Some(idx) => {
                        let bind = ETextureViewUsage::SRT(target.rt.clone());
                        match idx {
                            0 => { *slots.0 = EffectBindTexture2D01Comp::from(bind) },
                            1 => { *slots.1 = EffectBindTexture2D02Comp::from(bind) },
                            2 => { *slots.2 = EffectBindTexture2D03Comp::from(bind) },
                            3 => { *slots.3 = EffectBindTexture2D04Comp::from(bind) },
                            4 => { *slots.4 = EffectBindTexture2D05Comp::from(bind) },
                            5 => { *slots.5 = EffectBindTexture2D06Comp::from(bind) },
                            6 => { *slots.6 = EffectBindTexture2D07Comp::from(bind) },
                            7 => { *slots.7 = EffectBindTexture2D08Comp::from(bind) },
                            _ => { return; },
                        };
                    },
                    None => {
                        errors.record(entity, ErrorRecord::ERROR_MODIFY_ERROR_MATERIAL_TEXTURE);
                        // log::error!("texture_from_target Error No Slot");
                    },
                }
            } else {
                // log::error!("texture_from_target Error No Target");
            }
            
            param.url = EKeyTexture::SRT(0);
            textureparams.0.insert(param.slotname.clone(), Arc::new(param));
        } else {
            // log::error!("texture_from_target Error No Material");
        }
    });
}

pub fn sys_act_target_animation_uniform(
    mut cmds: ResMut<ActionListTargetAnimationUniform>,
    mut items: Query<(&mut BindEffect, &mut UniformAnimated)>,
    mut command: Commands,
    mut animatorablefloat: ResMut<ActionListAnimatorableFloat>,
    mut animatorablevec2s: ResMut<ActionListAnimatorableVec2>,
    mut animatorablevec3s: ResMut<ActionListAnimatorableVec3>,
    mut animatorablevec4s: ResMut<ActionListAnimatorableVec4>,
    mut animatorableuints: ResMut<ActionListAnimatorableUint>,
    anime_assets: TypeAnimeAssetMgrs,
    mut anime_contexts: TypeAnimeContexts,
    mut targetanimations: ResMut<ActionListAddTargetAnime>,
) {
    cmds.drain().drain(..).for_each(|OpsTargetAnimationUniform(idmat, attr, group, curve)| {
        if let Ok((mut bindvalue, mut animated)) = items.get_mut(idmat) {
            if let Some(bind) = &mut bindvalue.0 {
                if let Some(offset) = bind.animator(&attr, idmat, &mut command, &mut animatorablefloat, &mut animatorablevec2s, &mut animatorablevec3s, &mut animatorablevec4s, &mut animatorableuints) {
                    match offset.entity() {
                        Some(target) => {
                            animated.add(&attr);
                            match offset.atype() {
                                EAnimatorableType::Vec4 => if let Some(curve) = anime_assets.vec4s.get(&curve) {
                                    let anime = anime_contexts.vec4s.ctx.create_animation(0, AssetTypeFrameCurve::from(curve));
                                    targetanimations.push(OpsAddTargetAnimation::ops(group, target, anime));
                                },
                                EAnimatorableType::Vec3 => if let Some(curve) = anime_assets.vec3s.get(&curve) {
                                    let anime = anime_contexts.vec3s.ctx.create_animation(0, AssetTypeFrameCurve::from(curve));
                                    targetanimations.push(OpsAddTargetAnimation::ops(group, target, anime));
                                },
                                EAnimatorableType::Vec2 => if let Some(curve) = anime_assets.vec2s.get(&curve) {
                                    let anime = anime_contexts.vec2s.ctx.create_animation(0, AssetTypeFrameCurve::from(curve));
                                    targetanimations.push(OpsAddTargetAnimation::ops(group, target, anime));
                                },
                                EAnimatorableType::Float => if let Some(curve) = anime_assets.float.get(&curve) {
                                    let anime = anime_contexts.float.ctx.create_animation(0, AssetTypeFrameCurve::from(curve));
                                    targetanimations.push(OpsAddTargetAnimation::ops(group, target, anime));
                                },
                                EAnimatorableType::Uint => if let Some(curve) = anime_assets.uints.get(&curve) {
                                    let anime = anime_contexts.uints.ctx.create_animation(0, AssetTypeFrameCurve::from(curve));
                                    targetanimations.push(OpsAddTargetAnimation::ops(group, target, anime));
                                },
                                EAnimatorableType::Int => if let Some(curve) = anime_assets._ints.get(&curve) {
                                    let anime = anime_contexts._ints.ctx.create_animation(0, AssetTypeFrameCurve::from(curve));
                                    targetanimations.push(OpsAddTargetAnimation::ops(group, target, anime));
                                },
                            }
                        },
                        None => { },
                    }
                }
            }
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
    
    pub fn use_material(
        app: &mut App,
        cmd: OpsMaterialUse,
    ) {
        let mut cmds = app.world.get_resource_mut::<ActionListMaterialUse>().unwrap();
        cmds.push(cmd);
    }
}
