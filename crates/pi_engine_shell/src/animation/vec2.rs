use bevy::ecs::component::Component;
use pi_bevy_asset::{TAssetCapacity, AssetCapacity};
use pi_scene_math::Vector2;

use super::{TAnimatableComp, TAnimatableCompRecord};

#[derive(Debug, Clone, Copy, Component)]
pub struct AnimatorableVec2(pub Vector2);
impl Default for AnimatorableVec2 {
    fn default() -> Self {
        Self(Vector2::new(0.0, 0.0))
    }
}
impl pi_curves::curve::frame::FrameDataValue for AnimatorableVec2 {
    fn interpolate(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0.lerp(&rhs.0, amount))
    }

    fn hermite(value1: &Self, tangent1: &Self, value2: &Self, tangent2: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue, frame_delta: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        let result = Vector2::hermite(&value1.0, &tangent1.0, &value2.0, &tangent2.0, amount, frame_delta);
        return Self(result);
    }

    fn append(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 + rhs.0 * amount)
    }
    fn size() -> usize {
        4
    }
}
impl TAssetCapacity for AnimatorableVec2 {
    const ASSET_TYPE: &'static str = "AnimeVec2";
    fn capacity() -> AssetCapacity {
        AssetCapacity { flag: false, min: 200 * 1024, max: 400 * 1024, timeout: 1 * 60 * 1000 }
    }
}
impl TAnimatableComp for AnimatorableVec2 {}

#[derive(Clone, Copy, Component, Default)]
pub struct RecordAnimatorableVec2(pub AnimatorableVec2);
impl TAnimatableCompRecord<AnimatorableVec2> for RecordAnimatorableVec2 {
    fn comp(&self) -> AnimatorableVec2 {
        self.0.clone()
    }
}