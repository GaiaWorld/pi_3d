

use pi_animation::animation_group_manager::AnimationGroupManager;
use pi_curves::curve::frame::KeyFrameCurveValue;
use pi_engine_shell::prelude::*;

use crate::{
    scene::environment::scene_time::SceneTime,
    commands::DisposeReady,
    prelude::SceneAnimationEnable
};


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
    mut scenes: Query<(Entity, &SceneTime, &SceneAnimationEnable)>,
    mut animeglobal: ResMut<GlobalAnimeAbout>,
    mut scenectxs: ResMut<SceneAnimationContextMap>,
    mut animeevents: ResMut<GlobalAnimeEvents>,
    mut performance: ResMut<Performance>,
) {
    let time0 = pi_time::Instant::now();

    animeglobal.runtimeinfos.reset();
    scenes.iter_mut().for_each(|(id_scene, scene_time, enable)| {
        let ctx = if let Some(ctx) = scenectxs.get_mut(&id_scene) {
            ctx
        } else { return; };

        if enable.0 == false {
            return;
        }

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
                    let group = group_mgr.get_mut(id_group).unwrap();
                    group.anime(&mut animeglobal.runtimeinfos, delta_ms, group_info);
                }

                if let Some((idobj, frameevents, listen)) = animeglobal.group_records.get(&id_group) {
                    // log::warn!("Group : {:?}", listen);
                    if group_info.start_event && (listen & TagGroupListen::START) == TagGroupListen::START {
                        animeevents.push((*idobj, id_group, TagGroupListen::START, 0));
                    }
    
                    if (listen & TagGroupListen::FRAME) == TagGroupListen::FRAME {
                        if let Some(data) = frameevents.query(group_info.last_amount_in_second, group_info.amount_in_second) {
                            data.iter().for_each(|v| {
                                animeevents.push((*idobj, id_group, TagGroupListen::FRAME, *v));
                            });
                        }
                    }

                    if group_info.loop_event && (listen & TagGroupListen::LOOP) == TagGroupListen::LOOP {
                        animeevents.push((*idobj, id_group, TagGroupListen::LOOP, group_info.looped_count as u32));
                    }

                    if group_info.end_event && (listen & TagGroupListen::END) == TagGroupListen::END {
                        animeevents.push((*idobj, id_group, TagGroupListen::END, 0));
                    }
                };
            }
        }

    });

    let time1 = pi_time::Instant::now();
    performance.animationgroup = (time1 - time0).as_micros() as u32;
    log::debug!("SysSceneAnime: {:?}", time1 - time0);
}


pub fn sys_dispose_about_animationgroup(
    items: Query<(&DisposeReady, &SceneID, &AnimationGroups), Changed<DisposeReady>>,
    mut animeglobal: ResMut<GlobalAnimeAbout>,
    mut scenectxs: ResMut<SceneAnimationContextMap>,
) {
    items.iter().for_each(|(state, scene, groups)| {
        if state.0 == false { return; }

        groups.map.iter().for_each(|(_k, id_group)| {
            scenectxs.delete_group(&scene.0, *id_group);
            animeglobal.remove(id_group);
        });
    });
}