use std::{marker::PhantomData, time::Instant, fmt::Debug, mem::replace};

use pi_curves::curve::{frame::{FrameDataValue}};
use pi_engine_shell::prelude::*;

use crate::{flags::SceneID, scene::environment::scene_time::SceneTime};

use super::{base::{TypeAnimeContext, GlobalAnimeAbout, SceneAnimationContext, AnimationGroups}};

pub fn sys_listen_scene_anime_ctx(
    e: Event,
    items: Query<(&SceneID, &AnimationGroups)>,
    mut scenes: Query<&mut SceneAnimationContext>,
) {
    
    // log::debug!("Obj Dispose > SysSceneAnime 0");
    if let Some((id_scene, groups)) = items.get_by_entity(e.id) {
        // log::debug!("Obj Dispose > SysSceneAnime 1");
        if let Some(mut ctx) = scenes.get_mut(id_scene.0) {
            // log::debug!("Obj Dispose > SysSceneAnime 2");
            groups.map.iter().for_each(|(_, id_group)| {
                ctx.0.pause(id_group.clone());
                ctx.1.push(id_group.clone());
            });
        }
    }
}
// #[system]
pub fn sys_scene_anime_ctx(
    mut scenes: Query<(&mut SceneAnimationContext, &SceneTime)>,
    mut runtimeinfos: ResMut<GlobalAnimeAbout>,
) {
    let time0 = Instant::now();

    runtimeinfos.dispose_animations.clear();
    runtimeinfos.runtimeinfos.reset();
    scenes.iter_mut().for_each(|(mut ctx, scene_time)| {

        let mut dispose_groups = replace(&mut ctx.1, vec![]);

        dispose_groups.drain(..).for_each(|id_group| {
            ctx.0.del_animation_group(id_group);
        });

        ctx.0.anime_curve_calc(scene_time.delta_ms, &mut runtimeinfos.runtimeinfos)
    });

    let time1 = Instant::now();
    log::debug!("SysSceneAnime: {:?}", time1 - time0);
}

// /// 动画数据计算
// pub struct SysTypeAnime<D: FrameDataValue + Component + Debug>(PhantomData<D>);
// impl<D: FrameDataValue + Component + Debug> TSystemStageInfo for SysTypeAnime<D> {
//     fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
//         vec![
//             SysSceneAnime::key()
//         ]
//     }
// }
// #[setup]
// impl<D: FrameDataValue + Component + Debug> SysTypeAnime<D> {
//     #[system]
    pub fn sys_calc_type_anime<D: FrameDataValue + Component + Debug>(
        mut type_ctx: ResMut<TypeAnimeContext<D>>,
        mut runinfos: ResMut<GlobalAnimeAbout>,
        mut update_cmd: Commands,
    ) {
        let time0 = Instant::now();

        let ty = type_ctx.ctx.ty();
        let curves = type_ctx.ctx.curves();
        if let Some(list) = runinfos.runtimeinfos.list.get(ty) {
            for info in list {
                if let Some(Some(curve)) = curves.get(info.curve_id) {
                    // println!(">>>>>>>>>>>>>>>>>{}", info.amount_in_second);
                    let value = curve.as_ref().interple(info.amount_in_second);
                    // let result = AnimeResult {
                    //     value,
                    //     attr: info.attr,
                    //     weight: info.group_weight,
                    // };
                    // result_pool.record_result(info.target.clone(), info.attr, result);
    
                    // log::debug!("update_cmd: {:?}", value);
                    update_cmd.insert(info.target, value);
                }
            }
        } else {
            log::trace!("Not Found Anime Type: {}", ty);
        }

        let time1 = Instant::now();
        // log::debug!("{}: {:?}", Self::key(), time1 - time0);
    }

// /// 动画数据销毁
// pub struct SysTypeAnimeDispose<D: FrameDataValue + Component + Debug>(PhantomData<D>);
// impl<D: FrameDataValue + Component + Debug> TSystemStageInfo for SysTypeAnimeDispose<D> {
//     fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
//         vec![
//             SysAnimeModifyCommand::key()
//         ]
//     }
// }

// #[setup]
// impl<D: FrameDataValue + Component + Debug> SysTypeAnimeDispose<D> {
//     #[listen(entity=(GameObject, Delete))]
    pub(crate) fn sys_listen_type_anime_ctx<D: FrameDataValue + Component + Debug>(
        e: Event,
        items: Query<(&SceneID, &AnimationGroups)>,
        mut scenes: Query<&mut SceneAnimationContext>,
        mut type_ctx: ResMut<TypeAnimeContext<D>>,
    ) {
        if let Some((id_scene, groups)) = items.get_by_entity(e.id) {
            if let Some(ctx) = scenes.get_mut(id_scene.0) {
                log::debug!("Obj Dispose > SysTypeAnimeDispose");
                groups.map.iter().for_each(|(_, id_group)| {
                    if let Some(group) = ctx.0.animation_group(id_group.clone()) {
                        group.animations().iter().for_each(|item| {
                            type_ctx.ctx.remove_one(&item.animation);
                        });
                    }
                });
            }
        }
    }
// }