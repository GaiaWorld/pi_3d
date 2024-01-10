
use pi_scene_shell::prelude::*;

use crate::prelude::{TypeAnimeAssetMgrs, TypeAnimeContexts};

use super::command::{ActionListPropertyTargetAnimation, OpsPropertyTargetAnimation, EPropertyAnimationValueType};

pub fn sys_act_add_property_target_animation(
    mut cmds: ResMut<ActionListPropertyTargetAnimation>,
    anime_assets: TypeAnimeAssetMgrs,
    mut anime_contexts: TypeAnimeContexts,
    mut targetanimations: ResMut<ActionListAddTargetAnime>,
) {
    cmds.drain().drain(..).for_each(|OpsPropertyTargetAnimation(target, group, vtype, key)| {
        match vtype {
            EPropertyAnimationValueType::LocalPosition => {
                if let Some(curve) = anime_assets.position.get(&key) {
                    let anime = anime_contexts.position.ctx.create_animation(0, AssetTypeFrameCurve::from(curve));
                    targetanimations.push(OpsAddTargetAnimation::ops(group, target, anime));
                }
            }
            EPropertyAnimationValueType::LocalEuler => {
                if let Some(curve) = anime_assets.euler.get(&key) {
                    let anime = anime_contexts.euler.ctx.create_animation(0, AssetTypeFrameCurve::from(curve));
                    targetanimations.push(OpsAddTargetAnimation::ops(group, target, anime));
                }
            }
            EPropertyAnimationValueType::LocalQuaternion => {
                if let Some(curve) = anime_assets.quaternion.get(&key) {
                    let anime = anime_contexts.quaternion.ctx.create_animation(0, AssetTypeFrameCurve::from(curve));
                    targetanimations.push(OpsAddTargetAnimation::ops(group, target, anime));
                }
            }
            EPropertyAnimationValueType::LocalScaling => {
                if let Some(curve) = anime_assets.scaling.get(&key) {
                    let anime = anime_contexts.scaling.ctx.create_animation(0, AssetTypeFrameCurve::from(curve));
                    targetanimations.push(OpsAddTargetAnimation::ops(group, target, anime));
                }
            }
            EPropertyAnimationValueType::IndicesRange => {
                if let Some(curve) = anime_assets.indicerange_curves.get(&key) {
                    let anime = anime_contexts.indices_range.ctx.create_animation(0, AssetTypeFrameCurve::from(curve));
                    targetanimations.push(OpsAddTargetAnimation::ops(group, target, anime));
                }
            }
            EPropertyAnimationValueType::Fov => {
                if let Some(curve) = anime_assets.camerafov.get(&key) {
                    let anime = anime_contexts.camerafov.ctx.create_animation(0, AssetTypeFrameCurve::from(curve));
                    targetanimations.push(OpsAddTargetAnimation::ops(group, target, anime));
                }
            }
            EPropertyAnimationValueType::OrthSize => {
                if let Some(curve) = anime_assets.camerasize.get(&key) {
                    let anime = anime_contexts.camerasize.ctx.create_animation(0, AssetTypeFrameCurve::from(curve));
                    targetanimations.push(OpsAddTargetAnimation::ops(group, target, anime));
                }
            }
            EPropertyAnimationValueType::Enable => {
                if let Some(curve) = anime_assets.enable.get(&key) {
                    let anime = anime_contexts.enable.ctx.create_animation(0, AssetTypeFrameCurve::from(curve));
                    targetanimations.push(OpsAddTargetAnimation::ops(group, target, anime));
                }
            }
        }
    });
}
