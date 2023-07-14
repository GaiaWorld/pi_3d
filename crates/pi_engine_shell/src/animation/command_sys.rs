

use bevy::{
    app::{ prelude::* }, ecs::prelude::*,
    ecs::system::{EntityCommands}, 
};
use pi_animation::animation::AnimationInfo;
use pi_atom::Atom;
use pi_bevy_asset::ShareAssetMgr;
use pi_curves::curve::{frame::FrameDataValue, frame_curve::FrameCurve};
use core::fmt::Debug;


use crate::prelude::SingleEmptyEntity;

use super::base::*;
use super::command::*;

pub fn sys_anime_group_attach(
    mut cmds: ResMut<ActionListAnimeGroupAttach>,
    mut obj: Query<&mut AnimationGroups>,
    mut global: ResMut<SceneAnimationContextMap>,
) {
    // let mut list = 
    cmds.drain().drain(..).for_each(|OpsAnimationGroupAttach(scene, id_obj, id_group, count)| {
        if let Ok(mut groups) = obj.get_mut(id_obj) {
            if groups.map.contains_key(&id_group) == false {
                groups.map.insert(id_group.clone(), id_group);
            }
        } else {
            if count < 4 {
                cmds.push(OpsAnimationGroupAttach(scene, id_obj, id_group, count + 1))
            } else {
                global.delete_group(&scene, id_group);
            }
        }
    });
}

// pub fn sys_anime_group_create(
//     mut cmds: ResMut<ActionListAnimeGroupCreate>,
//     mut obj: Query<&mut AnimationGroups>,
// ) {
//     // let mut list = 
//     cmds.drain().drain(..).for_each(|OpsAnimationGroupCreation(id_obj, key_group, id_group)| {
//         if let Ok(mut groups) = obj.get_mut(id_obj) {
//             if groups.map.contains_key(&id_group) == false {
//                 groups.map.insert(id_group.clone(), id_group);
//             }
//         } else {
//             cmds.push(OpsAnimationGroupCreation(id_obj, key_group, id_group))
//         }
//     });
// }

// pub fn sys_anime_pause(
//     mut cmds: ResMut<ActionListAnimeGroupPause>,
//     obj: Query<(&SceneID, &AnimationGroups)>,
//     mut scenectxs: ResMut<SceneAnimationContextMap>,
// ) {
//     cmds.drain().drain(..).for_each(|OpsAnimationGroupPause(id_obj, key_group)| {
//         if let Ok((id_scene, groups)) = obj.get(id_obj) {
//             if let Some(ctx) = scenectxs.get_mut(&id_scene.0) {
//                 if let Some(id_group) = groups.map.get(&key_group) {
//                     ctx.0.pause(id_group.clone());
//                 }
//             }
//         } else {
//             cmds.push(OpsAnimationGroupPause(id_obj, key_group))
//         }
//     });
// }


// pub fn sys_anime_start(
//     mut cmds: ResMut<ActionListAnimeGroupStart>,
//     obj: Query<(&SceneID, & AnimationGroups)>,
//     mut scenectxs: ResMut<SceneAnimationContextMap>,
// ) {
//     cmds.drain().drain(..).for_each(|OpsAnimationGroupStart(id_obj, key_group, param)| {
//         if let Ok((id_scene, groups)) = obj.get(id_obj) {
//             if let Some(ctx) = scenectxs.get_mut(&id_scene.0) {
//                 if let Some(id_group) = groups.map.get(&key_group) {
//                     ctx.0.start_with_progress(id_group.clone(), param.speed, param.loop_mode, param.from, param.to, param.fps, param.amountcalc);
//                 }
//             }
//         } else {
//             cmds.push(OpsAnimationGroupStart(id_obj, key_group, param))
//         }
//     });
// }

// pub fn sys_anime_add_target_anime(
//     mut cmds: ResMut<ActionListAddTargetAnime>,
//     obj: Query<(&SceneID, & AnimationGroups)>,
//     mut scenectxs: ResMut<SceneAnimationContextMap>,
// ) {
//     // log::warn!("AddTargetAnime");
//     cmds.drain().drain(..).for_each(|OpsAddTargetAnimation(id_obj, id_target, key_group, animation)| {
//         // log::warn!("AddTargetAnime 0");
//         if let Ok((id_scene, groups)) = obj.get(id_obj) {
//             // log::warn!("AddTargetAnime 1");
//             if let Some(id_group) = groups.map.get(&key_group) {
//                 // log::warn!("AddTargetAnime 2");
//                 if let Some(ctx) = scenectxs.get_mut(&id_scene.0) {
//                     // log::warn!("AddTargetAnime Ok");
//                     ctx.0.add_target_animation_notype(animation, id_group.clone(), id_target);
//                 }
//             }
//         } else {
//             cmds.push(OpsAddTargetAnimation(id_obj, id_target, key_group, animation))
//         }
//     });
// }

pub struct ActionAnime;
impl ActionAnime {
    pub fn as_anime_group_target(
        commands: &mut EntityCommands,
    ) {
        commands.insert(AnimationGroups::default());
    }
    pub fn create_animation<D: TAnimatableComp>(
        app: &mut App,
        curve: AssetTypeFrameCurve<D>,
    ) -> AnimationInfo {
        let mut type_ctx = app.world.get_resource_mut::<TypeAnimeContext<D>>().unwrap();
        type_ctx.ctx.create_animation(0, curve)
    }
    
    pub fn check_anim_curve<D: TAnimatableComp>(
        app: &mut App,
        key: &IDAssetTypeFrameCurve,
    ) -> Option<AssetTypeFrameCurve<D>> {
        if let Some(value) = app.world.get_resource::<ShareAssetMgr<TypeFrameCurve<D>>>().unwrap().get(key) {
            Some(
                AssetTypeFrameCurve::<D>::from(value)
            )
        } else {
            None
        }
    }

    pub fn creat_anim_curve<D: TAnimatableComp>(
        app: &mut App,
        key: &IDAssetTypeFrameCurve,
        curve: FrameCurve<D>,
    ) -> Result<AssetTypeFrameCurve<D>, TypeFrameCurve<D>> {
        match app.world.get_resource_mut::<ShareAssetMgr<TypeFrameCurve<D>>>().unwrap().insert(key.clone(), TypeFrameCurve(curve)) {
            Ok(value) => {
                Ok(AssetTypeFrameCurve::<D>::from(value))
            },
            Err(value) => Err(value),
        }

    }

}

pub fn sys_calc_reset_while_animationgroup_start(
    mut cmds: ResMut<ActionListAnimeGroupStartReset>,
    scenes: Res<SceneAnimationContextMap>,
    mut items: Query<&mut FlagAnimationStartResetComp>,
) {
    cmds.drain().drain(..).for_each(|OpsAnimationGroupStartReset(idscene, idgroup)| {
        if let Some(animations) = scenes.query_group_animations(idscene, idgroup) {
            animations.iter().for_each(|v| {
                if let Ok(mut flag) = items.get_mut(v.target) {
                    *flag = FlagAnimationStartResetComp;
                }
            });
        }
    });
}

pub fn sys_calc_reset_animatablecomp<D: TAnimatableComp, R: TAnimatableCompRecord<D>>(
    mut items: Query<(&mut D, Option<&R>), Changed<FlagAnimationStartResetComp>>,
) {
    items.iter_mut().for_each(|(mut comp, record)| {
        if let Some(record) = record {
            *comp = record.comp();
        } else {
            *comp = D::default();
        }
    });
}

pub fn sys_calc_type_anime<D: TAnimatableComp>(
    type_ctx: Res<TypeAnimeContext<D>>,
    runinfos: Res<GlobalAnimeAbout>,
    mut items: Query<&mut D>,
    empty: Res<SingleEmptyEntity>,
) {
    let time0 = pi_time::Instant::now();

    let ty = type_ctx.ctx.ty();
    // log::warn!("Anime Run ");
    let curves = type_ctx.ctx.curves();
    if let Some(list) = runinfos.runtimeinfos.list.get(ty) {
        // log::warn!("Anime Run 1");
            // log::warn!("Anime Run {:?}", list);
        let mut last_target = empty.id();
        let mut last_value: D = D::default();
        let mut last_weight: f32 = 0.;
        for info in list {
            // log::warn!("Anime Run 1.5  {:?}", info.target);
                // log::warn!("Anime Run 2");
                if let Some(Some(curve)) = curves.get(info.curve_id) {
                    // println!(">>>>>>>>>>>>>>>>>{}", info.amount_in_second);
                    let value = curve.as_ref().interple(info.amount_in_second);
                    // log::warn!("Anime Amount: {:?}, Result {:?}", info.amount_in_second, value);
                    // commands.entity(info.target).insert(value);
                    if last_target == info.target {
                        last_weight += info.group_weight;
                        last_value = last_value.interpolate(&value, info.group_weight / last_weight);
                    } else {
                        if let Ok(mut item) = items.get_mut(last_target) {
                            *item = last_value.clone();
                        }

                        last_weight = info.group_weight;
                        last_value = value;
                        last_target = info.target;
                    }

                }
        }
        if let Ok(mut item) = items.get_mut(last_target) {
            *item = last_value;
        }
    } else {
        log::trace!("Not Found Anime Type: {}", ty);
    }

    let time1 = pi_time::Instant::now();
    // log::debug!("sys_calc_type_anime : {:?}", time1 - time0);
}

pub(crate) fn sys_apply_removed_data<D: TAnimatableComp>(
    mut type_ctx: ResMut<TypeAnimeContext<D>>,
    scenes: Res<SceneAnimationContextMap>,
) {
    scenes.apply_removed_animations(&mut type_ctx.ctx);
}
