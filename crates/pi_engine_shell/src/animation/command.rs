
use pi_animation::{animation::AnimationInfo, loop_mode::ELoopMode, amount::AnimationAmountCalc};

use pi_atom::Atom;
use pi_bevy_asset::ShareAssetMgr;
use pi_curves::{curve::{frame::{KeyFrameCurveValue, FrameDataValue}, FramePerSecond}, easing::EEasingMode};
use pi_slotmap::DefaultKey;
use pi_bevy_ecs_extend::action::ActionList;

use bevy::{
    ecs::prelude::*, 
    ecs::system::{SystemParam}
};

use super::base::*;

#[derive(Clone)]
pub struct OpsAnimationGroupAttach(pub(crate) Entity, pub(crate) Entity, pub(crate) DefaultKey, pub(crate) u8);
impl OpsAnimationGroupAttach {
    pub fn ops(scene: Entity, group_target: Entity, id: DefaultKey) -> Self {
        Self(scene, group_target, id, 0)
    }
}
pub type ActionListAnimeGroupAttach = ActionList<OpsAnimationGroupAttach>;

// #[derive(Clone)]
// pub struct OpsAnimationGroupCreation(pub(crate) Entity, pub(crate) Atom, pub(crate) DefaultKey);
// impl OpsAnimationGroupCreation {
//     pub fn ops(group_target: Entity, group_key: Atom, id: DefaultKey) -> Self {
//         Self(group_target, group_key, id)
//     }
// }
// pub type ActionListAnimeGroupCreate = ActionList<OpsAnimationGroupCreation>;

// #[derive(Clone)]
// pub struct OpsAnimationGroupDispose(pub(crate) Entity, pub(crate) DefaultKey);
// impl OpsAnimationGroupDispose {
//     pub fn ops(group_target: Entity, id: DefaultKey) -> Self {
//         Self(group_target, id)
//     }
// }
// pub type ActionListAnimeGroupDispose = ActionList<OpsAnimationGroupDispose>;

// #[derive(Clone)]
// pub struct OpsAnimationGroupPause(pub(crate) Entity, pub(crate) DefaultKey);
// impl OpsAnimationGroupPause {
//     pub fn ops(group_target: Entity, group_key: DefaultKey) -> Self {
//         Self(group_target, group_key)
//     }
// }
// pub type ActionListAnimeGroupPause = ActionList<OpsAnimationGroupPause>;

// pub struct OpsAnimationGroupStart(pub(crate) Entity, pub(crate) DefaultKey, pub(crate) AnimationGroupParam);
// impl OpsAnimationGroupStart {
//     pub fn ops(group_target: Entity, group_key: DefaultKey, param: AnimationGroupParam) -> Self {
//         Self(group_target, group_key, param)
//     }
// }
// pub type ActionListAnimeGroupStart = ActionList<OpsAnimationGroupStart>;

// pub struct OpsAddTargetAnimation(pub(crate) Entity, pub(crate) Entity, pub(crate) DefaultKey, pub(crate) AnimationInfo);
// impl OpsAddTargetAnimation {
//     pub fn ops(group_target: Entity, anime_target: Entity, group_name: DefaultKey, anime: AnimationInfo) -> Self {
//         Self(group_target, anime_target, group_name, anime)
//     }
// }
// pub type ActionListAddTargetAnime = ActionList<OpsAddTargetAnimation>;

pub struct AnimationGroupParam {
    pub speed: KeyFrameCurveValue,
    pub loop_mode: ELoopMode,
    pub from: KeyFrameCurveValue,
    pub to: KeyFrameCurveValue,
    pub fps: FramePerSecond,
    pub amountcalc: AnimationAmountCalc,
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
impl AnimationGroupParam {
    pub fn new(
        speed: f32,
        loop_mode: ELoopMode,
        from: f32,
        to: f32,
        fps: u16,
        amountcalc: AnimationAmountCalc,
    ) -> Self {
        Self {
            speed,
            loop_mode,
            from,
            to,
            fps,
            amountcalc,
        }
    }
}

// pub enum EEventCommand {
//     AddAnimationGroupFrameEvent(Entity, Atom, FrameIndex, Atom),
//     ListenAnimationGroupStart(Entity, Atom, OnStart),
//     ListenAnimationGroupFrame(Entity, Atom, OnFrameEvent<Atom>),
//     ListenAnimationGroupLoop(Entity, Atom, OnLoop),
//     ListenAnimationGroupEnd(Entity, Atom, OnEnd),
// }

#[derive(SystemParam)]
pub struct ActionSetAnimationGroup<'w> {
    pub attach: ResMut<'w, ActionListAnimeGroupAttach>,
    // pub create: ResMut<'w, ActionListAnimeGroupCreate>,
    // pub add_target_anime: ResMut<'w, ActionListAddTargetAnime>,
    // pub start: ResMut<'w, ActionListAnimeGroupStart>,
    // pub pause: ResMut<'w, ActionListAnimeGroupPause>,
    pub scene_ctxs: ResMut<'w, SceneAnimationContextMap>,
    pub global: ResMut<'w, GlobalAnimeAbout>,
    pub events: ResMut<'w, GlobalAnimeEvents>,
}