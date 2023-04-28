
use pi_animation::{animation::AnimationInfo, loop_mode::ELoopMode, amount::AnimationAmountCalc, animation_listener::{OnStart, OnFrameEvent, OnLoop, OnEnd}};

use pi_atom::Atom;
use pi_curves::{curve::{frame::{KeyFrameCurveValue, FrameDataValue}, FramePerSecond, FrameIndex, frame_curve::FrameCurve}, easing::EEasingMode};
use pi_engine_shell::prelude::*;

use crate::{flags::SceneID};

use super::base::*;

pub struct OpsAnimationGroupCreation(ObjectID, Atom);
impl OpsAnimationGroupCreation {
    pub fn ops(group_target: Entity, group_key: Atom) -> Self {
        Self(group_target, group_key)
    }
}
pub type ActionListAnimeGroupCreate = ActionList<OpsAnimationGroupCreation>;
pub fn sys_anime_group_create(
    mut cmds: ResMut<ActionListAnimeGroupCreate>,
    mut sce: Query<&mut SceneAnimationContext>,
    mut obj: Query<(&SceneID, &mut AnimationGroups)>,
) {
    // let mut list = 
    cmds.drain().drain(..).for_each(|OpsAnimationGroupCreation(id_obj, key_group)| {
        if let Ok((id_scene, mut groups)) = obj.get_mut(id_obj) {
            if groups.map.contains_key(&key_group) == false {
                if let Ok(mut ctx) = sce.get_mut(id_scene.0) {
                    let id_group = ctx.0.create_animation_group();
                    groups.map.insert(key_group, id_group);
                }
            }
        } else {
            cmds.push(OpsAnimationGroupCreation(id_obj, key_group))
        }
    });
}

pub type ActionListAnimePause = ActionList<(ObjectID, Atom)>;

pub fn sys_anime_pause(
    mut cmds: ResMut<ActionListAnimePause>,
    mut sce: Query<&mut SceneAnimationContext>,
    obj: Query<(&SceneID, & AnimationGroups)>,
) {
    cmds.drain().drain(..).for_each(|(id_obj, key_group)| {
        if let Ok((id_scene, groups)) = obj.get(id_obj) {
            if let Ok(mut ctx) = sce.get_mut(id_scene.0) {
                if let Some(id_group) = groups.map.get(&key_group) {
                    ctx.0.pause(id_group.clone());
                }
            }
        } else {
            cmds.push((id_obj, key_group))
        }
    });
}

pub struct OpsAnimationGroupStart(Entity, Atom, AnimationGroupParam);
impl OpsAnimationGroupStart {
    pub fn ops(group_target: Entity, group_key: Atom, param: AnimationGroupParam) -> Self {
        Self(group_target, group_key, param)
    }
}
pub type ActionListAnimeGroupStart = ActionList<OpsAnimationGroupStart>;
pub fn sys_anime_start(
    mut cmds: ResMut<ActionListAnimeGroupStart>,
    mut sce: Query<&mut SceneAnimationContext>,
    obj: Query<(&SceneID, & AnimationGroups)>,
) {
    cmds.drain().drain(..).for_each(|OpsAnimationGroupStart(id_obj, key_group, param)| {
        if let Ok((id_scene, groups)) = obj.get(id_obj) {
            if let Ok(mut ctx) = sce.get_mut(id_scene.0) {
                if let Some(id_group) = groups.map.get(&key_group) {
                    ctx.0.start_with_progress(id_group.clone(), param.speed, param.loop_mode, param.from, param.to, param.fps, param.amountcalc);
                }
            }
        } else {
            cmds.push(OpsAnimationGroupStart(id_obj, key_group, param))
        }
    });
}

pub struct OpsAddTargetAnimation(Entity, Entity, Atom, AnimationInfo);
impl OpsAddTargetAnimation {
    pub fn ops(group_target: Entity, anime_target: Entity, group_name: Atom, anime: AnimationInfo) -> Self {
        Self(group_target, anime_target, group_name, anime)
    }
}
pub type ActionListAddTargetAnime = ActionList<OpsAddTargetAnimation>;
pub fn sys_anime_add_target_anime(
    mut cmds: ResMut<ActionListAddTargetAnime>,
    mut sce: Query<&mut SceneAnimationContext>,
    obj: Query<(&SceneID, & AnimationGroups)>,
) {
    // log::warn!("AddTargetAnime");
    cmds.drain().drain(..).for_each(|OpsAddTargetAnimation(id_obj, id_target, key_group, animation)| {
        if let Ok((id_scene, groups)) = obj.get(id_obj) {
            // log::warn!("AddTargetAnime 1");
            if let Some(id_group) = groups.map.get(&key_group) {
                if let Ok(mut ctx) = sce.get_mut(id_scene.0) {
                    // log::warn!("AddTargetAnime Ok");
                    ctx.0.add_target_animation(animation, id_group.clone(), id_target);
                }
            }
        } else {
            cmds.push(OpsAddTargetAnimation(id_obj, id_target, key_group, animation))
        }
    });
}

pub struct AnimationGroupParam {
    speed: KeyFrameCurveValue,
    loop_mode: ELoopMode,
    from: KeyFrameCurveValue,
    to: KeyFrameCurveValue,
    fps: FramePerSecond,
    amountcalc: AnimationAmountCalc,
}
impl Default for AnimationGroupParam {
    fn default() -> Self {
        Self {
            speed: 1.0,
            loop_mode: ELoopMode::Positive(None),
            from: 0.,
            to: 1.,
            fps: 60,
            amountcalc: AnimationAmountCalc::from_easing(EEasingMode::None),
        }
    }
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
        id_obj: Entity,
        key_group: &Atom,
    ) {
        let mut cmds = app.world.get_resource_mut::<ActionListAnimeGroupCreate>().unwrap();
        cmds.push(OpsAnimationGroupCreation(id_obj, key_group.clone()));
    }
    pub fn pause_animation_group(
        app: &mut App,
        id_obj: Entity,
        key_group: &Atom,
    ) {
        let mut cmds = app.world.get_resource_mut::<ActionListAnimePause>().unwrap();
        cmds.push((id_obj, key_group.clone()));
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

pub enum EEventCommand {
    AddAnimationGroupFrameEvent(ObjectID, Atom, FrameIndex, Atom),
    ListenAnimationGroupStart(ObjectID, Atom, OnStart),
    ListenAnimationGroupFrame(ObjectID, Atom, OnFrameEvent<Atom>),
    ListenAnimationGroupLoop(ObjectID, Atom, OnLoop),
    ListenAnimationGroupEnd(ObjectID, Atom, OnEnd),
}