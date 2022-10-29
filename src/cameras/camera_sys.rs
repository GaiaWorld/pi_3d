use pi_ecs::{prelude::{Query, Id, ResMut}, query::With};
use pi_ecs_macros::setup;
use pi_ecs_utils::prelude::EntityTree;
use pi_render::rhi::dyn_uniform_buffer::DynUniformBuffer;
use pi_scene_math::{coordiante_system::CoordinateSytem3, Number};
use pi_slotmap_tree::Storage;
use log::trace;

use crate::{transforms::transform_node::{LocalTransform, GlobalTransform}, object::{GameObject, ObjectID}, cameras::{target_camera::TargetCameraParam, camera::{CameraParam, CameraRenderData}, free_camera::FreeCameraParam}, shaders::*};

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
    // #[system]
    // pub fn calc(
    //     mut query_cameras: Query<GameObject, (&TargetCameraParam, &LocalPosition, &mut LocalRotationMatrix)>,
    // ) {
    //     println!("Target Camera Control Calc:");
    //     let coordsys = CoordinateSytem3::left();
    //     for (target_camera, l_p, mut l_r) in query_cameras.iter_mut() {
    //         target_camera.calc_rotation(&coordsys, &l_p.0, &mut l_r.0);
    //     }
    // }
}

pub struct TargetCameraViewMatrixCalc;
#[setup]
impl TargetCameraViewMatrixCalc {
    #[system]
    pub fn calc(
        mut query_cameras: Query<GameObject, (ObjectID, &TargetCameraParam, &LocalTransform, &mut CameraRenderData)>,
        query_transforms: Query<GameObject, &GlobalTransform>,
        idtree: EntityTree<GameObject>,
    ) {
        println!("View Matrix Calc:");
        let coordsys = CoordinateSytem3::left();
        for (entity, target_camera, l_transform, mut camera_data) in query_cameras.iter_mut() {
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
            target_camera.view_matrix(&coordsys, &mut camera_data, &l_transform.position, p_m.as_ref(), p_iso.as_ref());
            println!("View Matrix {}", camera_data.view_matrix);
        }
    }
    // #[system]
    // pub fn calc(
    //     mut query_cameras: Query<GameObject, (ObjectID, &TargetCameraParam, &LocalPosition, &mut CameraGlobalPosition, &mut ViewMatrix)>,
    //     query_transforms: Query<GameObject, (&WorldMatrix, &GlobalIsometry)>,
    //     idtree: EntityTree<GameObject>,
    // ) {
    //     println!("View Matrix Calc:");
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

pub struct CameraTransformMatricCalc;
#[setup]
impl CameraTransformMatricCalc {
    #[system]
    pub fn calc(
        mut query_cameras: Query<GameObject, &mut CameraRenderData>,
    ) {
        println!("Transform Matrix Calc:");
        query_cameras.iter_mut().for_each(| mut camera | {
            // transform matrix
            camera.view_matrix.clone().mul_to(&camera.project_matrix.clone(), &mut camera.transform_matrix);
            println!("{}", camera.transform_matrix);
        });
    }
}