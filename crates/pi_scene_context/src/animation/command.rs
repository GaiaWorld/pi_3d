
use pi_animation::{animation::AnimationInfo, loop_mode::ELoopMode, amount::AnimationAmountCalc, animation_listener::{OnStart, OnFrameEvent, OnLoop, OnEnd}};

use pi_atom::Atom;
use pi_curves::{curve::{frame::{KeyFrameCurveValue, FrameDataValue}, FramePerSecond, FrameIndex, frame_curve::FrameCurve}, easing::EEasingMode};
use pi_engine_shell::prelude::*;
use pi_slotmap::DefaultKey;

use crate::{flags::SceneID};

use super::base::*;

#[derive(Clone)]
pub struct OpsAnimationGroupCreation(pub(crate) ObjectID, pub(crate) Atom, pub(crate) DefaultKey);
impl OpsAnimationGroupCreation {
    pub fn ops(group_target: Entity, group_key: Atom, id: DefaultKey) -> Self {
        Self(group_target, group_key, id)
    }
}
pub type ActionListAnimeGroupCreate = ActionList<OpsAnimationGroupCreation>;

#[derive(Clone)]
pub struct OpsAnimationGroupPause(pub(crate) ObjectID, pub(crate) Atom);
impl OpsAnimationGroupPause {
    pub fn ops(group_target: Entity, group_key: Atom) -> Self {
        Self(group_target, group_key)
    }
}
pub type ActionListAnimeGroupPause = ActionList<OpsAnimationGroupPause>;

pub struct OpsAnimationGroupStart(pub(crate) Entity, pub(crate) Atom, pub(crate) AnimationGroupParam);
impl OpsAnimationGroupStart {
    pub fn ops(group_target: Entity, group_key: Atom, param: AnimationGroupParam) -> Self {
        Self(group_target, group_key, param)
    }
}
pub type ActionListAnimeGroupStart = ActionList<OpsAnimationGroupStart>;

#[derive(Clone)]
pub struct OpsAddTargetAnimation(pub(crate) Entity, pub(crate) Entity, pub(crate) Atom, pub(crate) AnimationInfo);
impl OpsAddTargetAnimation {
    pub fn ops(group_target: Entity, anime_target: Entity, group_name: Atom, anime: AnimationInfo) -> Self {
        Self(group_target, anime_target, group_name, anime)
    }
}
pub type ActionListAddTargetAnime = ActionList<OpsAddTargetAnimation>;

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

pub enum EEventCommand {
    AddAnimationGroupFrameEvent(ObjectID, Atom, FrameIndex, Atom),
    ListenAnimationGroupStart(ObjectID, Atom, OnStart),
    ListenAnimationGroupFrame(ObjectID, Atom, OnFrameEvent<Atom>),
    ListenAnimationGroupLoop(ObjectID, Atom, OnLoop),
    ListenAnimationGroupEnd(ObjectID, Atom, OnEnd),
}