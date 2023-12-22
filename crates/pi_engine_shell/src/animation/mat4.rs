// use bevy::ecs::component::Component;
// use pi_bevy_asset::{TAssetCapacity, AssetCapacity};
// use pi_scene_math::Matrix;

// use super::{TAnimatableComp, TAnimatableCompRecord};

// #[derive(Debug, Clone, Copy, Component)]
// pub struct AnimatorableMat4(pub Matrix);
// impl Default for AnimatorableMat4 {
//     fn default() -> Self {
//         Self(Matrix::identity())
//     }
// }
// impl pi_curves::curve::frame::FrameDataValue for AnimatorableMat4 {
//     fn interpolate(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
//         Self(self.0.lerp(&rhs.0, amount))
//     }

//     fn hermite(value1: &Self, tangent1: &Self, value2: &Self, tangent2: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue, frame_delta: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
//         let result = Matrix::hermite(&value1.0, &tangent1.0, &value2.0, &tangent2.0, amount, frame_delta);
//         return Self(result);
//     }

//     fn append(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
//         Self(self.0 + rhs.0 * amount)
//     }
//     fn size() -> usize {
//         4
//     }
// }
// impl TAssetCapacity for AnimatorableMat4 {
//     const ASSET_TYPE: &'static str = "AnimeMat4";
//     fn capacity() -> AssetCapacity {
//         AssetCapacity { flag: false, min: 200 * 1024, max: 400 * 1024, timeout: 1 * 60 * 1000 }
//     }
// }
// impl TAnimatableComp for AnimatorableMat4 {}

// #[derive(Clone, Copy, Component, Default)]
// pub struct RecordAnimatorableMat4(pub AnimatorableMat4);
// impl TAnimatableCompRecord<AnimatorableMat4> for RecordAnimatorableMat4 {
//     fn comp(&self) -> AnimatorableMat4 {
//         self.0.clone()
//     }
// }