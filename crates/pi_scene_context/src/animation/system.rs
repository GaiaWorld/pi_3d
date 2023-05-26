use std::{fmt::Debug, mem::replace};

use pi_animation::{animation_listener::EAnimationEvent, animation_group_manager::AnimationGroupManager};
use pi_curves::curve::{frame::{FrameDataValue, KeyFrameCurveValue}};
use pi_engine_shell::prelude::*;

use crate::{scene::environment::scene_time::SceneTime};

use super::{base::*};

pub fn sys_listen_scene_anime_ctx(
    // e: Event,
    // items: Query<(&SceneID, &AnimationGroups)>,
    // mut scenes: Query<&mut SceneAnimationContext>,
) {
    
    // // log::debug!("Obj Dispose > SysSceneAnime 0");
    // if let Some((id_scene, groups)) = items.get_by_entity(e.id) {
    //     // log::debug!("Obj Dispose > SysSceneAnime 1");
    //     if let Some(mut ctx) = scenes.get_mut(id_scene.0) {
    //         // log::debug!("Obj Dispose > SysSceneAnime 2");
    //         groups.map.iter().for_each(|(_, id_group)| {
    //             ctx.0.pause(id_group.clone());
    //             ctx.1.push(id_group.clone());
    //         });
    //     }
    // }
}
// #[system]
pub fn sys_scene_anime_ctx(
    mut scenes: Query<(Entity, &SceneTime)>,
    mut animeglobal: ResMut<GlobalAnimeAbout>,
    mut scenectxs: ResMut<SceneAnimationContextMap>,
    mut animeevents: ResMut<GlobalAnimeEvents>,
) {
    let time0 = pi_time::Instant::now();

    animeglobal.dispose_animations.clear();
    animeglobal.runtimeinfos.reset();
    scenes.iter_mut().for_each(|(id_scene, scene_time)| {
        let ctx = if let Some(ctx) = scenectxs.get_mut(&id_scene) {
            ctx
        } else { return; };

        let mut dispose_groups = replace(&mut ctx.1, vec![]);

        dispose_groups.drain(..).for_each(|id_group| {
            ctx.0.del_animation_group(id_group);
        });

        // ctx.0.anime_curve_calc(scene_time.delta_ms, &mut runtimeinfos.runtimeinfos);
        {
            let delta_ms = scene_time.delta_ms as KeyFrameCurveValue * ctx.0.time_scale as KeyFrameCurveValue;
            let ctx = &mut ctx.0;
            for (i, group_info) in ctx.group_infos.iter_mut() {
                group_info.start_event = false;
                group_info.end_event = false;
                group_info.loop_event = false;
                group_info.last_amount_in_second = group_info.amount_in_second;

                if group_info.is_playing == true {
                    let group_mgr = &mut ctx.group_mgr;
                    let group = group_mgr.get_mut(i).unwrap();
                    group.anime(&mut animeglobal.runtimeinfos, delta_ms, group_info);
                }

                if let Some((idobj, name, frameevents, listen)) = animeglobal.group_records.get(&i) {

                    if group_info.start_event && (listen & TagGroupListen::START) == TagGroupListen::START {
                        animeevents.push((*idobj, name.get_hash(), TagGroupListen::START, 0));
                    }
    
                    if (listen & TagGroupListen::FRAME) == TagGroupListen::FRAME {
                        if let Some(data) = frameevents.query(group_info.last_amount_in_second, group_info.amount_in_second) {
                            data.iter().for_each(|v| {
                                animeevents.push((*idobj, name.get_hash(), TagGroupListen::FRAME, *v));
                            });
                        }
                    }

                    if group_info.loop_event && (listen & TagGroupListen::LOOP) == TagGroupListen::LOOP {
                        animeevents.push((*idobj, name.get_hash(), TagGroupListen::LOOP, group_info.looped_count as u32));
                    }

                    if group_info.end_event && (listen & TagGroupListen::END) == TagGroupListen::END {
                        animeevents.push((*idobj, name.get_hash(), TagGroupListen::END, 0));
                    }
                };
            }
        }

    });

    let time1 = pi_time::Instant::now();
    log::debug!("SysSceneAnime: {:?}", time1 - time0);
}


    pub fn sys_calc_type_anime<D: FrameDataValue + Component + Debug>(
        type_ctx: Res<TypeAnimeContext<D>>,
        runinfos: Res<GlobalAnimeAbout>,
        mut items: Query<&mut D>,
    ) {
        let time0 = pi_time::Instant::now();

        let ty = type_ctx.ctx.ty();
        let curves = type_ctx.ctx.curves();
        if let Some(list) = runinfos.runtimeinfos.list.get(ty) {
            for info in list {
                if let Ok(mut item) = items.get_mut(info.target) {
                    if let Some(Some(curve)) = curves.get(info.curve_id) {
                        // println!(">>>>>>>>>>>>>>>>>{}", info.amount_in_second);
                        let value = curve.as_ref().interple(info.amount_in_second);
                        // commands.entity(info.target).insert(value);
                        *item = value;
                    }
                }
            }
        } else {
            log::trace!("Not Found Anime Type: {}", ty);
        }

        let time1 = pi_time::Instant::now();
        log::debug!("sys_calc_type_anime : {:?}", time1 - time0);
    }

    pub(crate) fn sys_listen_type_anime_ctx<D: FrameDataValue + Component + Debug>(
        // e: Event,
        // items: Query<(&SceneID, &AnimationGroups)>,
        // mut scenes: Query<&mut SceneAnimationContext>,
        // mut type_ctx: ResMut<TypeAnimeContext<D>>,
    ) {
        // if let Some((id_scene, groups)) = items.get_by_entity(e.id) {
        //     if let Some(ctx) = scenes.get_mut(id_scene.0) {
        //         log::debug!("Obj Dispose > SysTypeAnimeDispose");
        //         groups.map.iter().for_each(|(_, id_group)| {
        //             if let Some(group) = ctx.0.animation_group(id_group.clone()) {
        //                 group.animations().iter().for_each(|item| {
        //                     type_ctx.ctx.remove_one(&item.animation);
        //                 });
        //             }
        //         });
        //     }
        // }
    }
