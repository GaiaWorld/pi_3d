
use pi_animation::{type_animation_context::{TypeAnimationContext, AnimationContextAmount}, animation_group_manager::AnimationGroupManagerDefault, animation::AnimationInfo, animation_group::AnimationGroupID};
use pi_assets::{asset::{Handle}};
use pi_atom::Atom;
use pi_curves::curve::{frame::{FrameDataValue, KeyFrameDataTypeAllocator}, frame_curve::FrameCurve};
use pi_engine_shell::prelude::*;
use pi_hash::XHashMap;

use crate::{ flags::SceneID};

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
impl<F: FrameDataValue+ 'static> AsRef<FrameCurve<F>> for AssetTypeFrameCurve<F> {
    fn as_ref(&self) -> &FrameCurve<F> {
        &self.0.0
    }
}

#[derive(Resource)]
pub struct TypeAnimeContext<D: FrameDataValue + 'static> {
    pub ctx: TypeAnimationContext<D, AssetTypeFrameCurve<D>>,
}

#[derive(Debug, Default)]
pub struct AnimationGroups {
    pub map: XHashMap<Atom, AnimationGroupID>,
}

#[derive(Resource)]
pub struct GlobalAnimeAbout {
    pub ty_alloc: KeyFrameDataTypeAllocator,
    pub runtimeinfos: pi_animation::runtime_info::RuntimeInfoMap<ObjectID>,
    pub dispose_animations: Vec<AnimationInfo>,
    pub dispose_animationgroups: Vec<(SceneID, AnimationGroupID)>,
}

pub struct SceneAnimationContext(pub(crate) AnimationContextAmount<ObjectID, AnimationGroupManagerDefault<ObjectID>>, pub(crate) Vec<AnimationGroupID>);
impl SceneAnimationContext {
    pub fn new() -> Self {
        Self(
            AnimationContextAmount::<ObjectID, AnimationGroupManagerDefault<ObjectID>>::default(
                AnimationGroupManagerDefault::<ObjectID>::default()
            ),
            vec![]
        )
    }
}