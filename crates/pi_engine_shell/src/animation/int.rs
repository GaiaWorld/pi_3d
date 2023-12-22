use bevy::ecs::component::Component;
use pi_bevy_asset::{TAssetCapacity, AssetCapacity};
use pi_scene_math::Number;

use super::{TAnimatableComp, TAnimatableCompRecord};

#[derive(Debug, Clone, Copy, Component)]
pub struct AnimatorableInt(pub i32);
impl Default for AnimatorableInt {
    fn default() -> Self {
        Self(0)
    }
}
impl pi_curves::curve::frame::FrameDataValue for AnimatorableInt {
    fn interpolate(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        let mut result = self.0 as f32 * (1.0 - amount) + rhs.0 as f32 * amount;
        result = result.min(i32::MAX as f32).max(i32::MIN as f32);
        Self(result as i32)
    }

    fn hermite(value1: &Self, tangent1: &Self, value2: &Self, tangent2: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue, frame_delta: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        let mut result = Number::hermite(&(value1.0 as f32), &(tangent1.0 as f32), &(value2.0 as f32), &(tangent2.0 as f32), amount, frame_delta);
        result = result.min(i32::MAX as f32).max(i32::MIN as f32);
        return Self(result as i32);
    }

    fn append(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        let mut result = self.0 as f32 + (rhs.0 as f32) * amount;
        result = result.min(i32::MAX as f32).max(i32::MIN as f32);
        Self(result as i32)
    }
    fn size() -> usize {
        4
    }
}
impl TAssetCapacity for AnimatorableInt {
    const ASSET_TYPE: &'static str = "AnimeInt";
    fn capacity() -> AssetCapacity {
        AssetCapacity { flag: false, min: 200 * 1024, max: 400 * 1024, timeout: 1 * 60 * 1000 }
    }
}
impl TAnimatableComp for AnimatorableInt {}


#[derive(Clone, Copy, Component, Default)]
pub struct RecordAnimatorableInt(pub AnimatorableInt);
impl TAnimatableCompRecord<AnimatorableInt> for RecordAnimatorableInt {
    fn comp(&self) -> AnimatorableInt {
        self.0.clone()
    }
}
