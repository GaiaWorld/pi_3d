
use pi_scene_math::{Vector3, Number, coordiante_system::CoordinateSytem3, camera::{TPerspectiveCameraTool, TOrthographicCameraTool}};

use crate::{viewer::{TViewerProjectMatrix, ViewerProjectionMatrix}};


#[derive(Debug, Clone, Copy)]
pub struct Camera;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, Copy)]
pub enum EFixedMode {
    VerticalFixed,
    HorizontalFixed,
}

#[derive(Debug, Clone, Copy)]
pub enum EFreeCameraMode {
    Perspective,
    Orthograhic,
}

#[derive(Debug, Clone, Copy)]
pub struct CameraNearFar(pub Number, pub Number);

#[derive(Debug, Clone, Copy)]
pub struct CameraOrthograhicParam {
    pub left: Number,
    pub right: Number,
    pub top: Number,
    pub bottom: Number,
}

#[derive(Debug, Clone, Copy)]
pub struct CameraFov(pub Number);

#[derive(Debug, Clone, Copy)]
pub struct CameraOrthSize(pub Number);

#[derive(Debug, Clone)]
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
