use pi_bevy_asset::TAssetCapacity;
use pi_engine_shell::prelude::*;
use pi_scene_math::{Vector3, Number, coordiante_system::CoordinateSytem3, camera::{TPerspectiveCameraTool, TOrthographicCameraTool}};

use crate::{viewer::prelude::*,};


#[derive(Debug, Clone, Copy, Component)]
pub struct Camera(pub bool);

#[derive(Debug, Clone, Component)]
pub struct CameraViewport {
    /// 0. ~ 1.
    pub x: f32,
    /// 0. ~ 1.
    pub y: f32,
    /// 0. ~ 1.
    pub w: f32,
    /// 0. ~ 1.
    pub h: f32,
    /// 0. ~ 1.
    pub mindepth: f32,
    /// 0. ~ 1.
    pub maxdepth: f32,
}
impl Default for CameraViewport {
    fn default() -> Self {
        Self {
            x: 0.,
            y: 0.,
            w: 1.,
            h: 1.,
            mindepth: -1.,
            maxdepth: 1.,
        }
    }
}

#[derive(Debug, Clone, Copy, Component, PartialEq, Eq)]
pub enum EFixedMode {
    VerticalFixed,
    HorizontalFixed,
}

#[derive(Debug, Clone, Copy, Component, PartialEq, Eq)]
pub enum EFreeCameraMode {
    Perspective,
    Orthograhic,
}

#[derive(Debug, Clone, Copy, Component)]
pub struct CameraNearFar(pub Number, pub Number);

#[derive(Debug, Clone, Copy, Component)]
pub struct CameraOrthograhicParam {
    pub left: Number,
    pub right: Number,
    pub top: Number,
    pub bottom: Number,
}

#[derive(Clone, Copy, Component, Default)]
pub struct RecordCameraFov(pub CameraFov);
impl TAnimatableCompRecord<CameraFov> for RecordCameraFov {
    fn comp(&self) -> CameraFov {
        self.0.clone()
    }
}

#[derive(Debug, Clone, Copy, Component, Deref, DerefMut)]
pub struct CameraFov(pub Number);
impl pi_curves::curve::frame::FrameDataValue for CameraFov {
    fn interpolate(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 * (1.0 - amount) + rhs.0 * amount)
    }

    fn hermite(value1: &Self, tangent1: &Self, value2: &Self, tangent2: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        let _1 = 1.;
        let _2 = 2.;
        let _3 = 3.;

        let squared = amount * amount;
        let cubed = amount * squared;
        let part1 = ((_2 * cubed) - (_3 * squared)) + _1;
        let part2 = (-_2 * cubed) + (_3 * squared);
        let part3 = (cubed - (_2 * squared)) + amount;
        let part4 = cubed - squared;

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

#[derive(Debug, Clone, Copy, Component, Deref, DerefMut)]
pub struct CameraOrthSize(pub Number);
impl pi_curves::curve::frame::FrameDataValue for CameraOrthSize {
    fn interpolate(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        Self(self.0 * (1.0 - amount) + rhs.0 * amount)
    }

    fn hermite(value1: &Self, tangent1: &Self, value2: &Self, tangent2: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        let _1 = 1.;
        let _2 = 2.;
        let _3 = 3.;

        let squared = amount * amount;
        let cubed = amount * squared;
        let part1 = ((_2 * cubed) - (_3 * squared)) + _1;
        let part2 = (-_2 * cubed) + (_3 * squared);
        let part3 = (cubed - (_2 * squared)) + amount;
        let part4 = cubed - squared;

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

#[derive(Debug, Clone, Copy, Component, Deref, DerefMut)]
pub struct CameraToScreen(pub bool);

#[derive(Debug, Clone, Component)]
pub struct CameraParam {
    pub up: Vector3,
    pub nearfar: CameraNearFar,
    pub orth: CameraOrthSize,
    pub fov: CameraFov,
    pub fixed_mode: EFixedMode,
    pub mode: EFreeCameraMode,
    pub viewport: CameraViewport,
}
impl CameraParam {
    pub fn create(
        mode: &EFreeCameraMode,
        fixed_mode: &EFixedMode,
        fov: &CameraFov,
        nearfar: &CameraNearFar,
        orth: &CameraOrthSize,
        viewport: &CameraViewport,
    ) -> Self {
        Self {
            up: Vector3::new(0., 1., 0.),
            nearfar: nearfar.clone(),
            orth: orth.clone(),
            fov: fov.clone(),
            fixed_mode: fixed_mode.clone(),
            mode: mode.clone(),
            viewport: viewport.clone(),
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
