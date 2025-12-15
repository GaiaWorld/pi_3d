

use pi_animation::animation_group_manager::AnimationGroupManager;
use pi_curves::curve::frame::KeyFrameCurveValue;
use pi_scene_shell::prelude::*;

use crate::{
    scene::environment::scene_time::SceneTime,
    prelude::SceneAnimationEnable
};

pub fn sys_scene_anime_ctx(
    mut scenes: Query<(&SceneTime, &SceneAnimationEnable, &mut SceneAnimationContext, &mut SceneAnimationGroupGoto)>,
    mut amounts: ResMut<GlobalAnimationGroupsAmout>,
    mut animeglobal: ResMut<GlobalAnimeAbout>,
    mut animeevents: ResMut<GlobalAnimeEvents>,
    mut performance: ResMut<Performance>,
) {
    // performance.systems.push(String::from("sys_scene_anime_ctx"));
    if performance.debug { performance.t_animationgroup = pi_time::Instant::now(); }

    animeglobal.runtimeinfos.reset();
    scenes.iter_mut().for_each(|(scene_time, enable, mut ctx, mut animegoto)| {

        if enable.0 == false { return; }

        // ctx.0.anime_curve_calc(scene_time.delta_ms, &mut runtimeinfos.runtimeinfos);
        {
            let delta_ms = scene_time.delta_ms() as KeyFrameCurveValue * ctx.time_scale as KeyFrameCurveValue;
            // log::warn!("Scene Anime Deltatime  {:?} {:?} {:?}", delta_ms, scene_time.delta_ms(), ctx.0.time_scale);
            let amounts = &mut amounts.0;
            ctx.map.iter().for_each(|(idobj, id_group)| {
                let id_group = *id_group;
                match amounts.group_infos.get(id_group) {
                    Some(_group_info) => {
                        let mut group_info = *_group_info;
                        group_info.start_event = false;
                        group_info.end_event = false;
                        group_info.loop_event = false;
                        group_info.last_amount_in_second = group_info.amount_in_second;

                        if group_info.is_playing == true {
                            let group_mgr = &mut amounts.group_mgr;
                            if let Some(group) = group_mgr.get_mut(id_group) {
                                group.anime(&mut animeglobal.runtimeinfos, delta_ms, &mut group_info);
                            }

                            if let Some((idobj, frameevents, listen)) = animeglobal.group_records.get(&id_group) {
                                // if group_info.end_event {
                                //     log::warn!("Group : {:?}", (idobj, id_group, group_info.end_event, (listen & TagGroupListen::END) == TagGroupListen::END));
                                // }
                                if group_info.start_event && (listen & TagGroupListen::START) == TagGroupListen::START {
                                    animeevents.push((*idobj, *idobj, TagGroupListen::START, 0));
                                }
                
                                if (listen & TagGroupListen::FRAME) == TagGroupListen::FRAME {
                                    if let Some(data) = frameevents.query(group_info.last_amount_in_second, group_info.amount_in_second) {
                                        data.iter().for_each(|v| {
                                            animeevents.push((*idobj, *idobj, TagGroupListen::FRAME, *v));
                                        });
                                    }
                                }

                                if group_info.loop_event && (listen & TagGroupListen::LOOP) == TagGroupListen::LOOP {
                                    animeevents.push((*idobj, *idobj, TagGroupListen::LOOP, group_info.looped_count as u32));
                                }

                                if group_info.end_event && (listen & TagGroupListen::END) == TagGroupListen::END {
                                    animeevents.push((*idobj, *idobj, TagGroupListen::END, 0));
                                }
                            } else {
                                // log::error!("AnimEvene Error: {:?}", id_group);
                            };
                        }
                    
                        let item = amounts.group_infos.get_mut(id_group).unwrap();
                        *item = group_info;
                    },
                    None => {
                        
                    },
                }
            });

            animegoto.drain().for_each(|(idgroup, amount)| {
                if let (Some(group_info), Some(group)) = (amounts.group_infos.get_mut(idgroup.0), amounts.group_mgr.get(idgroup.0)) {
                    group.goto_progress(amount, &mut animeglobal.runtimeinfos, group_info);
                }
            });
        }
    });

    if performance.debug { performance.animationgroup = (pi_time::Instant::now() - performance.t_animationgroup).as_micros() as u32; }
    // log::error!("SysSceneAnime: {:?}", animeevents.len());
}
