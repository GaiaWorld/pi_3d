use bevy::ecs::component::Component;
use pi_bevy_asset::{TAssetCapacity, AssetCapacity};
use pi_scene_math::{Vector3, Number};

use super::{TAnimatableComp, TAnimatableCompRecord};

#[derive(Debug, Clone, Copy, Component)]
pub struct AnimatorableVec3(pub Vector3);
impl Default for AnimatorableVec3 {
    fn default() -> Self {
        Self(Vector3::new(0.0, 0.0, 0.0))
    }
}
impl From<&[Number; 3]> for AnimatorableVec3 {
    fn from(v: &[Number; 3]) -> Self {
        Self(Vector3::new(v[0], v[1], v[2]))
    }
}
impl pi_curves::curve::frame::FrameDataValue for AnimatorableVec3 {
    fn interpolate(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0.lerp(&rhs.0, amount))
    }

    fn hermite(value1: &Self, tangent1: &Self, value2: &Self, tangent2: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue, frame_delta: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        let result = Vector3::hermite(&value1.0, &tangent1.0, &value2.0, &tangent2.0, amount, frame_delta);
        return Self(result);
    }

    fn append(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 + rhs.0 * amount)
    }
    fn size() -> usize {
        4 * 3
    }
}
impl TAssetCapacity for AnimatorableVec3 {
    const ASSET_TYPE: &'static str = "AnimeVec3";
    fn capacity() -> AssetCapacity {
        AssetCapacity { flag: false, min: 200 * 1024, max: 400 * 1024, timeout: 1 * 60 * 1000 }
    }
}
impl TAnimatableComp for AnimatorableVec3 {}

#[derive(Clone, Copy, Component, Default)]
pub struct RecordAnimatorableVec3(pub AnimatorableVec3);
impl TAnimatableCompRecord<AnimatorableVec3> for RecordAnimatorableVec3 {
    fn comp(&self) -> AnimatorableVec3 {
        self.0.clone()
    }
}