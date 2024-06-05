use crate::ecs::*;
use crate::object::BundleEntity;

use std::ops::Deref;

use crate::object::ActionEntity;
use crate::object::ActionListDisposeCan;
use crate::object::DisposeCan;
use crate::object::DisposeReady;
use crate::object::OpsDisposeCan;
use crate::prelude::{Performance, ErrorRecord};

use super::RecordAnimatorableUint;
use super::base::*;
use super::command::*;
use super::float::RecordAnimatorableFloat;
use super::int::RecordAnimatorableInt;
use super::vec2::RecordAnimatorableVec2;
use super::vec3::RecordAnimatorableVec3;
use super::vec4::RecordAnimatorableVec4;
use crate::animation::*;

pub type BundleAnimFloatA = (AnimatorableFloat, AnimatorableLink, RecordAnimatorableFloat, AnimatorableAttribute);
pub type BundleAnimFloatU = (AnimatorableFloat, AnimatorableLink, RecordAnimatorableFloat, AnimatorableUniform);
pub type BundleAnimVec2A  = (AnimatorableVec2,  AnimatorableLink, RecordAnimatorableVec2,  AnimatorableAttribute);
pub type BundleAnimVec2U  = (AnimatorableVec2,  AnimatorableLink, RecordAnimatorableVec2,  AnimatorableUniform);
pub type BundleAnimVec3A  = (AnimatorableVec3,  AnimatorableLink, RecordAnimatorableVec3,  AnimatorableAttribute);
pub type BundleAnimVec3U  = (AnimatorableVec3,  AnimatorableLink, RecordAnimatorableVec3,  AnimatorableUniform);
pub type BundleAnimVec4A  = (AnimatorableVec4,  AnimatorableLink, RecordAnimatorableVec4,  AnimatorableAttribute);
pub type BundleAnimVec4U  = (AnimatorableVec4,  AnimatorableLink, RecordAnimatorableVec4,  AnimatorableUniform);
pub type BundleAnimUintA  = (AnimatorableUint,  AnimatorableLink, RecordAnimatorableUint,  AnimatorableAttribute);
pub type BundleAnimUintU  = (AnimatorableUint,  AnimatorableLink, RecordAnimatorableUint,  AnimatorableUniform);
pub type BundleAnimSintA  = (AnimatorableSint,  AnimatorableLink, RecordAnimatorableInt,   AnimatorableAttribute);
pub type BundleAnimSintU  = (AnimatorableSint,  AnimatorableLink, RecordAnimatorableInt,   AnimatorableUniform);
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
    // mut alters: (
    //     Alter<(), (), EntityBundle, ()>,
    //     Alter<(), (), BundleAnimFloatA, ()>,
    //     Alter<(), (), BundleAnimFloatU, ()>,
    //     Alter<(), (), BundleAnimVec2A, ()>,
    //     Alter<(), (), BundleAnimVec2U, ()>,
    //     Alter<(), (), BundleAnimVec3A, ()>,
    //     Alter<(), (), BundleAnimVec3U, ()>,
    //     Alter<(), (), BundleAnimVec4A, ()>,
    //     Alter<(), (), BundleAnimVec4U, ()>,
    //     Alter<(), (), BundleAnimUintA, ()>,
    //     Alter<(), (), BundleAnimUintU, ()>,
    //     Alter<(), (), BundleAnimSintA, ()>,
    //     Alter<(), (), BundleAnimSintU, ()>,
    // )
) {
    cmds_float.drain().drain(..).for_each(|OpsAnimatorableFloat(entity, linked, value, etype)| {
        if let Some(mut cmd) = commands.get_entity(entity) {
            if items.contains(entity) == false {
                let bundle = ActionEntity::init();
                cmd.insert(bundle);
                // alters.0.alter(entity, bundle);
            }
            match etype {
                EAnimatorableEntityType::Uniform => {
                    let bundle = (value.clone(), AnimatorableLink(linked), RecordAnimatorableFloat(value.clone()), AnimatorableUniform);
                    cmd.insert(bundle);
                    // alters.2.alter(entity, bundle);
                },
                EAnimatorableEntityType::Attribute => {
                    let bundle = (value.clone(), AnimatorableLink(linked), RecordAnimatorableFloat(value.clone()), AnimatorableAttribute);
                    cmd.insert(bundle);
                    // alters.1.alter(entity, bundle);
                },
            };
        }
    });
    cmds_vec2.drain().drain(..).for_each(|OpsAnimatorableVec2(entity, linked, value, etype)| {
        if let Some(mut cmd) = commands.get_entity(entity) {
            if items.contains(entity) == false { cmd.insert(ActionEntity::init()); }
            match etype {
                EAnimatorableEntityType::Uniform => {
                    let bundle = (value.clone(), AnimatorableLink(linked), RecordAnimatorableVec2(value.clone()), AnimatorableUniform);
                    cmd.insert(bundle);
                    // alters.4.alter(entity, bundle);
                },
                EAnimatorableEntityType::Attribute => {
                    let bundle = (value.clone(), AnimatorableLink(linked), RecordAnimatorableVec2(value.clone()), AnimatorableAttribute);
                    cmd.insert(bundle);
                    // alters.3.alter(entity, bundle);
                },
            };
        }
    });
    cmds_vec3.drain().drain(..).for_each(|OpsAnimatorableVec3(entity, linked, value, etype)| {
        if let Some(mut cmd) = commands.get_entity(entity) {
            if items.contains(entity) == false { cmd.insert(ActionEntity::init()); }
            match etype {
                EAnimatorableEntityType::Uniform => {
                    let bundle = (value.clone(), AnimatorableLink(linked), RecordAnimatorableVec3(value.clone()), AnimatorableUniform);
                    cmd.insert(bundle);
                    // alters.6.alter(entity, bundle);
                },
                EAnimatorableEntityType::Attribute => {
                    let bundle = (value.clone(), AnimatorableLink(linked), RecordAnimatorableVec3(value.clone()), AnimatorableAttribute);
                    cmd.insert(bundle);
                    // alters.5.alter(entity, bundle);
                },
            };
        }
    });
    cmds_vec4.drain().drain(..).for_each(|OpsAnimatorableVec4(entity, linked, value, etype)| {
        if let Some(mut cmd) = commands.get_entity(entity) {
            if items.contains(entity) == false { cmd.insert(ActionEntity::init()); }
            match etype {
                EAnimatorableEntityType::Uniform => {
                    let bundle = (value.clone(), AnimatorableLink(linked), RecordAnimatorableVec4(value.clone()), AnimatorableUniform);
                    cmd.insert(bundle);
                    // alters.8.alter(entity, bundle);
                },
                EAnimatorableEntityType::Attribute => {
                    let bundle = (value.clone(), AnimatorableLink(linked), RecordAnimatorableVec4(value.clone()), AnimatorableAttribute);
                    cmd.insert(bundle);
                    // alters.7.alter(entity, bundle);
                },
            };
        }
    });
    // cmds_mat4.drain().drain(..).for_each(|OpsAnimatorableMat4(entity, linked, value)| {
    //     if let Some(mut cmd) = commands.get_entity(entity) {
    //         cmd.insert(value.clone(), AnimatorableLink(linked), RecordAnimatorableVec4(value.clone()));
    //     }
    // });
    cmds_uint.drain().drain(..).for_each(|OpsAnimatorableUint(entity, linked, value, etype)| {
        if let Some(mut cmd) = commands.get_entity(entity) {
            if items.contains(entity) == false { cmd.insert(ActionEntity::init()); }
            match etype {
                EAnimatorableEntityType::Uniform => {
                    let bundle = (value.clone(), AnimatorableLink(linked), RecordAnimatorableUint(value.clone()), AnimatorableUniform);
                    cmd.insert(bundle);
                    // alters.10.alter(entity, bundle);
                },
                EAnimatorableEntityType::Attribute => {
                    let bundle = (value.clone(), AnimatorableLink(linked), RecordAnimatorableUint(value.clone()), AnimatorableAttribute);
                    cmd.insert(bundle);
                    // alters.9.alter(entity, bundle);
                },
            };
        }
    });
    cmds_int.drain().drain(..).for_each(|OpsAnimatorableSint(entity, linked, value, etype)| {
        if let Some(mut cmd) = commands.get_entity(entity) {
            if items.contains(entity) == false { cmd.insert(ActionEntity::init()); }
            match etype {
                EAnimatorableEntityType::Uniform => {
                    let bundle = (value.clone(), AnimatorableLink(linked), RecordAnimatorableInt(value.clone()), AnimatorableUniform);
                    cmd.insert(bundle);
                    // alters.12.alter(entity, bundle);
                },
                EAnimatorableEntityType::Attribute => {
                    let bundle = (value.clone(), AnimatorableLink(linked), RecordAnimatorableInt(value.clone()), AnimatorableAttribute);
                    cmd.insert(bundle);
                    // alters.11.alter(entity, bundle);
                },
            };
        }
    });
}

pub fn sys_create_animation_group(
    mut cmds: ResMut<ActionListAnimeGroupCreate>,
    mut commands: Commands,
    mut scenes: Query<&mut SceneAnimationContext>,
    mut globals: ResMut<GlobalAnimeAbout>,
    // mut alter: Alter<(), (), BundleAnimGroup, ()>,
) {
    cmds.drain().drain(..).for_each(|OpsAnimationGroupCreation(scene, entity)| {
        if let Ok(mut ctx) = scenes.get_mut(scene) {
            if let Some(mut commands) = commands.get_entity(entity) {
                let id_group = ctx.0.create_animation_group();

                let bundle = (AnimationGroupKey(id_group), AnimationGroupScene(scene));
                commands.insert(bundle);
                // alter.alter(entity, bundle);

                globals.record_group(id_group, entity);
            }
        }
    });
}
/// 顺序严格的
pub fn sys_act_animation_group_action(
    mut cmds: ResMut<ActionListAnimationGroupAction>,
    mut addtargetanime_cmds: ResMut<ActionListAddTargetAnime>,
    mut frameevent_cmds: ResMut<ActionListAddAnimationFrameEvent>,
    mut listen_cmds: ResMut<ActionListAddAnimationListen>,
    items: Query<(&AnimationGroupKey, &AnimationGroupScene)>,
    mut scenes: Query<&mut SceneAnimationContext>,
    mut errors: ResMut<ErrorRecord>,
    mut globals: ResMut<GlobalAnimeAbout>,
) {
    addtargetanime_cmds.drain().drain(..).for_each(|OpsAddTargetAnimation(entity, target, animation)| {
        if let Ok( (groupkey, idscene) ) = items.get(entity) {
            if let Ok(mut ctx) = scenes.get_mut(idscene.0) {
                match ctx.0.add_target_animation_notype(animation, groupkey.0, target) {
                    Ok(_) => {},
                    Err(_) => { errors.record(entity, ErrorRecord::ERROR_ADD_TARGET_ANIMATION_FAIL); },
                }
            }
        }
    });
    frameevent_cmds.drain().drain(..).for_each(|OpsAddAnimationFrameEvent(entity, percent, data)| {
        if let Ok( (groupkey, _idscene) ) = items.get(entity) { globals.add_frame_event(groupkey.0, percent, data); }
    });
    listen_cmds.drain().drain(..).for_each(|listen| {
        match listen {
            OpsAddAnimationListen::Frame(entity) => { if let Ok( (groupkey, _idscene) ) = items.get(entity) { globals.add_frame_event_listen(groupkey.0); } },
            OpsAddAnimationListen::Start(entity) => { if let Ok( (groupkey, _idscene) ) = items.get(entity) { globals.add_start_listen(groupkey.0); } },
            OpsAddAnimationListen::Loop(entity) => { if let Ok( (groupkey, _idscene) ) = items.get(entity) { globals.add_loop_listen(groupkey.0); } },
            OpsAddAnimationListen::End(entity) => { if let Ok( (groupkey, _idscene) ) = items.get(entity) { globals.add_end_listen(groupkey.0); } },
        }
    });
    cmds.drain().drain(..).for_each(|act| {
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
        }
    });
}

pub fn sys_act_dispose_animation_group(
    mut cmds: ResMut<ActionListAnimeGroupDispose>,
    items: Query<(&AnimationGroupKey, &AnimationGroupScene)>,
    mut scenes: Query<&mut SceneAnimationContext>,
    mut disposecan: ResMut<ActionListDisposeCan>,
    mut globals: ResMut<GlobalAnimeAbout>,
) {
    cmds.drain().drain(..).for_each(|OpsAnimationGroupDispose(entity)| {
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
    mut cmds: ResMut<ActionListAnimeGroupStartReset>,
    groups: Query<(&AnimationGroupKey, &AnimationGroupScene)>,
    scenes: Query<&SceneAnimationContext>,
    mut items: Query<&mut FlagAnimationStartResetComp>,
) {
    cmds.drain().drain(..).for_each(|OpsAnimationGroupStartReset(entity)| {
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

/// 动画结束后将目标值 重置 为操作修改的值
pub fn sys_calc_reset_animatablecomp<D: TAnimatableComp, R: TAnimatableCompRecord<D>>(
    mut items: Query<(Entity, &mut D, Option<&R>, Option<&AnimatorableLink>), Changed<FlagAnimationStartResetComp>>,
    mut linkeds: Query<&mut TargetAnimatorableIsRunning>,
) {
    items.iter_mut().for_each(|(_entity, mut comp, record, linked)| {
        if let Some(record) = record {
            *comp = record.comp();
        } else {
            // log::error!("sys_calc_reset_animatablecomp {:?}", entity);
            *comp = D::default();
        }
        if let Some(linked) = linked {
            if let Ok(mut item) = linkeds.get_mut(linked.deref().clone()) {
                *item = TargetAnimatorableIsRunning;
            }
        }
    });
}

/// 动画计算
pub fn sys_calc_type_anime<D: TAnimatableComp>(
    type_ctx: Res<TypeAnimeContext<D>>,
    runinfos: Res<GlobalAnimeAbout>,
    mut items: Query<(&mut D, Option<&AnimatorableLink>)>,
    mut linkeds: Query<&mut TargetAnimatorableIsRunning>,
    mut performance: ResMut<Performance>,
    // empty: Res<SingleEmptyEntity>,
) {
    let time0 = pi_time::Instant::now();

    let ty = type_ctx.ctx.ty();
    // log::warn!("Anime Run ");
    let curves = type_ctx.ctx.curves();
    if let Some(map) = runinfos.runtimeinfos.get_type_list(ty) {

        for (target, info) in map {
            let mut last_value: D = D::default();
            let mut last_weight: f32 = 0.;

            if let Ok((mut item, linked)) = items.get_mut(*target) {
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
                    if let Some(linked) = linked {
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

    performance.animation += (pi_time::Instant::now() - time0).as_micros() as u32;
    // let time1 = pi_time::Instant::now();
    // log::debug!("sys_calc_type_anime : {:?}", time1 - time0);
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
