use pi_scene_math::{Vector3, Number, Perspective3, Orthographic3, Matrix};

use super::{camera::{Camera, CameraParam, ProjectionMatrix}, target_camera::TargetCameraParam};

pub enum EFovMode {
    VerticalFixed,
    HorizontalFixed,
}

pub enum EFreeCameraMode {
    Perspective,
    Orthograhic,
}

pub struct FreeCameraParam {
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
    fov: f32,
    fov_mode: EFovMode,
    mode: EFreeCameraMode,
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
    pub fn project_matrix(&self, camera: &CameraParam, c_p_m: &mut ProjectionMatrix, aspect: Number) {
        let fovy = match self.fov_mode {
            EFovMode::VerticalFixed => self.fov,
            EFovMode::HorizontalFixed => self.fov / aspect,
        };
        match self.mode {
            EFreeCameraMode::Perspective => {
                let p = Perspective3::new(aspect, fovy, camera.minz, camera.maxz);
                c_p_m.0.copy_from(p.as_matrix());
            },
            EFreeCameraMode::Orthograhic => {
                let p = Orthographic3::new(self.left, self.right, self.bottom, self.top, camera.minz, camera.maxz);
                c_p_m.0.copy_from(p.as_matrix());
            },
        };
    }
}
