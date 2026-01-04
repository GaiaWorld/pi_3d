use std::{ops::Deref, sync::Arc};
use pi_slotmap::Key;
use pi_scene_shell::{prelude::*, run_stage::EngineCustomPlugins};

use crate::{
    geometry::instance::types::ModelInstanceAttributes, object::ActionEntity, pass::*, prelude::{BindModel, ModelMatIdxs, RendererRenderTargetKey, TypeAnimeAssetMgrs, TypeAnimeContexts}
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

pub type MaterialBundle = (
    BundleEntity,
    (BindEffect, AssetResShaderEffectMeta),
    (
        TargetAnimatorableIsRunning,
        UniformAnimated,
        AssetKeyShaderEffect,
        MaterialRefs,
        UniformTextureWithSamplerParams,
        UniformTextureWithSamplerParamsDirty,
        FlagAnimationStartResetComp,
        DirtyMaterialRefs,
        TextureKeyList,
        EffectBindSampler2DList,
        EffectBindTexture2DList,
        EffectTextureSamplersComp,
    )
);

pub fn sys_create_material(
    mut cmds: ResMut<ActionListMaterialCreate>,
    asset_shader: Res<ShareAssetMgr<ShaderEffectMeta>>,
    mut allocator: ResMut<ResBindBufferAllocator>,
    device: Res<PiRenderDevice>,
    mut materialmgr: ResMut<MaterialDataMgr>,
    engineopt: Res<EngineCustomPlugins>,
    mut commands: Commands,
    mut disposereadylist: ResMut<ActionListDisposeReadyForRef>,
    mut _disposecanlist: ResMut<ActionListDisposeCan>,
    mut errors: ResMut<ResErrorRecord>,
    mut alter: Alter<(), (), MaterialBundle, ()>,
    // mut performance: ResMut<Performance>,
) {
    // performance.systems.push(String::from("sys_create_material"));
    cmds.drain().for_each(|OpsMaterialCreate(entity, key_shader, matarray, staticbind)| {
        // log::warn!("MaterialInit: {:?}", entity);
        if commands.get_entity(entity).is_none() { 
            // log::error!("Material: Not Found!! {:?}", key_shader);
            disposereadylist.push(OpsDisposeReadyForRef::ops(entity));
            return;
        };

        if let Some(meta) = asset_shader.get(&key_shader) {
            // log::error!("Material: oK!! {:?}", key_shader);

            // let bind = if staticbind { 
            //     materialmgr.allocate_initial(&key_shader, &meta, &device, &mut allocator, &engineopt, matarray)
            // } else {
            //     materialmgr.allocate(&key_shader, &meta, &device, &mut allocator, &engineopt, matarray)
            // };
            let bind = materialmgr.allocate(&key_shader, &meta, &device, &mut allocator, &engineopt, matarray);

            // log::error!("MaterialData Allocate: {:?}", (&key_shader, bind.is_some()));
            let effect_val_bind = BindEffectValues::new(&device, key_shader.clone(), meta.clone(), bind);
            // let mut matcmds = commands.entity(entity);

            let bundle = (
                ActionEntity::init(),
                (BindEffect(effect_val_bind, !staticbind), AssetResShaderEffectMeta::from(meta)),
                (
                    TargetAnimatorableIsRunning,
                    UniformAnimated::default(),
                    AssetKeyShaderEffect(key_shader),
                    MaterialRefs::default(),
                    UniformTextureWithSamplerParams::default(),
                    UniformTextureWithSamplerParamsDirty { isplacehodler: false },
                    FlagAnimationStartResetComp,
                    DirtyMaterialRefs::default(),
                    TextureKeyList::default(),
                    EffectBindSampler2DList::default(),
                    EffectBindTexture2DList::default(),
                    EffectTextureSamplersComp::default(),
                )
            );
            // matcmds.insert(bundle);
            let _ = alter.alter(entity, bundle);
        } else {
            errors.record(entity.index(), ErrorRecord::ERROR_MATERIAL_SHADER_NOTFOUND);
            // log::error!("ERROR_MATERIAL_SHADER_NOTFOUND: {:?}", key_shader);
        }
    });
    materialmgr.check();
}

pub fn sys_act_material_use(
    mut cmds: ResMut<ActionListMaterialUse>,
    mut renderobjectcmds: ResMut<ActionListPassObject>,
    mut materials: Query<(&mut MaterialRefs, &mut DirtyMaterialRefs, &BindEffect)>,
    mut meshes: Query<(& PassIDs, &BindModel, &mut ModelInstanceAttributes, &mut ModelMatIdxs)>,
    mut linkedtargets: Query<&mut LinkedMaterialID>,
    passes: Query<&PassMaterialID>,
    empty: Res<SingleEmptyEntity>,
    mut errors: ResMut<ResErrorRecord>,
    // mut performance: ResMut<Performance>,
) {
    // performance.systems.push(String::from("sys_act_material_use"));
    cmds.drain().for_each(|cmd| {
        match cmd {
            OpsMaterialUse::Use(id_mesh, id_mat, pass) => {
                if let Ok((mut materialrefs, mut flag, bindeffect)) = materials.get_mut(id_mat) {
                    // ShadowCaster 
                    if let Ok(mut matid) = linkedtargets.get_mut(id_mesh) {
                        let oldmat = matid.0;
                        if matid.0 != id_mat {
                            // use
                            if materialrefs.insert(id_mesh) { flag.0.push(id_mesh); flag.set_changed(); }
                            *matid = LinkedMaterialID(id_mat);

                            // unuse
                            if let Ok((mut materialrefs, mut flag, _)) = materials.get_mut(oldmat) {
                                if materialrefs.remove(&id_mesh) { flag.0.push(id_mesh); flag.set_changed(); }
                            }
                        }
                    // Model
                    } else if let Ok((passid, matidxs, mut instancedata, mut matidxrecord)) = meshes.get_mut(id_mesh) {
                        let passindex = pass.index();
                        let id_pass = passid.0[passindex];
                        if let (Some(matidxs), Some(bindeff)) = (&matidxs.matrix, &bindeffect.0) {
                            matidxs.update_matidxs(passindex, bindeff.bind.matidx());
                            matidxrecord.0[passindex] = bindeff.bind.matidx() as u16;
                            instancedata.update_matidx(passindex, bindeff.bind.matidx() as u16);
                        }
                        if let Ok(matid) = passes.get(id_pass) {
                            // log::error!("Material Use Pass {:?}", pass);
                            let oldmat = matid.0;
                            if oldmat != id_mat {
                                // use
                                // *matid = PassMaterialID(id_mat);
                                if materialrefs.insert(id_pass) { flag.0.push(id_pass); flag.set_changed(); }
                                
                                // unuse
                                if let Ok((mut materialrefs, mut flag, _)) = materials.get_mut(oldmat) {
                                    if materialrefs.remove(&id_pass) {
                                        flag.0.push(id_pass); flag.set_changed();
                                    }
                                }

                                renderobjectcmds.push(OpsPassObject::ops(id_mesh, id_mat, pass));
                            } else {
                                // log::error!("MatID Again!");
                            }
                        } else {
                            errors.record(id_mesh.index(), ErrorRecord::ERROR_USE_MATERIAL_NULL_TARGET);
                        }
                    } else {
                        errors.record(id_mesh.index(), ErrorRecord::ERROR_USE_MATERIAL_NULL_TARGET);
                    }
                } else {
                    errors.record(id_mesh.index(), ErrorRecord::ERROR_USE_MATERIAL_NULL_MAT);
                }
            },
            OpsMaterialUse::UnUse(id_mesh, _id_mat, pass) => {
                if let Ok(mut matid) = linkedtargets.get_mut(id_mesh) {
                    let old = matid.0;
                    *matid = LinkedMaterialID(empty.id());
                    // unuse
                    if let Ok((mut materialrefs, mut flag, _)) = materials.get_mut(old) {
                        if materialrefs.remove(&id_mesh) {
                            flag.0.push(id_mesh); flag.set_changed();
                        }
                    // } else {
                    //     cmds.push(OpsMaterialUse::UnUse(id_mesh, id_mat));
                    }
                // } else {
                //     cmds.push(OpsMaterialUse::UnUse(id_mesh, id_mat));
                }
            },
        }
    });
}

pub fn sys_act_material_value(
    mut cmdsvalb: ResMut<ActionListUniformValB>,
    mut cmdsval: ResMut<ActionListUniformVal>,

    mut animator_vec4: ResMut<ActionListAnimatorableVec4>,
    mut animator_vec3: ResMut<ActionListAnimatorableVec3>,
    mut animator_vec2: ResMut<ActionListAnimatorableVec2>,
    mut animator_float: ResMut<ActionListAnimatorableFloat>,
    mut animator_uint: ResMut<ActionListAnimatorableUint>,

    mut textureparams: Query<(&mut UniformTextureWithSamplerParams, &mut UniformTextureWithSamplerParamsDirty)>,
    mut bindvalues: Query<(&mut BindEffect, &mut UniformAnimated)>,
    targets: Res<CustomRenderTargets>,
    renderers: Query<&RendererRenderTargetKey>,
    mut command: Commands,
    mut animatorablefloat: ResMut<ActionListAnimatorableFloat>,
    mut animatorablevec2s: ResMut<ActionListAnimatorableVec2>,
    mut animatorablevec3s: ResMut<ActionListAnimatorableVec3>,
    mut animatorablevec4s: ResMut<ActionListAnimatorableVec4>,
    mut animatorableuints: ResMut<ActionListAnimatorableUint>,
    anime_assets: TypeAnimeAssetMgrs,
    mut anime_contexts: TypeAnimeContexts,
    mut targetanimations: ResMut<ActionListAnimationGroupAction>,
    // mut performance: ResMut<Performance>,
) {
    // performance.systems.push(String::from("sys_act_material_value"));

    cmdsvalb.drain().for_each(|cmd| {
        match cmd {
            OpsUniformValB::Mat4(entity, slot, val) => {
                if let Ok((mut bindvalue, _)) = bindvalues.get_mut(entity) {
                    // if bindvalue.1 == false { log::error!("Dirty Mat4: {:?}", &slot); }
                    // _alloc_bindeffect(entity, &mut bindvalue, &materials, &mut allocator, &device, &mut materialmgr, &engineopt, &passes, &mut models);
                    if let Some(bindvalue) = &mut bindvalue.0 {
                        let value = bytemuck::cast_slice(&val);
                        _bind_value(bindvalue, &slot, value);
                    }
                }
            },
            OpsUniformValB::Texture(entity, param) => {
                if let Ok((mut textureparams, mut flag)) = textureparams.get_mut(entity) {
                    textureparams.0.insert(param.slotname.clone(), Arc::new(param));
                    flag.set_changed();
                    return;
                }
            },
            OpsUniformValB::TextureFromRenderTarget(entity, mut param, key, tilloffslot) => {
                if let Ok((mut textureparams, mut flag)) = textureparams.get_mut(entity) {
                    // log::warn!("EUniformCommand::Texture");
                    if let Some(target) = targets.get(key) {
                        let tilloff = target.tilloff((0., 0., 1., 1.));
                        cmdsval.push(OpsUniformVal::vec4(entity, tilloffslot, tilloff.0, tilloff.1, tilloff.2, tilloff.3));
                        // log::error!("texture_from_target Target {:?}", key);
                    }
                    param.url = EKeyTexture::SRT(key);
                    textureparams.0.insert(param.slotname.clone(), Arc::new(param));
                    flag.set_changed();
                } else {
                    // log::error!("texture_from_target Error No Material");
                }
            },
            OpsUniformValB::TextureFromRenderInput(entity, mut param, idrenderer, tilloffslot) => {
                if let (Ok((mut textureparams, mut flag)), Ok(key)) = (textureparams.get_mut(entity), renderers.get(idrenderer)) {
                    if let Some(key) = key.0 {
                        // log::warn!("EUniformCommand::Texture");
                        if let Some(target) = targets.get(key) {
                            let tilloff = target.tilloff((0., 0., 1., 1.));
                            cmdsval.push(OpsUniformVal::vec4(entity, tilloffslot, tilloff.0, tilloff.1, tilloff.2, tilloff.3));
                            // log::error!("texture_from_renderer {:?}", (key, tilloff));
                        } else {
                            // log::error!("texture_from_renderer Error No RT {:?}", (key, idrenderer));
                        }
                        param.url = EKeyTexture::SRT(key);
                        textureparams.0.insert(param.slotname.clone(), Arc::new(param));
                        flag.set_changed();
                    } else {
                        // log::error!("texture_from_renderer Error No Key");
                    }
                } else {
                    // log::error!("texture_from_renderer Error No Material {:?}", (entity, idrenderer));
                }
            }
            OpsUniformValB::TargetAnimation(idmat, attr, group, curve) => {
                if let Ok((mut bindvalue, mut animated)) = bindvalues.get_mut(idmat) {
                    if let Some(bind) = &mut bindvalue.0 {
                        if let Some(offset) = bind.animator(&attr, idmat, &mut command, &mut animatorablefloat, &mut animatorablevec2s, &mut animatorablevec3s, &mut animatorablevec4s, &mut animatorableuints) {
                            match offset.entity() {
                                Some(target) => {
                                    animated.add(&attr);
                                    match offset.atype() {
                                        EAnimatorableType::Vec4 => if let Some(curve) = anime_assets.vec4s.get(&curve) {
                                            let anime = anime_contexts.vec4s.ctx.create_animation(0, AssetTypeFrameCurve::from(curve));
                                            targetanimations.push(OpsAnimationGroupAction::addtarget(group, target, anime));
                                        },
                                        EAnimatorableType::Vec3 => if let Some(curve) = anime_assets.vec3s.get(&curve) {
                                            let anime = anime_contexts.vec3s.ctx.create_animation(0, AssetTypeFrameCurve::from(curve));
                                            targetanimations.push(OpsAnimationGroupAction::addtarget(group, target, anime));
                                        },
                                        EAnimatorableType::Vec2 => if let Some(curve) = anime_assets.vec2s.get(&curve) {
                                            let anime = anime_contexts.vec2s.ctx.create_animation(0, AssetTypeFrameCurve::from(curve));
                                            targetanimations.push(OpsAnimationGroupAction::addtarget(group, target, anime));
                                        },
                                        EAnimatorableType::Float => if let Some(curve) = anime_assets.float.get(&curve) {
                                            let anime = anime_contexts.float.ctx.create_animation(0, AssetTypeFrameCurve::from(curve));
                                            targetanimations.push(OpsAnimationGroupAction::addtarget(group, target, anime));
                                        },
                                        EAnimatorableType::Uint => if let Some(curve) = anime_assets.uints.get(&curve) {
                                            let anime = anime_contexts.uints.ctx.create_animation(0, AssetTypeFrameCurve::from(curve));
                                            targetanimations.push(OpsAnimationGroupAction::addtarget(group, target, anime));
                                        },
                                        EAnimatorableType::Int => if let Some(curve) = anime_assets._ints.get(&curve) {
                                            let anime = anime_contexts._ints.ctx.create_animation(0, AssetTypeFrameCurve::from(curve));
                                            targetanimations.push(OpsAnimationGroupAction::addtarget(group, target, anime));
                                        },
                                    }
                                },
                                None => { },
                            }
                        }
                    }
                }
            },
        }
    });

    cmdsval.drain().for_each(|OpsUniformVal(linked, cmd)| {
        let mut bindvalue = if let Ok((bindvalue, _)) = bindvalues.get_mut(linked) {
            bindvalue
        } else { return; };

        // if bindvalue.1 == false { log::error!("Dirty : {:?}", &cmd); }
        // _alloc_bindeffect(linked, &mut bindvalue, &materials, &mut allocator, &device, &mut materialmgr, &engineopt, &passes, &mut models);
        match cmd {
            EUniformVal::Vec4( slot, x, y, z, w) => {
                if let Some(bindvalue) = &mut bindvalue.0 {
                    let val = [x, y, z, w];
                    let value = bytemuck::cast_slice(&val);
                    if let Some(target) = _bind_value(bindvalue, &slot, value) {
                        animator_vec4.push(OpsAnimatorableVec4::ops(target, linked, AnimatorableVec4::from(val.as_slice()), EAnimatorableEntityType::Uniform));
                    }
                }
            },
            EUniformVal::Vec3( slot, x, y, z) => {
                if let Some(bindvalue) = &mut bindvalue.0 {
                    let val = [x, y, z];
                    let value = bytemuck::cast_slice(&val);
                    if let Some(target) = _bind_value(bindvalue, &slot, value) {
                        animator_vec3.push(OpsAnimatorableVec3::ops(target, linked, AnimatorableVec3::from(val.as_slice()), EAnimatorableEntityType::Uniform));
                    }
                }
            },
            EUniformVal::Vec2( slot, x, y) => {
                if let Some(bindvalue) = &mut bindvalue.0 {
                    let val = [x, y];
                    let value = bytemuck::cast_slice(&val);
                    if let Some(target) = _bind_value(bindvalue, &slot, value) {
                        animator_vec2.push(OpsAnimatorableVec2::ops(target, linked, AnimatorableVec2::from(val.as_slice()), EAnimatorableEntityType::Uniform));
                    }
                }
            },
            EUniformVal::Float( slot, val) => {
                if let Some(bindvalue) = &mut bindvalue.0 {
                    let vv = [val];
                    let value = bytemuck::cast_slice(&vv);
                    if let Some(target) = _bind_value(bindvalue, &slot, value) {
                        animator_float.push(OpsAnimatorableFloat::ops(target, linked, AnimatorableFloat(val), EAnimatorableEntityType::Uniform));
                    }
                }
            },
            EUniformVal::Uint( slot, val) => {
                if let Some(bindvalue) = &mut bindvalue.0 {
                    let vv = [val];
                    let value = bytemuck::cast_slice(&vv);
                    if let Some(target) = _bind_value(bindvalue, &slot, value) {
                        animator_uint.push(OpsAnimatorableUint::ops(target, linked, AnimatorableUint(val), EAnimatorableEntityType::Uniform));
                    }
                }
            }
        }
    });
}

// 材质初始化时使用的统一的初始值Buffer, 当产生uniform数值操作时重新申请一个独享的
fn _alloc_bindeffect(
    idmat: Entity,
    bindeffect: &mut BindEffect,
    materials: &Query<(&MaterialRefs, &AssetResShaderEffectMeta, &AssetKeyShaderEffect)>,

    allocator: &mut ResBindBufferAllocator,
    device: &PiRenderDevice,
    materialmgr: &mut MaterialDataMgr,
    engineopt: &EngineCustomPlugins,

    passes: &Query<(&PassModelID, &PassTag)>,
    models: &mut Query<(&BindModel, &mut ModelInstanceAttributes, &mut ModelMatIdxs)>,
) {
    if bindeffect.1 == false {
        if let Ok((materialrefs, meta, key_shader)) = materials.get(idmat) {
            let meta = meta.0.as_ref().unwrap();
            let key_shader = key_shader.deref();
            let matarray = materialmgr.ismatarray(key_shader);
            let bind = materialmgr.allocate(key_shader, meta, device, allocator, engineopt, matarray);
            let effect_val_bind = BindEffectValues::new(device, key_shader.clone(), meta.clone(), bind);
            bindeffect.0 = effect_val_bind;
            bindeffect.1 = true;

            materialrefs.iter().for_each(|id_pass| {
                if let Ok((idmodel, pass)) = passes.get(*id_pass) {
                    if let Ok((matidxs, mut instancedata, mut matidxrecord)) = models.get_mut(idmodel.0) {
                        let passindex = pass.index();
                        if let (Some(matidxs), Some(bindeff)) = (&matidxs.matrix, &bindeffect.0) {
                            matidxs.update_matidxs(passindex, bindeff.bind.matidx());
                            matidxrecord.0[passindex] = bindeff.bind.matidx() as u16;
                            instancedata.update_matidx(passindex, bindeff.bind.matidx() as u16);
                        }
                    }
                }
            });
        }
    }
}

fn _bind_value(
    bindvalue: &mut BindEffectValues,
    slot: &Atom,
    value: &[u8],
) -> Option<Entity> {
    match bindvalue.offset(&slot) {
        Some(offset) => {
            let (strip, offset, _entity) = offset.strip_offset();
            if strip <= value.len() {
                bindvalue.update(offset, &value[0..strip]);
            }
            _entity
        },
        None => {
            None
        },
    }
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
        let cmds = app.world.get_resource_mut::<ActionListMaterialUse>().unwrap();
        cmds.push(cmd);
    }
}
