use std::{sync::Arc, marker::PhantomData};

use pi_animation::{type_animation_context::{TTypeFrameCurve, TypeAnimationContext, AnimationContextAmount}, animation_result_pool::TypeAnimationResultPoolDefault, animation_group_manager::AnimationGroupManagerDefault, animation::AnimationInfo, target_animation::TargetAnimation, animation_group::AnimationGroupID};
use pi_assets::{asset::{Handle, GarbageEmpty}, mgr::AssetMgr};
use pi_atom::Atom;
use pi_curves::curve::{frame::{FrameDataValue, KeyFrameDataTypeAllocator}, frame_curve::FrameCurve};
use pi_ecs::prelude::{Query, ResMut, Component, Commands, Setup};
use pi_ecs_macros::setup;
use pi_engine_shell::{object::{ObjectID, GameObject}, run_stage::{TSystemStageInfo, ERunStageChap}, plugin::Plugin, setup};
use pi_hash::XHashMap;

use crate::scene::scene_time::SceneTime;

pub struct TypeFrameCurve<F: FrameDataValue+ 'static>(pub FrameCurve<F>);
impl<F: FrameDataValue+ 'static> pi_assets::asset::Asset for TypeFrameCurve<F> {
    type Key = Atom;

    fn size(&self) -> usize {
        F::size() * self.0.values.len() + 2 * self.0.frames.len() + self.0.size()
    }
}

pub struct AssetTypeFrameCurve<F: FrameDataValue+ 'static>(pub Handle<TypeFrameCurve<F>>);
impl<F: FrameDataValue+ 'static> From<Handle<TypeFrameCurve<F>>> for AssetTypeFrameCurve<F> {
    fn from(value: Handle<TypeFrameCurve<F>>) -> Self {
        Self(value)
    }
}
impl<F: FrameDataValue+ 'static> TTypeFrameCurve<F> for AssetTypeFrameCurve<F> {
    fn curve(&self) -> &FrameCurve<F> {
        &self.0.0
    }
}

pub struct TypeAnimeContext<D: FrameDataValue + 'static> {
    pub ctx: TypeAnimationContext<Atom, D, AssetTypeFrameCurve<D>>,
}

#[derive(Debug, Default)]
pub struct AnimationGroups {
    pub map: XHashMap<Atom, AnimationGroupID>,
}

pub struct GlobalAnimeAbout {
    pub ty_alloc: KeyFrameDataTypeAllocator,
    pub runtimeinfos: pi_animation::runtime_info::RuntimeInfoMap<ObjectID>,
}

pub struct SceneAnimationContext(pub(crate) AnimationContextAmount<ObjectID, AnimationGroupManagerDefault<ObjectID>>);
impl SceneAnimationContext {
    pub fn new() -> Self {
        Self(
            AnimationContextAmount::<ObjectID, AnimationGroupManagerDefault<ObjectID>>::default(
                AnimationGroupManagerDefault::<ObjectID>::default()
            )
        )
    }
}