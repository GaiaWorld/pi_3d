
use derive_deref::Deref;
use pi_animation::{loop_mode::ELoopMode, amount::AnimationAmountCalc};

use pi_curves::{curve::{frame::KeyFrameCurveValue, FramePerSecond}, easing::EEasingMode};
use pi_slotmap::DefaultKey;
// use pi_bevy_ecs_extend::action::ActionList;

use bevy::{
    ecs::prelude::*, 
    ecs::system::SystemParam
};

use crate::prelude::ActionList;

use super::{base::*, float::AnimatorableFloat, vec2::AnimatorableVec2, vec3::AnimatorableVec3, vec4::AnimatorableVec4, uint::AnimatorableUint, int::AnimatorableInt};

#[derive(Component, Deref)]
pub struct AnimatorableLink(pub(crate) Entity);

#[derive(Component)]
pub struct TargetAnimatorableIsRunning;

pub struct OpsAnimatorableFloat(pub(crate) Entity, pub(crate) Entity, pub(crate) AnimatorableFloat);
impl OpsAnimatorableFloat {
    pub fn ops(target: Entity, linked: Entity, defualval: AnimatorableFloat) -> Self {
        Self(target, linked, defualval)
    }
}
pub type ActionListAnimatorableFloat = ActionList<OpsAnimatorableFloat>;

pub struct OpsAnimatorableVec2(pub(crate) Entity, pub(crate) Entity, pub(crate) AnimatorableVec2);
impl OpsAnimatorableVec2 {
    pub fn ops(target: Entity, linked: Entity, defualval: AnimatorableVec2) -> Self {
        Self(target, linked, defualval)
    }
}
pub type ActionListAnimatorableVec2 = ActionList<OpsAnimatorableVec2>;

pub struct OpsAnimatorableVec3(pub(crate) Entity, pub(crate) Entity, pub(crate) AnimatorableVec3);
impl OpsAnimatorableVec3 {
    pub fn ops(target: Entity, linked: Entity, defualval: AnimatorableVec3) -> Self {
        Self(target, linked, defualval)
    }
}
pub type ActionListAnimatorableVec3 = ActionList<OpsAnimatorableVec3>;

pub struct OpsAnimatorableVec4(pub(crate) Entity, pub(crate) Entity, pub(crate) AnimatorableVec4);
impl OpsAnimatorableVec4 {
    pub fn ops(target: Entity, linked: Entity, defualval: AnimatorableVec4) -> Self {
        Self(target, linked, defualval)
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


pub struct OpsAnimatorableUint(pub(crate) Entity, pub(crate) Entity, pub(crate) AnimatorableUint);
impl OpsAnimatorableUint {
    pub fn ops(target: Entity, linked: Entity, defualval: AnimatorableUint) -> Self {
        Self(target, linked, defualval)
    }
}
pub type ActionListAnimatorableUint = ActionList<OpsAnimatorableUint>;

pub struct OpsAnimatorableInt(pub(crate) Entity, pub(crate) Entity, pub(crate) AnimatorableInt);
impl OpsAnimatorableInt {
    pub fn ops(target: Entity, linked: Entity, defualval: AnimatorableInt) -> Self {
        Self(target, linked, defualval)
    }
}
pub type ActionListAnimatorableInt = ActionList<OpsAnimatorableInt>;


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

pub struct OpsAnimationGroupStartReset(pub(crate) Entity, pub(crate) DefaultKey);
impl OpsAnimationGroupStartReset {
    pub fn ops(group_target: Entity, group_key: DefaultKey) -> Self {
        Self(group_target, group_key)
    }
}
pub type ActionListAnimeGroupStartReset = ActionList<OpsAnimationGroupStartReset>;

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

// pub enum EEventCommand {
//     AddAnimationGroupFrameEvent(Entity, Atom, FrameIndex, Atom),
//     ListenAnimationGroupStart(Entity, Atom, OnStart),
//     ListenAnimationGroupFrame(Entity, Atom, OnFrameEvent<Atom>),
//     ListenAnimationGroupLoop(Entity, Atom, OnLoop),
//     ListenAnimationGroupEnd(Entity, Atom, OnEnd),
// }

#[derive(SystemParam)]
pub struct ActionSetAnimationGroup<'w> {
    // pub create: ResMut<'w, ActionListAnimeGroupCreate>,
    // pub add_target_anime: ResMut<'w, ActionListAddTargetAnime>,
    // pub start: ResMut<'w, ActionListAnimeGroupStart>,
    // pub pause: ResMut<'w, ActionListAnimeGroupPause>,
    pub attach: ResMut<'w, ActionListAnimeGroupAttach>,
    pub reset_while_start: ResMut<'w, ActionListAnimeGroupStartReset>,
}

#[derive(SystemParam)]
pub struct ResourceAnimationGroup<'w> {
    pub scene_ctxs: ResMut<'w, SceneAnimationContextMap>,
    pub global: ResMut<'w, GlobalAnimeAbout>,
    pub events: ResMut<'w, GlobalAnimeEvents>,
}