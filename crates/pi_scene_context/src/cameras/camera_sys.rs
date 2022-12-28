use pi_ecs::{prelude::{Query}, query::{Write, With, Or, Changed}};
use pi_ecs_macros::setup;
use pi_ecs_utils::prelude::EntityTree;
use pi_scene_math::{coordiante_system::CoordinateSytem3, Rotation3};
use pi_slotmap_tree::Storage;

use crate::{transforms::{transform_node::{GlobalTransform, LocalPosition, WorldMatrix, LocalRotation, }}, object::{GameObject, ObjectID}, cameras::{target_camera::TargetCameraParam}};

use super::{camera::{CameraParam, CameraViewMatrix, CameraGlobalPosition, CameraProjectionMatrix, CameraTransformMatrix}};

pub struct TargetCameraEffectLocalRotation;
#[setup]
impl TargetCameraEffectLocalRotation {
    #[system]
    pub fn calc(
        mut query_cameras: Query<GameObject, (&TargetCameraParam, &LocalPosition, Write<LocalRotation>)>,
    ) {
        //  println!("Target Camera Control Calc:");
        let coordsys = CoordinateSytem3::left();
        query_cameras.iter_mut().for_each(|(target_camera, lposition, mut lrotation)| {
            let mut rotation = Rotation3::identity();
            target_camera.calc_rotation(&coordsys, &lposition.0, &mut rotation);
            lrotation.write(LocalRotation(rotation));
        });
    }
}

pub struct TargetCameraViewMatrixCalc;
#[setup]
impl TargetCameraViewMatrixCalc {
    #[system]
    pub fn calc(
        mut query_cameras: Query<GameObject, (ObjectID, &CameraParam, &TargetCameraParam, &LocalPosition, &mut CameraViewMatrix, &mut CameraGlobalPosition), Or<(Changed<TargetCameraParam>, With<LocalPosition>)>>,
        query_transforms: Query<GameObject, &GlobalTransform>,
        dirty_globals: Query<GameObject, Changed<WorldMatrix>>,
        idtree: EntityTree<GameObject>,
    ) {
        //  println!("View Matrix Calc:");
        let coordsys = CoordinateSytem3::left();
        for (entity, camera, target_camera, l_position, mut camera_view, mut camera_pos) in query_cameras.iter_mut() {
            match idtree.get_up(entity) {
                Some(parent_id) => {
                    let parent_id = parent_id.parent();
                    let parent = query_transforms.get(parent_id);
                    match parent {
                        Some(parent) => {
                            if dirty_globals.get(parent_id).is_some() {
                                target_camera.view_matrix(&coordsys, &mut camera_view,  &mut camera_pos, &l_position.0, Some(&parent.matrix), Some(&parent.iso));
                            }
                        },
                        None => {
                            target_camera.view_matrix(&coordsys, &mut camera_view,  &mut camera_pos, &l_position.0, None, None);
                        },
                    }
                },
                None => {
                    
                },
            };
        }
    }
}


pub struct SysCameraProjectionCalc;
#[setup]
impl SysCameraProjectionCalc {
    #[system]
    pub fn calc(
        mut query_cameras: Query<GameObject, (&CameraParam, &mut CameraProjectionMatrix), Changed<CameraParam>>,
    ) {
        //  println!("Projection Matrix Calc:");
        query_cameras.iter_mut().for_each(|(camera, mut c_p_m)| {
            camera.project_matrix(&mut c_p_m, 1.0);
            //  println!("{}", c_p_m.0);
        });
    }
}

pub struct SysCameraTransformMatrix;
#[setup]
impl SysCameraTransformMatrix {
    #[system]
    pub fn calc(
        mut query_cameras: Query<GameObject, (&CameraViewMatrix, &CameraProjectionMatrix, &mut CameraTransformMatrix), Or<(Changed<CameraViewMatrix>, Changed<CameraTransformMatrix>)>>,
    ) {
        query_cameras.iter_mut().for_each(|(view_matrix, project_matrix, mut transform_matrix)| {
            // println!("SysCamera Transform Matrix: p = {:?}, v = {:?}", project_matrix.0, view_matrix.0);

            // view_matrix.0.
            // transform_matrix.0 = view_matrix.0 * project_matrix.0;
            // println!("Transform Matrix v * p {}", transform_matrix.0);
            transform_matrix.0 = project_matrix.0 * view_matrix.0;
            // println!("Transform Matrix p * v {}", transform_matrix.0);
            // view_matrix.0.mul_to(&project_matrix.0, &mut transform_matrix.0);
            // println!("Transform Matrix {}", transform_matrix.0);
            // project_matrix.0.mul_to(&view_matrix.0, &mut transform_matrix.0);
            // println!("Transform Matrix {}", transform_matrix.0);
            // project_matrix.0.mul_to(&view_matrix.0.transpose(), &mut transform_matrix.0);
            // println!("Transform Matrix {}", transform_matrix.0);
            // project_matrix.0.transpose().mul_to(&view_matrix.0, &mut transform_matrix.0);
            // println!("Transform Matrix {}", transform_matrix.0);
            // view_matrix.0.transpose().mul_to(&project_matrix.0, &mut transform_matrix.0);
            // println!("Transform Matrix {}", transform_matrix.0);
            // view_matrix.0.mul_to(&project_matrix.0.transpose(), &mut transform_matrix.0);
            // println!("Transform Matrix {}", transform_matrix.0);

            // let temp = Vector4::new(1., 1., 1., 1.);
            // println!(">>>>>>> {}", transform_matrix.0 * temp);
            // let temp = Vector4::new(1., 1., 2., 1.);
            // println!(">>>>>>> {}", transform_matrix.0 * temp);
            // let temp = Vector4::new(1., 1., -1., 1.);
            // println!(">>>>>>> {}", transform_matrix.0 * temp);
            
            // transform_matrix.0.transpose_mut();
        });
    }
}