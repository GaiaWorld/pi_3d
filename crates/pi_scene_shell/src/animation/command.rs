
use derive_deref::Deref;
use pi_animation::{loop_mode::ELoopMode, amount::AnimationAmountCalc, animation::AnimationInfo};

use pi_curves::{curve::{frame::KeyFrameCurveValue, FramePerSecond}, easing::EEasingMode};
use pi_world::{single_res::SingleResMut, system_params::SystemParam, world::Entity};

// use bevy_ecs::{prelude::*, system::SystemParam};

use crate::prelude::ActionList;

use super::{base::*, float::AnimatorableFloat, vec2::AnimatorableVec2, vec3::AnimatorableVec3, vec4::AnimatorableVec4, uint::AnimatorableUint, int::AnimatorableSint};

#[derive( Deref)]
pub struct AnimatorableLink(pub(crate) Entity);

// #[derive(Component)]
pub struct TargetAnimatorableIsRunning;

pub struct OpsAnimatorableFloat(pub(crate) Entity, pub(crate) Entity, pub(crate) AnimatorableFloat, pub(crate) EAnimatorableEntityType);
impl OpsAnimatorableFloat {
    pub fn ops(target: Entity, linked: Entity, defualval: AnimatorableFloat, etype: EAnimatorableEntityType) -> Self {
        Self(target, linked, defualval, etype)
    }
}
pub type ActionListAnimatorableFloat = ActionList<OpsAnimatorableFloat>;

pub struct OpsAnimatorableVec2(pub(crate) Entity, pub(crate) Entity, pub(crate) AnimatorableVec2, pub(crate) EAnimatorableEntityType);
impl OpsAnimatorableVec2 {
    pub fn ops(target: Entity, linked: Entity, defualval: AnimatorableVec2, etype: EAnimatorableEntityType) -> Self {
        Self(target, linked, defualval, etype)
    }
}
pub type ActionListAnimatorableVec2 = ActionList<OpsAnimatorableVec2>;

pub struct OpsAnimatorableVec3(pub(crate) Entity, pub(crate) Entity, pub(crate) AnimatorableVec3, pub(crate) EAnimatorableEntityType);
impl OpsAnimatorableVec3 {
    pub fn ops(target: Entity, linked: Entity, defualval: AnimatorableVec3, etype: EAnimatorableEntityType) -> Self {
        Self(target, linked, defualval, etype)
    }
}
pub type ActionListAnimatorableVec3 = ActionList<OpsAnimatorableVec3>;

pub struct OpsAnimatorableVec4(pub(crate) Entity, pub(crate) Entity, pub(crate) AnimatorableVec4, pub(crate) EAnimatorableEntityType);
impl OpsAnimatorableVec4 {
    pub fn ops(target: Entity, linked: Entity, defualval: AnimatorableVec4, etype: EAnimatorableEntityType) -> Self {
        Self(target, linked, defualval, etype)
    }
}
pub type ActionListAnimatorableVec4 = ActionList<OpsAnimatorableVec4>;

// pub struct OpsAnimatorableMat4(pub(crate) Entity, pub(crate) Entity, pub(crate) AnimatorableMat4);
// impl OpsAnimatorableMat4 {
//     pub fn ops(target: Entity, linked: Entity, defualval: AnimatorableMat4) -> Self {
//         Self(target, linked, defualval)
//     }
// }
// pub type ActionListAnimatorableMat4 = ActionList<OpsAnimatorableMat4>;


pub struct OpsAnimatorableUint(pub(crate) Entity, pub(crate) Entity, pub(crate) AnimatorableUint, pub(crate) EAnimatorableEntityType);
impl OpsAnimatorableUint {
    pub fn ops(target: Entity, linked: Entity, defualval: AnimatorableUint, etype: EAnimatorableEntityType) -> Self {
        Self(target, linked, defualval, etype)
    }
}
pub type ActionListAnimatorableUint = ActionList<OpsAnimatorableUint>;

pub struct OpsAnimatorableSint(pub(crate) Entity, pub(crate) Entity, pub(crate) AnimatorableSint, pub(crate) EAnimatorableEntityType);
impl OpsAnimatorableSint {
    pub fn ops(target: Entity, linked: Entity, defualval: AnimatorableSint, etype: EAnimatorableEntityType) -> Self {
        Self(target, linked, defualval, etype)
    }
}
pub type ActionListAnimatorableSint = ActionList<OpsAnimatorableSint>;

// pub struct OpsAnimationGroupAttach(pub(crate) Entity, pub(crate) Entity);
// impl OpsAnimationGroupAttach {
//     pub fn ops(scene: Entity, group: Entity) -> Self {
//         Self(scene, group)
//     }
// }
// pub type ActionListAnimeGroupAttach = ActionList<OpsAnimationGroupAttach>;

pub struct OpsAnimationGroupCreation(pub(crate) Entity, pub(crate) Entity);
impl OpsAnimationGroupCreation {
    pub fn ops(idscene: Entity, id: Entity) -> Self {
        Self(idscene, id)
    }
}
pub type ActionListAnimeGroupCreate = ActionList<OpsAnimationGroupCreation>;

pub struct OpsAnimationGroupDispose(pub(crate) Entity);
impl OpsAnimationGroupDispose {
    pub fn ops(group: Entity) -> Self {
        Self(group)
    }
}
pub type ActionListAnimeGroupDispose = ActionList<OpsAnimationGroupDispose>;

/// 必须确保 这三个操作 顺序绝对正确 所以放入同一个列表
pub enum OpsAnimationGroupAction {
    Start(Entity, AnimationGroupParam, pi_animation::base::TimeMS, pi_animation::base::EFillMode),
    Pause(Entity),
    Stop(Entity)
}
pub type ActionListAnimationGroupAction = ActionList<OpsAnimationGroupAction>;

pub struct OpsAnimationGroupStartReset(pub(crate) Entity);
impl OpsAnimationGroupStartReset {
    pub fn ops(group: Entity) -> Self {
        Self(group)
    }
}
pub type ActionListAnimeGroupStartReset = ActionList<OpsAnimationGroupStartReset>;

pub struct OpsAddTargetAnimation(pub(crate) Entity, pub(crate) Entity, pub(crate) AnimationInfo);
impl OpsAddTargetAnimation {
    pub fn ops(group: Entity, anime_target: Entity, anime: AnimationInfo) -> Self {
        Self(group, anime_target, anime)
    }
}
pub type ActionListAddTargetAnime = ActionList<OpsAddTargetAnimation>;

pub struct OpsAddAnimationFrameEvent(pub(crate) Entity, pub(crate) f32, pub(crate) AnimeFrameEventData);
impl OpsAddAnimationFrameEvent {
    pub fn ops(group: Entity, progress: f32, data: AnimeFrameEventData) -> Self {
        Self(group, progress, data)
    }
}
pub type ActionListAddAnimationFrameEvent = ActionList<OpsAddAnimationFrameEvent>;

pub struct OpsAnimationWeight(pub(crate) Entity, pub(crate) f32);
impl OpsAnimationWeight {
    pub fn ops(group: Entity, weight: f32) -> Self {
        Self(group, weight)
    }
}
pub type ActionListAnimationWeight = ActionList<OpsAnimationWeight>;

pub enum OpsAddAnimationListen {
    Frame(Entity),
    Start(Entity),
    Loop(Entity),
    End(Entity),
}
pub type ActionListAddAnimationListen = ActionList<OpsAddAnimationListen>;

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
        fps: FramePerSecond,
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

// #[derive(SystemParam)]
pub struct ActionSetAnimationGroup<'w> {
    pub create: SingleResMut<'w, ActionListAnimeGroupCreate>,
    pub add_target_anime: SingleResMut<'w, ActionListAddTargetAnime>,
    pub action: SingleResMut<'w, ActionListAnimationGroupAction>,
    pub dispose: SingleResMut<'w, ActionListAnimeGroupDispose>,
    pub reset_while_start: SingleResMut<'w, ActionListAnimeGroupStartReset>,
    pub listens: SingleResMut<'w, ActionListAddAnimationListen>,
    pub frameevents: SingleResMut<'w, ActionListAddAnimationFrameEvent>,
    pub weight: SingleResMut<'w, ActionListAnimationWeight>,
}

// #[derive(SystemParam)]
pub struct ResourceAnimationGroup<'w> {
    pub global: SingleResMut<'w, GlobalAnimeAbout>,
    pub events: SingleResMut<'w, GlobalAnimeEvents>,
}