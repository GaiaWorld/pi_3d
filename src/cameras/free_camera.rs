use pi_scene_math::{Vector3, Number, Perspective3, Orthographic3, Matrix};

use super::{camera::{Camera, CameraParam, CameraRenderData}, target_camera::TargetCameraParam};

#[derive(Debug, Clone, Copy)]
pub enum EFovMode {
    VerticalFixed,
    HorizontalFixed,
}

#[derive(Debug, Clone, Copy)]
pub enum EFreeCameraMode {
    Perspective,
    Orthograhic,
}

pub struct FreeCameraParam {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub fov: f32,
    pub fov_mode: EFovMode,
    pub mode: EFreeCameraMode,
}

impl Default for FreeCameraParam {
    fn default() -> Self {
        Self { 
            left: -10.,
            top: 10.,
            right: 10.,
            bottom: -10.,
            fov: Number::to_radians(75.0),
            fov_mode: EFovMode::VerticalFixed,
            mode: EFreeCameraMode::Perspective,
        }
    }
}

impl FreeCameraParam {
    const P0: Vector3 = Vector3::new(-1., -1., 0.);
    const P1: Vector3 = Vector3::new(-1., -1., 0.);
    const P2: Vector3 = Vector3::new(-1., -1., 0.);
    const P3: Vector3 = Vector3::new(-1., -1., 0.);
    const P4: Vector3 = Vector3::new(-1., -1., 0.);
    const P5: Vector3 = Vector3::new(-1., -1., 0.);
    const P6: Vector3 = Vector3::new(-1., -1., 0.);
    const P7: Vector3 = Vector3::new(-1., -1., 0.);

    ///
    /// * `aspect` width / height pixels ratio
    pub fn project_matrix(&self, camera: &CameraParam, c_p_m: &mut CameraRenderData, aspect: Number) {
        let fovy = match self.fov_mode {
            EFovMode::VerticalFixed => self.fov,
            EFovMode::HorizontalFixed => self.fov / aspect,
        };
        match self.mode {
            EFreeCameraMode::Perspective => {
                let p = Perspective3::new(aspect, fovy, camera.minz, camera.maxz);
                c_p_m.project_matrix.copy_from(p.as_matrix());
            },
            EFreeCameraMode::Orthograhic => {
                let p = Orthographic3::new(self.left, self.right, self.bottom, self.top, camera.minz, camera.maxz);
                c_p_m.project_matrix.copy_from(p.as_matrix());
            },
        };
    }
}
