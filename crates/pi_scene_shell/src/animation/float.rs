
use pi_bevy_asset::{TAssetCapacity, AssetCapacity};
use pi_scene_math::Number;

use super::{TAnimatableComp, TAnimatableCompRecord};

#[derive(Clone, Copy)]
pub struct AnimatorableFloat(pub Number);
impl Default for AnimatorableFloat {
    fn default() -> Self {
        Self(0.0)
    }
}
impl pi_curves::curve::frame::FrameDataValue for AnimatorableFloat {
    fn interpolate(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 * (1.0 - amount) + rhs.0 * amount)
    }

    fn hermite(value1: &Self, tangent1: &Self, value2: &Self, tangent2: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue, frame_delta: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        let result = Number::hermite(&value1.0, &tangent1.0, &value2.0, &tangent2.0, amount, frame_delta);
        return Self(result);
    }

    fn append(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 + rhs.0 * amount)
    }
    fn size() -> usize {
        4
    }
}
impl TAssetCapacity for AnimatorableFloat {
    const ASSET_TYPE: &'static str = "AnimeFloat";
    fn capacity() -> AssetCapacity {
        AssetCapacity { flag: false, min: 200 * 1024, max: 400 * 1024, timeout: 1 * 60 * 1000 }
    }
}
impl TAnimatableComp for AnimatorableFloat {}


#[derive(Clone, Copy, Default)]
pub struct RecordAnimatorableFloat(pub AnimatorableFloat);
impl TAnimatableCompRecord<AnimatorableFloat> for RecordAnimatorableFloat {
    fn comp(&self) -> AnimatorableFloat {
        self.0.clone()
    }
}
