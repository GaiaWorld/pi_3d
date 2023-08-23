use pi_scene_math::{Number, Vector3, Matrix, coordiante_system::CoordinateSytem3, vector::*};


use super::target_camera::TargetCameraParam;


pub struct ArcRotateCamera {
    pub alpha: Number,
    pub beta: Number,
    pub gamma: Number,
    pub radius: Number,
    pub y_to_up: Matrix,
    pub up_to_y: Matrix,
}

impl Default for ArcRotateCamera {
    fn default() -> Self {
        Self { alpha: 0., beta: 0., gamma: 0., radius: 16., y_to_up: Matrix::identity(), up_to_y: Matrix::identity(), }
    }
}

impl ArcRotateCamera {
    pub fn apply(&self, camera: &TargetCameraParam) {
        let cosa = self.alpha.cos();
        let sina = self.alpha.sin();
        let cosb = self.beta.cos();
        let mut sinb = self.beta.sin();

        if sinb == 0. { sinb = 0.00001; };
        let radius = if self.radius == 0. { 0.0001 } else { self.radius };

        let mut computation_vec = Vector3::new(radius * cosa * sinb, radius * cosb, radius * sina * sinb);

        if camera.up.x != 0. || camera.up.y != 0. || camera.up.z != 0. {
            CoordinateSytem3::transform_coordinates(&computation_vec.clone(), &self.y_to_up, &mut computation_vec);
        }

        let mut position: Vector3 = camera.target + computation_vec;

    }
}