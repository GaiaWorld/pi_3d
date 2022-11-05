use pi_ecs::{prelude::{Query}};
use pi_ecs_macros::setup;
use pi_ecs_utils::prelude::EntityTree;
use pi_scene_math::{coordiante_system::CoordinateSytem3, Vector4};
use pi_slotmap_tree::Storage;

use crate::{transforms::transform_node::{LocalTransform, GlobalTransform}, object::{GameObject, ObjectID}, cameras::{target_camera::TargetCameraParam, camera::{CameraRenderData}}};

use super::camera::{CameraParam, CameraViewMatrix, CameraGlobalPosition, CameraProjectionMatrix, CameraTransformMatrix};

pub struct TargetCameraEffectLocalRotation;
#[setup]
impl TargetCameraEffectLocalRotation {
    #[system]
    pub fn calc(
        mut query_cameras: Query<GameObject, (&TargetCameraParam, &mut LocalTransform)>,
    ) {
        println!("Target Camera Control Calc:");
        let coordsys = CoordinateSytem3::left();
        for (target_camera, mut l_transform) in query_cameras.iter_mut() {
            let l_p = l_transform.position.clone();
            target_camera.calc_rotation(&coordsys, &l_p, &mut l_transform.rotation);
        }
    }
}

pub struct TargetCameraViewMatrixCalc;
#[setup]
impl TargetCameraViewMatrixCalc {
    #[system]
    pub fn calc(
        mut query_cameras: Query<GameObject, (ObjectID, &CameraParam, &TargetCameraParam, &LocalTransform, &mut CameraViewMatrix, &mut CameraGlobalPosition)>,
        query_transforms: Query<GameObject, &GlobalTransform>,
        idtree: EntityTree<GameObject>,
    ) {
        println!("View Matrix Calc:");
        let coordsys = CoordinateSytem3::left();
        for (entity, camera, target_camera, l_transform, mut camera_view, mut camera_pos) in query_cameras.iter_mut() {
            let (p_m, p_iso) = match idtree.get_up(entity) {
                Some(parent) => {
                    let parent = query_transforms.get(parent.parent());
                    match parent {
                        Some(parent) => (Some(parent.matrix.clone()), Some(parent.iso.clone())),
                        None => (None, None),
                    }
                    
                },
                None => (None, None),
            };
            target_camera.view_matrix(&coordsys, &mut camera_view,  &mut camera_pos, &l_transform.position, p_m.as_ref(), p_iso.as_ref());
            println!("View Matrix {}", camera_view.0);
        }
    }
}


pub struct SysCameraProjectionCalc;
#[setup]
impl SysCameraProjectionCalc {
    #[system]
    pub fn calc(
        mut query_cameras: Query<GameObject, (&CameraParam, &mut CameraProjectionMatrix)>,
    ) {
        println!("Projection Matrix Calc:");
        query_cameras.iter_mut().for_each(|(camera, mut c_p_m)| {
            camera.project_matrix(&mut c_p_m, 1.0);
            println!("{}", c_p_m.0);
        });
    }
}

pub struct SysCameraTransformMatrix;
#[setup]
impl SysCameraTransformMatrix {
    #[system]
    pub fn calc(
        mut query_cameras: Query<GameObject, (&CameraViewMatrix, &CameraProjectionMatrix, &mut CameraTransformMatrix)>,
    ) {
        query_cameras.iter_mut().for_each(|(view_matrix, project_matrix, mut transform_matrix)| {
            
            println!("SysCamera Transform Matrix: p = {:?}, v = {:?}", project_matrix.0, view_matrix.0);

            // view_matrix.0.
            // transform_matrix.0 = view_matrix.0 * project_matrix.0;
            // println!("Transform Matrix v * p {}", transform_matrix.0);
            transform_matrix.0 = project_matrix.0 * view_matrix.0;
            println!("Transform Matrix p * v {}", transform_matrix.0);
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

            let temp = Vector4::new(1., 1., 1., 1.);
            println!(">>>>>>> {}", transform_matrix.0 * temp);
            let temp = Vector4::new(1., 1., 2., 1.);
            println!(">>>>>>> {}", transform_matrix.0 * temp);
            let temp = Vector4::new(1., 1., -1., 1.);
            println!(">>>>>>> {}", transform_matrix.0 * temp);
            
            transform_matrix.0.transpose_mut();
            
        });
    }
}