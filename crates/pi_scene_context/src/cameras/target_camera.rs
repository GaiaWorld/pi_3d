use derive_deref::{Deref, DerefMut};
use pi_scene_shell::prelude::*;
use pi_scene_math::{Vector3, Matrix, vector::{TToolVector3, TToolMatrix, TToolRotation}, coordiante_system::CoordinateSytem3, Isometry3, Number, Rotation3};

use crate::{
    viewer::prelude::*,
    transforms::prelude::*,
};

#[derive(Deref, DerefMut, Component, Default)]
pub struct CameraUp(pub Vector3);

#[derive(Deref, DerefMut, Component, Default)]
pub struct CameraTarget(pub Vector3);

/// 通过 设置 target 目标点 调整相机
/// * 计算 节点 `rotation`, `local position`
/// * 计算 相机 `view matrix`, `global position`
#[derive(Clone, Component)]
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
            ignore_parent_scale: true,
            dirty: true,
        }
    }
}
impl TargetCameraParam {
    pub fn create(
        up: Vector3,
        target: Vector3,
    ) -> Self {
        Self {
            target,
            up,
            ignore_parent_scale: true,
            dirty: true,
        }
    }
}

impl TViewerViewMatrix for TargetCameraParam {
    fn view_matrix(&self, coordsys: &CoordinateSytem3, local_pos: &LocalPosition, parent: Option<(&GlobalMatrix, Isometry3)>) -> (ViewerViewMatrix, ViewerGlobalPosition) {
        let mut position = local_pos.0.clone();
        let initial_focal_distance = (self.target - position).metric_distance(&Vector3::zeros());
        if (position.z - self.target.z).abs() < 0.0001 {
            position.z += 0.0001;
        }

        let mut refrence_point = Vector3::new(0., 0., 1.); refrence_point.scale_mut(initial_focal_distance);
        let mut cam_matrix = Isometry3::identity();
        coordsys.lookat(&position, &self.target, &CoordinateSytem3::up(), &mut cam_matrix);
        cam_matrix.inverse_mut();

        let rx = cam_matrix.rotation.to_rotation_matrix();
        let _transformed_reference_point = rx * refrence_point;
        
        let mut target = position + _transformed_reference_point;
        let mut up = rx * Vector3::new(0.0, 1.0, 0.0);
        

        if self.ignore_parent_scale {
            match parent {
                Some(parent) => {
                    let transformation = &parent.0.matrix;
                    let mut eye = Vector3::zeros();
                    CoordinateSytem3::transform_coordinates(&position, transformation, &mut eye);
                    // log::warn!("local_pos: {:?}", local_pos);
                    // log::warn!("eye: {:?}", eye);

                    // let mut target = local_pos.0 + target;
                    // if self.target.normalize().dot(&self.up).abs() == 1. {
                    //     target += Vector3::new(0., 0., 0.001);
                    // }
                    CoordinateSytem3::transform_coordinates(&target.clone(), transformation, &mut target);
                    // log::warn!("target: {:?}", target);


                    // let mut up = Vector3::zeros();
                    CoordinateSytem3::transform_normal(&up.clone(), transformation, &mut up);
                    // log::warn!("up: {:?}", up);

                    let mut iso = Isometry3::identity();
                    coordsys.lookat(&eye, &target, &up, &mut iso);
                    // iso.translation.clone_from(&Translation3::new(eye.x, eye.y, eye.z));
                    // log::warn!("Camera : {:?}", (position, self.target, eye, target, up));

                    (ViewerViewMatrix(iso.to_matrix()), ViewerGlobalPosition(eye))
                },
                None => {
                    let mut iso = Isometry3::identity();
                    let eye = position.clone();
                    // let mut target = local_pos.0 + self.target;
                    // if self.target.normalize().dot(&self.up).abs() == 1. {
                    //     target += Vector3::new(0., 0., 0.001);
                    // }
                    coordsys.lookat(&eye, &target, &up, &mut iso);

                    // iso.translation.clone_from(&Translation3::new(local_pos.0.x, local_pos.0.y, local_pos.0.z));
                    
                    (ViewerViewMatrix(iso.to_matrix()), ViewerGlobalPosition(eye))
                },
            }
        } else {
            let mut iso = Isometry3::identity();
            coordsys.lookat(&position, &target, &up, &mut iso);
    
            let eye = match parent {
                Some(parent) => {
                    iso = iso.inv_mul(&parent.1);
                    iso.inverse_mut();

                    let mut eye = position.clone();
                    CoordinateSytem3::transform_coordinates(&position, &parent.0.matrix, &mut eye);
                    // log::warn!("local_pos: {:?}", local_pos);
                    // log::warn!("eye: {:?}", eye);

                    // let trans = Translation3::new(eye.x, eye.y, eye.z);
                    // iso.translation.clone_from(&trans);

                    eye
                },
                None => {
                    // iso.translation.clone_from(&Translation3::new(local_pos.0.x, local_pos.0.y, local_pos.0.z));
                    position.clone()
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

#[cfg(test)]
mod test {
    use pi_scene_math::{Isometry3, Point3, Vector3};


    #[test]
    fn test() {
        let eye = Point3::new(0., f32::sqrt(2.) / 2., f32::sqrt(2.) / 2.);
        let target = Point3::new(0., 0., 0.);
        let up = Vector3::new(0., 1., 0.);

        let iso = Isometry3::look_at_lh(&eye, &target, &up);

        println!("ISO {}", iso);
        println!("ISO {}", iso * eye);
    }
}