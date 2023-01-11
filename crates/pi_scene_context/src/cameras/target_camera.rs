
use pi_scene_math::{Vector3, Matrix, vector::{TToolVector3, TToolMatrix, TToolRotation}, coordiante_system::CoordinateSytem3, Isometry3, Number, Rotation3};

use crate::viewer::{TViewerViewMatrix, ViewerGlobalPosition, ViewerViewMatrix};


/// 通过 设置 target 目标点 调整相机
/// * 计算 节点 `rotation`, `local position`
/// * 计算 相机 `view matrix`, `global position`
pub struct TargetCameraParam {
    pub up: Vector3,
    pub target: Vector3,
    pub ignore_parent_scale: bool,
    pub dirty: bool,
}

impl Default for TargetCameraParam {
    fn default() -> Self {
        Self {
            target: Vector3::new(0., 0., 1.),
            up: CoordinateSytem3::up(),
            ignore_parent_scale: false,
            dirty: true,
        }
    }
}

impl TViewerViewMatrix for TargetCameraParam {
    fn view_matrix(&self, coordsys: &CoordinateSytem3, local_pos: &crate::transforms::transform_node::LocalPosition, parent: Option<&crate::transforms::transform_node::GlobalTransform>) -> (crate::viewer::ViewerViewMatrix, crate::viewer::ViewerGlobalPosition) {
        if self.ignore_parent_scale {
            match parent {
                Some(parent) => {
                    let transformation = &parent.matrix;
                    let mut eye = Vector3::zeros();
                    CoordinateSytem3::transform_coordinates(&local_pos.0, transformation, &mut eye);

                    let mut target = Vector3::zeros();
                    CoordinateSytem3::transform_coordinates(&self.target, transformation, &mut target);

                    let mut up = Vector3::zeros();
                    CoordinateSytem3::transform_coordinates(&self.up, transformation, &mut up);

                    let mut iso = Isometry3::identity();
                    coordsys.lookat(&eye, &target, &up, &mut iso);

                    (ViewerViewMatrix(iso.to_matrix()), ViewerGlobalPosition(eye))
                },
                None => {
                    let mut iso = Isometry3::identity();
                    let eye = local_pos.0.clone();
                    coordsys.lookat(&eye, &self.target, &self.up, &mut iso);
                    
                    (ViewerViewMatrix(iso.to_matrix()), ViewerGlobalPosition(eye))
                },
            }
        } else {
            let mut iso = Isometry3::identity();
            coordsys.lookat(&local_pos.0, &self.target, &self.up, &mut iso);
    
            let eye = match parent {
                Some(parent) => {
                    iso = iso.inv_mul(&parent.iso);
                    let temp = iso.translation.vector;
                    iso.inverse_mut();
                    temp
                },
                None => {
                    iso.translation.vector
                },
            };

            (ViewerViewMatrix(iso.to_matrix()), ViewerGlobalPosition(eye))
        }
    }
}

impl TargetCameraParam {
    pub fn calc_rotation(&self, coordsys: &CoordinateSytem3, l_p: &Vector3, rotation: &mut Rotation3) {
        let mut reference = Vector3::new(0., 0., 1.);

        let mut eye: Vector3 = l_p.clone();
        if eye.z == self.target.z {
            eye.z -= Number::EPSILON;
        }

        let dir = self.target - eye;
        let distance = CoordinateSytem3::length(&dir);

        reference.scale_mut(distance);

        let mut iso = Isometry3::identity();
        coordsys.lookat(&eye, &self.target, &CoordinateSytem3::up(), &mut iso);
        iso.inverse_mut();

        let mat: Matrix = iso.to_matrix();
        let values = mat.as_slice();

        let rx = if values[10] == 0. {
            0.
        } else {
            Number::atan(values[6] / values[10])
        };
        let ry = if dir.x == 0. {
            0.
        } else if dir.x > 0. {
            -(dir.z / dir.x).atan() + Number::to_radians(90.)
        } else {
            -(dir.z / dir.x).atan() - Number::to_radians(90.)
        };
        let rz = 0.;

        coordsys.rotation_matrix_mut_yaw_pitch_roll(ry, rx, rz, rotation);
    }
}