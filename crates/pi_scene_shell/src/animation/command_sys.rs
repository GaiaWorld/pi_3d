use crate::ecs::*;

use std::ops::Deref;

use crate::object::ActionEntity;
use crate::object::ActionListDisposeCan;
use crate::object::DisposeCan;
use crate::object::DisposeReady;
use crate::object::OpsDisposeCan;
use crate::prelude::{Performance, ErrorRecord};

use super::base::*;
use super::command::*;
use crate::animation::*;

pub type BundleAnimFloatA = (AnimatorableFloat, AnimatorableLink, AnimatorableAttribute);
pub type BundleAnimFloatU = (AnimatorableFloat, AnimatorableLink, AnimatorableUniform);
pub type BundleAnimVec2A  = (AnimatorableVec2,  AnimatorableLink, AnimatorableAttribute);
pub type BundleAnimVec2U  = (AnimatorableVec2,  AnimatorableLink, AnimatorableUniform);
pub type BundleAnimVec3A  = (AnimatorableVec3,  AnimatorableLink, AnimatorableAttribute);
pub type BundleAnimVec3U  = (AnimatorableVec3,  AnimatorableLink, AnimatorableUniform);
pub type BundleAnimVec4A  = (AnimatorableVec4,  AnimatorableLink, AnimatorableAttribute);
pub type BundleAnimVec4U  = (AnimatorableVec4,  AnimatorableLink, AnimatorableUniform);
pub type BundleAnimUintA  = (AnimatorableUint,  AnimatorableLink, AnimatorableAttribute);
pub type BundleAnimUintU  = (AnimatorableUint,  AnimatorableLink, AnimatorableUniform);
pub type BundleAnimSintA  = (AnimatorableSint,  AnimatorableLink, AnimatorableAttribute);
pub type BundleAnimSintU  = (AnimatorableSint,  AnimatorableLink, AnimatorableUniform);
pub type BundleAnimGroup = (AnimationGroupKey, AnimationGroupScene);

pub fn sys_create_animatorable_entity(
    mut cmds_float: ResMut<ActionListAnimatorableFloat>,
    mut cmds_vec2: ResMut<ActionListAnimatorableVec2>,
    mut cmds_vec3: ResMut<ActionListAnimatorableVec3>,
    mut cmds_vec4: ResMut<ActionListAnimatorableVec4>,
    // mut cmds_mat4: ResMut<ActionListAnimatorableMat4>,
    mut cmds_uint: ResMut<ActionListAnimatorableUint>,
    mut cmds_int: ResMut<ActionListAnimatorableSint>,
    mut commands: Commands,
    items: Query<(), (With<DisposeReady>, With<DisposeCan>)>,
    mut recordfloat: ResMut<AnimeTargetRecordValues<AnimatorableFloat>>,
    mut recordvec2: ResMut<AnimeTargetRecordValues<AnimatorableVec2>>,
    mut recordvec3: ResMut<AnimeTargetRecordValues<AnimatorableVec3>>,
    mut recordvec4: ResMut<AnimeTargetRecordValues<AnimatorableVec4>>,
    mut recordsint: ResMut<AnimeTargetRecordValues<AnimatorableSint>>,
    mut recorduint: ResMut<AnimeTargetRecordValues<AnimatorableUint>>,
) {
    cmds_float.drain().for_each(|OpsAnimatorableFloat(entity, linked, value, etype)| {
        if let Some(mut cmd) = commands.get_entity(entity) {
            if items.contains(entity) == false {
                let bundle = ActionEntity::init();
                cmd.insert(bundle);
                // alters.0.alter(entity, bundle);
            }
            match etype {
                EAnimatorableEntityType::Uniform => {
                    let bundle = (value.clone(), AnimatorableLink(linked), AnimatorableUniform);
                    cmd.insert(bundle);
                    // alters.2.alter(entity, bundle);
                },
                EAnimatorableEntityType::Attribute => {
                    let bundle = (value.clone(), AnimatorableLink(linked), AnimatorableAttribute);
                    cmd.insert(bundle);
                    // alters.1.alter(entity, bundle);
                },
            };
            recordfloat.insert(entity, value);
        }
    });
    cmds_vec2.drain().for_each(|OpsAnimatorableVec2(entity, linked, value, etype)| {
        if let Some(mut cmd) = commands.get_entity(entity) {
            if items.contains(entity) == false { cmd.insert(ActionEntity::init()); }
            match etype {
                EAnimatorableEntityType::Uniform => {
                    let bundle = (value.clone(), AnimatorableLink(linked), AnimatorableUniform);
                    cmd.insert(bundle);
                    // alters.4.alter(entity, bundle);
                },
                EAnimatorableEntityType::Attribute => {
                    let bundle = (value.clone(), AnimatorableLink(linked), AnimatorableAttribute);
                    cmd.insert(bundle);
                    // alters.3.alter(entity, bundle);
                },
            };
            recordvec2.insert(entity, value);
        }
    });
    cmds_vec3.drain().for_each(|OpsAnimatorableVec3(entity, linked, value, etype)| {
        if let Some(mut cmd) = commands.get_entity(entity) {
            if items.contains(entity) == false { cmd.insert(ActionEntity::init()); }
            match etype {
                EAnimatorableEntityType::Uniform => {
                    let bundle = (value.clone(), AnimatorableLink(linked), AnimatorableUniform);
                    cmd.insert(bundle);
                    // alters.6.alter(entity, bundle);
                },
                EAnimatorableEntityType::Attribute => {
                    let bundle = (value.clone(), AnimatorableLink(linked), AnimatorableAttribute);
                    cmd.insert(bundle);
                    // alters.5.alter(entity, bundle);
                },
            };
            recordvec3.insert(entity, value);
        }
    });
    cmds_vec4.drain().for_each(|OpsAnimatorableVec4(entity, linked, value, etype)| {
        if let Some(mut cmd) = commands.get_entity(entity) {
            if items.contains(entity) == false { cmd.insert(ActionEntity::init()); }
            match etype {
                EAnimatorableEntityType::Uniform => {
                    let bundle = (value.clone(), AnimatorableLink(linked), AnimatorableUniform);
                    cmd.insert(bundle);
                    // alters.8.alter(entity, bundle);
                },
                EAnimatorableEntityType::Attribute => {
                    let bundle = (value.clone(), AnimatorableLink(linked), AnimatorableAttribute);
                    cmd.insert(bundle);
                    // alters.7.alter(entity, bundle);
                },
            };
            recordvec4.insert(entity, value);
        }
    });
    // cmds_mat4.drain().drain(..).for_each(|OpsAnimatorableMat4(entity, linked, value)| {
    //     if let Some(mut cmd) = commands.get_entity(entity) {
    //         cmd.insert(value.clone(), AnimatorableLink(linked), RecordAnimatorableVec4(value.clone()));
    //     }
    // });
    cmds_uint.drain().for_each(|OpsAnimatorableUint(entity, linked, value, etype)| {
        if let Some(mut cmd) = commands.get_entity(entity) {
            if items.contains(entity) == false { cmd.insert(ActionEntity::init()); }
            match etype {
                EAnimatorableEntityType::Uniform => {
                    let bundle = (value.clone(), AnimatorableLink(linked), AnimatorableUniform);
                    cmd.insert(bundle);
                    // alters.10.alter(entity, bundle);
                },
                EAnimatorableEntityType::Attribute => {
                    let bundle = (value.clone(), AnimatorableLink(linked), AnimatorableAttribute);
                    cmd.insert(bundle);
                    // alters.9.alter(entity, bundle);
                },
            };
            recorduint.insert(entity, value);
        }
    });
    cmds_int.drain().for_each(|OpsAnimatorableSint(entity, linked, value, etype)| {
        if let Some(mut cmd) = commands.get_entity(entity) {
            if items.contains(entity) == false { cmd.insert(ActionEntity::init()); }
            match etype {
                EAnimatorableEntityType::Uniform => {
                    let bundle = (value.clone(), AnimatorableLink(linked), AnimatorableUniform);
                    cmd.insert(bundle);
                    // alters.12.alter(entity, bundle);
                },
                EAnimatorableEntityType::Attribute => {
                    let bundle = (value.clone(), AnimatorableLink(linked), AnimatorableAttribute);
                    cmd.insert(bundle);
                    // alters.11.alter(entity, bundle);
                },
            };
            recordsint.insert(entity, value);
        }
    });
}

pub fn sys_create_animation_group(
    mut cmds: ResMut<ActionListAnimeGroupCreate>,
    // mut commands: Commands,
    mut scenes: Query<&mut SceneAnimationContext>,
    mut globals: ResMut<GlobalAnimeAbout>,
    mut alter: Alter<(), (), BundleAnimGroup, ()>,
) {
    cmds.drain().for_each(|OpsAnimationGroupCreation(scene, entity)| {
        if let Ok(mut ctx) = scenes.get_mut(scene) {
            // if let Some(mut commands) = commands.get_entity(entity) {
                let id_group = ctx.0.create_animation_group();

                let bundle = (AnimationGroupKey(id_group), AnimationGroupScene(scene));
                // commands.insert(bundle);
                alter.alter(entity, bundle);

                globals.record_group(id_group, entity);
            // }
        }
    });
}
/// 顺序严格的
pub fn sys_act_animation_group_action(
    mut cmdsaction: ResMut<ActionListAnimationGroupAction>,
    items: Query<(&AnimationGroupKey, &AnimationGroupScene)>,
    mut scenes: Query<&mut SceneAnimationContext>,
    mut errors: ResMut<ErrorRecord>,
    mut globals: ResMut<GlobalAnimeAbout>,
) {
    cmdsaction.drain().for_each(|act| {
        match act {
            OpsAnimationGroupAction::Start(entity, param, delay_time_ms, fillmode) => if let Ok( (groupkey, idscene) ) = items.get(entity) {
                if let Ok(mut ctx) = scenes.get_mut(idscene.0) {
                    match ctx.0.start_with_progress(groupkey.0, param.speed, param.loop_mode, param.from, param.to, param.fps, param.amountcalc, delay_time_ms, fillmode) {
                        Ok(_) => {},
                        Err(_) => { errors.record(entity, ErrorRecord::ERROR_ANIMATION_START_FAIL); },
                    }
                }
            },
            OpsAnimationGroupAction::Pause(entity) => if let Ok( (groupkey, idscene) ) = items.get(entity) {
                if let Ok(mut ctx) = scenes.get_mut(idscene.0) {
                    match ctx.0.pause(groupkey.0) {
                        Ok(_) => {},
                        Err(_) => { errors.record(entity, ErrorRecord::ERROR_ANIMATION_PAUSE_FAIL); },
                    }
                }
            },
            OpsAnimationGroupAction::Stop(entity) => if let Ok( (groupkey, idscene) ) = items.get(entity) {
                if let Ok(mut ctx) = scenes.get_mut(idscene.0) {
                    match ctx.0.stop(groupkey.0) {
                        Ok(_) => {},
                        Err(_) => { errors.record(entity, ErrorRecord::ERROR_ANIMATION_STOP_FAIL); },
                    }
                }
            },
            OpsAnimationGroupAction::AddTarget(entity, target, animation) => {
                if let Ok( (groupkey, idscene) ) = items.get(entity) {
                    if let Ok(mut ctx) = scenes.get_mut(idscene.0) {
                        match ctx.0.add_target_animation_notype(animation, groupkey.0, target) {
                            Ok(_) => {},
                            Err(_) => { errors.record(entity, ErrorRecord::ERROR_ADD_TARGET_ANIMATION_FAIL); },
                        }
                    }
                }
            },
            OpsAnimationGroupAction::FrameEvent(entity, percent, data) => {
                if let Ok( (groupkey, _idscene) ) = items.get(entity) { globals.add_frame_event(groupkey.0, percent, data); }
            },
            OpsAnimationGroupAction::ListenFrame(entity) => { if let Ok( (groupkey, _idscene) ) = items.get(entity) { globals.add_frame_event_listen(groupkey.0); } },
            OpsAnimationGroupAction::ListenStart(entity) => { if let Ok( (groupkey, _idscene) ) = items.get(entity) { globals.add_start_listen(groupkey.0); } },
            OpsAnimationGroupAction::ListenLoop(entity) => { if let Ok( (groupkey, _idscene) ) = items.get(entity) { globals.add_loop_listen(groupkey.0); } },
            OpsAnimationGroupAction::ListenEnd(entity) => { if let Ok( (groupkey, _idscene) ) = items.get(entity) { globals.add_end_listen(groupkey.0); } },
            OpsAnimationGroupAction::Weight(entity, weight) => {
                // todo!()
            },
        }
    });
}

pub fn sys_act_dispose_animation_group(
    mut cmdsdispose: ResMut<ActionListAnimeGroupDispose>,
    items: Query<(&AnimationGroupKey, &AnimationGroupScene)>,
    mut scenes: Query<&mut SceneAnimationContext>,
    mut disposecan: ResMut<ActionListDisposeCan>,
    mut globals: ResMut<GlobalAnimeAbout>,
) {
    cmdsdispose.drain().for_each(|OpsAnimationGroupDispose(entity)| {
        if let Ok( (groupkey, idscene) ) = items.get(entity) {
            if let Ok(mut ctx) = scenes.get_mut(idscene.0) {
                ctx.0.del_animation_group(groupkey.0);
            }
            globals.remove(&groupkey.0);
        }

        disposecan.push(OpsDisposeCan::ops(entity));
    });
}

pub fn sys_act_reset_while_animationgroup_start(
    mut cmdsresetwhilestart: ResMut<ActionListAnimeGroupStartReset>,
    groups: Query<(&AnimationGroupKey, &AnimationGroupScene)>,
    scenes: Query<&SceneAnimationContext>,
    mut items: Query<&mut FlagAnimationStartResetComp>,
) {
    cmdsresetwhilestart.drain().for_each(|OpsAnimationGroupStartReset(entity)| {
        if let Ok((groupkey, idscene)) = groups.get(entity) {
            if let Ok(ctx) = scenes.get(idscene.0) {
                if let Some(animationgroup) = ctx.0.animation_group(groupkey.0) {
                    animationgroup.animations().iter().for_each(|v| {
                        if let Ok(mut flag) = items.get_mut(v.target) {
                            *flag = FlagAnimationStartResetComp;
                        }
                    });
                }
            }
        } 
    });
}


/// 动画计算
pub fn sys_calc_type_anime<D: TAnimatableComp>(
    resetlist: ComponentChanged<FlagAnimationStartResetComp>,
    mut resetitems: Query<&mut D>,
    mut records: ResMut<AnimeTargetRecordValues<D>>,

    dispose: ComponentChanged<DisposeCan>,
    disposeitems: Query<&DisposeCan>,

    type_ctx: Res<TypeAnimeContext<D>>,
    runinfos: Res<GlobalAnimeAbout>,
    mut items: Query<&mut D>,
    links: Query<&AnimatorableLink>,
    mut linkeds: Query<&mut TargetAnimatorableIsRunning>,
    mut performance: ResMut<Performance>,
) {
    let time = if performance.debug { Some(pi_time::Instant::now()) } else { None };
    
    // 动画启动前将目标值 重置 为操作修改的值
    resetlist.iter().for_each(|_entity| {
        if let Ok(mut comp) = resetitems.get_mut(*_entity) {
            if let Some(record) = records.get(&_entity) {
                *comp = record.clone();
            } else {
                // log::error!("sys_calc_reset_animatablecomp {:?}", entity);
                *comp = D::default();
            }
        }

        if let Ok(linked) = links.get(*_entity) {
            if let Ok(mut item) = linkeds.get_mut(linked.deref().clone()) {
                *item = TargetAnimatorableIsRunning;
            }
        }
    });
    dispose.iter().for_each(|entity| {
        if let Ok(isdispose) = disposeitems.get(*entity) {
            if isdispose.0 {
                records.remove(&entity);
            }
        }
    });

    let ty = type_ctx.ctx.ty();
    // log::warn!("Anime Run ");
    let curves = type_ctx.ctx.curves();
    if let Some(map) = runinfos.runtimeinfos.get_type_list(ty) {

        for (target, info) in map {
            let mut last_value: D = D::default();
            let mut last_weight: f32 = 0.;

            if let Ok(mut item) = items.get_mut(*target) {
                let mut enable = false;
                info.iter().for_each(|info| {
                    if let Some(Some(curve)) = curves.get(info.curve_id) {
                        // log::error!("{:?}", (info.amount_in_second));
                        let value = curve.as_ref().interple(info.amount_in_second, &info.amount_calc);
                        last_weight += info.group_weight;
                        last_value  = last_value.interpolate(&value, info.group_weight / last_weight);
                    }
                    enable = true;
                });
                
                if enable {
                    *item = last_value;
                    if let Ok(linked) = links.get(*target) {
                        if let Ok(mut item) = linkeds.get_mut(linked.deref().clone()) {
                            *item = TargetAnimatorableIsRunning;
                        }
                    }
                }
            } else {
                // log::warn!("Animation Target NotFound:");
            }
        }
    } else {
        // // log::trace!("Not Found Anime Type: {}", ty);
    }

    if performance.debug { performance.animation += (pi_time::Instant::now() - time.unwrap()).as_micros() as u32; }
}

pub(crate) fn sys_apply_removed_data<D: TAnimatableComp>(
    mut type_ctx: ResMut<TypeAnimeContext<D>>,
    scenes: Query<& SceneAnimationContext>,
) {
    scenes.iter().for_each(| ctx | {
        ctx.0.apply_removed_animations(&mut type_ctx.ctx);
    });
}

pub fn sys_reset_anime_performance(
    mut performance: ResMut<Performance>,
) {
    performance.animation = 0;
}
