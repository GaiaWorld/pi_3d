
use pi_ecs::{prelude::{ResMut, EntityDelete, Query}, query::Write};
use pi_ecs_macros::setup;
use pi_scene_math::{Vector3, Matrix, vector::{TToolVector3, TToolMatrix, TToolRotation}, coordiante_system::CoordinateSytem3, Point3, Isometry3, Number, Rotation3};

use crate::object::{ObjectID, GameObject};

use super::camera::{CameraRenderData, CameraGlobalPosition, CameraViewMatrix};


/// 通过 设置 target 目标点 调整相机
/// * 计算 节点 `rotation`, `local position`
/// * 计算 相机 `view matrix`, `global position`
pub struct TargetCameraParam {
    pub up: Vector3,
    pub target: Vector3,
    pub ignore_parent_scale: bool,
}

impl Default for TargetCameraParam {
    fn default() -> Self {
        Self {
            target: Vector3::new(0., 0., 1.),
            up: CoordinateSytem3::up(),
            ignore_parent_scale: false,
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
    pub fn view_matrix(&self, coordsys: &CoordinateSytem3, view_matrix: &mut CameraViewMatrix, global_position: &mut CameraGlobalPosition, l_p: &Vector3, p_m: Option<&Matrix>, p_iso: Option<&Isometry3>) {
        if self.ignore_parent_scale {
            match p_m {
                Some(parent_world) => {
                    let transformation = parent_world;
                    let mut eye = Vector3::zeros();
                    CoordinateSytem3::transform_coordinates(l_p, transformation, &mut eye);
                    global_position.0.copy_from(&eye);

                    let mut target = Vector3::zeros();
                    CoordinateSytem3::transform_coordinates(&self.target, transformation, &mut target);

                    let mut up = Vector3::zeros();
                    CoordinateSytem3::transform_coordinates(&self.up, transformation, &mut up);

                    let mut iso = Isometry3::identity();
                    coordsys.lookat(&eye, &target, &up, &mut iso);

                    view_matrix.0.clone_from(&iso.to_matrix());
                },
                None => {
                    let mut iso = Isometry3::identity();
                    let eye = l_p;
                    coordsys.lookat(&eye, &self.target, &self.up, &mut iso);
                    
                    view_matrix.0.clone_from(&iso.to_matrix());
                    global_position.0.copy_from(&eye);
                },
            }
        } else {
            let mut iso = Isometry3::identity();
            let eye = l_p;
            coordsys.lookat(eye, &self.target, &self.up, &mut iso);
    
            match p_iso {
                Some(parent_iso) => {
                    iso = iso.inv_mul(parent_iso);
                    global_position.0 = iso.translation.vector;
                    iso.inverse_mut();
                },
                None => {
                    global_position.0 = iso.translation.vector;
                },
            }

            view_matrix.0.clone_from(&iso.to_matrix());
        }
    }
}


#[derive(Debug)]
pub enum TargetCameraCommand {
    Create(ObjectID),
    Destroy(ObjectID),
}


#[derive(Debug, Default)]
pub struct SingleTargetCameraCommandList {
    pub list: Vec<TargetCameraCommand>,
}

pub struct SysTargetCameraCommand;
#[setup]
impl SysTargetCameraCommand {
    #[system]
    pub fn cmds(
        mut cmds: ResMut<SingleTargetCameraCommandList>,
        mut cameras: Query<GameObject, Write<TargetCameraParam>>,
        mut entity_delete: EntityDelete<GameObject>,
    ) {
        cmds.list.drain(..).for_each(|cmd| {
            match cmd {
                TargetCameraCommand::Create(entity) => {
                    match cameras.get_mut(entity) {
                        Some(mut camera) => {
                            camera.insert_no_notify(TargetCameraParam::default());
                        },
                        None => todo!(),
                    }
                },
                TargetCameraCommand::Destroy(entity) => {
                    entity_delete.despawn(entity);
                },
            }
        });

    }
}