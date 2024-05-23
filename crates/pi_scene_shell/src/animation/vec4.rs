
use pi_bevy_asset::{TAssetCapacity, AssetCapacity};
use pi_scene_math::{Vector4, Number};
use pi_world_macros::Component;

use super::{TAnimatableComp, TAnimatableCompRecord};

#[derive(Clone, Copy, Component)]
pub struct AnimatorableVec4(pub Vector4);
impl From<&[Number]> for AnimatorableVec4 {
    fn from(v: &[Number]) -> Self {
        Self(Vector4::new(v[0], v[1], v[2], v[3]))
    }
}
impl Default for AnimatorableVec4 {
    fn default() -> Self {
        Self(Vector4::new(0.0, 0.0, 0.0, 0.0))
    }
}
impl pi_curves::curve::frame::FrameDataValue for AnimatorableVec4 {
    fn interpolate(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0.lerp(&rhs.0, amount))
    }

    fn hermite(value1: &Self, tangent1: &Self, value2: &Self, tangent2: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue, frame_delta: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        let result = Vector4::hermite(&value1.0, &tangent1.0, &value2.0, &tangent2.0, amount, frame_delta);
        return Self(result);
    }

    fn append(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 + rhs.0 * amount)
    }
    fn size() -> usize {
        4 * 4
    }
}
impl TAssetCapacity for AnimatorableVec4 {
    const ASSET_TYPE: &'static str = "AnimeVec4";
    fn capacity() -> AssetCapacity {
        AssetCapacity { flag: false, min: 200 * 1024, max: 400 * 1024, timeout: 1 * 60 * 1000 }
    }
}
impl TAnimatableComp for AnimatorableVec4 {}

#[derive(Clone, Copy, Default, Component)]
pub struct RecordAnimatorableVec4(pub AnimatorableVec4);
impl TAnimatableCompRecord<AnimatorableVec4> for RecordAnimatorableVec4 {
    fn comp(&self) -> AnimatorableVec4 {
        self.0.clone()
    }
}