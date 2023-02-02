use std::{sync::Arc, marker::PhantomData, time::Instant, fmt::Debug};

use pi_animation::{type_animation_context::{TTypeFrameCurve, TypeAnimationContext, AnimationContextAmount}, animation_result_pool::TypeAnimationResultPoolDefault, animation_group_manager::AnimationGroupManagerDefault, animation::AnimationInfo, target_animation::TargetAnimation, animation_group::AnimationGroupID};
use pi_assets::{asset::{Handle, GarbageEmpty}, mgr::AssetMgr};
use pi_atom::Atom;
use pi_curves::curve::{frame::{FrameDataValue, KeyFrameDataTypeAllocator}, frame_curve::FrameCurve};
use pi_ecs::prelude::{Query, ResMut, Component, Commands, Setup, Res};
use pi_ecs_macros::setup;
use pi_engine_shell::{object::{ObjectID, GameObject}, run_stage::{TSystemStageInfo, ERunStageChap}, plugin::Plugin, setup};

use crate::scene::scene_time::SceneTime;

use super::{base::{TypeAnimeContext, GlobalAnimeAbout, SceneAnimationContext}, command::SysAnimeModifyCommand};

/// 动画进度计算
pub struct SysSceneAnime;
impl TSystemStageInfo for SysSceneAnime {

}
#[setup]
impl SysSceneAnime {
    #[system]
    fn sys(
        mut scenes: Query<GameObject, (&mut SceneAnimationContext, &SceneTime)>,
        mut runtimeinfos: ResMut<GlobalAnimeAbout>,
    ) {
        let time0 = Instant::now();

        runtimeinfos.dispose_animations.clear();
        runtimeinfos.runtimeinfos.reset();
        scenes.iter_mut().for_each(|(mut ctx, scene_time)| {
            ctx.0.anime_curve_calc(scene_time.delta_ms, &mut runtimeinfos.runtimeinfos)
        });

        let time1 = Instant::now();
        log::info!("SysSceneAnime: {:?}", time1 - time0);
    }
}

/// 动画数据计算
pub struct SysTypeAnime<D: FrameDataValue + Component + Debug>(PhantomData<D>);
impl<D: FrameDataValue + Component + Debug> TSystemStageInfo for SysTypeAnime<D> {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysSceneAnime::key()
        ]
    }
}
#[setup]
impl<D: FrameDataValue + Component + Debug> SysTypeAnime<D> {
    #[system]
    fn sys(
        mut type_ctx: ResMut<TypeAnimeContext<D>>,
        mut runinfos: ResMut<GlobalAnimeAbout>,
        mut update_cmd: Commands<GameObject, D>,
    ) {
        let time0 = Instant::now();

        let ty = type_ctx.ctx.ty();
        let curves = type_ctx.ctx.curves();
        if let Some(list) = runinfos.runtimeinfos.list.get(ty) {
            for info in list {
                if let Some(Some(curve)) = curves.get(info.curve_id) {
                    // println!(">>>>>>>>>>>>>>>>>{}", info.amount_in_second);
                    let value = curve.curve().interple(info.amount_in_second);
                    // let result = AnimeResult {
                    //     value,
                    //     attr: info.attr,
                    //     weight: info.group_weight,
                    // };
                    // result_pool.record_result(info.target.clone(), info.attr, result);
    
                    // log::info!("update_cmd: {:?}", value);
                    update_cmd.insert(info.target, value);
                }
            }
        } else {
            log::trace!("Not Found Anime Type: {}", ty);
        }

        let time1 = Instant::now();
        log::info!("{}: {:?}", Self::key(), time1 - time0);
    }
}

/// 动画数据销毁
pub struct SysTypeAnimeDispose<D: FrameDataValue + Component + Debug>(PhantomData<D>);
impl<D: FrameDataValue + Component + Debug> TSystemStageInfo for SysTypeAnimeDispose<D> {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysAnimeModifyCommand::key()
        ]
    }
}

#[setup]
impl<D: FrameDataValue + Component + Debug> SysTypeAnimeDispose<D> {
    #[system]
    fn sys(
        mut type_ctx: ResMut<TypeAnimeContext<D>>,
        runinfos: Res<GlobalAnimeAbout>,
    ) {
        let ty = type_ctx.ctx.ty();
        runinfos.dispose_animations.iter().for_each(|item| {
            if item.ty == ty {
                type_ctx.ctx.remove_one(item);
            }
        });
    }
}
