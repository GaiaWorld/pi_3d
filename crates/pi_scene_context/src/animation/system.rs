

use pi_animation::animation_group_manager::AnimationGroupManager;
use pi_curves::curve::frame::KeyFrameCurveValue;
use pi_scene_shell::prelude::*;

use crate::{
    scene::environment::scene_time::SceneTime,
    prelude::SceneAnimationEnable
};

pub fn sys_scene_anime_ctx(
    mut scenes: Query<(&SceneTime, &SceneAnimationEnable, &mut SceneAnimationContext, &mut SceneAnimationGroupGoto)>,
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
            let delta_ms = scene_time.delta_ms() as KeyFrameCurveValue * ctx.0.time_scale as KeyFrameCurveValue;
            // log::warn!("Scene Anime Deltatime  {:?} {:?} {:?}", delta_ms, scene_time.delta_ms(), ctx.0.time_scale);
            let ctx = &mut ctx.0;
            for (id_group, group_info) in ctx.group_infos.iter_mut() {
                group_info.start_event = false;
                group_info.end_event = false;
                group_info.loop_event = false;
                group_info.last_amount_in_second = group_info.amount_in_second;

                if group_info.is_playing == true {
                    let group_mgr = &mut ctx.group_mgr;
                    if let Some(group) = group_mgr.get_mut(id_group) {
                        group.anime(&mut animeglobal.runtimeinfos, delta_ms, group_info);
                    }
                }

                if let Some((idobj, frameevents, listen)) = animeglobal.group_records.get(&id_group) {
                    // log::warn!("Group : {:?}", listen);
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
                };
            }

            animegoto.drain().for_each(|(idgroup, amount)| {
                if let (Some(group_info), Some(group)) = (ctx.group_infos.get_mut(idgroup.0), ctx.group_mgr.get(idgroup.0)) {
                    group.goto_progress(amount, &mut animeglobal.runtimeinfos, group_info);
                }
            });
        }
    });

    if performance.debug { performance.animationgroup = (pi_time::Instant::now() - performance.t_animationgroup).as_micros() as u32; }
    // log::error!("SysSceneAnime: {:?}", animeevents.len());
}
