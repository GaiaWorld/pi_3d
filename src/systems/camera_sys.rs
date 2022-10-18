use pi_ecs::{prelude::{Query, Id, ResMut}, query::With};
use pi_ecs_macros::setup;
use pi_ecs_utils::prelude::EntityTree;
use pi_render::rhi::dyn_uniform_buffer::DynUniformBuffer;
use pi_scene_math::{coordiante_system::CoordinateSytem3, Number};
use pi_slotmap_tree::Storage;
use log::trace;

use crate::{transforms::transform_node::{LocalTransform, GlobalTransform}, object::{GameObject, ObjectID}, cameras::{target_camera::TargetCameraParam, camera::{ViewMatrix, CameraGlobalPosition, ProjectionMatrix, CameraParam, TransformMatrix}, free_camera::FreeCameraParam}, shaders::*};

pub struct TargetCameraEffectLocalRotation;
#[setup]
impl TargetCameraEffectLocalRotation {
    #[system]
    pub fn cacl(
        mut query_cameras: Query<GameObject, (&TargetCameraParam, &mut LocalTransform)>,
    ) {
        println!("Target Camera Control Cacl:");
        let coordsys = CoordinateSytem3::left();
        for (target_camera, mut l_transform) in query_cameras.iter_mut() {
            let l_p = l_transform.position.clone();
            target_camera.cacl_rotation(&coordsys, &l_p, &mut l_transform.rotation);
        }
    }
    // #[system]
    // pub fn cacl(
    //     mut query_cameras: Query<GameObject, (&TargetCameraParam, &LocalPosition, &mut LocalRotationMatrix)>,
    // ) {
    //     println!("Target Camera Control Cacl:");
    //     let coordsys = CoordinateSytem3::left();
    //     for (target_camera, l_p, mut l_r) in query_cameras.iter_mut() {
    //         target_camera.cacl_rotation(&coordsys, &l_p.0, &mut l_r.0);
    //     }
    // }
}

pub struct TargetCameraViewMatrixCacl;
#[setup]
impl TargetCameraViewMatrixCacl {
    #[system]
    pub fn cacl(
        mut query_cameras: Query<GameObject, (ObjectID, &TargetCameraParam, &LocalTransform, Option<&mut CameraGlobalPosition>, &mut ViewMatrix)>,
        query_transforms: Query<GameObject, &GlobalTransform>,
        idtree: EntityTree<GameObject>,
    ) {
        println!("View Matrix Cacl:");
        let coordsys = CoordinateSytem3::left();
        for (entity, target_camera, l_transform, c_g_p, mut c_v_m) in query_cameras.iter_mut() {
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
            let mut temp = CameraGlobalPosition::default();
            match c_g_p {
                Some(mut c_g_p) => {
                    target_camera.view_matrix(&coordsys, &mut c_g_p, &mut c_v_m, &l_transform.position, p_m.as_ref(), p_iso.as_ref());
                },
                None => {
                    target_camera.view_matrix(&coordsys, &mut temp, &mut c_v_m, &l_transform.position, p_m.as_ref(), p_iso.as_ref());
                },
            }; 
            println!("{}", c_v_m.0);
        }
    }
    // #[system]
    // pub fn cacl(
    //     mut query_cameras: Query<GameObject, (ObjectID, &TargetCameraParam, &LocalPosition, &mut CameraGlobalPosition, &mut ViewMatrix)>,
    //     query_transforms: Query<GameObject, (&WorldMatrix, &GlobalIsometry)>,
    //     idtree: EntityTree<GameObject>,
    // ) {
    //     println!("View Matrix Cacl:");
    //     let coordsys = CoordinateSytem3::left();
    //     for (entity, target_camera, l_p, mut c_g_p, mut c_v_m) in query_cameras.iter_mut() {
    //         let (p_m, p_iso) = match idtree.get_up(entity) {
    //             Some(parent) => {
    //                 let parent = query_transforms.get(parent.parent());
    //                 match parent {
    //                     Some(parent) => (Some(parent.0), Some(parent.1)),
    //                     None => (None, None),
    //                 }
                    
    //             },
    //             None => (None, None),
    //         };
    //         target_camera.view_matrix(&coordsys, &mut c_g_p, &mut c_v_m, &l_p.0, p_m, p_iso);
    //         println!("{}", c_v_m.0);
    //     }
    // }
}

pub struct FreeCameraProjectionCacl;
#[setup]
impl FreeCameraProjectionCacl {
    #[system]
    pub fn cacl(
        mut query_cameras: Query<GameObject, (&CameraParam, &FreeCameraParam, &mut ProjectionMatrix)>,
    ) {
        println!("Projection Matrix Cacl:");
        query_cameras.iter_mut().for_each(|(camera, free_camera, mut c_p_m)| {
            free_camera.project_matrix(camera, &mut c_p_m, 1.0);
            println!("{}", c_p_m.0);
        });
    }
}

pub struct CameraTransformMatricCacl;
#[setup]
impl CameraTransformMatricCacl {
    #[system]
    pub fn cacl(
        mut query_cameras: Query<GameObject, (&ViewMatrix, &ProjectionMatrix, &mut TransformMatrix)>,
    ) {
        println!("Transform Matrix Cacl:");
        query_cameras.iter_mut().for_each(| (view_matrix, project_matrix, mut transform_matrix)| {
            // transform matrix
            view_matrix.0.mul_to(&project_matrix.0.clone(), &mut transform_matrix.0);
            println!("{}", transform_matrix.0);
        });
    }
}