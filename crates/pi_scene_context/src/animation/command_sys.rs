
use pi_animation::{animation::AnimationInfo, loop_mode::ELoopMode, amount::AnimationAmountCalc, animation_listener::{OnStart, OnFrameEvent, OnLoop, OnEnd}};

use pi_atom::Atom;
use pi_curves::{curve::{frame::{KeyFrameCurveValue, FrameDataValue}, FramePerSecond, FrameIndex, frame_curve::FrameCurve}, easing::EEasingMode};
use pi_engine_shell::prelude::*;
use pi_slotmap::DefaultKey;

use crate::{flags::SceneID};

use super::base::*;
use super::command::*;


pub fn sys_anime_group_create(
    mut cmds: ResMut<ActionListAnimeGroupCreate>,
    mut obj: Query<(&SceneID, &mut AnimationGroups)>,
) {
    // let mut list = 
    cmds.drain().drain(..).for_each(|OpsAnimationGroupCreation(id_obj, key_group, id_group)| {
        if let Ok((id_scene, mut groups)) = obj.get_mut(id_obj) {
            if groups.map.contains_key(&key_group) == false {
                groups.map.insert(key_group, id_group);
            }
        } else {
            cmds.push(OpsAnimationGroupCreation(id_obj, key_group, id_group))
        }
    });
}

pub fn sys_anime_pause(
    mut cmds: ResMut<ActionListAnimeGroupPause>,
    obj: Query<(&SceneID, & AnimationGroups)>,
    mut scenectxs: ResMut<SceneAnimationContextMap>,
) {
    cmds.drain().drain(..).for_each(|OpsAnimationGroupPause(id_obj, key_group)| {
        if let Ok((id_scene, groups)) = obj.get(id_obj) {
            if let Some(ctx) = scenectxs.get_mut(&id_scene.0) {
                if let Some(id_group) = groups.map.get(&key_group) {
                    ctx.0.pause(id_group.clone());
                }
            }
        } else {
            cmds.push(OpsAnimationGroupPause(id_obj, key_group))
        }
    });
}


pub fn sys_anime_start(
    mut cmds: ResMut<ActionListAnimeGroupStart>,
    obj: Query<(&SceneID, & AnimationGroups)>,
    mut scenectxs: ResMut<SceneAnimationContextMap>,
) {
    cmds.drain().drain(..).for_each(|OpsAnimationGroupStart(id_obj, key_group, param)| {
        if let Ok((id_scene, groups)) = obj.get(id_obj) {
            if let Some(ctx) = scenectxs.get_mut(&id_scene.0) {
                if let Some(id_group) = groups.map.get(&key_group) {
                    ctx.0.start_with_progress(id_group.clone(), param.speed, param.loop_mode, param.from, param.to, param.fps, param.amountcalc);
                }
            }
        } else {
            cmds.push(OpsAnimationGroupStart(id_obj, key_group, param))
        }
    });
}

pub fn sys_anime_add_target_anime(
    mut cmds: ResMut<ActionListAddTargetAnime>,
    obj: Query<(&SceneID, & AnimationGroups)>,
    mut scenectxs: ResMut<SceneAnimationContextMap>,
) {
    // log::warn!("AddTargetAnime");
    cmds.drain().drain(..).for_each(|OpsAddTargetAnimation(id_obj, id_target, key_group, animation)| {
        if let Ok((id_scene, groups)) = obj.get(id_obj) {
            // log::warn!("AddTargetAnime 1");
            if let Some(id_group) = groups.map.get(&key_group) {
                if let Some(ctx) = scenectxs.get_mut(&id_scene.0) {
                    // log::warn!("AddTargetAnime Ok");
                    ctx.0.add_target_animation(animation, id_group.clone(), id_target);
                }
            }
        } else {
            cmds.push(OpsAddTargetAnimation(id_obj, id_target, key_group, animation))
        }
    });
}

pub struct ActionAnime;
impl ActionAnime {
    pub(crate) fn as_anime_group_target(
        commands: &mut EntityCommands,
    ) {
        commands.insert(AnimationGroups::default());
    }
    pub fn create_animation_group(
        app: &mut App,
        id_scene: Entity,
        id_obj: Entity,
        key_group: &Atom,
        id_group: DefaultKey,
    ) {
        if let Some(id_group) = app.world.get_resource_mut::<SceneAnimationContextMap>().unwrap().create_group(id_scene) {
            app.world.get_resource_mut::<GlobalAnimeAbout>().unwrap().record_group(id_obj, key_group, id_group);
            let mut cmds = app.world.get_resource_mut::<ActionListAnimeGroupCreate>().unwrap();
            cmds.push(OpsAnimationGroupCreation(id_obj, key_group.clone(), id_group));
        }

    }
    pub fn pause_animation_group(
        app: &mut App,
        id_obj: Entity,
        key_group: &Atom,
    ) {
        let mut cmds = app.world.get_resource_mut::<ActionListAnimeGroupPause>().unwrap();
        cmds.push(OpsAnimationGroupPause(id_obj, key_group.clone()));
    }
    pub fn start_animation_group_percent(
        app: &mut App,
        id_obj: ObjectID,
        key_group: Atom,
        param: AnimationGroupParam,
    ) {
        let mut cmds = app.world.get_resource_mut::<ActionListAnimeGroupStart>().unwrap();
        cmds.push(OpsAnimationGroupStart(id_obj, key_group.clone(), param));
    }
    pub fn add_target_animation(
        app: &mut App,
        group_where: ObjectID,
        id_target: ObjectID,
        key_group: Atom,
        animation: AnimationInfo
    ) {
        let mut cmds = app.world.get_resource_mut::<ActionListAddTargetAnime>().unwrap();
        cmds.push(OpsAddTargetAnimation(group_where, id_target, key_group, animation));
    }

    pub fn create_animation<D: FrameDataValue + Component>(
        app: &mut App,
        curve: AssetTypeFrameCurve<D>,
    ) -> AnimationInfo {
        let mut type_ctx = app.world.get_resource_mut::<TypeAnimeContext<D>>().unwrap();
        type_ctx.ctx.create_animation(0, curve)
    }
    
    pub fn check_anim_curve<D: FrameDataValue + Component>(
        app: &mut App,
        key: &Atom,
    ) -> Option<AssetTypeFrameCurve<D>> {
        if let Some(value) = app.world.get_resource::<ShareAssetMgr<TypeFrameCurve<D>>>().unwrap().get(key) {
            Some(
                AssetTypeFrameCurve::<D>::from(value)
            )
        } else {
            None
        }
    }

    pub fn creat_anim_curve<D: FrameDataValue + Component>(
        app: &mut App,
        key: &Atom,
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
