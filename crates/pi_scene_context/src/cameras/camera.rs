
use derive_deref::{Deref, DerefMut};
use pi_scene_shell::prelude::*;
use pi_scene_math::{Vector3, Number, coordiante_system::CoordinateSytem3, camera::{TPerspectiveCameraTool, TOrthographicCameraTool}};

use crate::viewer::prelude::*;


#[derive(Clone, Copy, Component, Default)]
pub struct Camera(pub bool);

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum EFixedMode {
    #[default]
    VerticalFixed,
    HorizontalFixed,
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum EFreeCameraMode {
    #[default]
    Perspective,
    Orthograhic,
}

#[derive(Clone, Copy)]
pub struct CameraNearFar(pub Number, pub Number);
impl Default for CameraNearFar {
    fn default() -> Self {
        Self(0.1, 200.1)
    }
}

#[derive(Clone, Copy, Component)]
pub struct CameraOrthograhicParam {
    pub left: Number,
    pub right: Number,
    pub top: Number,
    pub bottom: Number,
}
impl Default for CameraOrthograhicParam {
    fn default() -> Self {
        Self { left: -4., right: 4., top: 4., bottom: -4. }
    }
}

#[derive(Clone, Copy, Component, Default)]
pub struct RecordCameraFov(pub CameraFov);
impl TAnimatableCompRecord<CameraFov> for RecordCameraFov {
    fn comp(&self) -> CameraFov {
        self.0.clone()
    }
}

#[derive(Clone, Copy, Component, Deref, DerefMut)]
pub struct CameraFov(pub Number);
impl pi_curves::curve::frame::FrameDataValue for CameraFov {
    fn interpolate(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 * (1.0 - amount) + rhs.0 * amount)
    }

    fn hermite(value1: &Self, tangent1: &Self, value2: &Self, tangent2: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue, frame_delta: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        let _1 = 1.;
        let _2 = 2.;
        let _3 = 3.;

        let squared = amount * amount;
        let cubed = amount * squared;
        let part1 = ((_2 * cubed) - (_3 * squared)) + _1;
        let part2 = (-_2 * cubed) + (_3 * squared);
        let part3 = ((cubed - (_2 * squared)) + amount) * frame_delta;
        let part4 = (cubed - squared) * frame_delta;

        let result = (((value1.0 * part1) + (value2.0 * part2)) + (tangent1.0 * part3)) + (tangent2.0 * part4);
        return Self(result);
    }

    fn append(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 + rhs.0 * amount)
    }
    fn size() -> usize {
        1 * 4
    }
}
impl Default for CameraFov {
    fn default() -> Self {
        Self(0.7)
    }
}
impl TAssetCapacity for CameraFov {
    const ASSET_TYPE: &'static str = "AnimeCurveCameraFov";
    fn capacity() -> AssetCapacity {
        AssetCapacity { flag: false, min: 500 * 1024 , max: 1024 * 1024, timeout: 1 * 60 * 1000 }
    }
}
impl TAnimatableComp for CameraFov {}


#[derive(Clone, Copy, Component, Default)]
pub struct RecordCameraOrthSize(pub CameraOrthSize);
impl TAnimatableCompRecord<CameraOrthSize> for RecordCameraOrthSize {
    fn comp(&self) -> CameraOrthSize {
        self.0.clone()
    }
}

#[derive(Clone, Copy, Component, Deref, DerefMut)]
pub struct CameraOrthSize(pub Number);
impl pi_curves::curve::frame::FrameDataValue for CameraOrthSize {
    fn interpolate(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 * (1.0 - amount) + rhs.0 * amount)
    }

    fn hermite(value1: &Self, tangent1: &Self, value2: &Self, tangent2: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue, frame_delta: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        let _1 = 1.;
        let _2 = 2.;
        let _3 = 3.;

        let squared = amount * amount;
        let cubed = amount * squared;
        let part1 = ((_2 * cubed) - (_3 * squared)) + _1;
        let part2 = (-_2 * cubed) + (_3 * squared);
        let part3 = ((cubed - (_2 * squared)) + amount) * frame_delta;
        let part4 = (cubed - squared) * frame_delta;

        let result = (((value1.0 * part1) + (value2.0 * part2)) + (tangent1.0 * part3)) + (tangent2.0 * part4);
        return Self(result);
    }

    fn append(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 + rhs.0 * amount)
    }
    fn size() -> usize {
        1 * 4
    }
}
impl Default for CameraOrthSize {
    fn default() -> Self {
        Self(4.)
    }
}
impl TAssetCapacity for CameraOrthSize {
    const ASSET_TYPE: &'static str = "AnimeCurveCameraOrthSize";
    fn capacity() -> AssetCapacity {
        AssetCapacity { flag: false, min: 500 * 1024 , max: 1024 * 1024, timeout: 1 * 60 * 1000 }
    }
}
impl TAnimatableComp for CameraOrthSize {}

#[derive(Clone, Component)]
pub struct CameraParam {
    pub up: Vector3,
    pub nearfar: CameraNearFar,
    pub orth: CameraOrthSize,
    pub fov: CameraFov,
    pub fixed_mode: EFixedMode,
    pub mode: EFreeCameraMode,
}
impl Default for CameraParam {
    fn default() -> Self {
        Self {
            up: Vector3::new(0., 1., 0.),
            nearfar: CameraNearFar::default(),
            orth: CameraOrthSize::default(),
            fov: CameraFov::default(),
            fixed_mode: EFixedMode::default(),
            mode: EFreeCameraMode::default(),
        }
    }
}
impl CameraParam {
    pub fn create(
        mode: &EFreeCameraMode,
        fixed_mode: &EFixedMode,
        fov: &CameraFov,
        nearfar: &CameraNearFar,
        orth: &CameraOrthSize,
    ) -> Self {
        Self {
            up: Vector3::new(0., 1., 0.),
            nearfar: nearfar.clone(),
            orth: orth.clone(),
            fov: fov.clone(),
            fixed_mode: fixed_mode.clone(),
            mode: mode.clone(),
        }
    }
}
impl TViewerProjectMatrix for CameraParam {
    fn project_matrix(&self, aspect: Number) -> ViewerProjectionMatrix {
        match self.mode {
            EFreeCameraMode::Perspective => {
                let m = match self.fixed_mode {
                    EFixedMode::VerticalFixed => CoordinateSytem3::perspective_lh(self.fov.0, aspect, self.nearfar.0, self.nearfar.1, true),
                    EFixedMode::HorizontalFixed => CoordinateSytem3::perspective_lh(self.fov.0, aspect, self.nearfar.0, self.nearfar.1, false),
                };
                ViewerProjectionMatrix(m)
            },
            EFreeCameraMode::Orthograhic => {
                let value = self.orth.0;
                let m = match self.fixed_mode {
                    EFixedMode::VerticalFixed => {
                        let left = -value * aspect;
                        let right = value * aspect;
                        let top = value;
                        let bottom = -value;
                        CoordinateSytem3::orthographic_lh(left, right, bottom, top, self.nearfar.0, self.nearfar.1)
                    },
                    EFixedMode::HorizontalFixed => {
                        let left = -value;
                        let right = value;
                        let top = value / aspect;
                        let bottom = -value / aspect;
                        CoordinateSytem3::orthographic_lh(left, right, bottom, top, self.nearfar.0, self.nearfar.1)
                    },
                };
                ViewerProjectionMatrix(m)
            },
        }
    }
}